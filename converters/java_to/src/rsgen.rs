//! Rust(Poem / RPoem)の骨組みを書き出す。形は 2Poem(同じエコシステムの Python→Poem/RPoem 変換)の出力に合わせる:
//! ルーティング・パス/クエリ/本文の型・データクラス(serde 構造体)は写し、本体は「単純な return」だけ移して、他は `todo!` 相当の応答にする。

use crate::body::snake;
use crate::endpoints::{Endpoint, Kind};
use crate::lexer::Tok;
use crate::parse::Class;
use crate::pygen::{is_dto, Out};
use std::collections::{BTreeSet, HashSet};

const KEYWORDS: [&str; 41] = [
    "fn", "let", "mut", "type", "match", "loop", "move", "ref", "self", "use", "mod", "crate", "super", "where", "async", "await", "dyn", "impl", "trait", "struct", "enum", "const", "static", "pub", "in", "for", "if", "else", "while", "return", "break", "continue", "as", "true", "false", "unsafe", "box", "do", "final", "macro", "override",
];

pub fn rust_ident(n: &str) -> String {
    let mut s: String = snake(n).chars().map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' }).collect();
    s = s.trim_end_matches('_').to_string();
    if s.is_empty() {
        s = "handler".into();
    }
    if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        s = format!("h_{s}");
    }
    if KEYWORDS.contains(&s.as_str()) {
        s.push('_');
    }
    s
}

pub fn rs_type(t: &str, dtos: &HashSet<String>) -> String {
    let t = t.trim();
    if let Some(inner) = t.strip_suffix("[]") {
        return format!("Vec<{}>", rs_type(inner, dtos));
    }
    let (base, args) = match t.find('<') {
        Some(i) => (&t[..i], t[i + 1..t.len() - 1].to_string()),
        None => (t, String::new()),
    };
    let base = base.rsplit('.').next().unwrap_or(base);
    let argv: Vec<String> = split_args(&args).into_iter().map(|a| rs_type(&a, dtos)).collect();
    match base {
        "String" | "char" | "Character" | "CharSequence" | "UUID" | "LocalDate" | "LocalDateTime" | "Date" | "Instant" | "ZonedDateTime" => "String".into(),
        "int" => "i32".into(),
        "Integer" => "Option<i32>".into(),
        "long" => "i64".into(),
        "Long" | "BigInteger" => "Option<i64>".into(),
        "short" => "i16".into(),
        "byte" => "i8".into(),
        "double" | "BigDecimal" => "f64".into(),
        "Double" | "Float" => "Option<f64>".into(),
        "float" => "f32".into(),
        "boolean" => "bool".into(),
        "Boolean" => "Option<bool>".into(),
        "void" | "Void" => "()".into(),
        "List" | "ArrayList" | "LinkedList" | "Collection" | "Iterable" | "Set" | "HashSet" | "LinkedHashSet" => format!("Vec<{}>", argv.first().cloned().unwrap_or_else(|| "serde_json::Value".into())),
        "Map" | "HashMap" | "LinkedHashMap" | "TreeMap" => format!("std::collections::HashMap<{}, {}>", argv.first().cloned().unwrap_or_else(|| "String".into()), argv.get(1).cloned().unwrap_or_else(|| "serde_json::Value".into())),
        "Optional" => format!("Option<{}>", argv.first().cloned().unwrap_or_else(|| "serde_json::Value".into())),
        "ResponseEntity" | "Response" | "Mono" | "Flux" | "CompletableFuture" | "Uni" => argv.first().cloned().unwrap_or_else(|| "serde_json::Value".into()),
        b if dtos.contains(b) => b.to_string(),
        _ => "serde_json::Value".into(),
    }
}

fn split_args(s: &str) -> Vec<String> {
    let mut out = vec![];
    let mut d = 0;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '<' => {
                d += 1;
                cur.push(ch);
            }
            '>' => {
                d -= 1;
                cur.push(ch);
            }
            ',' if d == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn unique_name(base: &str, used: &mut HashSet<String>, verb: &str) -> String {
    let mut n = rust_ident(base);
    if matches!(n.as_str(), "get" | "post" | "put" | "delete" | "patch" | "head" | "options") {
        n = format!("{n}_handler");
    }
    if used.contains(&n) {
        n = format!("{n}_{verb}");
    }
    let mut i = 2;
    let orig = n.clone();
    while used.contains(&n) {
        n = format!("{orig}_{i}");
        i += 1;
    }
    used.insert(n.clone());
    n
}

enum Simple {
    Text(String),
    Json(String),
    /// 複数文の本体(`return` は `@RET(式)@` の目印)
    Block(String),
}

/// `return "a" + id + 1;` のような単純な戻りだけ Rust にする。
fn simple_return(body: &[Tok], vars: &[(String, String)], is_expr: bool, dtos: &HashSet<String>) -> Option<Simple> {
    if let Some(s) = simple_return1(body, vars, is_expr) {
        return Some(s);
    }
    if is_expr {
        return None;
    }
    let inner = if body.first().is_some_and(|t| t.is("{")) && body.last().is_some_and(|t| t.is("}")) { &body[1..body.len() - 1] } else { body };
    crate::rsblock::rs_block(inner, vars, &mut HashSet::new(), dtos, 0).filter(|b| b.contains("@RET(")).map(Simple::Block)
}

fn simple_return1(body: &[Tok], vars: &[(String, String)], is_expr: bool) -> Option<Simple> {
    let toks: Vec<Tok> = if is_expr {
        body.to_vec()
    } else {
        let (first, rest) = (body.first()?, &body[1..]);
        if !first.id("return") || !rest.last()?.is(";") || rest.iter().filter(|t| t.is(";")).count() != 1 {
            return None;
        }
        rest[..rest.len() - 1].to_vec()
    };
    let parts = crate::parse::split_top(&toks, "+");
    let mut fmt = String::new();
    let mut args: Vec<String> = vec![];
    let mut all_text = false;
    let mut single_non_text: Option<String> = None;
    for part in &parts {
        match part.as_slice() {
            [Tok::Str(s)] => {
                all_text = true;
                fmt.push_str(&s.replace('{', "{{").replace('}', "}}"));
            }
            [Tok::Num(n)] => {
                fmt.push_str("{}");
                args.push(n.clone());
                single_non_text = Some(n.clone());
            }
            [Tok::Id(v)] if v == "true" || v == "false" => {
                fmt.push_str("{}");
                args.push(v.clone());
                single_non_text = Some(v.clone());
            }
            [Tok::Id(v)] => {
                let (_, r) = vars.iter().find(|(j, _)| j == v)?;
                fmt.push_str("{}");
                args.push(r.clone());
                single_non_text = Some(r.clone());
            }
            // req.params(":id") / ctx.pathParam("id") / ctx.queryParam("q") / request.getParameter("q")
            [Tok::Id(_), Tok::P(d), Tok::Id(m), Tok::P(o), Tok::Str(s), Tok::P(c)] if d == "." && o == "(" && c == ")" && matches!(m.as_str(), "params" | "pathParam" | "queryParam" | "queryParams" | "getParameter") => {
                let key = s.trim_start_matches(':');
                let (_, r) = vars.iter().find(|(j, _)| j == key)?;
                fmt.push_str("{}");
                args.push(r.clone());
                all_text = true;
            }
            // ctx.result(<式>) / ctx.json(...) は式文のみ未対応
            _ => return None,
        }
    }
    if parts.len() == 1 && !all_text {
        return single_non_text.map(Simple::Json);
    }
    Some(Simple::Text(if args.is_empty() { format!("{:?}.to_string()", unesc(&fmt)) } else { format!("format!({:?}{})", fmt, args.iter().map(|a| format!(", {a}")).collect::<String>()) }))
}

/// `@RET(式)@` を、ハンドラの戻り値の形(`@` が式に置き換わる)に包む。最後の行は `return` を付けない。
fn wrap_ret(block: &str, shape: &str) -> String {
    let mut out = String::new();
    let mut rest = block;
    while let Some(i) = rest.find("@RET(") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 5..];
        let j = after.find(")@").unwrap_or(0);
        out.push_str(&shape.replace('@', &after[..j]));
        rest = &after[j + 2..];
    }
    out.push_str(rest);
    out.trim_end().trim_start_matches("    ").to_string()
}

fn unesc(s: &str) -> String {
    s.replace("{{", "{").replace("}}", "}")
}

fn comment(e: &Endpoint, src: &str) -> String {
    let mut s = format!("/// 元: {} {}.{} ({}) -> Java 戻り型: {}\n", e.file.replace('\\', "/").rsplit('/').next().unwrap_or(""), e.class, e.method, e.framework, if e.ret.is_empty() { "void" } else { &e.ret });
    for l in src.lines().take(8) {
        s.push_str(&format!("/// > {}\n", l.trim_end()));
    }
    s
}

pub fn generate(units: &[crate::parse::Unit], endpoints: &[Endpoint], target: &str, name: &str, rpoem_path: &str) -> Out {
    let classes: Vec<&Class> = units.iter().flat_map(|u| u.classes.iter()).collect();
    let dtos: HashSet<String> = classes.iter().filter(|c| is_dto(c)).map(|c| c.name.clone()).collect();
    let rpoem = target == "rpoem";
    let mut warnings: Vec<String> = vec![];
    let mut todo = 0usize;
    let mut structs = String::new();
    for c in classes.iter().filter(|c| dtos.contains(&c.name)) {
        structs.push_str("#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]\n#[serde(default)]\n");
        structs.push_str(&format!("pub struct {} {{\n", c.name));
        for f in c.fields.iter().filter(|f| !f.is_static) {
            let rn = rust_ident(&f.name);
            let t = rs_type(&f.ty, &dtos);
            let t = if is_prim_rs(&t) || t.starts_with("Option<") || t.starts_with("Vec<") || t == "String" || t.starts_with("std::collections") || t.contains("Value") || dtos.contains(&t) { t } else { t };
            if rn != f.name {
                structs.push_str(&format!("    #[serde(rename = \"{}\")]\n", f.name));
            }
            structs.push_str(&format!("    pub {rn}: {t},\n"));
        }
        structs.push_str("}\n\n");
    }
    for c in classes.iter().filter(|c| c.kind == "enum") {
        structs.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]\npub enum {} {{\n", c.name));
        for (i, k) in c.enum_consts.iter().enumerate() {
            if i == 0 {
                structs.push_str("    #[default]\n");
            }
            structs.push_str(&format!("    {k},\n"));
        }
        if c.enum_consts.is_empty() {
            structs.push_str("    #[default]\n    Unknown,\n");
        }
        structs.push_str("}\n\n");
    }

    let mut handlers = String::new();
    let mut routes: Vec<(String, String, String)> = vec![]; // (path, verb, fn)
    let mut used: HashSet<String> = HashSet::new();
    let mut sorted: Vec<&Endpoint> = endpoints.iter().collect();
    sorted.sort_by_key(|e| e.path.matches('{').count());
    for e in sorted {
        let verb = match e.verb.as_str() {
            "get" | "post" | "put" | "delete" | "head" => e.verb.clone(),
            "patch" if rpoem => {
                warnings.push(format!("{}: PATCH は RPoem の互換面に無いため PUT として登録しました(要確認)", e.path));
                "put".into()
            }
            "patch" => "patch".into(),
            v => {
                warnings.push(format!("{} {}: {} は未対応のため飛ばしました", e.class, e.method, v.to_uppercase()));
                continue;
            }
        };
        let fn_name = unique_name(&format!("{}_{}", e.method, ""), &mut used, &verb);
        let fn_name = fn_name.trim_end_matches('_').to_string();
        let mut path = e.path.clone();
        for p in &e.params {
            if p.kind == Kind::Path && p.java != snake(&p.var) {
                path = path.replace(&format!("{{{}}}", p.java), &format!("{{{}}}", snake(&p.var)));
            }
        }
        // 元の Java(コメント用): 本体の先頭数行は Tok から再構成しない(メソッド本体は pygen 側のみ保持)
        let src = String::new();
        let vars: Vec<(String, String)> = e.params.iter().filter(|p| p.kind != Kind::Ctx && p.kind != Kind::Body).flat_map(|p| vec![(p.java.clone(), rust_ident(&p.var)), (p.var.clone(), rust_ident(&p.var))]).collect();
        let body_src: Option<(Vec<Tok>, bool)> = if let Some(b) = &e.lambda_body {
            Some((b.clone(), e.lambda_is_expr))
        } else {
            classes.iter().find(|c| c.name == e.class).and_then(|c| c.methods.iter().find(|m| m.name == e.method)).and_then(|m| m.body.clone()).map(|b| (b, false))
        };
        let simple = body_src.as_ref().and_then(|(b, is_expr)| simple_return(b, &vars, *is_expr, &dtos));
        let (h, n_todo) = if rpoem { rpoem_handler(e, &fn_name, &dtos, &simple, &src, &mut warnings) } else { poem_handler(e, &fn_name, &dtos, &simple, &src, &mut warnings) };
        todo += n_todo;
        handlers.push_str(&h);
        routes.push((path, verb, fn_name));
    }

    // main.rs
    let mut verbs: BTreeSet<String> = routes.iter().map(|r| r.1.clone()).collect();
    verbs.insert("get".into());
    let mut route_lines = String::new();
    let mut by_path: Vec<(String, Vec<(String, String)>)> = vec![];
    for (p, v, f) in &routes {
        if let Some(x) = by_path.iter_mut().find(|x| &x.0 == p) {
            x.1.push((v.clone(), f.clone()));
        } else {
            by_path.push((p.clone(), vec![(v.clone(), f.clone())]));
        }
    }
    for (p, vs) in &by_path {
        let mut expr = String::new();
        for (v, f) in vs {
            let call = if rpoem { format!("{v}({f}())") } else { format!("{v}({f})") };
            if expr.is_empty() {
                expr = call;
            } else {
                expr = format!("{expr}.{call}");
            }
        }
        route_lines.push_str(&format!("        .at(\"{p}\", {expr})\n"));
    }
    let cpu = "    println!(\"{}\", open_cpu::runtime_report());\n";
    let main_rs = if rpoem {
        let vs: Vec<String> = verbs.into_iter().collect();
        format!(
            "//! Java_to が生成した骨組み(rpoem)。ルーティングとパスパラメータの型は写してあり、本体は単純な return 以外 TODO。\n#![allow(unused_variables, unused_imports, dead_code)]\nuse open_runo_poem_compat::{{{}, Json, Params, Request, Response, Route, Server, StatusCode, TcpListener}};\n\n{structs}{handlers}\n#[tokio::main]\nasync fn main() {{\n    let app = Route::new()\n{route_lines};\n{cpu}    let (addr, handle) = Server::new(TcpListener::bind(([0, 0, 0, 0], 3000))).run(app).await.expect(\"bind :3000\");\n    println!(\"listening on {{addr}}\");\n    handle.await.ok();\n}}\n",
            vs.join(", ")
        )
    } else {
        let vs: Vec<String> = verbs.into_iter().collect();
        format!(
            "//! Java_to が生成した骨組み(poem)。ルーティングと型は写してあり、本体は単純な return 以外 TODO。\n#![allow(unused_variables, unused_imports, dead_code)]\nuse poem::{{{}, handler, listener::TcpListener, web::{{Json, Path, Query}}, Request, Route, Server}};\nuse serde_json::{{json, Value}};\n\n{structs}{handlers}\n#[tokio::main]\nasync fn main() -> Result<(), std::io::Error> {{\n{cpu}    let app = Route::new()\n{route_lines};\n    Server::new(TcpListener::bind(\"0.0.0.0:3000\")).run(app).await\n}}\n",
            vs.join(", ")
        )
    };
    let main_rs = if routes.is_empty() { main_rs.replace("Route::new()\n;", "Route::new();") } else { main_rs };
    let cargo = if rpoem {
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nopen-runo-poem-compat = {{ path = \"{p}/crates/open-runo-poem-compat\" }}\nopen-runo-poem-compat-macro = {{ path = \"{p}/crates/open-runo-poem-compat-macro\" }}\nopen-runo-view = {{ path = \"{p}/crates/open-runo-view\" }}\nserde = {{ version = \"1\", features = [\"derive\"] }}\nserde_json = \"1\"\nopen-cpu = {{ git = \"https://github.com/aon-co-jp/open-cpu\" }}\ntokio = {{ version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"net\"] }}\n",
            p = rpoem_path
        )
    } else {
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\npoem = \"3\"\nserde = {{ version = \"1\", features = [\"derive\"] }}\nserde_json = \"1\"\nopen-cpu = {{ git = \"https://github.com/aon-co-jp/open-cpu\" }}\ntokio = {{ version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"net\"] }}\n")
    };
    Out { files: vec![("Cargo.toml".into(), cargo), ("src/main.rs".into(), main_rs), (".gitignore".into(), "target/\n".into())], warnings, todo_count: todo }
}

fn is_prim_rs(t: &str) -> bool {
    matches!(t, "i32" | "i64" | "i16" | "i8" | "f64" | "f32" | "bool")
}

struct Extract {
    sig_poem: Vec<String>,
    prelude: Vec<String>,
    structs: String,
}

fn collect(e: &Endpoint, fn_name: &str, dtos: &HashSet<String>, rpoem: bool, warnings: &mut Vec<String>) -> Extract {
    let camel: String = fn_name.split('_').map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() }).collect();
    let mut sig = vec![];
    let mut prelude = vec![];
    let mut structs = String::new();
    let paths: Vec<_> = e.params.iter().filter(|p| p.kind == Kind::Path).collect();
    let queries: Vec<_> = e.params.iter().filter(|p| p.kind == Kind::Query || p.kind == Kind::Form).collect();
    if rpoem {
        for p in &paths {
            let id = rust_ident(&p.var);
            let t = rs_type(&p.ty, dtos);
            let t = t.strip_prefix("Option<").and_then(|x| x.strip_suffix('>')).map(str::to_string).unwrap_or(t);
            prelude.push(format!("    let {id}: {t} = params.get(\"{}\").and_then(|s| s.parse().ok()).unwrap_or_default();", snake(&p.var)));
        }
        if !queries.is_empty() || e.params.iter().any(|p| p.kind == Kind::Body || p.kind == Kind::Header) {
            warnings.push(format!("{} {}: クエリ/本文/ヘッダーの取り出しは RPoem の互換面での書き方が未確認のため TODO にしました", e.class, e.method));
        }
    } else {
        if !paths.is_empty() {
            structs.push_str(&format!("#[derive(serde::Deserialize)]\nstruct {camel}Path {{\n"));
            for p in &paths {
                structs.push_str(&format!("    {}: {},\n", rust_ident(&p.var), rs_type(&p.ty, dtos)));
            }
            structs.push_str("}\n\n");
            let names: Vec<String> = paths.iter().map(|p| rust_ident(&p.var)).collect();
            sig.push(format!("Path({camel}Path {{ {} }}): Path<{camel}Path>", names.join(", ")));
        }
        if !queries.is_empty() {
            structs.push_str(&format!("#[derive(serde::Deserialize, Default)]\n#[serde(default)]\nstruct {camel}Query {{\n"));
            for p in &queries {
                let t = rs_type(&p.ty, dtos);
                let t = if t.starts_with("Option<") { t } else { format!("Option<{t}>") };
                structs.push_str(&format!("    #[serde(rename = \"{}\")]\n    {}: {},\n", p.java, rust_ident(&p.var), t));
            }
            structs.push_str("}\n\n");
            let names: Vec<String> = queries.iter().map(|p| rust_ident(&p.var)).collect();
            sig.push(format!("Query({camel}Query {{ {} }}): Query<{camel}Query>", names.join(", ")));
            for p in &queries {
                let id = rust_ident(&p.var);
                let t = rs_type(&p.ty, dtos);
                let base = t.strip_prefix("Option<").and_then(|x| x.strip_suffix('>')).unwrap_or(&t).to_string();
                if let Some(d) = &p.default {
                    let lit = match base.as_str() {
                        "String" => format!("{:?}.to_string()", d),
                        "bool" => d.clone(),
                        _ => d.clone(),
                    };
                    prelude.push(format!("    let {id}: {base} = {id}.unwrap_or({lit});"));
                } else if p.required && !t.starts_with("Option<") {
                    prelude.push(format!("    let {id}: {base} = {id}.unwrap_or_default();"));
                }
            }
        }
        if let Some(b) = e.params.iter().find(|p| p.kind == Kind::Body) {
            let t = rs_type(&b.ty, dtos);
            sig.push(format!("Json({}): Json<{t}>", rust_ident(&b.var)));
        }
        if e.params.iter().any(|p| p.kind == Kind::Header) {
            sig.push("req: &Request".into());
            for p in e.params.iter().filter(|p| p.kind == Kind::Header) {
                prelude.push(format!("    let {}: Option<String> = req.header(\"{}\").map(|s| s.to_string());", rust_ident(&p.var), p.java));
            }
        }
    }
    Extract { sig_poem: sig, prelude, structs }
}

fn poem_handler(e: &Endpoint, name: &str, dtos: &HashSet<String>, simple: &Option<Simple>, src: &str, warnings: &mut Vec<String>) -> (String, usize) {
    let x = collect(e, name, dtos, false, warnings);
    let (ret, body, todo) = match simple {
        Some(Simple::Text(t)) => ("String".to_string(), t.clone(), 0),
        Some(Simple::Json(j)) => ("Json<Value>".to_string(), format!("Json(json!({j}))"), 0),
        Some(Simple::Block(b)) => ("Json<Value>".to_string(), wrap_ret(b, "Json(json!(@))"), 0),
        None => {
            let ps: Vec<String> = e.params.iter().filter(|p| p.kind != Kind::Ctx && p.kind != Kind::Body).map(|p| format!("\"{}\": {}", p.java, rust_ident(&p.var))).collect();
            ("Json<Value>".to_string(), format!("Json(json!({{\"todo\": \"{name}\"{}}}))", if ps.is_empty() { String::new() } else { format!(", {}", ps.join(", ")) }), 1)
        }
    };
    let todo_line = if todo > 0 { "    // TODO(Java_to): Java の本体を移植する\n" } else { "" };
    let h = format!("{}{}{}#[handler]\nasync fn {name}({}) -> {ret} {{\n{}{todo_line}    {body}\n}}\n\n", x.structs, comment(e, src), "", x.sig_poem.join(", "), x.prelude.join("\n") + if x.prelude.is_empty() { "" } else { "\n" });
    (h, todo)
}

fn rpoem_handler(e: &Endpoint, name: &str, dtos: &HashSet<String>, simple: &Option<Simple>, src: &str, warnings: &mut Vec<String>) -> (String, usize) {
    let x = collect(e, name, dtos, true, warnings);
    let paths = e.params.iter().any(|p| p.kind == Kind::Path);
    let other = e.params.iter().any(|p| p.kind != Kind::Path);
    let simple = if other { &None } else { simple };
    let (body, todo) = match simple {
        Some(Simple::Text(t)) => (format!("Json(serde_json::json!({t})).into_response(StatusCode::OK)"), 0),
        Some(Simple::Json(j)) => (format!("Json(serde_json::json!({j})).into_response(StatusCode::OK)"), 0),
        Some(Simple::Block(b)) => (wrap_ret(b, "Json(serde_json::json!(@)).into_response(StatusCode::OK)"), 0),
        None => {
            let ps: Vec<String> = e.params.iter().filter(|p| p.kind == Kind::Path).map(|p| format!("\"{}\": {}", p.java, rust_ident(&p.var))).collect();
            (format!("Json(serde_json::json!({{\"todo\": \"{name}\"{}}})).into_response(StatusCode::OK)", if ps.is_empty() { String::new() } else { format!(", {}", ps.join(", ")) }), 1)
        }
    };
    let todo_line = if todo > 0 { "    // TODO(Java_to): Java の本体を移植する\n" } else { "" };
    let unused = if paths { "params" } else { "_params" };
    let h = format!(
        "{}#[open_runo_poem_compat_macro::handler]\nasync fn {name}(_req: Request, {unused}: Params) -> Response {{\n{}{todo_line}    {body}\n}}\n\n",
        comment(e, src),
        x.prelude.join("\n") + if x.prelude.is_empty() { "" } else { "\n" }
    );
    (h, todo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idents() {
        assert_eq!(rust_ident("getUserById"), "get_user_by_id");
        assert_eq!(rust_ident("type"), "type_");
    }
    #[test]
    fn types() {
        let d = HashSet::new();
        assert_eq!(rs_type("List<String>", &d), "Vec<String>");
        assert_eq!(rs_type("long", &d), "i64");
        assert_eq!(rs_type("Optional<Integer>", &d), "Option<Option<i32>>");
    }
}
