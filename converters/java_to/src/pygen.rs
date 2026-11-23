//! FastAPI / MFastAPI 用の Python プロジェクトを書き出す。
//! models.py(データクラス→pydantic・enum)、services.py(その他のクラス)、controllers.py(ルートを持つクラス)、
//! main.py(ルート登録)、java_rt.py(小さな実行時部品)、requirements.txt。

use crate::body::{py_ident, snake, translate_body, translate_expr, Ctx};
use crate::endpoints::{Endpoint, Kind};
use crate::parse::{Class, Field, Method, Unit};
use std::collections::{BTreeSet, HashMap, HashSet};

pub const JAVA_RT: &str = include_str!("java_rt.py");

pub struct Project<'a> {
    pub units: &'a [Unit],
    pub endpoints: Vec<Endpoint>,
}

fn all_classes<'a>(p: &'a Project) -> Vec<&'a Class> {
    p.units.iter().flat_map(|u| u.classes.iter()).collect()
}

const ACCESSOR: [&str; 3] = ["get", "set", "is"];

fn is_accessor(m: &Method, c: &Class) -> bool {
    if m.is_ctor {
        return true;
    }
    if matches!(m.name.as_str(), "equals" | "hashCode" | "toString" | "canEqual" | "builder") {
        return true;
    }
    for pre in ACCESSOR {
        if let Some(rest) = m.name.strip_prefix(pre) {
            if rest.chars().next().is_some_and(|ch| ch.is_uppercase()) {
                let prop = lower_first(rest);
                return c.fields.iter().any(|f| f.name == prop || f.name.eq_ignore_ascii_case(rest)) || c.fields.is_empty();
            }
        }
    }
    false
}

fn lower_first(s: &str) -> String {
    let mut ch = s.chars();
    ch.next().map(|f| f.to_lowercase().collect::<String>() + ch.as_str()).unwrap_or_default()
}

const NON_DTO_ANNS: [&str; 13] = ["RestController", "Controller", "Service", "Component", "Repository", "Configuration", "Path", "WebServlet", "ApplicationScoped", "Singleton", "RequestScoped", "Stateless", "Named"];

pub fn is_dto(c: &Class) -> bool {
    if c.kind == "record" {
        return true;
    }
    if c.kind != "class" || c.has_ann(&NON_DTO_ANNS) {
        return false;
    }
    if c.has_ann(&["Data", "Value", "Getter", "Setter", "Entity", "Builder", "AllArgsConstructor", "NoArgsConstructor", "JsonIgnoreProperties", "Embeddable", "Document", "Table"]) && c.methods.iter().all(|m| is_accessor(m, c)) {
        return true;
    }
    !c.fields.is_empty() && c.methods.iter().all(|m| is_accessor(m, c)) && c.extends.iter().all(|e| !e.ends_with("HttpServlet"))
}

fn is_repo_iface(c: &Class) -> bool {
    c.kind == "interface" && c.extends.iter().any(|e| ["JpaRepository", "CrudRepository", "PagingAndSortingRepository", "ListCrudRepository", "MongoRepository", "Repository", "PanacheRepository", "ReactiveCrudRepository"].contains(&e.split('<').next().unwrap_or("")))
}

pub fn py_type(t: &str, dtos: &HashSet<String>, enums: &HashSet<String>) -> String {
    let t = t.trim();
    if let Some(inner) = t.strip_suffix("[]") {
        return format!("list[{}]", py_type(inner, dtos, enums));
    }
    let (base, args) = match t.find('<') {
        Some(i) => (&t[..i], t[i + 1..t.len() - 1].to_string()),
        None => (t, String::new()),
    };
    let base = base.rsplit('.').next().unwrap_or(base);
    let argv: Vec<String> = split_generic(&args).into_iter().map(|a| py_type(&a, dtos, enums)).collect();
    match base {
        "String" | "char" | "Character" | "CharSequence" | "UUID" => "str".into(),
        "int" | "long" | "short" | "byte" | "Integer" | "Long" | "Short" | "Byte" | "BigInteger" => "int".into(),
        "double" | "float" | "Double" | "Float" | "BigDecimal" | "Number" => "float".into(),
        "boolean" | "Boolean" => "bool".into(),
        "void" | "Void" => "None".into(),
        "Object" | "" => "Any".into(),
        "LocalDate" => "datetime.date".into(),
        "LocalDateTime" | "Date" | "Instant" | "ZonedDateTime" | "OffsetDateTime" | "Timestamp" => "datetime.datetime".into(),
        "List" | "ArrayList" | "LinkedList" | "Collection" | "Iterable" | "Set" | "HashSet" | "LinkedHashSet" | "Stream" => format!("list[{}]", argv.first().cloned().unwrap_or_else(|| "Any".into())),
        "Map" | "HashMap" | "LinkedHashMap" | "TreeMap" | "ConcurrentHashMap" => format!("dict[{}, {}]", argv.first().cloned().unwrap_or_else(|| "str".into()), argv.get(1).cloned().unwrap_or_else(|| "Any".into())),
        "Optional" | "ResponseEntity" | "Response" | "Mono" | "Flux" | "CompletableFuture" | "Uni" | "Multi" | "Future" => {
            let a = argv.first().cloned().unwrap_or_else(|| "Any".into());
            if base == "Optional" { format!("{a} | None") } else { a }
        }
        b if dtos.contains(b) || enums.contains(b) => b.to_string(),
        _ => "Any".into(),
    }
}

fn split_generic(s: &str) -> Vec<String> {
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

fn is_prim(t: &str) -> bool {
    matches!(t, "int" | "long" | "short" | "byte" | "double" | "float" | "boolean" | "char")
}

struct Mod {
    lines: Vec<String>,
    uses: BTreeSet<&'static str>,
}

fn header(uses: &BTreeSet<&'static str>, extra: &[&str]) -> String {
    let mut h = String::from("# Java_to が生成した Python です。TODO(Java_to) と書かれた所は自動では移せなかった箇所(元の Java はコメントに残してあります)。\nfrom __future__ import annotations\n\n");
    for u in uses {
        match *u {
            "datetime" => h.push_str("import datetime\n"),
            "math" => h.push_str("import math\n"),
            "time" => h.push_str("import time\n"),
            "uuid" => h.push_str("import uuid\n"),
            "random" => h.push_str("import random\n"),
            "os" => h.push_str("import os\nimport sys\n"),
            "re" => h.push_str("import re\n"),
            _ => {}
        }
    }
    h.push_str("import traceback\nfrom typing import Any\n");
    for e in extra {
        h.push_str(e);
        h.push('\n');
    }
    h.push('\n');
    h
}

pub struct Out {
    pub files: Vec<(String, String)>,
    pub warnings: Vec<String>,
    pub todo_count: usize,
}

pub fn generate(p: &Project, target: &str, name: &str) -> Out {
    let classes = all_classes(p);
    let dto_names: HashSet<String> = classes.iter().filter(|c| is_dto(c)).map(|c| c.name.clone()).collect();
    let enum_names: HashSet<String> = classes.iter().filter(|c| c.kind == "enum").map(|c| c.name.clone()).collect();
    let mut svc_methods: HashSet<String> = HashSet::new();
    for c in &classes {
        if !dto_names.contains(&c.name) && c.kind != "enum" {
            for m in &c.methods {
                svc_methods.insert(m.name.clone());
            }
        }
    }
    let class_map: HashMap<&str, &Class> = classes.iter().map(|c| (c.name.as_str(), *c)).collect();
    let ctrl_names: HashSet<&str> = p.endpoints.iter().filter(|e| e.class != "Routes").map(|e| e.class.as_str()).collect();
    let mut warnings = vec![];
    let mut todo = 0usize;

    // ---- models.py ----
    let mut models = Mod { lines: vec![], uses: BTreeSet::new() };
    models.uses.insert("datetime");
    for c in &classes {
        if c.kind == "enum" {
            models.lines.push(format!("class {}(str, Enum):", c.name));
            if c.enum_consts.is_empty() {
                models.lines.push("    pass".into());
            }
            for k in &c.enum_consts {
                models.lines.push(format!("    {k} = \"{k}\""));
            }
            models.lines.push(String::new());
        } else if dto_names.contains(&c.name) {
            models.lines.push(format!("class {}(BaseModel):", c.name));
            models.lines.push("    model_config = ConfigDict(populate_by_name=True)".into());
            let mut any = false;
            for f in c.fields.iter().filter(|f| !f.is_static) {
                any = true;
                let t = py_type(&f.ty, &dto_names, &enum_names);
                let (pn, alias) = field_name(&f.name);
                let json_name = f.anns.iter().find(|a| a.name == "JsonProperty").and_then(|a| a.value());
                let alias = json_name.or(alias);
                let ann = if is_prim(&f.ty) { t.clone() } else if t.contains("| None") || t == "Any" { t.clone() } else { format!("{t} | None") };
                let default = if is_prim(&f.ty) { Some(crate::body::default_value(&f.ty)) } else { Some("None".to_string()) };
                let line = match (&alias, &default) {
                    (Some(a), Some(d)) => format!("    {pn}: {ann} = Field({d}, alias=\"{a}\")"),
                    (Some(a), None) => format!("    {pn}: {ann} = Field(..., alias=\"{a}\")"),
                    (None, Some(d)) => format!("    {pn}: {ann} = {d}"),
                    (None, None) => format!("    {pn}: {ann}"),
                };
                models.lines.push(line);
            }
            for f in c.fields.iter().filter(|f| f.is_static) {
                let _ = f;
            }
            if !any {
                models.lines.push("    pass".into());
            }
            models.lines.push(String::new());
        }
    }

    // ---- services.py(DTO・enum・コントローラー以外) ----
    let mut svc = Mod { lines: vec![], uses: BTreeSet::new() };
    let ordered = topo(&classes);
    let impls: HashMap<String, Vec<&str>> = {
        let mut m: HashMap<String, Vec<&str>> = HashMap::new();
        for c in &classes {
            for i in c.implements.iter().chain(c.extends.iter()) {
                m.entry(i.split('<').next().unwrap_or("").to_string()).or_default().push(c.name.as_str());
            }
        }
        m
    };
    let mut ctrl = Mod { lines: vec![], uses: BTreeSet::new() };
    for c in &ordered {
        if dto_names.contains(&c.name) || c.kind == "enum" || c.kind == "annotation" {
            continue;
        }
        let is_ctrl = ctrl_names.contains(c.name.as_str());
        let target_mod = if is_ctrl { &mut ctrl } else { &mut svc };
        gen_class(c, &class_map, &impls, &dto_names, &enum_names, &svc_methods, target_mod, &mut warnings, &mut todo);
    }

    // ---- main.py ----
    let mut main_m = Mod { lines: vec![], uses: BTreeSet::new() };
    let mut routes: Vec<&Endpoint> = p.endpoints.iter().collect();
    routes.sort_by_key(|e| e.path.matches('{').count());
    let mut used_names: HashSet<String> = HashSet::new();
    let mut inst: BTreeSet<String> = BTreeSet::new();
    for e in &routes {
        if e.class != "Routes" && !e.class.is_empty() && !is_servlet(e) {
            inst.insert(e.class.clone());
        }
    }
    for c in &inst {
        main_m.lines.push(format!("_{} = {}()", snake(c), c));
    }
    main_m.lines.push(String::new());
    for e in &routes {
        gen_route(e, &class_map, &dto_names, &enum_names, &svc_methods, &mut main_m, &mut used_names, &mut warnings, &mut todo);
    }

    let mut files: Vec<(String, String)> = vec![];
    files.push(("java_rt.py".into(), JAVA_RT.to_string()));
    files.push((
        "models.py".into(),
        format!("{}from enum import Enum\nfrom pydantic import BaseModel, ConfigDict, Field\n\n\n{}", header(&models.uses, &[]), models.lines.join("\n")),
    ));
    files.push(("services.py".into(), format!("{}{}\n", header(&svc.uses, &["from fastapi import HTTPException", "from java_rt import *", "from models import *"]), svc.lines.join("\n"))));
    files.push(("controllers.py".into(), format!("{}{}\n", header(&ctrl.uses, &["from fastapi import HTTPException", "from java_rt import *", "from models import *", "from services import *"]), ctrl.lines.join("\n"))));
    let app_title = name.replace('"', "");
    let mut m = header(&main_m.uses, &["from fastapi import Body, Cookie, FastAPI, Form, Header, HTTPException, Query, Request", "from fastapi.responses import PlainTextResponse", "from java_rt import *", "from models import *", "from services import *", "from controllers import *"]);
    m.push_str(&format!("app = FastAPI(title=\"{app_title}\")\n\n"));
    m.push_str(&main_m.lines.join("\n"));
    m.push('\n');
    files.push(("main.py".into(), m));
    files.push(("requirements.txt".into(), format!("fastapi\nuvicorn\npydantic>=2{}\n", if target == "mfastapi" { "\nmfastapi" } else { "" })));
    if target == "mfastapi" {
        files.push(("run_mfastapi.py".into(), "import mfastapi\nimport uvicorn\nfrom main import app\n\nmfastapi.accelerate(app)  # 全ルートの登録後に呼ぶ\n\nif __name__ == \"__main__\":\n    uvicorn.run(app, host=\"0.0.0.0\", port=8000)\n".into()));
    }
    Out { files, warnings, todo_count: todo }
}

fn is_servlet(e: &Endpoint) -> bool {
    e.framework == "Servlet"
}

fn field_name(n: &str) -> (String, Option<String>) {
    let py = py_ident(n);
    let py = if py.starts_with('_') { format!("f{py}") } else { py };
    if py != n {
        (py, Some(n.to_string()))
    } else {
        (py, None)
    }
}

fn topo<'a>(classes: &[&'a Class]) -> Vec<&'a Class> {
    let names: HashSet<&str> = classes.iter().map(|c| c.name.as_str()).collect();
    let mut done: HashSet<&str> = HashSet::new();
    let mut out = vec![];
    fn visit<'a>(c: &'a Class, classes: &[&'a Class], names: &HashSet<&str>, done: &mut HashSet<&'a str>, out: &mut Vec<&'a Class>, depth: usize) {
        if done.contains(c.name.as_str()) || depth > 50 {
            return;
        }
        done.insert(c.name.as_str());
        for b in c.extends.iter().chain(c.implements.iter()) {
            let bn = b.split('<').next().unwrap_or("");
            if names.contains(bn) {
                if let Some(bc) = classes.iter().find(|x| x.name == bn) {
                    visit(bc, classes, names, done, out, depth + 1);
                }
            }
        }
        out.push(c);
    }
    for c in classes {
        visit(c, classes, &names, &mut done, &mut out, 0);
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn gen_class(c: &Class, class_map: &HashMap<&str, &Class>, impls: &HashMap<String, Vec<&str>>, dtos: &HashSet<String>, enums: &HashSet<String>, svc_methods: &HashSet<String>, m: &mut Mod, warnings: &mut Vec<String>, todo: &mut usize) {
    let bases: Vec<String> = c
        .extends
        .iter()
        .chain(c.implements.iter())
        .map(|b| b.split('<').next().unwrap_or("").to_string())
        .filter(|b| class_map.contains_key(b.as_str()) && !dtos.contains(b))
        .collect();
    let repo = is_repo_iface(c);
    let mut base_list = bases.clone();
    if repo {
        base_list.insert(0, "InMemoryRepository".into());
        m.uses.insert("rt");
    }
    let bl = if base_list.is_empty() { String::new() } else { format!("({})", base_list.join(", ")) };
    m.lines.push(format!("class {}{}:", c.name, bl));
    let mut body_lines: Vec<String> = vec![];
    // static フィールド
    let mut statics: HashSet<String> = HashSet::new();
    for f in c.fields.iter().filter(|f| f.is_static) {
        statics.insert(f.name.clone());
    }
    for mth in c.methods.iter().filter(|x| x.is_static) {
        statics.insert(mth.name.clone());
    }
    let inst_fields: Vec<&Field> = c.fields.iter().filter(|f| !f.is_static).collect();
    let methods_set: HashSet<String> = c.methods.iter().map(|x| x.name.clone()).collect();
    let mk_ctx = |in_static: bool| -> Ctx {
        let mut ctx = Ctx { class_name: c.name.clone(), in_static, ..Default::default() };
        ctx.dto = dtos.clone();
        ctx.svc_methods = svc_methods.clone();
        ctx.methods = methods_set.clone();
        ctx.statics = statics.clone();
        for f in &inst_fields {
            ctx.fields.insert(f.name.clone());
            ctx.locals.insert(format!("this.{}", f.name), f.ty.clone());
        }
        ctx
    };
    // 自分のクラス名を使う static の初期値(`getLogger(X.class)` など)は、クラス定義の後で代入する
    let mut post_lines: Vec<String> = vec![];
    let mentions_self = |v: &str| {
        let cn = c.name.as_str();
        v.match_indices(cn).any(|(i, _)| {
            let before = v[..i].chars().next_back().is_none_or(|ch| !(ch.is_alphanumeric() || ch == '_'));
            let after = v[i + cn.len()..].chars().next().is_none_or(|ch| !(ch.is_alphanumeric() || ch == '_'));
            before && after
        })
    };
    for f in c.fields.iter().filter(|f| f.is_static) {
        let mut ctx = mk_ctx(true);
        let v = if f.init.is_empty() { crate::body::default_value(&f.ty) } else { translate_expr(&f.init, &mut ctx) };
        for u in ctx.uses.iter() {
            m.uses.insert(u);
        }
        if mentions_self(&v) {
            body_lines.push(format!("    {} = None", f.name));
            post_lines.push(format!("{}.{} = {}", c.name, f.name, v));
        } else {
            body_lines.push(format!("    {} = {}", f.name, v));
        }
    }
    // __init__
    let ctor = c.methods.iter().find(|x| x.is_ctor && x.body.as_ref().is_some_and(|b| has_logic(b, &inst_fields)));
    let mut init_lines: Vec<String> = vec![];
    let mut ictx = mk_ctx(false);
    for f in &inst_fields {
        let v = if !f.init.is_empty() {
            translate_expr(&f.init, &mut ictx)
        } else {
            wire_value(&f.ty, class_map, impls, dtos)
        };
        init_lines.push(format!("        self.{} = {}", snake(&f.name), v));
    }
    if let Some(ct) = ctor {
        let params: Vec<String> = ct.params.iter().map(|p| format!("{}=None", snake(&p.name))).collect();
        body_lines.push(format!("    def __init__(self{}):", if params.is_empty() { String::new() } else { format!(", {}", params.join(", ")) }));
        body_lines.extend(init_lines.iter().cloned());
        for pr in &ct.params {
            ictx.locals.insert(pr.name.clone(), pr.ty.clone());
            ictx.param_map.insert(pr.name.clone(), snake(&pr.name));
        }
        let lines = translate_body(ct.body.as_ref().unwrap(), &mut ictx, 2);
        body_lines.extend(lines);
        body_lines.push(String::new());
    } else if !init_lines.is_empty() || !bases.is_empty() && !repo {
        body_lines.push("    def __init__(self):".into());
        if !bases.is_empty() && !repo && bases.iter().any(|b| class_map.get(b.as_str()).is_some_and(|bc| !bc.fields.is_empty())) {
            body_lines.push("        super().__init__()".into());
        }
        body_lines.extend(init_lines.iter().cloned());
        if init_lines.is_empty() && bases.is_empty() {
            body_lines.push("        pass".into());
        }
        body_lines.push(String::new());
    }
    for u in ictx.uses.iter() {
        m.uses.insert(u);
    }
    *todo += ictx.notes.len();
    warnings.extend(ictx.notes.iter().map(|n| format!("{}.<init>: {n}", c.name)));
    // メソッド
    let mut seen: HashMap<String, usize> = HashMap::new();
    for mth in c.methods.iter().filter(|x| !x.is_ctor) {
        let base = snake(&mth.name);
        let count = seen.entry(base.clone()).or_insert(0);
        *count += 1;
        let pyname = if *count == 1 { base.clone() } else {
            warnings.push(format!("{}.{}: オーバーロード(引数違いの同名メソッド)は {}_{} に改名(呼び出し側の対応は要確認)", c.name, mth.name, base, count));
            format!("{}_{}", base, count)
        };
        let mut ctx = mk_ctx(mth.is_static);
        for p in &mth.params {
            ctx.locals.insert(p.name.clone(), p.ty.clone());
            ctx.param_map.insert(p.name.clone(), snake(&p.name));
        }
        let params: Vec<String> = mth.params.iter().map(|p| snake(&p.name)).collect();
        let first = if mth.is_static { "cls" } else { "self" };
        if mth.is_static {
            body_lines.push("    @classmethod".into());
        }
        body_lines.push(format!("    def {pyname}({first}{}):", if params.is_empty() { String::new() } else { format!(", {}", params.join(", ")) }));
        match &mth.body {
            Some(b) => {
                let lines = translate_body(b, &mut ctx, 2);
                body_lines.extend(lines);
            }
            None => {
                if repo {
                    let many = ["List", "Collection", "Iterable", "Set", "Stream", "Page"].iter().any(|k| mth.ret.starts_with(k));
                    body_lines.push(format!("        return self._derived(\"{pyname}\", [{}], {})", params.join(", "), if many { "True" } else { "False" }));
                } else if c.kind == "interface" {
                    body_lines.push(format!("        raise NotImplementedError(\"TODO(Java_to): インターフェースのメソッド {}.{} を実装する\")", c.name, mth.name));
                } else {
                    body_lines.push(format!("        raise NotImplementedError(\"TODO(Java_to): 抽象メソッド {}.{}\")", c.name, mth.name));
                }
            }
        }
        for u in ctx.uses.iter() {
            m.uses.insert(u);
        }
        *todo += ctx.notes.len();
        warnings.extend(ctx.notes.iter().map(|n| format!("{}.{}: {n}", c.name, mth.name)));
        body_lines.push(String::new());
    }
    if body_lines.is_empty() {
        body_lines.push("    pass".into());
        body_lines.push(String::new());
    }
    m.lines.extend(body_lines);
    m.lines.extend(post_lines);
    m.lines.push(String::new());
}

fn has_logic(body: &[crate::lexer::Tok], fields: &[&Field]) -> bool {
    // コンストラクターが `this.x = x;` だけなら DI とみなす(自動配線で置き換える)
    let stmts = crate::parse::split_top(body, ";");
    stmts.iter().filter(|s| !s.is_empty()).any(|s| {
        let t: Vec<String> = s.iter().map(|x| x.text()).collect();
        !(t.len() >= 4 && t[0] == "this" && t[1] == "." && fields.iter().any(|f| f.name == t[2]) && t[3] == "=")
    })
}

fn wire_value(ty: &str, class_map: &HashMap<&str, &Class>, impls: &HashMap<String, Vec<&str>>, dtos: &HashSet<String>) -> String {
    let base = ty.split('<').next().unwrap_or("").rsplit('.').next().unwrap_or("");
    if dtos.contains(base) {
        return "None".into();
    }
    if let Some(c) = class_map.get(base) {
        if c.kind == "interface" {
            if is_repo_iface(c) {
                return format!("{base}()");
            }
            if let Some(v) = impls.get(base) {
                if v.len() == 1 {
                    return format!("{}()", v[0]);
                }
            }
            return format!("{base}()");
        }
        if c.kind == "class" {
            return format!("{base}()");
        }
    }
    crate::body::default_value(ty)
}

#[allow(clippy::too_many_arguments)]
fn gen_route(e: &Endpoint, class_map: &HashMap<&str, &Class>, dtos: &HashSet<String>, enums: &HashSet<String>, svc_methods: &HashSet<String>, m: &mut Mod, used: &mut HashSet<String>, warnings: &mut Vec<String>, todo: &mut usize) {
    let verb = match e.verb.as_str() {
        "get" | "post" | "put" | "delete" | "patch" | "head" | "options" => e.verb.clone(),
        v => {
            warnings.push(format!("{} {}: 未対応の動詞 {v}", e.class, e.method));
            return;
        }
    };
    let mut path = e.path.clone();
    let mut sig: Vec<String> = vec![];
    let mut call: Vec<String> = vec![];
    let mut needs_request = false;
    // Python の引数名は snake_case。パスの `{java名}` も合わせる
    for pr in &e.params {
        if pr.kind == Kind::Path && pr.java != snake(&pr.var) {
            path = path.replace(&format!("{{{}}}", pr.java), &format!("{{{}}}", snake(&pr.var)));
        }
    }
    let mut path_vars_in_template: HashSet<String> = HashSet::new();
    for seg in path.split('/') {
        if seg.starts_with('{') && seg.ends_with('}') {
            path_vars_in_template.insert(seg[1..seg.len() - 1].to_string());
        }
    }
    let mut ctx = Ctx { class_name: e.class.clone(), ..Default::default() };
    ctx.dto = dtos.clone();
    ctx.svc_methods = svc_methods.clone();
    ctx.servlet = is_servlet(e);
    let mut required_first: Vec<String> = vec![];
    let mut optional_after: Vec<String> = vec![];
    for pr in &e.params {
        let pyn = snake(&pr.var);
        let t = py_type(&pr.ty, dtos, enums);
        match pr.kind {
            Kind::Path => {
                required_first.push(format!("{pyn}: {t}"));
                call.push(pyn.clone());
                ctx.path_vars.insert(pr.java.clone(), pyn.clone());
            }
            Kind::Query | Kind::Form | Kind::Header | Kind::Cookie => {
                let ctor = match pr.kind {
                    Kind::Header => "Header",
                    Kind::Cookie => "Cookie",
                    Kind::Form => "Form",
                    _ => "Query",
                };
                let alias = if pr.java != pyn || pr.kind == Kind::Header { format!(", alias=\"{}\"", pr.java) } else { String::new() };
                let tt = if t.ends_with("| None") || t == "Any" { t.clone() } else { t.clone() };
                if pr.required && pr.default.is_none() {
                    required_first.push(format!("{pyn}: {tt} = {ctor}(...{alias})"));
                } else {
                    let d = match &pr.default {
                        Some(d) => py_default(d, &pr.ty),
                        None => "None".into(),
                    };
                    let ann = if pr.default.is_some() { tt.clone() } else { format!("{tt} | None") };
                    optional_after.push(format!("{pyn}: {ann} = {ctor}({d}{alias})"));
                }
                call.push(pyn.clone());
                ctx.query_vars.insert(pr.java.clone(), pyn.clone());
            }
            Kind::Body => {
                let tt = if t == "str" { "str".to_string() } else { t.clone() };
                required_first.push(format!("{pyn}: {tt} = Body(...)"));
                call.push(pyn.clone());
                ctx.body_var = Some(pyn.clone());
            }
            Kind::Ctx => {
                needs_request = true;
                call.push(if pr.ty.contains("HttpServletRequest") || pr.ty.contains("HttpHeaders") || pr.ty.contains("UriInfo") { "request".into() } else { "None".into() });
            }
        }
        ctx.locals.insert(pr.var.clone(), pr.ty.clone());
        ctx.param_map.insert(pr.var.clone(), pyn);
    }
    // 本文引数の後ろに Path が来るなど、Python の「既定値なしは既定値ありの前」の制約に合わせて並べ替える
    let (with_def, no_def): (Vec<String>, Vec<String>) = required_first.into_iter().partition(|s| s.contains(" = "));
    sig.extend(no_def);
    if needs_request {
        sig.push("request: Request".into());
    }
    sig.extend(with_def);
    sig.extend(optional_after);
    let base = snake(&e.method);
    let mut name = format!("{}_{}", base, verb);
    if !used.insert(name.clone()) {
        let mut i = 2;
        while !used.insert(format!("{name}_{i}")) {
            i += 1;
        }
        name = format!("{name}_{i}");
    }
    let status = e.status.map(|s| format!(", status_code={s}")).unwrap_or_default();
    m.lines.push(format!("@app.{verb}(\"{path}\"{status})"));
    m.lines.push(format!("def {name}({}):", sig.join(", ")));
    m.lines.push(format!("    # 元: {}:{} ({})", e.file.replace('\\', "/").rsplit('/').next().unwrap_or(""), e.method, e.framework));
    if e.framework == "Servlet" {
        // サーブレット: 本体をそのまま関数へ(出力は _out に集めて返す)
        let c = class_map.get(e.class.as_str()).copied();
        let body = c.and_then(|c| c.methods.iter().find(|x| x.name == e.method)).and_then(|x| x.body.clone()).unwrap_or_default();
        m.lines.push("    _out: list[str] = []".into());
        let lines = translate_body(&body, &mut ctx, 1);
        m.lines.extend(lines);
        m.lines.push("    return PlainTextResponse(\"\".join(_out))".into());
    } else if let Some(lb) = &e.lambda_body {
        ctx.lambda_params = e.lambda_params.clone();
        ctx.ret_calls = true;
        if e.lambda_is_expr {
            let ex = translate_expr(lb, &mut ctx);
            let ex = ex.trim_start_matches('\u{1}').to_string();
            m.lines.push(format!("    return {ex}"));
        } else {
            let lines = translate_body(lb, &mut ctx, 1);
            m.lines.extend(lines);
        }
    } else {
        m.lines.push(format!("    return to_response(_{}.{}({}))", snake(&e.class), base, call.join(", ")));
    }
    *todo += ctx.notes.len();
    warnings.extend(ctx.notes.iter().map(|n| format!("{} {}: {n}", e.class, e.method)));
    m.uses.extend(ctx.uses.iter());
    m.lines.push(String::new());
}

fn py_default(d: &str, ty: &str) -> String {
    match ty {
        "int" | "long" | "short" | "byte" | "Integer" | "Long" | "double" | "float" | "Double" | "Float" => d.to_string(),
        "boolean" | "Boolean" => if d == "true" { "True".into() } else { "False".into() },
        _ => format!("{:?}", d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_unit;

    #[test]
    fn dto_detection_and_types() {
        let u = parse_unit("U.java", "class User { private Long id; private String name; public Long getId() { return id; } public void setName(String n) { this.name = n; } }\n@Service class S { public int f() { return 1; } }");
        assert!(is_dto(&u.classes[0]));
        assert!(!is_dto(&u.classes[1]));
        let d = HashSet::new();
        assert_eq!(py_type("List<Map<String, Integer>>", &d, &d), "list[dict[str, int]]");
        assert_eq!(py_type("Optional<String>", &d, &d), "str | None");
    }
}
