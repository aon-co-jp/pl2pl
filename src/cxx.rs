//! `pl2pl c2cpp` / `cpp2c`: C と C++ の相互変換の最初の一歩。
//! c2cpp: 標準ヘッダーを `<cstdio>` 形へ、C++ の予約語と同じ名前の変数を `名前_` へ、キャストの無い malloc を報告。
//! cpp2c: `std::cout << ...` を printf へ(int/long/double/bool/文字列リテラル/char*、double は cout 既定と同じ %g)、
//! ヘッダーを C 形へ、`nullptr`→`NULL`、`bool` 用に `<stdbool.h>`、`int main()`→`int main(void)`。
//! class・template・new/delete・参照・std::string/vector などは黙って捨てず TODO として残し、レポートへ載せる。
use regex::Regex;
use std::collections::HashMap;

const HDR: &[(&str, &str)] = &[
    ("stdio.h", "cstdio"),
    ("stdlib.h", "cstdlib"),
    ("string.h", "cstring"),
    ("math.h", "cmath"),
    ("ctype.h", "cctype"),
    ("time.h", "ctime"),
    ("limits.h", "climits"),
    ("stdint.h", "cstdint"),
    ("assert.h", "cassert"),
    ("float.h", "cfloat"),
    ("stddef.h", "cstddef"),
];
const CPP_KW: &[&str] = &["new", "delete", "class", "this", "template", "typename", "namespace", "private", "public", "protected", "virtual", "operator", "friend", "throw", "try", "catch", "using", "export", "nullptr", "explicit", "mutable"];

fn base(name: &str) -> String {
    let b = name.rsplit('/').next().unwrap_or(name);
    b.rsplit_once('.').map(|x| x.0).unwrap_or(b).to_string()
}

pub fn c2cpp(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let inc = Regex::new(r"^(\s*#\s*include\s*)<([\w./]+)>").unwrap();
    let malloc = Regex::new(r"=\s*(malloc|calloc|realloc)\s*\(").unwrap();
    let mut out = String::from("// pl2pl c2cpp で変換(最初の一歩)。TODO(c2cpp) の行は手で直してください。\n");
    let kws: Vec<Regex> = CPP_KW.iter().map(|k| Regex::new(&format!(r"\b{k}\b")).unwrap()).collect();
    for (i, line) in src.lines().enumerate() {
        let mut l = line.to_string();
        if let Some(c) = inc.captures(&l) {
            if c[2] == *"stdbool.h" {
                out.push_str("// <stdbool.h> は C++ では不要(bool は組み込み)\n");
                continue;
            }
            if let Some((_, n)) = HDR.iter().find(|(h, _)| *h == &c[2]) {
                l = inc.replace(&l, format!("${{1}}<{n}>")).to_string();
            }
        }
        if !l.trim_start().starts_with('#') {
            for (k, re) in CPP_KW.iter().zip(&kws) {
                if re.is_match(&l) {
                    l = re.replace_all(&l, format!("{k}_")).to_string();
                    rep.push(format!("{name}:{}: C++ の予約語 `{k}` を `{k}_` に変えました", i + 1));
                }
            }
            if malloc.is_match(&l) {
                l = format!("{l} // TODO(c2cpp): C++ では void* から暗黙に変換できないのでキャストが必要");
                rep.push(format!("{name}:{}: malloc 系の戻り値にキャストが必要 `{}`", i + 1, line.trim()));
            }
        }
        out.push_str(&l);
        out.push('\n');
    }
    vec![(format!("{}.cpp", base(name)), out)]
}

fn ty_of(e: &str, vars: &HashMap<String, &'static str>) -> Option<&'static str> {
    let e = e.trim();
    if e.starts_with('"') && e.ends_with('"') {
        return Some("lit");
    }
    if e.starts_with('\'') && e.ends_with('\'') {
        return Some("%c");
    }
    if Regex::new(r"^-?\d+$").unwrap().is_match(e) {
        return Some("%d");
    }
    if Regex::new(r"^-?\d+\.\d*([eE][-+]?\d+)?$").unwrap().is_match(e) {
        return Some("%g");
    }
    let ids: Vec<&str> = Regex::new(r"[A-Za-z_]\w*").unwrap().find_iter(e).map(|m| m.as_str()).collect();
    if ids.is_empty() {
        return None;
    }
    let tys: Vec<&str> = ids.iter().map(|i| vars.get(*i).copied().unwrap_or("?")).collect();
    if tys.contains(&"?") {
        return None;
    }
    if tys.contains(&"%s") {
        return if ids.len() == 1 { Some("%s") } else { None };
    }
    if tys.contains(&"%g") || Regex::new(r"\d\.\d").unwrap().is_match(e) {
        return Some("%g");
    }
    if tys.contains(&"%ld") {
        return Some("%ld");
    }
    Some("%d") // int / bool(cout は bool を 1/0 で出す)
}

pub fn cpp2c(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let inc = Regex::new(r"^\s*#\s*include\s*<([\w./]+)>").unwrap();
    let decl = Regex::new(r"^\s*(?:const\s+)?((?:int|long|double|float|bool|unsigned)\b|char\s*\*)\s*(\w[^;(]*);").unwrap();
    let cout = Regex::new(r"^(\s*)(?:std::)?cout\s*<<\s*(.*);\s*$").unwrap();
    let bad = Regex::new(r"\b(class|template|typename|new|delete|std::string|std::vector|std::map|namespace|try|catch|throw|operator|virtual|auto)\b|std::(?:cin|cerr)|\bcin\b|&\s*\w+\s*[,)=]").unwrap();
    let mut vars: HashMap<String, &'static str> = HashMap::new();
    let mut body = String::new();
    let mut need_bool = false;
    let mut todo = 0;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        if let Some(c) = inc.captures(line) {
            let h = &c[1];
            if let Some((ch, _)) = HDR.iter().find(|(_, cpp)| *cpp == h) {
                body.push_str(&format!("#include <{ch}>\n"));
            } else if h == "iostream" {
                // cout を printf にするので stdio.h(先頭に必ず付ける)
            } else {
                todo += 1;
                body.push_str(&format!("/* TODO(cpp2c): C に無いヘッダー {t} */\n"));
                rep.push(format!("{name}:{}: C に無いヘッダー `{t}`", i + 1));
            }
            continue;
        }
        if t.starts_with("using namespace std") {
            continue;
        }
        if t.starts_with("//") {
            body.push_str(line);
            body.push('\n');
            continue;
        }
        if let Some(c) = decl.captures(line) {
            let ty = match c[1].replace(' ', "").as_str() {
                "int" | "unsigned" | "bool" => "%d",
                "long" => "%ld",
                "double" | "float" => "%g",
                _ => "%s",
            };
            if &c[1] == "bool" {
                need_bool = true;
            }
            for v in c[2].split(',') {
                let n = v.split('=').next().unwrap_or("").trim().trim_start_matches('*').to_string();
                vars.insert(n, ty);
            }
        }
        if let Some(c) = cout.captures(line) {
            let (mut f, mut a, mut ok) = (String::new(), vec![], true);
            for it in c[2].split("<<").map(|x| x.trim()) {
                if it == "std::endl" || it == "endl" || it == "'\\n'" {
                    f.push_str("\\n");
                    continue;
                }
                match ty_of(it, &vars) {
                    Some("lit") => f.push_str(&it[1..it.len() - 1].replace('%', "%%")),
                    Some(s) => {
                        f.push_str(s);
                        a.push(it.to_string());
                    }
                    None => ok = false,
                }
            }
            if ok {
                body.push_str(&format!("{}printf(\"{f}\"{});\n", &c[1], a.iter().map(|x| format!(", {x}")).collect::<String>()));
            } else {
                todo += 1;
                body.push_str(&format!("{}/* TODO(cpp2c): 型のわからない cout: {t} */\n", &c[1]));
                rep.push(format!("{name}:{}: 型のわからない cout `{t}`", i + 1));
            }
            continue;
        }
        if bad.is_match(t) {
            todo += 1;
            body.push_str(&format!("/* TODO(cpp2c): C++ だけの書き方: {} */\n", t.replace("*/", "* /")));
            rep.push(format!("{name}:{}: C++ だけの書き方 `{t}`", i + 1));
            continue;
        }
        let mut l = line.replace("std::printf", "printf").replace("std::puts", "puts").replace("std::", "");
        l = Regex::new(r"\bnullptr\b").unwrap().replace_all(&l, "NULL").to_string();
        l = Regex::new(r"\bint\s+main\s*\(\s*\)").unwrap().replace_all(&l, "int main(void)").to_string();
        if Regex::new(r"\b(bool|true|false)\b").unwrap().is_match(&l) {
            need_bool = true;
        }
        body.push_str(&l);
        body.push('\n');
    }
    if todo > 0 {
        rep.push(format!("{name}: 未対応の行 {todo} 個(TODO(cpp2c) として残しました)"));
    }
    let mut head = String::from("/* pl2pl cpp2c で変換(最初の一歩)。TODO(cpp2c) の行は手で直してください。 */\n#include <stdio.h>\n");
    if need_bool {
        head.push_str("#include <stdbool.h>\n");
    }
    if body.contains("NULL") && !body.contains("<stddef.h>") {
        head.push_str("#include <stddef.h>\n");
    }
    let body = body.replace("#include <stdio.h>\n", "");
    vec![(format!("{}.c", base(name)), head + &body)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c_to_cpp() {
        let mut r = vec![];
        let o = &c2cpp("#include <stdio.h>\n#include <stdbool.h>\n#include <stdlib.h>\nint main(void) {\n    int new = 3;\n    int *p = malloc(4);\n    printf(\"%d\\n\", new);\n}\n", "a.c", &mut r)[0];
        assert_eq!(o.0, "a.cpp");
        assert!(o.1.contains("#include <cstdio>") && o.1.contains("#include <cstdlib>") && o.1.contains("int new_ = 3;") && o.1.contains("TODO(c2cpp)") && !o.1.contains("#include <stdbool.h>"), "{}", o.1);
        assert_eq!(r.len(), 3, "{r:?}");
    }
    #[test]
    fn cpp_to_c() {
        let mut r = vec![];
        let src = "#include <iostream>\n#include <cmath>\nusing namespace std;\nint main() {\n    int n = 3;\n    double x = 2.5;\n    bool ok = true;\n    const char *s = \"hi\";\n    int *p = nullptr;\n    cout << \"n=\" << n << \" x=\" << x * 2 << \" ok=\" << ok << \" \" << s << endl;\n    std::cout << sqrt(x) << '\\n';\n    return 0;\n}\n";
        let o = &cpp2c(src, "b.cpp", &mut r)[0];
        assert_eq!(o.0, "b.c");
        for w in ["#include <stdio.h>", "#include <stdbool.h>", "#include <math.h>", "int main(void) {", "int *p = NULL;", "printf(\"n=%d x=%g ok=%d %s\\n\", n, x * 2, ok, s);"] {
            assert!(o.1.contains(w), "missing {w}\n{}", o.1);
        }
        // sqrt(x) は関数呼び出しで型が分からない → TODO
        assert!(o.1.contains("TODO(cpp2c): 型のわからない cout") && r.len() == 2, "{}\n{r:?}", o.1);
    }
    #[test]
    fn cpp_only_kept() {
        let mut r = vec![];
        let o = &cpp2c("#include <vector>\nclass A {};\nint main() {\n    std::vector<int> v;\n}\n", "c.cpp", &mut r)[0];
        assert!(o.1.matches("TODO(cpp2c):").count() == 3, "{}", o.1);
    }
}
