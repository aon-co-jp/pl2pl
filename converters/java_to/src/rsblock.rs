//! 複数文のメソッド本体を Rust にする(ローカル変数・代入・if/else・for/while・return)。
//! 対応外の文に当たったら None を返し、呼び出し側は従来どおり TODO の応答にする。
//! `return` は `@RET(式)@` の目印で出し、ハンドラ側(Poem / RPoem)が戻り値の形に包む。

use crate::lexer::Tok;
use crate::rsgen::{rs_type, rust_ident};
use std::collections::HashSet;

fn rs_expr(toks: &[Tok], vars: &[(String, String)], locals: &HashSet<String>) -> Option<String> {
    if toks.is_empty() {
        return None;
    }
    let parts = crate::parse::split_top(toks, "+");
    if parts.len() > 1 && parts.iter().any(|p| matches!(p.as_slice(), [Tok::Str(_)])) {
        let mut fmt = String::new();
        let mut args = vec![];
        for p in &parts {
            if let [Tok::Str(s)] = p.as_slice() {
                fmt.push_str(&s.replace('{', "{{").replace('}', "}}"));
            } else {
                fmt.push_str("{}");
                args.push(rs_expr(p, vars, locals)?);
            }
        }
        return Some(format!("format!({:?}{})", fmt, args.iter().map(|a| format!(", {a}")).collect::<String>()));
    }
    let mut out: Vec<String> = vec![];
    let mut k = 0;
    while k < toks.len() {
        let t = &toks[k];
        k += 1;
        // `x.m(...)` / `Math.f(...)` の単純な呼び出し
        if let (Tok::Id(recv), Some(d), Some(Tok::Id(m)), Some(o)) = (t, toks.get(k), toks.get(k + 1), toks.get(k + 2)) {
            if d.is(".") && o.is("(") {
                let close = match_close(toks, k + 2, "(", ")")?;
                let args: Vec<Vec<Tok>> = crate::parse::split_top(&toks[k + 3..close], ",").into_iter().filter(|a| !a.is_empty()).collect();
                let a: Option<Vec<String>> = args.iter().map(|x| rs_expr(x, vars, locals)).collect();
                let a = a?;
                let r = if recv == "Math" {
                    match (m.as_str(), a.as_slice()) {
                        ("max", [x, y]) => format!("({x}).max({y})"),
                        ("min", [x, y]) => format!("({x}).min({y})"),
                        ("abs", [x]) => format!("({x}).abs()"),
                        ("pow", [x, y]) => format!("({x} as f64).powf({y} as f64)"),
                        ("sqrt", [x]) => format!("({x} as f64).sqrt()"),
                        _ => return None,
                    }
                } else if recv == "String" && m == "valueOf" && a.len() == 1 {
                    format!("({}).to_string()", a[0])
                } else {
                    let rv = if locals.contains(recv) { rust_ident(recv) } else { vars.iter().find(|(j, _)| j == recv)?.1.clone() };
                    match (m.as_str(), a.as_slice()) {
                        ("length", []) | ("size", []) => format!("({rv}.len() as i32)"),
                        ("toUpperCase", []) => format!("{rv}.to_uppercase()"),
                        ("toLowerCase", []) => format!("{rv}.to_lowercase()"),
                        ("trim", []) => format!("{rv}.trim().to_string()"),
                        ("isEmpty", []) => format!("{rv}.is_empty()"),
                        ("contains", [x]) => format!("{rv}.contains({x}.as_str())"),
                        ("startsWith", [x]) => format!("{rv}.starts_with({x}.as_str())"),
                        ("endsWith", [x]) => format!("{rv}.ends_with({x}.as_str())"),
                        ("equals", [x]) => format!("({rv} == {x})"),
                        ("get", [x]) => format!("{rv}[({x}) as usize].clone()"),
                        _ => return None,
                    }
                };
                out.push(r);
                k = close + 1;
                continue;
            }
        }
        match t {
            Tok::Num(n) => out.push(n.trim_end_matches(['L', 'l', 'f', 'F', 'd', 'D']).to_string()),
            Tok::Str(s) => out.push(format!("{:?}.to_string()", s)),
            Tok::Id(v) if v == "true" || v == "false" => out.push(v.clone()),
            Tok::Id(v) if locals.contains(v) => out.push(rust_ident(v)),
            Tok::Id(v) => out.push(vars.iter().find(|(j, _)| j == v)?.1.clone()),
            Tok::P(p) if matches!(p.as_str(), "+" | "-" | "*" | "/" | "%" | "(" | ")" | "<" | ">" | "<=" | ">=" | "==" | "!=" | "&&" | "||" | "!") => out.push(p.clone()),
            _ => return None,
        }
    }
    Some(out.join(" "))
}

fn is_type_start(t: &Tok) -> bool {
    matches!(t, Tok::Id(s) if matches!(s.as_str(), "int" | "long" | "double" | "float" | "boolean" | "String" | "short" | "byte" | "var"))
}

fn stmt_end(toks: &[Tok], from: usize) -> Option<usize> {
    let mut d = 0i32;
    for (k, x) in toks.iter().enumerate().skip(from) {
        if x.is("(") || x.is("{") || x.is("[") {
            d += 1;
        } else if x.is(")") || x.is("}") || x.is("]") {
            d -= 1;
        } else if x.is(";") && d == 0 {
            return Some(k);
        }
    }
    None
}

fn match_close(toks: &[Tok], open: usize, o: &str, c: &str) -> Option<usize> {
    if !toks.get(open)?.is(o) {
        return None;
    }
    let mut d = 0i32;
    for (k, x) in toks.iter().enumerate().skip(open) {
        if x.is(o) {
            d += 1;
        } else if x.is(c) {
            d -= 1;
            if d == 0 {
                return Some(k);
            }
        }
    }
    None
}

/// 次の文(波かっこ付きなら中身)と、その直後の位置。
fn sub_stmt(toks: &[Tok], from: usize) -> Option<(&[Tok], usize)> {
    let t = toks.get(from)?;
    if t.is("{") {
        let e = match_close(toks, from, "{", "}")?;
        return Some((&toks[from + 1..e], e + 1));
    }
    if t.id("if") || t.id("for") || t.id("while") {
        let c = match_close(toks, from + 1, "(", ")")?;
        let (_, mut n) = sub_stmt(toks, c + 1)?;
        if t.id("if") && toks.get(n).is_some_and(|x| x.id("else")) {
            n = sub_stmt(toks, n + 1)?.1;
        }
        return Some((&toks[from..n], n));
    }
    let e = stmt_end(toks, from)?;
    Some((&toks[from..=e], e + 1))
}

pub fn rs_block(toks: &[Tok], vars: &[(String, String)], locals: &mut HashSet<String>, dtos: &HashSet<String>, depth: usize) -> Option<String> {
    let ind = "    ".repeat(depth + 1);
    let mut out = String::new();
    let mut i = 0;
    while i < toks.len() {
        let t = &toks[i];
        if t.is(";") {
            i += 1;
        } else if t.id("return") {
            let e = stmt_end(toks, i)?;
            let ex = rs_expr(&toks[i + 1..e], vars, locals)?;
            out.push_str(&format!("{ind}return @RET({ex})@;\n"));
            i = e + 1;
        } else if t.id("if") {
            let c = match_close(toks, i + 1, "(", ")")?;
            let cond = rs_expr(&toks[i + 2..c], vars, locals)?;
            let (then, next) = sub_stmt(toks, c + 1)?;
            out.push_str(&format!("{ind}if {cond} {{\n{}{ind}}}", rs_block(then, vars, &mut locals.clone(), dtos, depth + 1)?));
            i = next;
            if toks.get(i).is_some_and(|x| x.id("else")) {
                let (els, n2) = sub_stmt(toks, i + 1)?;
                out.push_str(&format!(" else {{\n{}{ind}}}", rs_block(els, vars, &mut locals.clone(), dtos, depth + 1)?));
                i = n2;
            }
            out.push('\n');
        } else if t.id("while") {
            let c = match_close(toks, i + 1, "(", ")")?;
            let cond = rs_expr(&toks[i + 2..c], vars, locals)?;
            let (b, next) = sub_stmt(toks, c + 1)?;
            out.push_str(&format!("{ind}while {cond} {{\n{}{ind}}}\n", rs_block(b, vars, &mut locals.clone(), dtos, depth + 1)?));
            i = next;
        } else if t.id("for") {
            let c = match_close(toks, i + 1, "(", ")")?;
            let sp = crate::parse::split_top(&toks[i + 2..c], ";");
            let (b, next) = sub_stmt(toks, c + 1)?;
            // 拡張 for: for (int x : xs)
            if let [Tok::Id(_), Tok::Id(v), colon, Tok::Id(coll)] = &toks[i + 2..c] {
                if colon.is(":") && locals.contains(coll) {
                    let mut inner = locals.clone();
                    inner.insert(v.clone());
                    out.push_str(&format!("{ind}for {} in {}.clone() {{
{}{ind}}}
", rust_ident(v), rust_ident(coll), rs_block(b, vars, &mut inner, dtos, depth + 1)?));
                    i = next;
                    continue;
                }
            }
            if !(sp.len() == 3 && sp[0].len() >= 4 && is_type_start(&sp[0][0]) && sp[0][2].is("=")) {
                return None;
            }
            let Tok::Id(v) = &sp[0][1] else { return None };
            let from = rs_expr(&sp[0][3..], vars, locals)?;
            let mut inner = locals.clone();
            inner.insert(v.clone());
            let (op, lim) = match sp[1].as_slice() {
                [Tok::Id(a), Tok::P(o), rest @ ..] if a == v && (o == "<" || o == "<=") && !rest.is_empty() => (o.clone(), rest),
                _ => return None,
            };
            let step_ok = matches!(sp[2].as_slice(), [Tok::Id(a), Tok::P(p)] if a == v && p == "++") || matches!(sp[2].as_slice(), [Tok::P(p), Tok::Id(a)] if a == v && p == "++");
            if !step_ok {
                return None;
            }
            let to = rs_expr(lim, vars, &inner)?;
            let range = if op == "<" { format!("{from}..{to}") } else { format!("{from}..={to}") };
            out.push_str(&format!("{ind}for {} in {range} {{\n{}{ind}}}\n", rust_ident(v), rs_block(b, vars, &mut inner, dtos, depth + 1)?));
            i = next;
        } else if (t.id("List") || t.id("ArrayList")) && matches!(toks.get(i + 1), Some(x) if x.is("<")) && matches!(toks.get(i + 3), Some(x) if x.is(">")) {
            // List<T> xs = new ArrayList<>();
            let (Tok::Id(ety), Some(Tok::Id(v))) = (&toks[i + 2], toks.get(i + 4)) else { return None };
            if !toks.get(i + 5).is_some_and(|x| x.is("=")) || !toks.get(i + 6).is_some_and(|x| x.id("new")) {
                return None;
            }
            let e = stmt_end(toks, i)?;
            let ok = toks[i + 7..e].iter().all(|x| match x {
                Tok::Id(n) => n == "ArrayList" || n == "LinkedList" || n == ety,
                Tok::P(p) => matches!(p.as_str(), "<" | ">" | "<>" | "(" | ")"),
                _ => false,
            });
            if !ok {
                return None;
            }
            locals.insert(v.clone());
            out.push_str(&format!("{ind}let mut {}: Vec<{}> = Vec::new();
", rust_ident(v), rs_type(match ety.as_str() { "Integer" => "int", "Long" => "long", "Double" => "double", "Float" => "float", "Boolean" => "boolean", o => o }, dtos)));
            i = e + 1;
        } else if matches!((t, toks.get(i + 1), toks.get(i + 2), toks.get(i + 3)), (Tok::Id(v), Some(d), Some(m), Some(o)) if locals.contains(v) && d.is(".") && m.id("add") && o.is("(")) {
            // xs.add(x);
            let Tok::Id(v) = t else { return None };
            let cl = match_close(toks, i + 3, "(", ")")?;
            let arg = rs_expr(&toks[i + 4..cl], vars, locals)?;
            out.push_str(&format!("{ind}{}.push({arg});
", rust_ident(v)));
            i = stmt_end(toks, i)? + 1;
        } else if is_type_start(t) && matches!(toks.get(i + 1), Some(Tok::Id(_))) && toks.get(i + 2).is_some_and(|x| x.is("=") || x.is(";")) {
            let (Tok::Id(ty), Tok::Id(v)) = (t, &toks[i + 1]) else { return None };
            let e = stmt_end(toks, i)?;
            let rt = if ty == "var" { String::new() } else { format!(": {}", rs_type(ty, dtos)) };
            let init = if toks[i + 2].is("=") { rs_expr(&toks[i + 3..e], vars, locals)? } else { "Default::default()".into() };
            locals.insert(v.clone());
            out.push_str(&format!("{ind}let mut {}{rt} = {init};\n", rust_ident(v)));
            i = e + 1;
        } else if let (Tok::Id(v), Some(op)) = (t, toks.get(i + 1)) {
            if !locals.contains(v) {
                return None;
            }
            let e = stmt_end(toks, i)?;
            if matches!(op, Tok::P(p) if matches!(p.as_str(), "=" | "+=" | "-=" | "*=" | "/=" | "%=")) {
                let ex = rs_expr(&toks[i + 2..e], vars, locals)?;
                out.push_str(&format!("{ind}{} {} {ex};\n", rust_ident(v), op.text()));
            } else if op.is("++") || op.is("--") {
                out.push_str(&format!("{ind}{} {}= 1;\n", rust_ident(v), if op.is("++") { "+" } else { "-" }));
            } else {
                return None;
            }
            i = e + 1;
        } else {
            return None;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    fn run(src: &str) -> Option<String> {
        let vars = vec![("n".to_string(), "n".to_string())];
        rs_block(&lex(src), &vars, &mut HashSet::new(), &HashSet::new(), 0)
    }
    #[test]
    fn locals_if_for() {
        let r = run("int s = 0; for (int i = 0; i < n; i++) { if (i % 2 == 0) { s += i; } else { s -= 1; } } return s;").unwrap();
        assert!(r.contains("let mut s: i32 = 0;"));
        assert!(r.contains("for i in 0..n {"));
        assert!(r.contains("} else {"));
        assert!(r.contains("return @RET(s)@;"));
    }
    #[test]
    fn strings() {
        let r = run("String g = \"hi \" + n; return g + \"!\";").unwrap();
        assert!(r.contains("format!(\"hi {}\", n)"));
    }
    #[test]
    fn list_operations() {
        let r = run("List<Integer> xs = new ArrayList<>(); for (int i = 0; i < n; i++) { xs.add(i * 2); } int s = 0; for (int x : xs) { s += x; } int f = xs.get(0); return s + xs.size() + f;").unwrap();
        assert!(r.contains("let mut xs: Vec<i32> = Vec::new();"), "{r}");
        assert!(r.contains("xs.push("), "{r}");
        assert!(r.contains("for x in xs.clone()"), "{r}");
        assert!(r.contains("xs[(0) as usize].clone()"), "{r}");
        assert!(r.contains("xs.len() as i32"), "{r}");
    }
    #[test]
    fn method_calls() {
        let r = run("String u = \"ab\".toUpperCase(); return Math.max(n, 3);");
        assert!(r.is_none());
        let r = run("int m = Math.max(n, 3); int l = Math.abs(m); return l;").unwrap();
        assert!(r.contains("(n).max(3)"));
        assert!(r.contains("(m).abs()"));
    }
    #[test]
    fn unsupported() {
        assert!(run("foo(); return 1;").is_none());
    }
}
