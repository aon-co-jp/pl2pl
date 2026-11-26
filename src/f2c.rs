//! `pl2pl f2c`: とても単純な Fortran(program、integer / real、do〈刻み付き〉、if、print *)を C へ書き換える最初の一歩。
//! わからない行は黙って捨てず `/* TODO(f2c): 元の行 */` として残し、レポートにも載せる。配列・subroutine・module・書式付き出力・`**` は未対応。
//! `print *` は gfortran に合わせる(各項目の前に空白 1 個、整数は幅 11、文字列はそのまま)。real は倍精度 double で書く(Fortran の real は単精度なので、小数の桁は一致しない場合がある)。
use crate::f2rust::{ops, split_commas};
use regex::Regex;
use std::collections::HashSet;

fn is_int_expr(e: &str, ints: &HashSet<String>) -> bool {
    let ids: Vec<String> = Regex::new(r"[A-Za-z_]\w*").unwrap().find_iter(e).map(|m| m.as_str().to_string()).collect();
    !e.contains('.') && ids.iter().all(|i| ints.contains(i))
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let prog = Regex::new(r"(?i)^program\s+\w+$").unwrap();
    let endprog = Regex::new(r"(?i)^end\s*program(\s+\w+)?$").unwrap();
    let decl = Regex::new(r"(?i)^(integer|real|double\s+precision)(\s*\(\s*\w+\s*\))?\s*(::)?\s*(.+)$").unwrap();
    let dol = Regex::new(r"(?i)^do\s+(\w+)\s*=\s*([^,]+),\s*([^,]+)(?:,\s*([^,]+))?$").unwrap();
    let enddo = Regex::new(r"(?i)^end\s*do$").unwrap();
    let ifl = Regex::new(r"(?i)^(else\s+)?if\s*\((.*)\)\s*then$").unwrap();
    let els = Regex::new(r"(?i)^else$").unwrap();
    let endif = Regex::new(r"(?i)^end\s*if$").unwrap();
    let print = Regex::new(r"(?i)^print\s*\*\s*,\s*(.+)$").unwrap();
    let assign = Regex::new(r"^\w+\s*=\s*[^=].*$").unwrap();
    let mut out = String::from("/* pl2pl f2c で変換(最初の一歩)。TODO(f2c) の行は手で直してください。 */\n#include <stdio.h>\n\n");
    let mut ints: HashSet<String> = HashSet::new();
    let (mut depth, mut todo) = (0usize, 0);
    for (i, raw) in src.lines().enumerate() {
        let l = raw.split('!').next().unwrap_or("").trim();
        if l.is_empty() || l.eq_ignore_ascii_case("implicit none") {
            continue;
        }
        let ind = "    ".repeat(depth);
        if prog.is_match(l) {
            out.push_str("int main(void) {\n");
            depth += 1;
        } else if endprog.is_match(l) {
            out.push_str("    return 0;\n}\n");
            depth = depth.saturating_sub(1);
        } else if enddo.is_match(l) || endif.is_match(l) {
            depth = depth.saturating_sub(1);
            out.push_str(&format!("{}}}\n", "    ".repeat(depth)));
        } else if let Some(c) = dol.captures(l) {
            let step = c.get(4).map(|m| ops(m.as_str().trim())).unwrap_or_else(|| "1".into());
            let neg = step.trim_start().starts_with('-');
            let (v, a, b) = (&c[1], ops(c[2].trim()), ops(c[3].trim()));
            let inc = if step == "1" { format!("{v}++") } else { format!("{v} += {step}") };
            out.push_str(&format!("{ind}for ({v} = {a}; {v} {} {b}; {inc}) {{\n", if neg { ">=" } else { "<=" }));
            depth += 1;
        } else if let Some(c) = ifl.captures(l) {
            if c.get(1).is_some() {
                out.push_str(&format!("{}}} else if ({}) {{\n", "    ".repeat(depth.saturating_sub(1)), ops(c[2].trim())));
            } else {
                out.push_str(&format!("{ind}if ({}) {{\n", ops(c[2].trim())));
                depth += 1;
            }
        } else if els.is_match(l) {
            out.push_str(&format!("{}}} else {{\n", "    ".repeat(depth.saturating_sub(1))));
        } else if let Some(c) = decl.captures(l) {
            let is_int = c[1].to_lowercase().starts_with("integer");
            let t = if is_int { "int" } else { "double" };
            for v in split_commas(&c[4]) {
                let (n, init) = match v.split_once('=') {
                    Some((a, b)) => (a.trim().to_string(), ops(b.trim())),
                    None => (v.trim().to_string(), "0".into()),
                };
                if n.contains('(') {
                    todo += 1;
                    out.push_str(&format!("{ind}/* TODO(f2c): 配列は未対応: {l} */\n"));
                    rep.push(format!("{name}:{}: 配列は未対応 `{l}`", i + 1));
                    break;
                }
                if is_int {
                    ints.insert(n.clone());
                }
                out.push_str(&format!("{ind}{t} {n} = {init};\n"));
            }
        } else if let Some(c) = print.captures(l) {
            let (mut f, mut a) = (String::new(), vec![]);
            for it in split_commas(&c[1]) {
                let it = it.trim();
                if (it.starts_with('\'') && it.ends_with('\'') || it.starts_with('"') && it.ends_with('"')) && it.len() >= 2 {
                    f.push(' ');
                    f.push_str(&it[1..it.len() - 1].replace('%', "%%").replace('\\', "\\\\").replace('"', "\\\""));
                } else if is_int_expr(it, &ints) {
                    f.push_str(" %11d");
                    a.push(ops(it));
                } else {
                    f.push_str(" %g");
                    a.push(ops(it));
                }
            }
            out.push_str(&format!("{ind}printf(\"{f}\\n\"{});\n", a.iter().map(|x| format!(", {x}")).collect::<String>()));
        } else if assign.is_match(l) && !l.contains("**") {
            out.push_str(&format!("{ind}{};\n", ops(l)));
        } else {
            todo += 1;
            out.push_str(&format!("{ind}/* TODO(f2c): {l} */\n"));
            rep.push(format!("{name}:{}: 未対応 `{l}`", i + 1));
        }
    }
    if todo > 0 {
        rep.push(format!("{name}: 未対応の行 {todo} 個(TODO(f2c) として残しました)"));
    }
    let base = name.rsplit('/').next().unwrap_or(name);
    let base = base.rsplit_once('.').map(|x| x.0).unwrap_or(base);
    vec![(format!("{base}.c"), out)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut r = vec![];
        let o = convert("program t\n  implicit none\n  integer :: i, s = 0\n  do i = 10, 1, -3\n    s = s + i\n  end do\n  if (s .gt. 5) then\n    print *, 'sum', s\n  end if\nend program t\n", "t.f90", &mut r);
        let t = &o[0].1;
        assert!(t.contains("for (i = 10; i >= 1; i += -3) {") && t.contains("if (s > 5) {") && t.contains("printf(\" sum %11d\\n\", s);") && t.contains("return 0;"), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
    #[test]
    fn unsupported_kept() {
        let mut r = vec![];
        let o = convert("program t\n  real :: a(3)\n  call foo()\nend program t\n", "u.f90", &mut r);
        assert!(o[0].1.contains("TODO(f2c)") && r.len() >= 2);
    }
}
