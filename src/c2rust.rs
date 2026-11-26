//! `pl2pl c2rust`: 単純な C を Rust へ書き換える。行ごとの変換で、式は型つきの小さな構文解析器で読む。
//! わからない行は黙って捨てず `// TODO(c2rust): 元の行` として残し、レポートにも載せる。
//! 対応: 整数・小数の変数、四則と比較(整数と小数を混ぜた式は C と同じく小数へ暗黙変換)、if / while / for、
//! 関数、printf、固定長の配列(1次元・2次元)、配列を関数に渡す(`int a[]` → `&mut [i32]`)、
//! 単純な構造体(`struct P { int x; double y; };`、変数・初期化・配列・`.`/`->`)、
//! 出力用のポインタ引数(`void f(int *x)` → `x: &mut i32`、呼び出し `f(&v)` → `f(&mut v)`)、break / continue。
//! 未対応(TODO のまま): ポインタ変数・動的確保・文字列操作・マクロ・ファイル I/O。
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum T {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Arr(Box<T>, usize),
    Slice(Box<T>),
    St(String),
    Ptr(Box<T>),
}

fn rs(t: &T) -> String {
    match t {
        T::I32 => "i32".into(),
        T::I64 => "i64".into(),
        T::F32 => "f32".into(),
        T::F64 => "f64".into(),
        T::Bool => "bool".into(),
        T::Arr(e, n) => format!("[{}; {n}]", rs(e)),
        T::Slice(e) => format!("&mut [{}]", rs(e)),
        T::St(s) => s.clone(),
        T::Ptr(e) => format!("&mut {}", rs(e)),
    }
}

fn zero(t: &T) -> String {
    match t {
        T::I32 | T::I64 => "0".into(),
        T::F32 | T::F64 => "0.0".into(),
        T::Bool => "false".into(),
        T::Arr(e, n) => format!("[{}; {n}]", zero(e)),
        T::St(s) => format!("{s}::default()"),
        _ => "Default::default()".into(),
    }
}

fn base_ty(t: &str) -> Option<T> {
    let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(s) = t.strip_prefix("struct ") {
        return STRUCTS.with(|m| m.borrow().contains_key(s.trim())).then(|| T::St(s.trim().to_string()));
    }
    match t.as_str() {
        "int" | "short" | "unsigned" | "unsigned int" => Some(T::I32),
        "long" | "long long" => Some(T::I64),
        "double" => Some(T::F64),
        "float" => Some(T::F32),
        _ => None,
    }
}

fn is_float(t: &T) -> bool {
    matches!(t, T::F32 | T::F64)
}
fn is_num(t: &T) -> bool {
    matches!(t, T::I32 | T::I64 | T::F32 | T::F64 | T::Bool)
}

thread_local! {
    static VARS: RefCell<HashMap<String, T>> = Default::default();
    static STRUCTS: RefCell<HashMap<String, Vec<(String, T)>>> = Default::default();
    static FUNCS: RefCell<HashMap<String, (Vec<T>, Option<T>)>> = Default::default();
}

fn var(n: &str) -> Option<T> {
    VARS.with(|v| v.borrow().get(n).cloned())
}
fn set_var(n: &str, t: T) {
    VARS.with(|v| v.borrow_mut().insert(n.to_string(), t));
}

fn is_lit_int(e: &str) -> bool {
    Regex::new(r"^-?\d+$").unwrap().is_match(e)
}
fn is_lit_float(e: &str) -> bool {
    Regex::new(r"^-?\d+\.\d+$").unwrap().is_match(e)
}

/// 型を合わせる(C の暗黙の変換に相当)
fn coerce(e: &str, from: &T, to: &T) -> Option<String> {
    if from == to {
        return Some(e.to_string());
    }
    let e = if *from == T::Bool { format!("(({e}) as i32)") } else { e.to_string() };
    let from = if *from == T::Bool { &T::I32 } else { from };
    if from == to {
        return Some(e);
    }
    Some(match (from, to) {
        (T::I32 | T::I64, T::F32 | T::F64) if is_lit_int(&e) => format!("{e}.0"),
        (T::F64, T::F32) if is_lit_float(&e) => e,
        (T::I32, T::I64) if is_lit_int(&e) => e,
        (a, b) if is_num(a) && is_num(b) && *b != T::Bool => format!("(({e}) as {})", rs(b)),
        _ => return None,
    })
}

fn to_bool(e: &str, t: &T) -> Option<String> {
    Some(match t {
        T::Bool => e.to_string(),
        T::F32 | T::F64 => format!("({e} != 0.0)"),
        T::I32 | T::I64 => format!("({e} != 0)"),
        _ => return None,
    })
}

fn toks(s: &str) -> Option<Vec<String>> {
    let c: Vec<char> = s.chars().collect();
    let mut v = vec![];
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        if ch.is_ascii_digit() || (ch == '.' && i + 1 < c.len() && c[i + 1].is_ascii_digit()) {
            let st = i;
            while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '.' || ((c[i] == '+' || c[i] == '-') && (c[i - 1] == 'e' || c[i - 1] == 'E'))) {
                i += 1;
            }
            let mut n: String = c[st..i].iter().collect();
            if n.starts_with("0x") || n.starts_with("0X") {
                return None;
            }
            let fl = n.contains('.') || n.contains('e') || n.contains('E') || n.ends_with('f') || n.ends_with('F');
            n = n.trim_end_matches(|x: char| "fFlLuU".contains(x)).to_string();
            if fl {
                if n.starts_with('.') {
                    n.insert(0, '0');
                }
                if n.ends_with('.') {
                    n.push('0');
                }
                if !n.contains('.') && !n.contains('e') && !n.contains('E') {
                    n.push_str(".0");
                }
                v.push(format!("#{n}")); // 小数の目印
            } else {
                v.push(n);
            }
            continue;
        }
        if ch.is_alphabetic() || ch == '_' {
            let st = i;
            while i < c.len() && (c[i].is_alphanumeric() || c[i] == '_') {
                i += 1;
            }
            v.push(c[st..i].iter().collect());
            continue;
        }
        if i + 1 < c.len() {
            let two: String = c[i..i + 2].iter().collect();
            if ["->", "&&", "||", "<=", ">=", "==", "!=", "++", "--"].contains(&two.as_str()) {
                v.push(two);
                i += 2;
                continue;
            }
        }
        if "+-*/%<>!&()[].,".contains(ch) {
            v.push(ch.to_string());
            i += 1;
            continue;
        }
        return None;
    }
    Some(v)
}

struct Px {
    t: Vec<String>,
    i: usize,
}

fn prec(op: &str) -> Option<u8> {
    Some(match op {
        "||" => 1,
        "&&" => 2,
        "==" | "!=" => 3,
        "<" | ">" | "<=" | ">=" => 4,
        "+" | "-" => 5,
        "*" | "/" | "%" => 6,
        _ => return None,
    })
}

/// 二項演算。整数と小数が混ざれば小数へ、i32 と i64 が混ざれば i64 へそろえる。
fn bin(op: &str, l: (String, T), r: (String, T)) -> Option<(String, T)> {
    if op == "&&" || op == "||" {
        return Some((format!("{} {op} {}", to_bool(&l.0, &l.1)?, to_bool(&r.0, &r.1)?), T::Bool));
    }
    if !is_num(&l.1) || !is_num(&r.1) {
        return None;
    }
    let target = if is_float(&l.1) || is_float(&r.1) {
        if l.1 == T::F64 || r.1 == T::F64 || !(is_float(&l.1) && is_float(&r.1)) { T::F64 } else { T::F32 }
    } else if l.1 == T::I64 || r.1 == T::I64 {
        T::I64
    } else {
        T::I32
    };
    let (a, b) = (coerce(&l.0, &l.1, &target)?, coerce(&r.0, &r.1, &target)?);
    let cmp = matches!(op, "==" | "!=" | "<" | ">" | "<=" | ">=");
    Some((format!("{a} {op} {b}"), if cmp { T::Bool } else { target }))
}

impl Px {
    fn peek(&self) -> Option<&str> {
        self.t.get(self.i).map(|s| s.as_str())
    }
    fn eat(&mut self, s: &str) -> bool {
        if self.peek() == Some(s) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn expr(&mut self, minp: u8) -> Option<(String, T)> {
        let mut l = self.unary()?;
        while let Some(op) = self.peek().map(|s| s.to_string()) {
            let Some(p) = prec(&op) else { break };
            if p < minp {
                break;
            }
            self.i += 1;
            let r = self.expr(p + 1)?;
            l = bin(&op, l, r)?;
        }
        Some(l)
    }
    fn unary(&mut self) -> Option<(String, T)> {
        if self.eat("-") {
            let (e, t) = self.unary()?;
            return Some((format!("-{e}"), t));
        }
        if self.eat("+") {
            return self.unary();
        }
        if self.eat("!") {
            let (e, t) = self.unary()?;
            return Some((format!("!{}", to_bool(&e, &t)?), T::Bool));
        }
        if self.eat("*") {
            let (e, t) = self.unary()?;
            return match t {
                T::Ptr(inner) => Some((format!("*{e}"), *inner)),
                _ => None,
            };
        }
        if self.eat("&") {
            let (e, t) = self.unary()?;
            return Some((format!("&mut {e}"), T::Ptr(Box::new(t))));
        }
        // キャスト (double)x
        if self.peek() == Some("(") {
            let save = self.i;
            self.i += 1;
            let mut words = vec![];
            while let Some(w) = self.peek() {
                if w == ")" {
                    break;
                }
                words.push(w.to_string());
                self.i += 1;
            }
            if self.eat(")") {
                if let Some(ct) = base_ty(&words.join(" ")).filter(is_num) {
                    let (e, t) = self.unary()?;
                    return Some((coerce(&e, &t, &ct)?, ct));
                }
            }
            self.i = save;
        }
        self.postfix()
    }
    fn postfix(&mut self) -> Option<(String, T)> {
        let (mut e, mut t) = self.primary()?;
        loop {
            if self.eat("[") {
                let (ix, it) = self.expr(0)?;
                if !self.eat("]") || !matches!(it, T::I32 | T::I64) {
                    return None;
                }
                t = match t {
                    T::Arr(el, _) | T::Slice(el) => *el,
                    _ => return None,
                };
                e = if is_lit_int(&ix) { format!("{e}[{ix}]") } else { format!("{e}[({ix}) as usize]") };
            } else if self.peek() == Some(".") || self.peek() == Some("->") {
                let arrow = self.peek() == Some("->");
                self.i += 1;
                let f = self.t.get(self.i)?.clone();
                self.i += 1;
                let sname = match (&t, arrow) {
                    (T::St(s), false) => s.clone(),
                    (T::Ptr(b), true) => match &**b {
                        T::St(s) => s.clone(),
                        _ => return None,
                    },
                    _ => return None,
                };
                t = STRUCTS.with(|m| m.borrow().get(&sname).and_then(|fs| fs.iter().find(|(n, _)| *n == f).map(|x| x.1.clone())))?;
                e = format!("{e}.{f}");
            } else {
                return Some((e, t));
            }
        }
    }
    fn primary(&mut self) -> Option<(String, T)> {
        let tk = self.t.get(self.i)?.clone();
        self.i += 1;
        if tk == "(" {
            let (e, t) = self.expr(0)?;
            return self.eat(")").then(|| (format!("({e})"), t));
        }
        if let Some(f) = tk.strip_prefix('#') {
            return Some((f.to_string(), T::F64));
        }
        if tk.chars().next()?.is_ascii_digit() {
            return Some((tk, T::I32));
        }
        if !(tk.chars().next()?.is_alphabetic() || tk.starts_with('_')) {
            return None;
        }
        if self.eat("(") {
            let (ps, ret) = FUNCS.with(|m| m.borrow().get(&tk).cloned())?;
            let mut args = vec![];
            if !self.eat(")") {
                loop {
                    let a = self.expr(0)?;
                    let pt = ps.get(args.len())?;
                    args.push(match (pt, &a.1) {
                        (T::Slice(_), T::Arr(..)) => format!("&mut {}", a.0),
                        (T::Slice(_), T::Slice(_)) => a.0.clone(),
                        (T::Ptr(_), T::Ptr(_)) => a.0.clone(),
                        (T::St(x), T::St(y)) if x == y => a.0.clone(),
                        _ => coerce(&a.0, &a.1, pt)?,
                    });
                    if self.eat(")") {
                        break;
                    }
                    if !self.eat(",") {
                        return None;
                    }
                }
            }
            if args.len() != ps.len() {
                return None;
            }
            return Some((format!("{tk}({})", args.join(", ")), ret.unwrap_or(T::I32)));
        }
        Some((tk.clone(), var(&tk)?))
    }
}

/// 式を Rust へ。(文字列, 型)
fn ex(s: &str) -> Option<(String, T)> {
    let mut p = Px { t: toks(s.trim())?, i: 0 };
    let r = p.expr(0)?;
    (p.i == p.t.len()).then_some(r)
}

fn ex_to(s: &str, to: &T) -> Option<String> {
    let (e, t) = ex(s)?;
    coerce(&e, &t, to)
}

fn cond(s: &str) -> Option<String> {
    let (e, t) = ex(s)?;
    to_bool(&e, &t)
}

/// printf の書式を Rust の `{}` へ。
fn fmt(args: &str) -> Option<String> {
    let parts = split_args(args);
    let f = parts.first()?.trim();
    if !(f.starts_with('"') && f.ends_with('"') && f.len() >= 2) {
        return None;
    }
    let spec = Regex::new(r"%(\.\d+)?(l|ll)?[dfsc]|%%|%(l|ll)?u").unwrap();
    let mut n = 0;
    let body = &f[1..f.len() - 1];
    let out = spec.replace_all(body, |c: &regex::Captures| {
        let m = &c[0];
        if m == "%%" {
            return "%".to_string();
        }
        n += 1;
        format!("\u{1}{}\u{3}", c.get(1).map(|p| p.as_str()).unwrap_or(""))
    });
    let esc = out.replace('{', "{{").replace('}', "}}");
    let tmp = Regex::new("\u{1}([^\u{3}]*)\u{3}").unwrap().replace_all(&esc, |c: &regex::Captures| if c[1].is_empty() { "{}".to_string() } else { format!("{{:{}}}", &c[1]) }).to_string();
    if n != parts.len() - 1 {
        return None;
    }
    let rest: Option<Vec<String>> = parts[1..].iter().map(|x| ex(x.trim()).map(|r| r.0)).collect();
    let rest = rest?;
    Some(if rest.is_empty() { format!("\"{tmp}\"") } else { format!("\"{tmp}\", {}", rest.join(", ")) })
}

/// カンマで分ける(括弧・波括弧・引用符の中は分けない)
fn split_args(s: &str) -> Vec<String> {
    let (mut v, mut d, mut q, mut cur) = (vec![], 0i32, false, String::new());
    let mut prev = ' ';
    for c in s.chars() {
        if c == '"' && prev != '\\' {
            q = !q;
        }
        if !q {
            if c == '(' || c == '{' || c == '[' {
                d += 1;
            }
            if c == ')' || c == '}' || c == ']' {
                d -= 1;
            }
            if c == ',' && d == 0 {
                v.push(std::mem::take(&mut cur));
                prev = c;
                continue;
            }
        }
        cur.push(c);
        prev = c;
    }
    if !cur.trim().is_empty() {
        v.push(cur);
    }
    v
}

/// 初期化子(`{...}` の入れ子、構造体、ふつうの式)を型に合わせて Rust へ
fn init(text: &str, t: &T) -> Option<String> {
    let s = text.trim();
    match t {
        T::Arr(el, n) if s.starts_with('{') => {
            let inner = s.strip_prefix('{')?.strip_suffix('}')?;
            let items: Vec<String> = split_args(inner).iter().map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
            if items.len() > *n {
                return None;
            }
            if items.is_empty() {
                return Some(zero(t));
            }
            let mut v: Vec<String> = items.iter().map(|x| init(x, el)).collect::<Option<_>>()?;
            while v.len() < *n {
                v.push(zero(el));
            }
            Some(format!("[{}]", v.join(", ")))
        }
        T::St(sn) if s.starts_with('{') => {
            let inner = s.strip_prefix('{')?.strip_suffix('}')?;
            let items: Vec<String> = split_args(inner).iter().map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
            let fs = STRUCTS.with(|m| m.borrow().get(sn).cloned())?;
            if items.len() > fs.len() {
                return None;
            }
            let parts: Vec<String> = fs.iter().enumerate().map(|(k, (fname, ft))| Some(format!("{fname}: {}", match items.get(k) { Some(x) => init(x, ft)?, None => zero(ft) }))).collect::<Option<_>>()?;
            Some(format!("{sn} {{ {} }}", parts.join(", ")))
        }
        _ => {
            let (e, et) = ex(s)?;
            if et == *t {
                Some(e)
            } else {
                coerce(&e, &et, t)
            }
        }
    }
}

/// `[2][3]` を寸法の並びへ(空は None)
fn dims(s: &str) -> Option<Vec<Option<usize>>> {
    Regex::new(r"\[\s*(\d*)\s*\]").unwrap().captures_iter(s).map(|c| if c[1].is_empty() { Some(None) } else { c[1].parse().ok().map(Some) }).collect()
}

fn wrap(base: T, ds: &[Option<usize>]) -> Option<T> {
    let mut t = base;
    for d in ds.iter().rev() {
        t = T::Arr(Box::new(t), (*d)?);
    }
    Some(t)
}

const TYPES: &str = r"struct\s+\w+|long\s+long|unsigned\s+int|int|long|double|float|short|unsigned";

/// 関数の引数 1 つ: (名前, 型)
fn param(p: &str) -> Option<(String, T)> {
    let re = Regex::new(&format!(r"^({TYPES})\s*(\*?)\s*(\w+)((?:\s*\[\s*\d*\s*\])*)$")).unwrap();
    let c = re.captures(p.trim())?;
    let b = base_ty(&c[1])?;
    let ds = dims(&c[4])?;
    let t = if !c[2].is_empty() {
        if !ds.is_empty() {
            return None;
        }
        T::Ptr(Box::new(b))
    } else if ds.is_empty() {
        b
    } else {
        T::Slice(Box::new(wrap(b, &ds[1..])?))
    };
    Some((c[3].to_string(), t))
}

/// `a = b;` を (左辺, 演算子, 右辺) へ。`==` などは代入ではない。
fn split_assign(l: &str) -> Option<(String, String, String)> {
    let b = l.strip_suffix(';')?.as_bytes();
    let mut d = 0i32;
    for k in 0..b.len() {
        match b[k] {
            b'(' | b'[' => d += 1,
            b')' | b']' => d -= 1,
            b'=' if d == 0 => {
                if k + 1 < b.len() && b[k + 1] == b'=' {
                    return None;
                }
                let pc = if k > 0 { b[k - 1] } else { b' ' };
                if matches!(pc, b'=' | b'!' | b'<' | b'>') {
                    return None;
                }
                let (lhs_end, op) = if matches!(pc, b'+' | b'-' | b'*' | b'/' | b'%') { (k - 1, (pc as char).to_string()) } else { (k, String::new()) };
                let s = std::str::from_utf8(b).ok()?;
                return Some((s[..lhs_end].trim().to_string(), op, s[k + 1..].trim().to_string()));
            }
            _ => {}
        }
    }
    None
}

fn assign(l: &str) -> Option<String> {
    let (lhs, op, rhs) = split_assign(l)?;
    let (ls, lt) = ex(&lhs)?;
    let (rs_, rt) = ex(&rhs)?;
    if op.is_empty() {
        return Some(format!("{ls} = {};", init(&rhs, &lt).or_else(|| coerce(&rs_, &rt, &lt))?));
    }
    if lt == rt {
        return Some(format!("{ls} {op}= {rs_};"));
    }
    // 型が違う複合代入は C と同じく「計算してから左辺の型へ戻す」
    let (e, t) = bin(&op, (ls.clone(), lt.clone()), (rs_, rt))?;
    Some(format!("{ls} = {};", coerce(&e, &t, &lt)?))
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let func = Regex::new(&format!(r"^({TYPES}|void)\s+(\w+)\s*\(([^)]*)\)\s*\{{?\s*$")).unwrap();
    let decl = Regex::new(&format!(r"^({TYPES})\s+(\w+)((?:\s*\[\s*\d*\s*\])*)\s*(?:=\s*(.+))?;$")).unwrap();
    let sdef = Regex::new(r"^struct\s+(\w+)\s*\{$").unwrap();
    let forl = Regex::new(r"^for\s*\(\s*(?:int|long)?\s*(\w+)\s*=\s*([^;]+);\s*(\w+)\s*(<=|<)\s*([^;]+);\s*(\w+)(\+\+|\s*\+=\s*1)\s*\)\s*\{$").unwrap();
    let ifl = Regex::new(r"^(\}\s*else\s+)?if\s*\((.*)\)\s*\{$").unwrap();
    let whl = Regex::new(r"^while\s*\((.*)\)\s*\{$").unwrap();
    let call = Regex::new(r"^(printf|puts)\s*\((.*)\)\s*;$").unwrap();
    let cout = Regex::new(r"^(?:std::)?cout\s*<<\s*(.*);$").unwrap();
    let incp = Regex::new(r"^(.+?)\s*(\+\+|--)\s*;$").unwrap();
    let incq = Regex::new(r"^(\+\+|--)\s*(.+);$").unwrap();
    let callst = Regex::new(r"^\w+\s*\(.*\)\s*;$").unwrap();
    VARS.with(|v| v.borrow_mut().clear());
    STRUCTS.with(|v| v.borrow_mut().clear());
    FUNCS.with(|v| v.borrow_mut().clear());
    // 先に構造体と関数の形を集める(定義の順番によらず呼べるように)
    {
        let mut cur: Option<(String, Vec<(String, T)>)> = None;
        let field = Regex::new(&format!(r"^({TYPES})\s+(\w+)((?:\s*\[\s*\d+\s*\])*)\s*;$")).unwrap();
        for raw in src.lines() {
            let l = raw.split("//").next().unwrap_or("").trim();
            if let Some(c) = sdef.captures(l) {
                cur = Some((c[1].to_string(), vec![]));
                continue;
            }
            if let Some((n, fs)) = cur.as_mut() {
                if l.starts_with('}') {
                    let (n, fs) = (n.clone(), std::mem::take(fs));
                    STRUCTS.with(|m| m.borrow_mut().insert(n, fs));
                    cur = None;
                } else if let Some(c) = field.captures(l) {
                    if let Some(t) = base_ty(&c[1]).and_then(|b| wrap(b, &dims(&c[3])?)) {
                        fs.push((c[2].to_string(), t));
                    }
                }
                continue;
            }
            if let Some(c) = func.captures(l) {
                let ret = if &c[1] == "void" || &c[2] == "main" { None } else { base_ty(&c[1]) };
                let ps = c[3].trim();
                let pv: Option<Vec<T>> = if ps.is_empty() || ps == "void" { Some(vec![]) } else { ps.split(',').map(|p| param(p).map(|x| x.1)).collect() };
                if let Some(pv) = pv {
                    FUNCS.with(|m| m.borrow_mut().insert(c[2].to_string(), (pv, ret)));
                }
            }
        }
    }
    let mut out = String::from("// pl2pl c2rust で変換。TODO(c2rust) の行は手で直してください。\n#![allow(unused_mut, unused_variables, unused_parens, unused_assignments, non_snake_case)]\n\n");
    let mut in_main = false;
    let mut ret_ty: Option<T> = None;
    let mut stack: Vec<bool> = vec![];
    let mut in_struct = false;
    let mut todo = 0;
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split("//").next().unwrap_or("").trim();
        if l.is_empty() || l.starts_with("#include") || l.starts_with("using namespace") {
            continue;
        }
        let ind = "    ".repeat(stack.len());
        if let Some(c) = sdef.captures(l) {
            if let Some(fs) = STRUCTS.with(|m| m.borrow().get(&c[1]).cloned()) {
                out.push_str(&format!("#[derive(Clone, Copy, Default, Debug)]\nstruct {} {{\n", &c[1]));
                for (n, t) in fs {
                    out.push_str(&format!("    {n}: {},\n", rs(&t)));
                }
                out.push_str("}\n\n");
                in_struct = true;
                continue;
            }
        }
        if in_struct {
            if l.starts_with('}') {
                in_struct = false;
            }
            continue;
        }
        if let Some(c) = func.captures(l) {
            let ps = c[3].trim();
            let pv: Option<Vec<(String, T)>> = if ps.is_empty() || ps == "void" { Some(vec![]) } else { ps.split(',').map(param).collect() };
            if let Some(pv) = pv {
                let is_main = &c[2] == "main";
                let rt = if is_main || &c[1] == "void" { None } else { base_ty(&c[1]) };
                let r = rt.as_ref().map(|t| format!(" -> {}", rs(t))).unwrap_or_default();
                let args: Vec<String> = pv
                    .iter()
                    .map(|(n, t)| {
                        set_var(n, t.clone());
                        if matches!(t, T::Slice(_) | T::Ptr(_)) { format!("{n}: {}", rs(t)) } else { format!("mut {n}: {}", rs(t)) }
                    })
                    .collect();
                out.push_str(&format!("{ind}fn {}({}){r} {{\n", &c[2], args.join(", ")));
                stack.push(is_main);
                in_main = is_main;
                ret_ty = rt;
                continue;
            }
        }
        if l == "}" {
            stack.pop();
            out.push_str(&format!("{}}}\n", "    ".repeat(stack.len())));
            continue;
        }
        if l.starts_with("} else {") || l == "else {" {
            out.push_str(&format!("{}}} else {{\n", "    ".repeat(stack.len().saturating_sub(1))));
            continue;
        }
        let mut emit: Option<String> = None;
        if let Some(c) = forl.captures(l) {
            if c[1] == c[3] && c[1] == c[6] {
                set_var(&c[1], T::I32);
                if let (Some(a), Some(b)) = (ex_to(c[2].trim(), &T::I32), ex_to(c[5].trim(), &T::I32)) {
                    out.push_str(&format!("{ind}for {} in {a}..{}{b} {{\n", &c[1], if &c[4] == "<=" { "=" } else { "" }));
                    stack.push(false);
                    continue;
                }
            }
        }
        if let Some(c) = ifl.captures(l) {
            if let Some(e) = cond(c[2].trim()) {
                let pre = if c.get(1).is_some() { format!("{}}} else ", "    ".repeat(stack.len().saturating_sub(1))) } else { ind.clone() };
                out.push_str(&format!("{pre}if {e} {{\n"));
                if c.get(1).is_none() {
                    stack.push(false);
                }
                continue;
            }
        }
        if let Some(c) = whl.captures(l) {
            let cd = c[1].trim();
            if Regex::new(r"^[1-9]\d*$").unwrap().is_match(cd) {
                emit = Some("loop {".into());
            } else if let Some(e) = cond(cd) {
                emit = Some(format!("while {e} {{"));
            }
            if let Some(e) = emit {
                out.push_str(&format!("{ind}{e}\n"));
                stack.push(false);
                continue;
            }
        }
        if let Some(c) = call.captures(l) {
            emit = if &c[1] == "puts" {
                let a = c[2].trim();
                (a.starts_with('"') && a.ends_with('"')).then(|| format!("println!({a});"))
            } else {
                fmt(&c[2]).map(|f| format!("print!({f});"))
            };
        } else if let Some(c) = cout.captures(l) {
            let (mut f, mut a, mut nl, mut ok) = (String::new(), vec![], false, true);
            for it in c[1].split("<<").map(str::trim) {
                if it == "endl" || it == "std::endl" {
                    nl = true;
                } else if it.starts_with('"') && it.ends_with('"') && it.len() >= 2 {
                    f.push_str(&it[1..it.len() - 1].replace('{', "{{").replace('}', "}}"));
                } else if let Some((e, _)) = ex(it) {
                    f.push_str("{}");
                    a.push(e);
                } else {
                    ok = false;
                }
            }
            if nl {
                f.push_str("\n");
            }
            if ok {
                emit = Some(format!("print!(\"{f}\"{});", a.iter().map(|x| format!(", {x}")).collect::<String>()));
            }
        } else if l == "break;" || l == "continue;" {
            emit = Some(l.to_string());
        } else if let Some(c) = decl.captures(l) {
            emit = (|| {
                let b = base_ty(&c[1])?;
                let mut ds = dims(&c[3])?;
                if ds.first() == Some(&None) {
                    let it = c.get(4)?.as_str().trim();
                    let inner = it.strip_prefix('{')?.strip_suffix('}')?;
                    ds[0] = Some(split_args(inner).iter().filter(|x| !x.trim().is_empty()).count());
                }
                let t = wrap(b, &ds)?;
                let v = match c.get(4) {
                    Some(m) => init(m.as_str(), &t)?,
                    None => zero(&t),
                };
                set_var(&c[2], t.clone());
                Some(format!("let mut {}: {} = {v};", &c[2], rs(&t)))
            })();
        } else if let Some(r) = l.strip_prefix("return").filter(|r| r.starts_with(' ') || r.starts_with(';')) {
            let r = r.trim().trim_end_matches(';').trim();
            emit = if in_main || r.is_empty() {
                Some("return;".into())
            } else {
                ret_ty.as_ref().and_then(|t| init(r, t)).map(|e| format!("return {e};"))
            };
        } else if let Some(a) = assign(l) {
            emit = Some(a);
        } else if let Some(c) = incp.captures(l).map(|c| (c[1].to_string(), c[2].to_string())).or_else(|| incq.captures(l).map(|c| (c[2].to_string(), c[1].to_string()))) {
            emit = ex(&c.0).filter(|(_, t)| is_num(t)).map(|(e, t)| format!("{e} {}= {};", if c.1 == "++" { "+" } else { "-" }, if is_float(&t) { "1.0" } else { "1" }));
        } else if callst.is_match(l) {
            emit = ex(l.trim_end_matches(';')).map(|(e, _)| format!("{e};"));
        }
        if let Some(e) = emit {
            out.push_str(&format!("{ind}{e}\n"));
            continue;
        }
        todo += 1;
        out.push_str(&format!("{ind}// TODO(c2rust): {l}\n"));
        rep.push(format!("{name}:{}: 未対応 `{l}`", i + 1));
    }
    if todo > 0 {
        rep.push(format!("{name}: 未対応の行 {todo} 個(TODO(c2rust) として残しました)"));
    }
    let base = name.rsplit('/').next().unwrap_or(name).rsplit_once('.').map(|x| x.0).unwrap_or("prog");
    vec![(format!("{base}.rs"), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn conv(s: &str) -> (String, Vec<String>) {
        let mut r = vec![];
        let o = convert(s, "t.c", &mut r);
        (o[0].1.clone(), r)
    }
    #[test]
    fn basic() {
        let (t, r) = conv("#include <stdio.h>\nint main(void) {\n  int s = 0;\n  for (int i = 1; i <= 3; i++) {\n    s += i;\n  }\n  printf(\"sum=%d %.1f\\n\", s, 2.5);\n  return 0;\n}\n");
        assert!(t.contains("fn main() {"), "{t}");
        assert!(t.contains("for i in 1..=3 {"), "{t}");
        assert!(t.contains("print!(\"sum={} {:.1}\\n\", s, 2.5);"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn unsupported_kept() {
        let (t, r) = conv("int main() {\n  int *p = 0;\n}\n");
        assert!(t.contains("TODO(c2rust)") && !r.is_empty());
    }
    #[test]
    fn arrays_break_continue() {
        let (t, r) = conv("int main() {\n  int a[5] = {3, 1, 4};\n  double w[3];\n  int i = 0;\n  a[1] = a[i + 2] * 2;\n  w[1] = w[0] + 1;\n  while (1) {\n    i++;\n    if (i > 3) {\n      break;\n    }\n    continue;\n  }\n  return 0;\n}\n");
        assert!(t.contains("let mut a: [i32; 5] = [3, 1, 4, 0, 0];"), "{t}");
        assert!(t.contains("let mut w: [f64; 3] = [0.0; 3];"), "{t}");
        assert!(t.contains("a[1] = a[(i + 2) as usize] * 2;"), "{t}");
        assert!(t.contains("w[1] = w[0] + 1.0;"), "{t}");
        assert!(t.contains("loop {"), "{t}");
        assert!(t.contains("break;") && t.contains("continue;"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn implicit_promotion() {
        let (t, r) = conv("int main() {\n  int k = 3;\n  double x = k * 1.5;\n  double y = k / 2 + x;\n  int n = x;\n  n += x;\n  double z = k;\n  return 0;\n}\n");
        assert!(t.contains("let mut x: f64 = ((k) as f64) * 1.5;"), "{t}");
        // k / 2 は整数の割り算のまま、その後で小数へ
        assert!(t.contains("let mut y: f64 = ((k / 2) as f64) + x;"), "{t}");
        assert!(t.contains("let mut n: i32 = ((x) as i32);"), "{t}");
        assert!(t.contains("n = ((((n) as f64) + x) as i32);"), "{t}");
        assert!(t.contains("let mut z: f64 = ((k) as f64);"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn array_params_2d_struct_ptr() {
        let src = "struct P {\n  int x;\n  double y;\n};\nint sum(int a[], int n) {\n  int s = 0;\n  for (int i = 0; i < n; i++) {\n    s += a[i];\n  }\n  return s;\n}\nvoid inc(int *x) {\n  *x = *x + 1;\n}\nvoid mv(struct P *p) {\n  p->x += 1;\n}\nint main() {\n  int a[3] = {1, 2, 3};\n  int m[2][2] = {{1, 2}, {3}};\n  struct P p = {1, 2.5};\n  struct P ps[2];\n  int v = 0;\n  inc(&v);\n  mv(&p);\n  ps[1].y = p.x * 0.5;\n  m[1][1] = sum(a, 3);\n  return 0;\n}\n";
        let (t, r) = conv(src);
        assert!(t.contains("struct P {\n    x: i32,\n    y: f64,\n}"), "{t}");
        assert!(t.contains("fn sum(a: &mut [i32], mut n: i32) -> i32 {"), "{t}");
        assert!(t.contains("fn inc(x: &mut i32) {"), "{t}");
        assert!(t.contains("*x = *x + 1;"), "{t}");
        assert!(t.contains("p.x += 1;"), "{t}");
        assert!(t.contains("let mut m: [[i32; 2]; 2] = [[1, 2], [3, 0]];"), "{t}");
        assert!(t.contains("let mut p: P = P { x: 1, y: 2.5 };"), "{t}");
        assert!(t.contains("let mut ps: [P; 2] = [P::default(); 2];"), "{t}");
        assert!(t.contains("inc(&mut v);"), "{t}");
        assert!(t.contains("ps[1].y = ((p.x) as f64) * 0.5;"), "{t}");
        assert!(t.contains("m[1][1] = sum(&mut a, 3);"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
}
