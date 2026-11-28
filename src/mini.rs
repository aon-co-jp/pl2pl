//! `pl2pl c2f` / `rust2c` / `rust2f`: 単純な手続き(main の中の変数・for・while・if・出力)を
//! 小さな中間表現 `St` に読み、C または Fortran へ書き出す最初の一歩。
//! 読めるもの: C(int/long/double/bool の宣言、`for (int i = a; i < b; i++)` 形、while、if/else、printf の %d %ld %lld %f %.Nf %g %s %c)、
//! Rust(`fn main`、`let [mut] x[: i32|i64|f64|bool] = ...`、`for i in a..b` / `a..=b`、while、if/else、`println!`/`print!` の `{}` `{:N}` `{:.N}`)。
//! わからない行は黙って捨てず TODO として残し、レポートにも載せる。関数・配列・構造体・ポインタ・文字列変数は未対応。
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ty {
    Int,
    Long,
    Double,
    Bool,
    Str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Part {
    Lit(String),
    Val { expr: String, ty: Ty, width: Option<usize>, prec: Option<usize>, g: bool },
}

/// 式はすべて C の書き方(`&&` `||` `!` `!=` `%`、`(double)(x)`、`true`/`false`)で持つ。
#[derive(Clone, Debug, PartialEq)]
pub enum St {
    Decl { name: String, ty: Ty, init: Option<String> },
    Assign { lhs: String, op: String, rhs: String },
    For { var: String, from: String, to: String, incl: bool, down: bool, step: String },
    While(String),
    If(String),
    ElseIf(String),
    Else,
    End,
    Print { parts: Vec<Part>, nl: bool },
    Todo(String),
}

fn todo(out: &mut Vec<St>, rep: &mut Vec<String>, tag: &str, name: &str, i: usize, l: &str) {
    out.push(St::Todo(l.to_string()));
    rep.push(format!("{name}:{}: 未対応({tag}) `{l}`", i + 1));
}

fn split_args(s: &str) -> Vec<String> {
    let (mut v, mut cur, mut d, mut q) = (vec![], String::new(), 0i32, false);
    let mut prev = ' ';
    for c in s.chars() {
        if c == '"' && prev != '\\' {
            q = !q;
        }
        if !q && (c == '(' || c == '[') {
            d += 1;
        }
        if !q && (c == ')' || c == ']') {
            d -= 1;
        }
        if c == ',' && d == 0 && !q {
            v.push(cur.trim().to_string());
            cur.clear();
        } else {
            cur.push(c);
        }
        prev = c;
    }
    if !cur.trim().is_empty() {
        v.push(cur.trim().to_string());
    }
    v
}

/// "...\n..." の文字列を、改行ごとに Print へ区切るための下ごしらえ。
fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"").replace("\\\\", "\\")
}

fn push_prints(out: &mut Vec<St>, parts: Vec<Part>, nl_end: bool) {
    // Lit の中の改行で区切る
    let mut cur: Vec<Part> = vec![];
    for p in parts {
        match p {
            Part::Lit(t) => {
                let segs: Vec<&str> = t.split('\n').collect();
                for (k, s) in segs.iter().enumerate() {
                    if !s.is_empty() {
                        cur.push(Part::Lit(s.to_string()));
                    }
                    if k + 1 < segs.len() {
                        out.push(St::Print { parts: std::mem::take(&mut cur), nl: true });
                    }
                }
            }
            v => cur.push(v),
        }
    }
    if !cur.is_empty() || nl_end {
        out.push(St::Print { parts: cur, nl: nl_end });
    }
}

fn block_line(l: &str, out: &mut Vec<St>) -> Option<String> {
    // 先頭の `}` を End/Else に。残りを返す。
    let t = l.trim();
    if let Some(r) = t.strip_prefix('}') {
        let r = r.trim();
        if r.is_empty() {
            out.push(St::End);
            return None;
        }
        if r.starts_with("else") {
            return Some(format!("}}{r}"));
        }
        out.push(St::End);
        return Some(r.to_string());
    }
    Some(t.to_string())
}

// ---------------- C → 中間表現 ----------------

pub fn parse_c(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<St> {
    let mainre = Regex::new(r"^int\s+main\s*\(\s*(void)?\s*\)\s*\{$").unwrap();
    let decl = Regex::new(r"^(int|long long|long|double|float|bool|_Bool)\s+([^;(]+);$").unwrap();
    let forre = Regex::new(r"^for\s*\(\s*(?:int\s+|long\s+)?(\w+)\s*=\s*([^;]+);\s*(\w+)\s*(<=|<|>=|>)\s*([^;]+);\s*(\w+)\s*(\+\+|--|\+=\s*[^)]+|-=\s*[^)]+)\s*\)\s*\{$").unwrap();
    let whilere = Regex::new(r"^while\s*\((.*)\)\s*\{$").unwrap();
    let ifre = Regex::new(r"^(\}\s*else\s+)?if\s*\((.*)\)\s*\{$").unwrap();
    let elsere = Regex::new(r"^\}\s*else\s*\{$").unwrap();
    let printf = Regex::new(r#"^printf\s*\(\s*"((?:[^"\\]|\\.)*)"\s*(?:,(.*))?\)\s*;$"#).unwrap();
    let puts = Regex::new(r#"^puts\s*\(\s*"((?:[^"\\]|\\.)*)"\s*\)\s*;$"#).unwrap();
    let assign = Regex::new(r"^([A-Za-z_]\w*)\s*(\+|-|\*|/|%)?=\s*([^=].*);$").unwrap();
    let incdec = Regex::new(r"^(?:(\w+)\s*(\+\+|--)|(\+\+|--)\s*(\w+))\s*;$").unwrap();
    let spec = Regex::new(r"%(-?\d+)?(?:\.(\d+))?(lld|ld|l?f|d|i|g|s|c|%)").unwrap();
    let mut out = vec![];
    let mut in_main = false;
    let mut depth = 0usize;
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split("//").next().unwrap_or("").trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with("/*") || l.starts_with('*') {
            continue;
        }
        if !in_main {
            if mainre.is_match(l) {
                in_main = true;
                depth = 1;
            } else {
                rep.push(format!("{name}:{}: main の外は未対応 `{l}`", i + 1));
            }
            continue;
        }
        let Some(l) = block_line(l, &mut out) else {
            depth -= 1;
            if depth == 0 {
                out.pop();
                in_main = false;
            }
            continue;
        };
        if l.starts_with("return") {
            continue;
        }
        if let Some(c) = forre.captures(&l) {
            let (v, a, cmp, b, s) = (&c[1], c[2].trim(), &c[4], c[5].trim(), c[7].trim());
            if c[3] != *v || c[6] != *v {
                todo(&mut out, rep, "for の形", name, i, &l);
                continue;
            }
            let down = cmp.starts_with('>');
            let step = match s {
                "++" | "--" => "1".to_string(),
                _ => s[2..].trim().to_string(),
            };
            out.push(St::For { var: v.into(), from: a.into(), to: b.into(), incl: cmp.ends_with('='), down, step });
            depth += 1;
        } else if let Some(c) = ifre.captures(&l) {
            if c.get(1).is_some() {
                out.push(St::ElseIf(c[2].trim().into()));
            } else {
                out.push(St::If(c[2].trim().into()));
                depth += 1;
            }
        } else if elsere.is_match(&l) {
            out.push(St::Else);
        } else if let Some(c) = whilere.captures(&l) {
            out.push(St::While(c[1].trim().into()));
            depth += 1;
        } else if let Some(c) = decl.captures(&l) {
            let ty = match &c[1] {
                "int" => Ty::Int,
                "long" | "long long" => Ty::Long,
                "bool" | "_Bool" => Ty::Bool,
                _ => Ty::Double,
            };
            for v in split_args(&c[2]) {
                let (n, init) = match v.split_once('=') {
                    Some((a, b)) => (a.trim().to_string(), Some(b.trim().to_string())),
                    None => (v.trim().to_string(), None),
                };
                if n.contains('[') || n.contains('*') {
                    todo(&mut out, rep, "配列・ポインタ", name, i, &l);
                    break;
                }
                out.push(St::Decl { name: n, ty, init });
            }
        } else if let Some(c) = printf.captures(&l) {
            let fmt = &c[1];
            let args = c.get(2).map(|m| split_args(m.as_str())).unwrap_or_default();
            let (mut parts, mut last, mut k, mut bad) = (vec![], 0, 0, false);
            for m in spec.captures_iter(fmt) {
                let w = m.get(0).unwrap();
                parts.push(Part::Lit(unescape(&fmt[last..w.start()])));
                last = w.end();
                let conv = &m[3];
                if conv == "%" {
                    parts.push(Part::Lit("%".into()));
                    continue;
                }
                let Some(a) = args.get(k) else {
                    bad = true;
                    break;
                };
                k += 1;
                let width = m.get(1).and_then(|x| x.as_str().parse::<i64>().ok());
                if width.map(|x| x < 0).unwrap_or(false) {
                    bad = true; // 左寄せは未対応
                }
                let ty = match conv {
                    "d" | "i" => Ty::Int,
                    "ld" | "lld" => Ty::Long,
                    "s" | "c" => Ty::Str,
                    _ => Ty::Double,
                };
                if conv == "c" || (conv == "s" && !a.starts_with('"')) {
                    bad = true;
                }
                let prec = m.get(2).and_then(|x| x.as_str().parse().ok()).or(if conv.ends_with('f') { Some(6) } else { None });
                parts.push(Part::Val { expr: a.clone(), ty, width: width.map(|x| x as usize), prec, g: conv == "g" });
            }
            parts.push(Part::Lit(unescape(&fmt[last..])));
            if bad || k != args.len() {
                todo(&mut out, rep, "printf の書式", name, i, &l);
                continue;
            }
            push_prints(&mut out, parts, false);
        } else if let Some(c) = puts.captures(&l) {
            push_prints(&mut out, vec![Part::Lit(unescape(&c[1]))], true);
        } else if let Some(c) = incdec.captures(&l) {
            let (v, op) = match c.get(1) {
                Some(v) => (v.as_str(), &c[2]),
                None => (&c[4], &c[3]),
            };
            out.push(St::Assign { lhs: v.into(), op: op[..1].into(), rhs: "1".into() });
        } else if let Some(c) = assign.captures(&l) {
            out.push(St::Assign { lhs: c[1].into(), op: c.get(2).map(|m| m.as_str().to_string()).unwrap_or_default(), rhs: c[3].trim().into() });
        } else {
            todo(&mut out, rep, "C", name, i, &l);
        }
    }
    out
}

// ---------------- Rust → 中間表現 ----------------

fn rust_expr(e: &str) -> String {
    let cast = Regex::new(r"(\([^()]*\)|[A-Za-z_]\w*|\d+(?:\.\d+)?)\s+as\s+(f64|f32|i32|i64|usize|u32|u64)").unwrap();
    let mut s = e.trim().to_string();
    loop {
        let n = cast
            .replace_all(&s, |c: &regex::Captures| {
                let t = match &c[2] {
                    "f64" | "f32" => "double",
                    "i64" | "u64" => "long long",
                    _ => "int",
                };
                format!("(({t}){})", &c[1])
            })
            .to_string();
        if n == s {
            break;
        }
        s = n;
    }
    // 数値の型接尾辞(1i32, 2.0f64, 3_000)
    let suf = Regex::new(r"\b(\d[\d_]*(?:\.\d+)?)(?:_?(i32|i64|u32|u64|usize|f64|f32))\b").unwrap();
    let s = suf.replace_all(&s, "$1").to_string();
    Regex::new(r"(\d)_(\d)").unwrap().replace_all(&s, "$1$2").to_string()
}

fn infer(e: &str, vars: &HashMap<String, Ty>) -> Ty {
    let e = e.trim();
    if e == "true" || e == "false" || Regex::new(r"==|!=|<|>|&&|\|\|").unwrap().is_match(e) && !e.contains("?") {
        return Ty::Bool;
    }
    if e.contains("(double)") || Regex::new(r"\d\.\d|\d\.\s|\d\.$").unwrap().is_match(e) {
        return Ty::Double;
    }
    let ids: Vec<&str> = Regex::new(r"[A-Za-z_]\w*").unwrap().find_iter(e).map(|m| m.as_str()).collect();
    let tys: Vec<Ty> = ids.iter().filter_map(|i| vars.get(*i).copied()).collect();
    if tys.contains(&Ty::Double) {
        Ty::Double
    } else if tys.contains(&Ty::Long) || e.contains("(long long)") {
        Ty::Long
    } else if ids.len() == 1 && tys == [Ty::Bool] {
        Ty::Bool
    } else {
        Ty::Int
    }
}

pub fn parse_rust(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<St> {
    let mainre = Regex::new(r"^fn\s+main\s*\(\s*\)\s*\{$").unwrap();
    let letre = Regex::new(r"^let\s+(mut\s+)?([A-Za-z_]\w*)\s*(?::\s*(\w+))?\s*(?:=\s*(.+))?;$").unwrap();
    let forre = Regex::new(r"^for\s+(\w+)\s+in\s+(\(?)(.+?)\s*\.\.(=?)\s*(.+?)(\)?)(?:\.rev\(\))?\s*\{$").unwrap();
    let forrange = Regex::new(r"^for\s+(\w+)\s+in\s+\(\s*(.+?)\s*\.\.(=?)\s*(.+?)\s*\)((?:\s*\.\s*(?:rev\(\)|step_by\([^()]+\)))+)\s*\{$").unwrap();
    let whilere = Regex::new(r"^while\s+(.+)\s*\{$").unwrap();
    let ifre = Regex::new(r"^(\}\s*else\s+)?if\s+(.+)\s*\{$").unwrap();
    let elsere = Regex::new(r"^\}\s*else\s*\{$").unwrap();
    let printre = Regex::new(r#"^(println|print)!\s*\(\s*(?:"((?:[^"\\]|\\.)*)"\s*(?:,(.*))?)?\)\s*;$"#).unwrap();
    let assign = Regex::new(r"^([A-Za-z_]\w*)\s*(\+|-|\*|/|%)?=\s*([^=].*);$").unwrap();
    let hole = Regex::new(r"\{(\w*)(?::(\d+)?(?:\.(\d+))?)?\}").unwrap();
    let mut out = vec![];
    let mut vars: HashMap<String, Ty> = HashMap::new();
    let mut in_main = false;
    let mut depth = 0usize;
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split("//").next().unwrap_or("").trim();
        if l.is_empty() || l.starts_with("#[") || l.starts_with("use ") {
            continue;
        }
        if !in_main {
            if mainre.is_match(l) {
                in_main = true;
                depth = 1;
            } else {
                rep.push(format!("{name}:{}: main の外は未対応 `{l}`", i + 1));
            }
            continue;
        }
        let Some(l) = block_line(l, &mut out) else {
            depth -= 1;
            if depth == 0 {
                out.pop();
                in_main = false;
            }
            continue;
        };
        if let Some(c) = forrange.captures(&l) {
            // for i in (a..b).rev() / (a..=b).step_by(k) / (a..b).step_by(k).rev() など
            let (v, a, incl, b, tail) = (&c[1], rust_expr(&c[2]), !c[3].is_empty(), rust_expr(&c[4]), c[5].replace(' ', ""));
            let step = Regex::new(r"\.step_by\(([^()]+)\)").unwrap().captures(&tail).map(|x| rust_expr(&x[1]));
            let rev = tail.ends_with(".rev()");
            let rest = Regex::new(r"\.step_by\([^()]+\)|\.rev\(\)").unwrap().replace_all(&tail, "");
            if !rest.is_empty() || (rev && step.is_some()) {
                // step_by の後の rev は最後の値が端数で決まるので未対応
                todo(&mut out, rep, "for の範囲", name, i, &l);
                depth += 1;
                continue;
            }
            vars.insert(v.to_string(), Ty::Int);
            if rev {
                let from = if incl { b } else { format!("{b} - 1") };
                out.push(St::For { var: v.into(), from, to: a, incl: true, down: true, step: "1".into() });
            } else {
                out.push(St::For { var: v.into(), from: a, to: b, incl, down: false, step: step.unwrap_or_else(|| "1".into()) });
            }
            depth += 1;
        } else if let Some(c) = forre.captures(&l) {
            if l.contains(".rev()") || l.contains("step_by") {
                todo(&mut out, rep, "rev/step_by", name, i, &l);
                depth += 1;
                continue;
            }
            vars.insert(c[1].to_string(), Ty::Int);
            out.push(St::For { var: c[1].into(), from: rust_expr(&c[3]), to: rust_expr(&c[5]), incl: !c[4].is_empty(), down: false, step: "1".into() });
            depth += 1;
        } else if let Some(c) = ifre.captures(&l) {
            if c.get(1).is_some() {
                out.push(St::ElseIf(rust_expr(&c[2])));
            } else {
                out.push(St::If(rust_expr(&c[2])));
                depth += 1;
            }
        } else if elsere.is_match(&l) {
            out.push(St::Else);
        } else if let Some(c) = whilere.captures(&l) {
            out.push(St::While(rust_expr(&c[1])));
            depth += 1;
        } else if let Some(c) = letre.captures(&l) {
            let n = c[2].to_string();
            let init = c.get(4).map(|m| rust_expr(m.as_str()));
            if init.as_deref().map(|e| Regex::new(r#"\w!|\.\w+\(|"|&|\[|::"#).unwrap().is_match(e)).unwrap_or(false) {
                todo(&mut out, rep, "式", name, i, &l);
                continue;
            }
            let ty = match c.get(3).map(|m| m.as_str()) {
                Some("i32" | "u32" | "usize" | "i16" | "u8" | "i8") => Ty::Int,
                Some("i64" | "u64") => Ty::Long,
                Some("f64" | "f32") => Ty::Double,
                Some("bool") => Ty::Bool,
                Some(_) => {
                    todo(&mut out, rep, "型", name, i, &l);
                    continue;
                }
                None => match &init {
                    Some(e) => infer(e, &vars),
                    None => {
                        todo(&mut out, rep, "型なし宣言", name, i, &l);
                        continue;
                    }
                },
            };
            if vars.contains_key(&n) {
                rep.push(format!("{name}:{}: 同名の let(シャドーイング)は未対応 `{l}`", i + 1));
                out.push(St::Todo(l.clone()));
                continue;
            }
            vars.insert(n.clone(), ty);
            out.push(St::Decl { name: n, ty, init });
        } else if let Some(c) = printre.captures(&l) {
            let nl = &c[1] == "println";
            let fmt = c.get(2).map(|m| m.as_str()).unwrap_or("");
            let args = c.get(3).map(|m| split_args(m.as_str())).unwrap_or_default();
            let (mut parts, mut last, mut k, mut bad) = (vec![], 0, 0, false);
            let fmt2 = fmt.replace("{{", "\u{1}").replace("}}", "\u{2}");
            for m in hole.captures_iter(&fmt2) {
                let w = m.get(0).unwrap();
                parts.push(Part::Lit(unescape(&fmt2[last..w.start()]).replace('\u{1}', "{").replace('\u{2}', "}")));
                last = w.end();
                let e = if !m[1].is_empty() {
                    m[1].to_string()
                } else {
                    let Some(a) = args.get(k) else {
                        bad = true;
                        break;
                    };
                    k += 1;
                    rust_expr(a)
                };
                let ty = infer(&e, &vars);
                let prec: Option<usize> = m.get(3).and_then(|x| x.as_str().parse().ok());
                let ty = if prec.is_some() && ty != Ty::Double { Ty::Double } else { ty };
                // 小数の {} は Rust と同じ最短表記を出す関数 rs_f64 を使う(prec: None かつ g: false)
                parts.push(Part::Val { expr: e, ty, width: m.get(2).and_then(|x| x.as_str().parse().ok()), prec, g: false });
            }
            parts.push(Part::Lit(unescape(&fmt2[last..]).replace('\u{1}', "{").replace('\u{2}', "}")));
            if bad || k != args.len() {
                todo(&mut out, rep, "println の書式", name, i, &l);
                continue;
            }
            push_prints(&mut out, parts, nl);
        } else if let Some(c) = assign.captures(&l) {
            out.push(St::Assign { lhs: c[1].into(), op: c.get(2).map(|m| m.as_str().to_string()).unwrap_or_default(), rhs: rust_expr(&c[3]) });
        } else {
            todo(&mut out, rep, "Rust", name, i, &l);
        }
    }
    out
}

// ---------------- 中間表現 → C ----------------

fn c_ty(t: Ty) -> &'static str {
    match t {
        Ty::Int => "int",
        Ty::Long => "long long",
        Ty::Double => "double",
        Ty::Bool => "bool",
        Ty::Str => "const char *",
    }
}

fn c_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('%', "%%").replace('\n', "\\n").replace('\t', "\\t")
}

pub fn emit_c(sts: &[St], tag: &str) -> String {
    let mut o = format!("/* pl2pl {tag} で変換(最初の一歩)。TODO({tag}) の行は手で直してください。 */\n#include <stdio.h>\n#include <stdbool.h>\n\nint main(void) {{\n");
    let mut d = 1usize;
    for s in sts {
        let ind = "    ".repeat(d);
        match s {
            St::Decl { name, ty, init } => o.push_str(&format!("{ind}{} {name} = {};\n", c_ty(*ty), init.clone().unwrap_or_else(|| "0".into()))),
            St::Assign { lhs, op, rhs } => o.push_str(&format!("{ind}{lhs} {op}= {rhs};\n")),
            St::For { var, from, to, incl, down, step } => {
                let cmp = match (down, incl) {
                    (false, false) => "<",
                    (false, true) => "<=",
                    (true, false) => ">",
                    (true, true) => ">=",
                };
                let inc = match (down, step.as_str()) {
                    (false, "1") => format!("{var}++"),
                    (true, "1") => format!("{var}--"),
                    (false, s) => format!("{var} += {s}"),
                    (true, s) => format!("{var} -= {s}"),
                };
                o.push_str(&format!("{ind}for (int {var} = {from}; {var} {cmp} {to}; {inc}) {{\n"));
                d += 1;
            }
            St::While(c) => {
                o.push_str(&format!("{ind}while ({c}) {{\n"));
                d += 1;
            }
            St::If(c) => {
                o.push_str(&format!("{ind}if ({c}) {{\n"));
                d += 1;
            }
            St::ElseIf(c) => o.push_str(&format!("{}}} else if ({c}) {{\n", "    ".repeat(d - 1))),
            St::Else => o.push_str(&format!("{}}} else {{\n", "    ".repeat(d - 1))),
            St::End => {
                d = d.saturating_sub(1).max(1);
                o.push_str(&format!("{}}}\n", "    ".repeat(d)));
            }
            St::Print { parts, nl } => {
                let (mut f, mut a) = (String::new(), vec![]);
                for p in parts {
                    match p {
                        Part::Lit(t) => f.push_str(&c_str(t)),
                        Part::Val { expr, ty, width, prec, g } => {
                            let w = width.map(|x| x.to_string()).unwrap_or_default();
                            match ty {
                                Ty::Int => f.push_str(&format!("%{w}d")),
                                Ty::Long => f.push_str(&format!("%{w}lld")),
                                Ty::Bool => {
                                    f.push_str(&format!("%{w}s"));
                                    a.push(format!("({expr}) ? \"true\" : \"false\""));
                                    continue;
                                }
                                Ty::Str => f.push_str(&format!("%{w}s")),
                                Ty::Double if *g => f.push_str(&format!("%{w}g")),
                                Ty::Double if prec.is_none() => {
                                    f.push_str(&format!("%{w}s"));
                                    a.push(format!("rs_f64({expr})"));
                                    continue;
                                }
                                Ty::Double => f.push_str(&format!("%{w}.{}f", prec.unwrap_or(6))),
                            }
                            a.push(expr.clone());
                        }
                    }
                }
                if *nl {
                    f.push_str("\\n");
                }
                o.push_str(&format!("{ind}printf(\"{f}\"{});\n", a.iter().map(|x| format!(", {x}")).collect::<String>()));
            }
            St::Todo(l) => o.push_str(&format!("{ind}/* TODO({tag}): {} */\n", l.replace("*/", "* /"))),
        }
    }
    o.push_str("    return 0;\n}\n");
    if o.contains("rs_f64(") {
        // Rust の {} と同じ最短表記(読み戻して同じ値になる最小の桁数、指数表記なし)
        o = o.replacen("int main(void) {", RS_F64_C, 1);
    }
    o
}

const RS_F64_C: &str = "#include <stdlib.h>\n/* Rust の f64 の {} と同じ最短表記 */\nstatic const char *rs_f64(double x) {\n    static char buf[8][400];\n    static int k = 0;\n    char *b = buf[k++ % 8];\n    for (int p = 0; p <= 340; p++) {\n        snprintf(b, 400, \"%.*f\", p, x);\n        if (strtod(b, NULL) == x) break;\n    }\n    return b;\n}\n\nint main(void) {";

const RS_F64_F: &str = "contains\n  ! Rust の f64 の {} と同じ最短表記(読み戻して同じ値になる最小の桁数、指数表記なし)\n  function rs_f64(x) result(r)\n    real(8), intent(in) :: x\n    character(len=:), allocatable :: r\n    character(len=400) :: b\n    character(len=16) :: f\n    integer :: p\n    real(8) :: y\n    do p = 0, 340\n      write(f, '(a,i0,a)') '(f0.', p, ')'\n      write(b, f) x\n      read(b, *) y\n      if (y == x) exit\n    end do\n    r = trim(adjustl(b))\n    if (r(len(r):len(r)) == '.') r = r(1:len(r)-1)\n    if (r(1:1) == '.') r = '0' // r\n    if (len(r) >= 2) then\n      if (r(1:2) == '-.') r = '-0' // r(2:)\n    end if\n  end function rs_f64\n";


// ---------------- 中間表現 → Fortran ----------------

/// C の書き方の式を Fortran へ(演算子・小数リテラルを倍精度に・型変換)。
pub fn f_expr(e: &str) -> String {
    let mut s = e.to_string();
    s = Regex::new(r"\(\s*double\s*\)\s*(\([^()]*\)|\w+)").unwrap().replace_all(&s, "real($1, 8)").to_string();
    s = Regex::new(r"\(\s*(?:long long|long)\s*\)\s*(\([^()]*\)|\w+)").unwrap().replace_all(&s, "int($1, 8)").to_string();
    s = Regex::new(r"\(\s*int\s*\)\s*(\([^()]*\)|\w+)").unwrap().replace_all(&s, "int($1)").to_string();
    s = Regex::new(r"(\([^()]*\)|\w+)\s*%\s*(\([^()]*\)|\w+)").unwrap().replace_all(&s, "mod($1, $2)").to_string();
    s = s.replace("&&", " .and. ").replace("||", " .or. ").replace("!=", "/=");
    s = Regex::new(r"!\s*").unwrap().replace_all(&s, ".not. ").to_string();
    s = Regex::new(r"\btrue\b").unwrap().replace_all(&s, ".true.").to_string();
    s = Regex::new(r"\bfalse\b").unwrap().replace_all(&s, ".false.").to_string();
    // 1.5 → 1.5d0(倍精度)。指数付き 1e3 → 1d3
    s = Regex::new(r"\b(\d+\.\d*)([eE]([-+]?\d+))?").unwrap().replace_all(&s, |c: &regex::Captures| format!("{}d{}", &c[1], c.get(3).map(|m| m.as_str()).unwrap_or("0"))).to_string();
    // int に収まらない整数リテラルは 8 バイト(_8)に
    s = Regex::new(r"\b\d{10,}\b").unwrap().replace_all(&s, |c: &regex::Captures| if c[0].parse::<i64>().map(|v| v > i32::MAX as i64).unwrap_or(true) { format!("{}_8", &c[0]) } else { c[0].to_string() }).to_string();
    Regex::new(r"\s+").unwrap().replace_all(s.trim(), " ").to_string()
}

fn f_ty(t: Ty) -> &'static str {
    match t {
        Ty::Int => "integer",
        Ty::Long => "integer(8)",
        Ty::Double => "real(8)",
        Ty::Bool => "logical",
        Ty::Str => "character(len=256)",
    }
}

pub fn emit_f(sts: &[St], tag: &str, prog: &str, rep: &mut Vec<String>) -> String {
    let mut o = format!("! pl2pl {tag} で変換(最初の一歩)。TODO({tag}) の行は手で直してください。\nprogram {prog}\n  implicit none\n");
    // 宣言は先頭へまとめる(Fortran の決まり)。for の変数も integer として宣言
    let mut seen: Vec<String> = vec![];
    for s in sts {
        let (n, t) = match s {
            St::Decl { name, ty, .. } => (name.clone(), *ty),
            St::For { var, .. } => (var.clone(), Ty::Int),
            _ => continue,
        };
        if !seen.iter().any(|x| x.eq_ignore_ascii_case(&n)) {
            o.push_str(&format!("  {} :: {n}\n", f_ty(t)));
            seen.push(n);
        }
    }
    let mut d = 1usize;
    for s in sts {
        let ind = "  ".repeat(d);
        match s {
            St::Decl { name, ty, init } => {
                let v = init.clone().unwrap_or_else(|| if *ty == Ty::Bool { "false".into() } else { "0".into() });
                o.push_str(&format!("{ind}{name} = {}\n", f_expr(&v)));
            }
            St::Assign { lhs, op, rhs } => {
                let r = f_expr(rhs);
                let v = match op.as_str() {
                    "" => r,
                    "%" => format!("mod({lhs}, {r})"),
                    o => format!("{lhs} {o} ({r})"),
                };
                o.push_str(&format!("{ind}{lhs} = {v}\n"));
            }
            St::For { var, from, to, incl, down, step } => {
                let (f, t, st) = (f_expr(from), f_expr(to), f_expr(step));
                let t = match (incl, down) {
                    (true, _) => t,
                    (false, false) => format!("{t} - 1"),
                    (false, true) => format!("{t} + 1"),
                };
                let st = match (down, st.as_str()) {
                    (false, "1") => String::new(),
                    (false, s) => format!(", {s}"),
                    (true, s) => format!(", -{s}"),
                };
                o.push_str(&format!("{ind}do {var} = {f}, {t}{st}\n"));
                d += 1;
            }
            St::While(c) => {
                o.push_str(&format!("{ind}do while ({})\n", f_expr(c)));
                d += 1;
            }
            St::If(c) => {
                o.push_str(&format!("{ind}if ({}) then\n", f_expr(c)));
                d += 1;
            }
            St::ElseIf(c) => o.push_str(&format!("{}else if ({}) then\n", "  ".repeat(d - 1), f_expr(c))),
            St::Else => o.push_str(&format!("{}else\n", "  ".repeat(d - 1))),
            St::End => {
                d = d.saturating_sub(1).max(1);
                // 直前に開いたのが do か if かを数えて閉じ方を決める
                o.push_str(&format!("{}{}\n", "  ".repeat(d), "END_PLACEHOLDER"));
            }
            St::Print { parts, nl } => {
                let (mut f, mut a) = (vec![], vec![]);
                for p in parts {
                    match p {
                        Part::Lit(t) => {
                            f.push("a".to_string());
                            a.push(format!("'{}'", t.replace('\'', "''")));
                        }
                        Part::Val { expr, ty, width, prec, g } => {
                            let w = width.unwrap_or(0);
                            match ty {
                                Ty::Int | Ty::Long => f.push(format!("i{w}")),
                                Ty::Bool => {
                                    f.push("a".into());
                                    a.push(format!("trim(merge('true ', 'false', {}))", f_expr(expr)));
                                    continue;
                                }
                                Ty::Str => f.push("a".into()),
                                Ty::Double if prec.is_none() && !*g => {
                                    f.push("a".into());
                                    if width.is_some() {
                                        rep.push(format!("{prog}: 小数の {{:N}} の幅は無視しました"));
                                    }
                                    a.push(format!("rs_f64({})", f_expr(expr)));
                                    continue;
                                }
                                Ty::Double if *g => {
                                    f.push("g0".into());
                                    rep.push(format!("{prog}: %g の小数は g0 にしました(桁数・表記が C と違う場合あり)"));
                                }
                                Ty::Double => f.push(format!("f{w}.{}", prec.unwrap_or(6))),
                            }
                            a.push(f_expr(expr));
                        }
                    }
                }
                let adv = if *nl { "" } else { ", advance='no'" };
                if f.is_empty() {
                    o.push_str(&format!("{ind}write(*, '(a)'{adv}) ''\n"));
                } else {
                    o.push_str(&format!("{ind}write(*, '({})'{adv}) {}\n", f.join(","), a.join(", ")));
                }
            }
            St::Todo(l) => o.push_str(&format!("{ind}! TODO({tag}): {l}\n")),
        }
    }
    // END_PLACEHOLDER を、開いた構文に合わせて end do / end if に置き換える
    let mut stack: Vec<&str> = vec![];
    let mut res = String::new();
    for line in o.lines() {
        let t = line.trim_start();
        if t.starts_with("do ") {
            stack.push("end do");
        } else if t.starts_with("if (") && t.ends_with("then") {
            stack.push("end if");
        }
        if t == "END_PLACEHOLDER" {
            let e = stack.pop().unwrap_or("end");
            res.push_str(&line.replace("END_PLACEHOLDER", e));
        } else {
            res.push_str(line);
        }
        res.push('\n');
    }
    if res.contains("rs_f64(") {
        res.push_str(RS_F64_F);
    }
    res.push_str(&format!("end program {prog}\n"));
    res
}

fn base(name: &str) -> String {
    let b = name.rsplit('/').next().unwrap_or(name);
    b.rsplit_once('.').map(|x| x.0).unwrap_or(b).to_string()
}

fn prog_name(name: &str) -> String {
    let b: String = base(name).chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    if b.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false) { b } else { format!("p_{b}") }
}

fn summary(rep: &mut Vec<String>, name: &str, sts: &[St], tag: &str) {
    let n = sts.iter().filter(|s| matches!(s, St::Todo(_))).count();
    if n > 0 {
        rep.push(format!("{name}: 未対応の行 {n} 個(TODO({tag}) として残しました)"));
    }
}

pub fn c2f(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let sts = parse_c(src, name, rep);
    summary(rep, name, &sts, "c2f");
    vec![(format!("{}.f90", base(name)), emit_f(&sts, "c2f", &prog_name(name), rep))]
}

pub fn rust2c(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let sts = parse_rust(src, name, rep);
    summary(rep, name, &sts, "rust2c");
    vec![(format!("{}.c", base(name)), emit_c(&sts, "rust2c"))]
}

pub fn rust2f(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let sts = parse_rust(src, name, rep);
    summary(rep, name, &sts, "rust2f");
    vec![(format!("{}.f90", base(name)), emit_f(&sts, "rust2f", &prog_name(name), rep))]
}

#[cfg(test)]
mod tests {
    use super::*;
    const C: &str = "#include <stdio.h>\nint main(void) {\n    int s = 0, n = 10;\n    double x = 1.5;\n    for (int i = 0; i < n; i++) {\n        if (i % 2 == 0 && i != 4) {\n            s += i;\n        } else if (i > 7) {\n            s -= 1;\n        } else {\n            x = x * 2.0;\n        }\n    }\n    while (n > 0) {\n        n -= 3;\n    }\n    printf(\"s=%d x=%.2f n=%5d\\n\", s, x, n);\n    return 0;\n}\n";
    #[test]
    fn c2f_basic() {
        let mut r = vec![];
        let t = &c2f(C, "a.c", &mut r)[0].1;
        for want in ["integer :: s", "real(8) :: x", "x = 1.5d0", "do i = 0, n - 1", "if (mod(i, 2) == 0 .and. i /= 4) then", "else if (i > 7) then", "s = s + (i)", "x = x * 2.0d0", "end if", "end do", "do while (n > 0)", "write(*, '(a,i0,a,f0.2,a,i5)') 's=', s, ' x=', x, ' n=', n", "end program a"] {
            assert!(t.contains(want), "missing {want}\n{t}");
        }
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn c2f_todo_kept() {
        let mut r = vec![];
        let t = &c2f("int main(void) {\n    int a[3];\n    foo(1);\n}\n", "b.c", &mut r)[0].1;
        assert!(t.contains("! TODO(c2f): int a[3];") && t.contains("! TODO(c2f): foo(1);") && r.len() >= 3, "{t}\n{r:?}");
    }
    const R: &str = "fn main() {\n    let mut s: i64 = 0;\n    let n = 5;\n    let mut x = 0.5;\n    for i in 1..=n {\n        if i % 2 == 1 {\n            s += i as i64 * 3_000;\n        } else {\n            x *= 2.0;\n        }\n    }\n    let ok = s > 10;\n    println!(\"s={} x={:.3} n={:4} ok={}\", s, x, n, ok);\n    print!(\"{{done}}\");\n    println!();\n}\n";
    #[test]
    fn rust2c_basic() {
        let mut r = vec![];
        let t = &rust2c(R, "r.rs", &mut r)[0].1;
        for want in ["long long s = 0;", "int n = 5;", "double x = 0.5;", "for (int i = 1; i <= n; i++) {", "s += ((long long)i) * 3000;", "x *= 2.0;", "bool ok = s > 10;", "printf(\"s=%lld x=%.3f n=%4d ok=%s\\n\", s, x, n, (ok) ? \"true\" : \"false\");", "printf(\"{done}\");", "printf(\"\\n\");"] {
            assert!(t.contains(want), "missing {want}\n{t}");
        }
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn rust2f_basic() {
        let mut r = vec![];
        let t = &rust2f(R, "r.rs", &mut r)[0].1;
        for want in ["integer(8) :: s", "integer :: i", "logical :: ok", "do i = 1, n", "s = s + ((int(i, 8)) * 3000)", "x = x * (2.0d0)", "write(*, '(a,i0,a,f0.3,a,i4,a,a)')", "write(*, '(a)', advance='no') '{done}'"] {
            assert!(t.contains(want), "missing {want}\n{t}");
        }
    }
    #[test]
    fn rust_unsupported() {
        let mut r = vec![];
        let t = &rust2c("fn f() {}\nfn main() {\n    let v = vec![1];\n    let x = 1;\n    let x = 2;\n}\n", "u.rs", &mut r)[0].1;
        assert!(t.contains("TODO(rust2c)") && r.len() >= 2, "{t}\n{r:?}");
    }
}
#[cfg(test)]
mod big_lit {
    #[test]
    fn big_int_literal_is_kind8() {
        assert_eq!(super::f_expr("((long long)i) * 3000000000"), "(int(i, 8)) * 3000000000_8");
    }
}
#[cfg(test)]
mod rev_step {
    #[test]
    fn rust_rev_and_step_by() {
        let mut r = vec![];
        let src = "fn main() {\n    for i in (0..5).rev() {\n    }\n    for j in (1..=10).step_by(3) {\n    }\n    for k in (0..9).step_by(2).rev() {\n    }\n}\n";
        let t = &super::rust2c(src, "s.rs", &mut r)[0].1;
        assert!(t.contains("for (int i = 5 - 1; i >= 0; i--) {") && t.contains("for (int j = 1; j <= 10; j += 3) {") && t.contains("TODO(rust2c): for k"), "{t}");
        let f = &super::rust2f(src, "s.rs", &mut vec![])[0].1;
        assert!(f.contains("do i = 5 - 1, 0, -1") && f.contains("do j = 1, 10, 3"), "{f}");
    }
}
