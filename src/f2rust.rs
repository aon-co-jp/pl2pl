//! `pl2pl f2rust`: とても単純な Fortran(program、integer / real、do、if、print *)を Rust へ書き換える最初の一歩。
//! わからない行は黙って捨てず `// TODO(f2rust): 元の行` として残し、レポートにも載せる。
//! 配列・subroutine/function・module・書式付き出力・`**` は未対応(TODO のまま)。
//! `print *` は gfortran に合わせる(各項目の前に空白 1 個、整数は幅 11)。小数(real)の桁数は近似で、gfortran と一致するのは整数と文字列。
use regex::Regex;

thread_local! { static INTS: std::cell::RefCell<std::collections::HashSet<String>> = Default::default(); }

/// 整数だけの式か(変数が integer と宣言済み、または整数リテラル)。gfortran の `print *` は整数を幅 11 で出す。
fn is_int_expr(e: &str) -> bool {
    let ids: Vec<String> = Regex::new(r"[A-Za-z_]\w*").unwrap().find_iter(e).map(|m| m.as_str().to_string()).collect();
    !e.contains('.') && INTS.with(|s| ids.iter().all(|i| s.borrow().contains(i)))
}

pub(crate) fn ops(e: &str) -> String {
    let mut s = e.to_string();
    for (a, b) in [(".eq.", "=="), (".ne.", "!="), (".gt.", ">"), (".lt.", "<"), (".ge.", ">="), (".le.", "<="), (".and.", "&&"), (".or.", "||"), (".not.", "!"), ("/=", "!=")] {
        let re = Regex::new(&format!("(?i){}", regex::escape(a))).unwrap();
        s = re.replace_all(&s, b).to_string();
    }
    s
}

pub(crate) fn split_commas(s: &str) -> Vec<String> {
    let (mut v, mut d, mut q, mut cur) = (vec![], 0, None::<char>, String::new());
    for c in s.chars() {
        match q {
            Some(qc) if c == qc => q = None,
            Some(_) => {}
            None => match c {
                '"' | '\'' => q = Some(c),
                '(' => d += 1,
                ')' => d -= 1,
                ',' if d == 0 => {
                    v.push(std::mem::take(&mut cur));
                    continue;
                }
                _ => {}
            },
        }
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        v.push(cur);
    }
    v
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let prog = Regex::new(r"(?i)^program\s+\w+$").unwrap();
    let endprog = Regex::new(r"(?i)^end\s*program(\s+\w+)?$").unwrap();
    let decl = Regex::new(r"(?i)^(integer|real|double\s+precision)(\s*\(\s*\w+\s*\))?\s*(::)?\s*(.+)$").unwrap();
    let dol = Regex::new(r"(?i)^do\s+(\w+)\s*=\s*([^,]+),\s*([^,]+)$").unwrap();
    let enddo = Regex::new(r"(?i)^end\s*do$").unwrap();
    let ifl = Regex::new(r"(?i)^(else\s+)?if\s*\((.*)\)\s*then$").unwrap();
    let els = Regex::new(r"(?i)^else$").unwrap();
    let endif = Regex::new(r"(?i)^end\s*if$").unwrap();
    let print = Regex::new(r"(?i)^print\s*\*\s*,\s*(.+)$").unwrap();
    let assign = Regex::new(r"^\w+\s*=\s*[^=].*$").unwrap();
    let mut out = String::from("// pl2pl f2rust で変換(最初の一歩)。TODO(f2rust) の行は手で直してください。\n#![allow(unused_mut, unused_variables, unused_parens)]\n\n");
    INTS.with(|s| s.borrow_mut().clear());
    let mut depth = 0usize;
    let mut todo = 0;
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split('!').next().unwrap_or("").trim();
        if l.is_empty() || l.eq_ignore_ascii_case("implicit none") {
            continue;
        }
        let ind = "    ".repeat(depth);
        if prog.is_match(l) {
            out.push_str("fn main() {\n");
            depth += 1;
        } else if endprog.is_match(l) || enddo.is_match(l) || endif.is_match(l) {
            depth = depth.saturating_sub(1);
            out.push_str(&format!("{}}}\n", "    ".repeat(depth)));
        } else if let Some(c) = dol.captures(l) {
            out.push_str(&format!("{ind}for {} in {}..={} {{\n", &c[1], ops(c[2].trim()), ops(c[3].trim())));
            depth += 1;
        } else if let Some(c) = ifl.captures(l) {
            if c.get(1).is_some() {
                out.push_str(&format!("{}}} else if {} {{\n", "    ".repeat(depth.saturating_sub(1)), ops(c[2].trim())));
            } else {
                out.push_str(&format!("{ind}if {} {{\n", ops(c[2].trim())));
                depth += 1;
            }
        } else if els.is_match(l) {
            out.push_str(&format!("{}}} else {{\n", "    ".repeat(depth.saturating_sub(1))));
        } else if let Some(c) = decl.captures(l) {
            let t = if c[1].to_lowercase().starts_with("integer") { "i32" } else { "f64" };
            for v in split_commas(&c[4]) {
                let (n, init) = match v.split_once('=') {
                    Some((a, b)) => (a.trim().to_string(), ops(b.trim())),
                    None => (v.trim().to_string(), if t == "i32" { "0".into() } else { "0.0".into() }),
                };
                if n.contains('(') {
                    todo += 1;
                    out.push_str(&format!("{ind}// TODO(f2rust): 配列は未対応: {l}\n"));
                    rep.push(format!("{name}:{}: 配列は未対応 `{l}`", i + 1));
                    break;
                }
                if t == "i32" {
                    INTS.with(|s| s.borrow_mut().insert(n.clone()));
                }
                out.push_str(&format!("{ind}let mut {n}: {t} = {init};\n"));
            }
        } else if let Some(c) = print.captures(l) {
            // gfortran の `print *`: 各項目の前に空白 1 個、整数は幅 11、文字列はそのまま(実機 gfortran 16 で確認)
            let (mut f, mut a) = (String::new(), vec![]);
            for it in split_commas(&c[1]) {
                let it = it.trim();
                if (it.starts_with(char::from(39)) && it.ends_with(char::from(39)) || it.starts_with('"') && it.ends_with('"')) && it.len() >= 2 {
                    f.push(' ');
                    f.push_str(&it[1..it.len() - 1].replace('{', "{{").replace('}', "}}"));
                } else if is_int_expr(it) {
                    f.push_str(" {:>11}");
                    a.push(ops(it));
                } else {
                    f.push_str(" {}");
                    a.push(ops(it));
                }
            }
            out.push_str(&format!("{ind}println!(\"{f}\"{});
", a.iter().map(|x| format!(", {x}")).collect::<String>()));
        } else if assign.is_match(l) && !l.contains("**") {
            out.push_str(&format!("{ind}{};\n", ops(l)));
        } else {
            todo += 1;
            out.push_str(&format!("{ind}// TODO(f2rust): {l}\n"));
            rep.push(format!("{name}:{}: 未対応 `{l}`", i + 1));
        }
    }
    if todo > 0 {
        rep.push(format!("{name}: 未対応の行 {todo} 個(TODO(f2rust) として残しました)"));
    }
    let base = name.rsplit('/').next().unwrap_or(name);
    let base = base.rsplit_once('.').map(|x| x.0).unwrap_or(base);
    vec![(format!("{base}.rs"), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut r = vec![];
        let o = convert("program t\n  implicit none\n  integer :: i, s = 0\n  do i = 1, 3\n    s = s + i\n  end do\n  if (s .gt. 5) then\n    print *, 'sum', s\n  else\n    print *, 'small'\n  end if\nend program t\n", "t.f90", &mut r);
        let t = &o[0].1;
        assert!(t.contains("for i in 1..=3 {") && t.contains("if s > 5 {") && t.contains("println!(\" sum {:>11}\", s);"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn unsupported_kept() {
        let mut r = vec![];
        let o = convert("program t\n  real :: a(3)\n  call foo()\nend program t\n", "u.f90", &mut r);
        assert!(o[0].1.contains("TODO(f2rust)") && r.len() >= 2);
    }
}
