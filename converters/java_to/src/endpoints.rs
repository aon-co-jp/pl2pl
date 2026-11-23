//! Java のルート定義(Spring MVC / JAX-RS / Micronaut / Quarkus / Servlet / Spark / Javalin)を共通の形へ取り出す。

use crate::lexer::Tok;
use crate::parse::{split_top, Ann, Class, Method, Unit};

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Path,
    Query,
    Body,
    Header,
    Cookie,
    Form,
    Ctx, // HttpServletRequest 等(中身は本体の変換側で扱う)
}

#[derive(Clone, Debug)]
pub struct EParam {
    pub java: String, // 通信上の名前
    pub var: String,  // Java の変数名
    pub ty: String,
    pub kind: Kind,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Endpoint {
    pub file: String,
    pub framework: &'static str,
    pub class: String,
    pub method: String, // Java のメソッド名(Spark/Javalin は合成名)
    pub verb: String,   // get/post/put/delete/patch/head/options
    pub path: String,   // `/u/{id}`
    pub params: Vec<EParam>,
    pub ret: String,
    pub lambda_body: Option<Vec<Tok>>, // Spark/Javalin のラムダ本体(式または文)
    pub lambda_is_expr: bool,
    pub lambda_params: Vec<String>,
    pub status: Option<u16>,
    pub notes: Vec<String>,
}

fn join_path(a: &str, b: &str) -> String {
    let mut s = format!("/{}/{}", a.trim_matches('/'), b.trim_matches('/'));
    while s.contains("//") {
        s = s.replace("//", "/");
    }
    if s.len() > 1 {
        s = s.trim_end_matches('/').to_string();
    }
    // `{id:\d+}` / `:id`(Spark)
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    let mut depth = 0;
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                depth += 1;
                out.push(c);
            }
            '}' => {
                depth -= 1;
                out.push(c);
            }
            ':' if depth > 0 => {
                // 正規表現部分を捨てる
                for d in chars.by_ref() {
                    if d == '}' {
                        out.push('}');
                        depth -= 1;
                        break;
                    }
                }
            }
            ' ' if depth > 0 => {}
            _ => out.push(c),
        }
    }
    out
}

fn spark_path(p: &str) -> String {
    p.split('/').map(|seg| if let Some(n) = seg.strip_prefix(':') { format!("{{{n}}}") } else { seg.to_string() }).collect::<Vec<_>>().join("/")
}

fn req_bool(a: &Ann, key: &str, default: bool) -> bool {
    a.get(key).map(|v| v != "false").unwrap_or(default)
}

fn verb_of_spring(m: &Method) -> Option<(String, String)> {
    for a in &m.anns {
        let v = match a.name.as_str() {
            "GetMapping" => "get",
            "PostMapping" => "post",
            "PutMapping" => "put",
            "DeleteMapping" => "delete",
            "PatchMapping" => "patch",
            "RequestMapping" => {
                let verb = a.get("method").map(|s| s.rsplit('.').next().unwrap_or("").to_lowercase()).filter(|s| !s.is_empty()).unwrap_or_else(|| "get".into());
                return Some((verb, a.value().unwrap_or_default()));
            }
            _ => continue,
        };
        return Some((v.to_string(), a.value().unwrap_or_default()));
    }
    None
}

fn verb_of_jaxrs(m: &Method) -> Option<String> {
    m.anns.iter().find_map(|a| match a.name.as_str() {
        "GET" => Some("get".to_string()),
        "POST" => Some("post".to_string()),
        "PUT" => Some("put".to_string()),
        "DELETE" => Some("delete".to_string()),
        "PATCH" => Some("patch".to_string()),
        "HEAD" => Some("head".to_string()),
        "OPTIONS" => Some("options".to_string()),
        _ => None,
    })
}

fn verb_of_micronaut(m: &Method) -> Option<(String, String)> {
    m.anns.iter().find_map(|a| {
        let v = match a.name.as_str() {
            "Get" => "get",
            "Post" => "post",
            "Put" => "put",
            "Delete" => "delete",
            "Patch" => "patch",
            _ => return None,
        };
        Some((v.to_string(), a.value().unwrap_or_default()))
    })
}

fn spring_param(p: &crate::parse::Param, jaxrs: bool) -> EParam {
    let mut e = EParam { java: p.name.clone(), var: p.name.clone(), ty: p.ty.clone(), kind: Kind::Query, required: true, default: None };
    let mut matched = false;
    for a in &p.anns {
        let nm = a.get("name").or_else(|| a.get("value")).cloned().filter(|s| !s.is_empty());
        match a.name.as_str() {
            "PathVariable" | "PathParam" => {
                e.kind = Kind::Path;
                matched = true;
                if let Some(n) = nm {
                    e.java = n;
                }
            }
            "RequestParam" | "QueryParam" | "QueryValue" => {
                e.kind = Kind::Query;
                matched = true;
                if let Some(n) = nm {
                    e.java = n;
                }
                e.required = req_bool(a, "required", true);
                e.default = a.get("defaultValue").cloned();
            }
            "RequestBody" | "Body" => {
                e.kind = Kind::Body;
                matched = true;
            }
            "RequestHeader" | "HeaderParam" | "Header" => {
                e.kind = Kind::Header;
                matched = true;
                if let Some(n) = nm {
                    e.java = n;
                }
                e.required = req_bool(a, "required", true);
                e.default = a.get("defaultValue").cloned();
            }
            "CookieValue" | "CookieParam" => {
                e.kind = Kind::Cookie;
                matched = true;
                if let Some(n) = nm {
                    e.java = n;
                }
            }
            "FormParam" | "RequestPart" => {
                e.kind = Kind::Form;
                matched = true;
                if let Some(n) = nm {
                    e.java = n;
                }
            }
            "DefaultValue" => e.default = a.value(),
            _ => {}
        }
    }
    if e.default.is_some() {
        e.required = false;
    }
    if !matched {
        // 注釈なし: JAX-RS では本文、Spring ではクエリ(単純型)
        let t = p.ty.as_str();
        if t.contains("HttpServletRequest") || t.contains("HttpServletResponse") || t.contains("HttpSession") || t == "Principal" || t.contains("HttpHeaders") || t.contains("UriInfo") || t == "Model" || t == "ModelMap" || t.contains("RoutingContext") || t == "Context" {
            e.kind = Kind::Ctx;
        } else if is_simple(t) && !jaxrs {
            e.kind = Kind::Query;
            e.required = false;
        } else {
            e.kind = Kind::Body;
        }
    }
    e
}

pub fn is_simple(t: &str) -> bool {
    matches!(
        t,
        "String" | "int" | "long" | "short" | "byte" | "double" | "float" | "boolean" | "char" | "Integer" | "Long" | "Short" | "Byte" | "Double" | "Float" | "Boolean" | "Character" | "BigDecimal" | "BigInteger" | "UUID" | "LocalDate" | "LocalDateTime" | "Date" | "Instant"
    )
}

fn status_of(m: &Method) -> Option<u16> {
    for a in &m.anns {
        if a.name == "ResponseStatus" {
            if let Some(v) = a.get("value").or_else(|| a.get("code")) {
                let up = v.rsplit('.').next().unwrap_or("").to_uppercase();
                return Some(match up.as_str() {
                    "CREATED" => 201,
                    "ACCEPTED" => 202,
                    "NO_CONTENT" => 204,
                    "OK" => 200,
                    _ => return None,
                });
            }
        }
    }
    None
}

fn collect_str_calls(body: &[Tok], names: &[&str]) -> Vec<String> {
    let mut out = vec![];
    for w in body.windows(4) {
        if let (Tok::Id(n), Tok::P(p), Tok::Str(s)) = (&w[0], &w[1], &w[2]) {
            if p == "(" && names.contains(&n.as_str()) && !out.contains(s) {
                out.push(s.clone());
            }
        }
    }
    out
}

/// `getParameter("x")` / `queryParam("x")` / `queryParams("x")` などの文字列引数。
pub fn query_names(body: &[Tok]) -> Vec<String> {
    collect_str_calls(body, &["getParameter", "queryParams", "queryParam", "queryParamAsClass", "getParam", "param"])
}

/// OpenAPI(openapi.yml)の1操作。OpenAPI 先行の Spring(`implements XxxApi`)で、メソッド名=operationId から経路を引く。
#[derive(Clone, Debug)]
pub struct ApiOp {
    pub op: String,
    pub verb: String,
    pub path: String,
}

/// openapi.yml から `paths:` の (operationId, 動詞, 経路) を取り出す(YAML の字下げだけを見る簡易版)。
pub fn parse_openapi(text: &str) -> Vec<ApiOp> {
    let mut out = vec![];
    let mut in_paths = false;
    let (mut path, mut verb) = (String::new(), String::new());
    for line in text.lines() {
        let t = line.trim_end();
        if t.is_empty() || t.trim_start().starts_with('#') {
            continue;
        }
        let ind = t.len() - t.trim_start().len();
        let body = t.trim_start();
        if ind == 0 {
            in_paths = body == "paths:";
            continue;
        }
        if !in_paths {
            continue;
        }
        if ind == 2 && body.ends_with(':') && body.starts_with('/') {
            path = body.trim_end_matches(':').trim_matches(|c| c == '\'' || c == '"').to_string();
            verb.clear();
        } else if ind == 4 && body.ends_with(':') && matches!(body.trim_end_matches(':'), "get" | "post" | "put" | "delete" | "patch" | "head" | "options") {
            verb = body.trim_end_matches(':').to_string();
        } else if ind == 6 && body.starts_with("operationId:") && !path.is_empty() && !verb.is_empty() {
            let op = body["operationId:".len()..].trim().trim_matches(|c| c == '\'' || c == '"').to_string();
            out.push(ApiOp { op, verb: verb.clone(), path: path.clone() });
        }
    }
    out
}

pub fn extract(units: &[Unit], warnings: &mut Vec<String>) -> Vec<Endpoint> {
    extract_with(units, warnings, &[])
}

pub fn extract_with(units: &[Unit], warnings: &mut Vec<String>, ops: &[ApiOp]) -> Vec<Endpoint> {
    let mut out = vec![];
    for u in units {
        let imps = u.imports.join(" ");
        for c in &u.classes {
            extract_class(u, c, &imps, &mut out, warnings, ops);
        }
        extract_lambda_routes(u, &imps, &mut out);
    }
    out
}

fn fw_of(imps: &str, spring: bool) -> &'static str {
    if spring {
        return if imps.contains("org.springframework.boot") { "Spring Boot" } else { "Spring MVC" };
    }
    if imps.contains("io.micronaut") {
        "Micronaut"
    } else if imps.contains("io.quarkus") || imps.contains("io.smallrye") {
        "Quarkus"
    } else if imps.contains("org.glassfish.jersey") || imps.contains("com.sun.jersey") {
        "Jersey"
    } else if imps.contains("org.jboss.resteasy") {
        "RESTEasy"
    } else if imps.contains("io.dropwizard") {
        "Dropwizard"
    } else if imps.contains("jakarta.ws.rs") {
        "JAX-RS (jakarta)"
    } else {
        "JAX-RS"
    }
}

fn extract_class(u: &Unit, c: &Class, imps: &str, out: &mut Vec<Endpoint>, warnings: &mut Vec<String>, ops: &[ApiOp]) {
    let spring = c.has_ann(&["RestController", "Controller"]) && !imps.contains("io.micronaut");
    let micro = c.ann("Controller").is_some() && imps.contains("io.micronaut");
    let jaxrs = c.ann("Path").is_some() || c.methods.iter().any(|m| verb_of_jaxrs(m).is_some());
    let servlet = c.extends.iter().any(|e| e.ends_with("HttpServlet") || e.ends_with("GenericServlet"));
    if spring {
        let base = c.ann("RequestMapping").and_then(|a| a.value()).unwrap_or_default();
        for m in &c.methods {
            if let Some((verb, sub)) = verb_of_spring(m) {
                out.push(Endpoint {
                    file: u.file.clone(),
                    framework: fw_of(imps, true),
                    class: c.name.clone(),
                    method: m.name.clone(),
                    verb,
                    path: join_path(&base, &sub),
                    params: m.params.iter().map(|p| spring_param(p, false)).collect(),
                    ret: m.ret.clone(),
                    lambda_body: None,
                    lambda_is_expr: false,
                    lambda_params: vec![],
                    status: status_of(m),
                    notes: vec![],
                });
            } else if let (true, Some(op)) = (c.implements.iter().any(|i| i.ends_with("Api")), ops.iter().find(|o| o.op == m.name)) {
                // OpenAPI 先行: 注釈は生成コード側にあるので、経路の {名前} と型から引数の種類を決める
                let path = join_path("", &op.path);
                let params = m
                    .params
                    .iter()
                    .map(|p| {
                        let mut e = spring_param(p, false);
                        e.kind = if op.path.contains(&format!("{{{}}}", p.name)) {
                            Kind::Path
                        } else if is_simple(&p.ty) {
                            Kind::Query
                        } else {
                            Kind::Body
                        };
                        e.required = e.kind != Kind::Query;
                        e
                    })
                    .collect();
                out.push(Endpoint {
                    file: u.file.clone(),
                    framework: fw_of(imps, true),
                    class: c.name.clone(),
                    method: m.name.clone(),
                    verb: op.verb.clone(),
                    path,
                    params,
                    ret: m.ret.clone(),
                    lambda_body: None,
                    lambda_is_expr: false,
                    lambda_params: vec![],
                    status: None,
                    notes: vec!["OpenAPI(openapi.yml)の operationId から経路を決定".into()],
                });
            }
        }
    } else if micro {
        let base = c.ann("Controller").and_then(|a| a.value()).unwrap_or_default();
        for m in &c.methods {
            if let Some((verb, sub)) = verb_of_micronaut(m) {
                out.push(Endpoint {
                    file: u.file.clone(),
                    framework: "Micronaut",
                    class: c.name.clone(),
                    method: m.name.clone(),
                    verb,
                    path: join_path(&base, &sub),
                    params: m.params.iter().map(|p| spring_param(p, false)).collect(),
                    ret: m.ret.clone(),
                    lambda_body: None,
                    lambda_is_expr: false,
                    lambda_params: vec![],
                    status: status_of(m),
                    notes: vec![],
                });
            }
        }
    } else if servlet {
        let pats = c.ann("WebServlet").and_then(|a| a.get("urlPatterns").cloned().or_else(|| a.value()));
        let Some(pat) = pats else {
            warnings.push(format!("{}: {} は HttpServlet ですが @WebServlet が無く、web.xml のマッピングは未解釈です(パスを /{} と仮定)", short(&u.file), c.name, c.name.to_lowercase()));
            servlet_methods(u, c, &format!("/{}", c.name.to_lowercase()), out);
            return;
        };
        servlet_methods(u, c, &pat.replace("/*", ""), out);
    } else if jaxrs {
        let base = c.ann("Path").and_then(|a| a.value()).unwrap_or_default();
        for m in &c.methods {
            if let Some(verb) = verb_of_jaxrs(m) {
                let sub = m.ann("Path").and_then(|a| a.value()).unwrap_or_default();
                out.push(Endpoint {
                    file: u.file.clone(),
                    framework: fw_of(imps, false),
                    class: c.name.clone(),
                    method: m.name.clone(),
                    verb,
                    path: join_path(&base, &sub),
                    params: m.params.iter().map(|p| spring_param(p, true)).collect(),
                    ret: m.ret.clone(),
                    lambda_body: None,
                    lambda_is_expr: false,
                    lambda_params: vec![],
                    status: None,
                    notes: vec![],
                });
            }
        }
    }
}

fn servlet_methods(u: &Unit, c: &Class, base: &str, out: &mut Vec<Endpoint>) {
    for m in &c.methods {
        let verb = match m.name.as_str() {
            "doGet" => "get",
            "doPost" => "post",
            "doPut" => "put",
            "doDelete" => "delete",
            "doHead" => "head",
            _ => continue,
        };
        let body = m.body.clone().unwrap_or_default();
        let mut params: Vec<EParam> = query_names(&body)
            .into_iter()
            .map(|n| EParam { java: n.clone(), var: n, ty: "String".into(), kind: if verb == "get" { Kind::Query } else { Kind::Form }, required: false, default: None })
            .collect();
        for p in &m.params {
            params.push(EParam { java: p.name.clone(), var: p.name.clone(), ty: p.ty.clone(), kind: Kind::Ctx, required: false, default: None });
        }
        out.push(Endpoint {
            file: u.file.clone(),
            framework: "Servlet",
            class: c.name.clone(),
            method: m.name.clone(),
            verb: verb.into(),
            path: join_path(base, ""),
            params,
            ret: "void".into(),
            lambda_body: None,
            lambda_is_expr: false,
            lambda_params: vec![],
            status: None,
            notes: vec![],
        });
    }
}

fn short(p: &str) -> String {
    p.replace('\\', "/").rsplit('/').next().unwrap_or(p).to_string()
}

fn extract_lambda_routes(u: &Unit, imps: &str, out: &mut Vec<Endpoint>) {
    let spark = imps.contains("spark.Spark") || imps.contains("com.sparkjava") || imps.contains("spark.");
    let javalin = imps.contains("io.javalin");
    if !(spark || javalin) {
        return;
    }
    let t = &u.tokens;
    let mut i = 0;
    let mut n = 0;
    while i + 3 < t.len() {
        let verb = match &t[i] {
            Tok::Id(s) if matches!(s.as_str(), "get" | "post" | "put" | "delete" | "patch" | "head") => s.clone(),
            _ => {
                i += 1;
                continue;
            }
        };
        if t[i + 1].is("(") && matches!(t[i + 2], Tok::Str(_)) && t[i + 3].is(",") {
            // 呼び出しの終わりの `)` を探す
            let mut d = 0;
            let mut j = i + 1;
            while j < t.len() {
                if t[j].is("(") {
                    d += 1;
                } else if t[j].is(")") {
                    d -= 1;
                    if d == 0 {
                        break;
                    }
                }
                j += 1;
            }
            let path = if let Tok::Str(s) = &t[i + 2] { s.clone() } else { String::new() };
            let rest = &t[i + 4..j.min(t.len())];
            // `(a, b) -> ...` / `a -> ...`
            if let Some(arrow) = rest.iter().position(|x| x.is("->")) {
                let head: Vec<String> = rest[..arrow].iter().filter(|x| matches!(x, Tok::Id(_))).map(|x| x.text()).collect();
                let body = &rest[arrow + 1..];
                let (b, is_expr) = if body.first().is_some_and(|x| x.is("{")) && body.last().is_some_and(|x| x.is("}")) { (body[1..body.len() - 1].to_vec(), false) } else { (body.to_vec(), true) };
                n += 1;
                let mut params = vec![];
                for seg in spark_path(&path).split('/') {
                    if seg.starts_with('{') && seg.ends_with('}') {
                        let n = seg[1..seg.len() - 1].to_string();
                        params.push(EParam { java: n.clone(), var: n, ty: "String".into(), kind: Kind::Path, required: true, default: None });
                    }
                }
                for q in query_names(&b) {
                    params.push(EParam { java: q.clone(), var: q, ty: "String".into(), kind: Kind::Query, required: false, default: None });
                }
                out.push(Endpoint {
                    file: u.file.clone(),
                    framework: if spark { "Spark Java" } else { "Javalin" },
                    class: "Routes".into(),
                    method: format!("{}_{}", verb, n),
                    verb,
                    path: join_path("", &spark_path(&path)),
                    params,
                    ret: "Object".into(),
                    lambda_body: Some(b),
                    lambda_is_expr: is_expr,
                    lambda_params: head,
                    status: None,
                    notes: vec![],
                });
            }
            i = j.max(i + 1);
        } else {
            i += 1;
        }
    }
    let _ = split_top;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_unit;

    #[test]
    fn spring_endpoints() {
        let u = parse_unit(
            "C.java",
            r#"import org.springframework.web.bind.annotation.*;
            @RestController @RequestMapping("/api/")
            class C { @GetMapping("/u/{id:\\d+}") User get(@PathVariable("id") Long id, @RequestParam(required=false) String q) { return null; }
                      @RequestMapping(value="/x", method=RequestMethod.POST) void x(@RequestBody User u) {} }"#,
        );
        let mut w = vec![];
        let e = extract(&[u], &mut w);
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].path, "/api/u/{id}");
        assert_eq!(e[0].params[0].kind, Kind::Path);
        assert!(!e[0].params[1].required);
        assert_eq!(e[1].verb, "post");
        assert_eq!(e[1].params[0].kind, Kind::Body);
    }
    #[test]
    fn jaxrs_and_spark() {
        let u = parse_unit("R.java", "import jakarta.ws.rs.*; @Path(\"/items\") class R { @GET @Path(\"{id}\") public Item one(@PathParam(\"id\") int id, @QueryParam(\"q\") @DefaultValue(\"a\") String q) { return null; } }");
        let s = parse_unit("S.java", "import spark.Spark; class S { void m() { Spark.get(\"/hello/:name\", (req, res) -> \"Hi \" + req.params(\":name\")); } }");
        let mut w = vec![];
        let e = extract(&[u, s], &mut w);
        assert_eq!(e[0].path, "/items/{id}");
        assert_eq!(e[0].params[1].default.as_deref(), Some("a"));
        assert_eq!(e[1].path, "/hello/{name}");
        assert!(e[1].lambda_is_expr);
    }

    #[test]
    fn openapi_first_routes() {
        let y = "openapi: 3.0.1
paths:
  /owners/{ownerId}:
    get:
      operationId: getOwner
    put:
      operationId: updateOwner
  /owners:
    post:
      operationId: addOwner
components:
  x: 1
";
        let ops = parse_openapi(y);
        assert_eq!(ops.len(), 3, "{ops:?}");
        let src = "import org.springframework.web.bind.annotation.RestController;
@RestController
public class OwnerCtl implements OwnersApi {
  public ResponseEntity<OwnerDto> getOwner(Integer ownerId) { return null; }
  public ResponseEntity<OwnerDto> updateOwner(Integer ownerId, OwnerDto dto) { return null; }
  public ResponseEntity<OwnerDto> addOwner(OwnerDto dto) { return null; }
}
";
        let u = crate::parse::parse_unit("O.java", src);
        let mut w = vec![];
        let e = extract_with(&[u], &mut w, &ops);
        assert_eq!(e.len(), 3, "{e:?}");
        let up = e.iter().find(|x| x.method == "updateOwner").unwrap();
        assert_eq!((up.verb.as_str(), up.path.as_str()), ("put", "/owners/{ownerId}"));
        assert_eq!(up.params[0].kind, Kind::Path);
        assert_eq!(up.params[1].kind, Kind::Body);
    }
}
