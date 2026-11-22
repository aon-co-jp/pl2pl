//! Java のメソッド本体(文・式)を Python へ移す。範囲は「よく使う構文」に限り、
//! 移せないものは `raise NotImplementedError("TODO(Java_to): ...")` と元の Java のコメントを残す(黙って捨てない)。

use crate::lexer::Tok;
use crate::parse::split_top;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Ctx {
    pub fields: HashSet<String>,        // 現在のクラスのインスタンスフィールド名
    pub statics: HashSet<String>,       // 現在のクラスの static フィールド/メソッド名
    pub methods: HashSet<String>,       // 現在のクラスのメソッド名
    pub dto: HashSet<String>,           // データクラス名
    pub svc_methods: HashSet<String>,   // データクラス以外が持つメソッド名(`getX` を属性にしないため)
    pub locals: HashMap<String, String>, // 変数名 → Java の型
    pub param_map: HashMap<String, String>, // Java 変数名 → Python 名(変更時)
    pub query_vars: HashMap<String, String>, // 通信上のクエリ名 → Python 名
    pub path_vars: HashMap<String, String>,
    pub body_var: Option<String>,
    pub servlet: bool,
    pub lambda_params: Vec<String>,
    pub hoisted: Vec<String>, // ブロック本体のラムダを文の直前へ出した def(インデント0起点)
    pub lam_n: usize,
    pub notes: Vec<String>,
    pub uses: HashSet<&'static str>, // `time` `uuid` `math` `re` `rt`
    pub in_static: bool,
    pub class_name: String,
    pub ret_calls: bool,
}

pub fn snake(n: &str) -> String {
    let mut out = String::new();
    let cs: Vec<char> = n.chars().collect();
    for (i, c) in cs.iter().enumerate() {
        if c.is_uppercase() {
            let prev_lower = i > 0 && (cs[i - 1].is_lowercase() || cs[i - 1].is_ascii_digit());
            let next_lower = i + 1 < cs.len() && cs[i + 1].is_lowercase();
            if i > 0 && (prev_lower || (next_lower && cs[i - 1].is_uppercase())) {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(*c);
        }
    }
    py_ident(&out)
}

pub fn py_ident(n: &str) -> String {
    const KW: [&str; 35] = [
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
    ];
    if KW.contains(&n) {
        format!("{n}_")
    } else {
        n.to_string()
    }
}

fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

#[derive(Clone, Default)]
pub struct E {
    pub s: String,
    pub ty: String, // "int" "double" "String" "boolean" "" (不明)
    pub setter: Option<(String, String)>,
}
fn e(s: impl Into<String>, ty: &str) -> E {
    E { s: s.into(), ty: ty.into(), setter: None }
}

const PRIMS: [&str; 8] = ["int", "long", "short", "byte", "double", "float", "boolean", "char"];

pub fn is_intlike(t: &str) -> bool {
    matches!(t, "int" | "long" | "short" | "byte" | "Integer" | "Long" | "Short" | "Byte")
}
pub fn is_floatlike(t: &str) -> bool {
    matches!(t, "double" | "float" | "Double" | "Float" | "BigDecimal")
}

pub struct Tr<'a> {
    pub t: &'a [Tok],
    pub i: usize,
    pub c: &'a mut Ctx,
}

impl<'a> Tr<'a> {
    pub fn new(t: &'a [Tok], c: &'a mut Ctx) -> Self {
        Tr { t, i: 0, c }
    }
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }
    fn peek_n(&self, n: usize) -> Option<&Tok> {
        self.t.get(self.i + n)
    }
    fn at(&self, p: &str) -> bool {
        self.peek().is_some_and(|t| t.is(p))
    }
    fn at_id(&self, p: &str) -> bool {
        self.peek().is_some_and(|t| t.id(p))
    }
    fn eat(&mut self, p: &str) -> bool {
        if self.at(p) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn done(&self) -> bool {
        self.i >= self.t.len()
    }
    fn balanced(&mut self, open: &str, close: &str) -> Vec<Tok> {
        let mut d = 0;
        let mut out = vec![];
        while let Some(t) = self.peek().cloned() {
            self.i += 1;
            if t.is(open) {
                d += 1;
                if d == 1 {
                    continue;
                }
            } else if t.is(close) {
                d -= 1;
                if d == 0 {
                    break;
                }
            }
            out.push(t);
        }
        out
    }

    // ---------------- 文 ----------------

    pub fn stmts(&mut self, ind: usize, out: &mut Vec<String>) {
        while !self.done() {
            self.stmt(ind, out);
        }
    }

    fn pad(ind: usize) -> String {
        "    ".repeat(ind)
    }

    fn todo(&mut self, ind: usize, out: &mut Vec<String>, what: &str, toks: &[Tok]) {
        let src: String = toks.iter().map(|t| t.text()).collect::<Vec<_>>().join(" ");
        let src: String = src.chars().take(160).collect();
        self.c.notes.push(format!("{what}: {src}"));
        out.push(format!("{}# Java: {}", Self::pad(ind), src.replace('\n', " ")));
        out.push(format!("{}raise NotImplementedError(\"TODO(Java_to): {}\")", Self::pad(ind), what));
    }

    fn skip_stmt(&mut self) -> Vec<Tok> {
        let st = self.i;
        let mut d = 0i32;
        while let Some(t) = self.peek() {
            if t.is("(") || t.is("{") || t.is("[") {
                d += 1;
            } else if t.is(")") || t.is("}") || t.is("]") {
                d -= 1;
            } else if t.is(";") && d <= 0 {
                self.i += 1;
                break;
            }
            self.i += 1;
            if d == 0 && self.t.get(self.i - 1).is_some_and(|x| x.is("}")) && !self.at("else") && !self.at("catch") && !self.at("finally") && !self.at_id("while") {
                break;
            }
        }
        self.t[st..self.i].to_vec()
    }

    /// switch 文を if/elif/else へ。フォールスルー・途中の break・default の合流は変換せず false(呼び出し側が TODO にする)。
    fn switch_stmt(&mut self, ind: usize, out: &mut Vec<String>, assign: Option<&str>) -> bool {
        self.i += 1;
        if !self.at("(") {
            return false;
        }
        let head = self.balanced("(", ")");
        if !self.at("{") {
            return false;
        }
        let inner = self.balanced("{", "}");
        // 節: (ラベル群, default か, 矢印形か, 本体)
        let mut clauses: Vec<(Vec<Vec<Tok>>, bool, bool, Vec<Tok>)> = vec![];
        let mut i = 0usize;
        while i < inner.len() {
            let is_default = inner[i].id("default");
            if !(inner[i].id("case") || is_default) {
                return false;
            }
            i += 1;
            let mut label: Vec<Tok> = vec![];
            let mut d = 0i32;
            let mut arrow = false;
            while i < inner.len() {
                let t = &inner[i];
                if t.is("(") || t.is("[") || t.is("{") {
                    d += 1;
                } else if t.is(")") || t.is("]") || t.is("}") {
                    d -= 1;
                } else if d == 0 && (t.is(":") || t.is("->")) {
                    arrow = t.is("->");
                    i += 1;
                    break;
                }
                label.push(t.clone());
                i += 1;
            }
            let labels = if is_default { vec![] } else { split_top(&label, ",") };
            let st = i;
            let mut d = 0i32;
            while i < inner.len() {
                let t = &inner[i];
                if d == 0 && (t.id("case") || t.id("default")) {
                    break;
                }
                if t.is("(") || t.is("[") || t.is("{") {
                    d += 1;
                } else if t.is(")") || t.is("]") || t.is("}") {
                    d -= 1;
                }
                i += 1;
            }
            clauses.push((labels, is_default, arrow, inner[st..i].to_vec()));
        }
        if clauses.is_empty() || clauses.iter().any(|c| c.2 != clauses[0].2) {
            return false;
        }
        let is_arrow = clauses[0].2;
        if assign.is_some() && (!is_arrow || clauses.iter().any(|c| c.3.iter().any(|t| t.id("yield")))) {
            return false;
        }
        // 古い形: 空の節は次の節と合流(case 1: case 2: ...)
        let mut merged: Vec<(Vec<Vec<Tok>>, bool, Vec<Tok>)> = vec![];
        let mut pend: Vec<Vec<Tok>> = vec![];
        let mut pend_default = false;
        let n = clauses.len();
        for (k, (labels, is_def, arrow, mut body)) in clauses.into_iter().enumerate() {
            pend.extend(labels);
            pend_default |= is_def;
            if body.is_empty() && !arrow && k + 1 < n {
                continue;
            }
            if !arrow {
                let ends_break = body.len() >= 2 && body[body.len() - 1].is(";") && body[body.len() - 2].id("break");
                if ends_break {
                    body.truncate(body.len() - 2);
                } else {
                    // break が無い節は、最後の節か、return/throw で終わるときだけ許す
                    let exits = body.len() >= 2 && body[body.len() - 1].is(";") && body.iter().rev().skip(1).take(60).any(|t| t.id("return") || t.id("throw"));
                    let tail_ok = k + 1 == n || (exits && !body.iter().any(|t| t.is("{") || t.id("if")));
                    if !tail_ok {
                        return false;
                    }
                }
                if body.iter().any(|t| t.id("break")) {
                    return false;
                }
            }
            if pend_default && !pend.is_empty() {
                return false;
            }
            merged.push((std::mem::take(&mut pend), pend_default, body));
            pend_default = false;
        }
        if !pend.is_empty() || pend_default {
            return false;
        }
        let v = format!("_sw{ind}");
        let subj = self.sub_expr(&head).s;
        let p = Self::pad(ind);
        out.push(format!("{p}{v} = {subj}"));
        let mut first = true;
        let mut default_body: Option<Vec<Tok>> = None;
        for (labels, is_def, body) in merged {
            if is_def {
                default_body = Some(body);
                continue;
            }
            let ls: Vec<String> = labels.iter().map(|l| self.sub_expr(l).s).collect();
            let cond = if ls.len() == 1 { format!("{v} == {}", ls[0]) } else { format!("{v} in ({})", ls.join(", ")) };
            out.push(format!("{p}{} {cond}:", if first { "if" } else { "elif" }));
            first = false;
            self.switch_body(ind + 1, out, &body, assign);
        }
        if let Some(b) = default_body {
            out.push(format!("{p}{}", if first { "if True:" } else { "else:" }));
            self.switch_body(ind + 1, out, &b, assign);
        }
        true
    }

    fn switch_body(&mut self, ind: usize, out: &mut Vec<String>, body: &[Tok], assign: Option<&str>) {
        let before = out.len();
        if let Some(a) = assign {
            if !body.first().is_some_and(|t| t.is("{") || t.id("throw")) {
                let mut e: Vec<Tok> = body.to_vec();
                if e.last().is_some_and(|t| t.is(";")) {
                    e.pop();
                }
                let x = self.sub_expr(&e).s;
                out.push(format!("{}{a}{x}", Self::pad(ind)));
                return;
            }
        }
        let toks: Vec<Tok> = if body.first().is_some_and(|t| t.is("{")) && body.last().is_some_and(|t| t.is("}")) { body[1..body.len() - 1].to_vec() } else { body.to_vec() };
        let mut sub = Tr { t: &toks, i: 0, c: &mut *self.c };
        sub.stmts(ind, out);
        if out.len() == before {
            out.push(format!("{}pass", Self::pad(ind)));
        }
    }

    fn body(&mut self, ind: usize, out: &mut Vec<String>) {
        let before = out.len();
        if self.at("{") {
            let inner = self.balanced("{", "}");
            let mut sub = Tr { t: &inner, i: 0, c: &mut *self.c };
            sub.stmts(ind, out);
        } else {
            self.stmt(ind, out);
        }
        if out.len() == before {
            out.push(format!("{}pass", Self::pad(ind)));
        }
    }

    fn stmt(&mut self, ind: usize, out: &mut Vec<String>) {
        let start = out.len();
        let mark = self.c.hoisted.len();
        self.stmt_inner(ind, out);
        if self.c.hoisted.len() > mark {
            let pad = Self::pad(ind);
            let defs: Vec<String> = self.c.hoisted.drain(mark..).map(|l| format!("{pad}{l}")).collect();
            let at = start.min(out.len());
            out.splice(at..at, defs);
        }
    }

    fn stmt_inner(&mut self, ind: usize, out: &mut Vec<String>) {
        let p = Self::pad(ind);
        let Some(tok) = self.peek().cloned() else { return };
        match &tok {
            Tok::P(s) if s == ";" => {
                self.i += 1;
            }
            Tok::P(s) if s == "{" => {
                let inner = self.balanced("{", "}");
                let mut sub = Tr { t: &inner, i: 0, c: &mut *self.c };
                sub.stmts(ind, out);
            }
            Tok::Id(k) if k == "if" => {
                self.i += 1;
                let cond = self.balanced("(", ")");
                let cs = self.sub_expr(&cond).s;
                out.push(format!("{p}if {cs}:"));
                self.body(ind + 1, out);
                while self.at_id("else") {
                    self.i += 1;
                    if self.at_id("if") {
                        self.i += 1;
                        let cond = self.balanced("(", ")");
                        let cs = self.sub_expr(&cond).s;
                        out.push(format!("{p}elif {cs}:"));
                        self.body(ind + 1, out);
                    } else {
                        out.push(format!("{p}else:"));
                        self.body(ind + 1, out);
                        break;
                    }
                }
            }
            Tok::Id(k) if k == "while" => {
                self.i += 1;
                let cond = self.balanced("(", ")");
                let cs = self.sub_expr(&cond).s;
                out.push(format!("{p}while {cs}:"));
                self.body(ind + 1, out);
            }
            Tok::Id(k) if k == "do" => {
                self.i += 1;
                out.push(format!("{p}while True:"));
                self.body(ind + 1, out);
                if self.at_id("while") {
                    self.i += 1;
                    let cond = self.balanced("(", ")");
                    let cs = self.sub_expr(&cond).s;
                    out.push(format!("{}if not ({cs}):", Self::pad(ind + 1)));
                    out.push(format!("{}break", Self::pad(ind + 2)));
                    self.eat(";");
                }
            }
            Tok::Id(k) if k == "for" => self.for_stmt(ind, out),
            Tok::Id(k) if k == "return" && self.peek_n(1).is_some_and(|t| t.id("switch")) => {
                let st = self.i;
                let mark = out.len();
                self.i += 1;
                if self.switch_stmt(ind, out, Some("return ")) {
                    self.eat(";");
                } else {
                    out.truncate(mark);
                    self.i = st;
                    let toks = self.skip_stmt();
                    self.todo(ind, out, "switch 式", &toks);
                }
            }
            Tok::Id(k) if k == "return" => {
                self.i += 1;
                if self.eat(";") {
                    out.push(format!("{p}return"));
                } else {
                    let st = self.i;
                    let ex = self.expr(0);
                    self.eat(";");
                    let _ = st;
                    out.push(format!("{p}return {}", ex.s));
                }
            }
            Tok::Id(k) if k == "throw" => {
                self.i += 1;
                let ex = self.throw_expr();
                self.eat(";");
                out.push(format!("{p}raise {ex}"));
            }
            Tok::Id(k) if k == "break" || k == "continue" => {
                self.i += 1;
                if matches!(self.peek(), Some(Tok::Id(_))) {
                    self.i += 1; // ラベルは無視
                    self.c.notes.push("ラベル付き break/continue".into());
                }
                self.eat(";");
                out.push(format!("{p}{k}"));
            }
            Tok::Id(k) if k == "try" => self.try_stmt(ind, out),
            Tok::Id(k) if k == "synchronized" => {
                self.i += 1;
                let _ = self.balanced("(", ")");
                self.body(ind, out);
            }
            Tok::Id(k) if k == "switch" => {
                let st = self.i;
                let mark = out.len();
                if !self.switch_stmt(ind, out, None) {
                    out.truncate(mark);
                    self.i = st;
                    let toks = self.skip_stmt();
                    self.todo(ind, out, "switch 文", &toks);
                }
            }
            Tok::Id(k) if k == "assert" => {
                let _ = self.skip_stmt();
            }
            _ if self.switch_assign_at() => {
                let st = self.i;
                let mark = out.len();
                let mut j = self.i;
                while !self.t[j].is("=") {
                    j += 1;
                }
                let lhs: Vec<Tok> = self.t[self.i..j].to_vec();
                let target = match lhs.last() {
                    Some(t) if lhs.len() == 1 || matches!(t, Tok::Id(_)) && lhs.len() > 1 && !lhs.iter().any(|x| x.is(".") || x.is("[")) => self.sub_expr(&[t.clone()]).s,
                    _ => self.sub_expr(&lhs).s,
                };
                self.i = j + 1;
                if self.switch_stmt(ind, out, Some(&format!("{target} = "))) {
                    self.eat(";");
                } else {
                    out.truncate(mark);
                    self.i = st;
                    let toks = self.skip_stmt();
                    self.todo(ind, out, "switch 式", &toks);
                }
            }
            _ => {
                if self.looks_like_decl() {
                    self.decl(ind, out);
                } else {
                    let st = self.i;
                    let line = self.expr_stmt();
                    match line {
                        Some(l) => out.push(format!("{p}{l}")),
                        None => {
                            self.i = st;
                            let toks = self.skip_stmt();
                            self.todo(ind, out, "未対応の式", &toks);
                        }
                    }
                }
            }
        }
    }

    fn throw_expr(&mut self) -> String {
        // `new X("msg")` を HTTP エラーなどへ
        if self.at_id("new") {
            let save = self.i;
            self.i += 1;
            let name = self.qname();
            if self.at("(") {
                let args_t = self.balanced("(", ")");
                let args: Vec<E> = split_top(&args_t, ",").iter().filter(|x| !x.is_empty()).map(|a| self.sub_expr(a)).collect();
                let msg = args.last().map(|a| a.s.clone()).unwrap_or_else(|| "\"\"".into());
                let short = name.rsplit('.').next().unwrap_or("");
                let code = match short {
                    "NotFoundException" | "NoSuchElementException" | "EntityNotFoundException" | "ResourceNotFoundException" => Some(404),
                    "BadRequestException" | "IllegalArgumentException" | "ValidationException" => Some(400),
                    "ForbiddenException" | "AccessDeniedException" => Some(403),
                    "NotAuthorizedException" | "UnauthorizedException" | "AuthenticationException" => Some(401),
                    "ConflictException" => Some(409),
                    "IllegalStateException" => Some(409),
                    _ => None,
                };
                self.c.uses.insert("http");
                if short == "ResponseStatusException" {
                    // new ResponseStatusException(HttpStatus.NOT_FOUND, "msg")
                    let st = args.first().map(|a| status_from(&a.s)).unwrap_or(500);
                    let m = args.get(1).map(|a| a.s.clone()).unwrap_or_else(|| "\"\"".into());
                    return format!("HTTPException(status_code={st}, detail={m})");
                }
                if let Some(c) = code {
                    return format!("HTTPException(status_code={c}, detail={msg})");
                }
                return format!("Exception({})", if args.is_empty() { String::new() } else { msg });
            }
            self.i = save;
        }
        self.expr(0).s
    }

    fn qname(&mut self) -> String {
        let mut s = String::new();
        if let Some(Tok::Id(x)) = self.peek().cloned() {
            self.i += 1;
            s = x;
        }
        while self.at(".") && matches!(self.peek_n(1), Some(Tok::Id(_))) {
            self.i += 1;
            if let Some(Tok::Id(x)) = self.peek().cloned() {
                self.i += 1;
                s.push('.');
                s.push_str(&x);
            }
        }
        s
    }

    fn skip_generics(&mut self) {
        let mut d = 0i32;
        while let Some(t) = self.peek().cloned() {
            self.i += 1;
            match &t {
                Tok::P(p) if p == "<" => d += 1,
                Tok::P(p) if p == ">" => d -= 1,
                Tok::P(p) if p == ">>" => d -= 2,
                Tok::P(p) if p == ">>>" => d -= 3,
                _ => {}
            }
            if d <= 0 {
                break;
            }
        }
    }

    /// `Type name =` / `Type name;` / `final var x =` の判定(先読みのみ、位置は戻す)。
    /// `x = switch ...` / `Type x = switch ...` の文頭か。
    fn switch_assign_at(&self) -> bool {
        let mut j = self.i;
        let mut d = 0i32;
        while let Some(t) = self.t.get(j) {
            if t.is("(") || t.is("[") || t.is("{") {
                d += 1;
            } else if t.is(")") || t.is("]") || t.is("}") {
                d -= 1;
            } else if d == 0 && t.is(";") {
                return false;
            } else if d == 0 && t.is("=") {
                return self.t.get(j + 1).is_some_and(|n| n.id("switch")) && j > self.i;
            }
            j += 1;
        }
        false
    }

    fn looks_like_decl(&mut self) -> bool {
        let save = self.i;
        let ok = (|| {
            while self.at_id("final") {
                self.i += 1;
            }
            let Some(Tok::Id(first)) = self.peek().cloned() else { return false };
            if ["new", "this", "super", "return", "else", "case", "default"].contains(&first.as_str()) {
                return false;
            }
            self.qname();
            if self.at("<") {
                self.skip_generics();
            }
            while self.at("[") && self.peek_n(1).is_some_and(|t| t.is("]")) {
                self.i += 2;
            }
            matches!(self.peek(), Some(Tok::Id(_))) && self.peek_n(1).is_some_and(|t| t.is("=") || t.is(";") || t.is(",") || t.is(":"))
        })();
        self.i = save;
        ok
    }

    fn decl(&mut self, ind: usize, out: &mut Vec<String>) {
        let p = Self::pad(ind);
        while self.at_id("final") {
            self.i += 1;
        }
        let mut ty = self.qname();
        if self.at("<") {
            let st = self.i;
            self.skip_generics();
            let g: String = self.t[st..self.i].iter().map(|t| t.text()).collect();
            ty.push_str(&g);
        }
        while self.at("[") {
            self.i += 2;
            ty.push_str("[]");
        }
        loop {
            let Some(Tok::Id(name)) = self.peek().cloned() else { break };
            self.i += 1;
            while self.at("[") {
                self.i += 2;
            }
            let py = py_ident(&snake(&name));
            if self.eat("=") {
                let ex = if self.at("{") {
                    let inner = self.balanced("{", "}");
                    let items: Vec<String> = split_top(&inner, ",").iter().filter(|x| !x.is_empty()).map(|a| self.sub_expr(a).s).collect();
                    e(format!("[{}]", items.join(", ")), "")
                } else {
                    self.expr(0)
                };
                let real_ty = if ty == "var" { ex.ty.clone() } else { ty.clone() };
                self.c.locals.insert(name.clone(), real_ty);
                self.c.param_map.insert(name.clone(), py.clone());
                out.push(format!("{p}{py} = {}", ex.s));
            } else {
                self.c.locals.insert(name.clone(), ty.clone());
                self.c.param_map.insert(name.clone(), py.clone());
                out.push(format!("{p}{py} = {}", default_value(&ty)));
            }
            if !self.eat(",") {
                break;
            }
        }
        self.eat(";");
    }

    fn for_stmt(&mut self, ind: usize, out: &mut Vec<String>) {
        let p = Self::pad(ind);
        self.i += 1;
        let head = self.balanced("(", ")");
        let semis = split_top(&head, ";");
        if semis.len() == 3 {
            let (init, cond, upd) = (&semis[0], &semis[1], &semis[2]);
            // 典型形: int i = A; i < B; i++
            if let Some(range) = classic_range(init, cond, upd, self) {
                let (var, a, b, step) = range;
                self.c.locals.insert(var.clone(), "int".into());
                let py = snake(&var);
                self.c.param_map.insert(var.clone(), py.clone());
                let rng = if step == 1 && a == "0" { format!("range({b})") } else if step == 1 { format!("range({a}, {b})") } else { format!("range({a}, {b}, {step})") };
                out.push(format!("{p}for {py} in {rng}:"));
                self.body(ind + 1, out);
                return;
            }
            // 一般形: init; while cond: body; update
            if !init.is_empty() {
                let mut sub = Tr { t: init, i: 0, c: &mut *self.c };
                sub.stmts(ind, out);
            }
            let cs = if cond.is_empty() { "True".to_string() } else { self.sub_expr(cond).s };
            out.push(format!("{p}while {cs}:"));
            let mut inner = vec![];
            self.body(ind + 1, &mut inner);
            if inner.iter().any(|l| l.trim() == "continue") {
                self.c.notes.push("for 文の continue は更新式を飛ばす(要確認)".into());
            }
            if inner.len() == 1 && inner[0].trim() == "pass" {
                inner.clear();
            }
            out.extend(inner);
            for u in split_top(upd, ",") {
                if u.is_empty() {
                    continue;
                }
                let mut sub = Tr { t: &u, i: 0, c: &mut *self.c };
                if let Some(l) = sub.expr_stmt() {
                    out.push(format!("{}{l}", Self::pad(ind + 1)));
                }
            }
            return;
        }
        // for-each: `Type x : expr`
        let colon = head.iter().position(|t| t.is(":"));
        if let Some(ci) = colon {
            let lhs = &head[..ci];
            let name = lhs.iter().rev().find_map(|t| if let Tok::Id(s) = t { Some(s.clone()) } else { None }).unwrap_or_else(|| "item".into());
            let ty: String = lhs[..lhs.len().saturating_sub(1)].iter().filter(|t| !t.id("final")).map(|t| t.text()).collect();
            let it = self.sub_expr(&head[ci + 1..]);
            let py = snake(&name);
            self.c.locals.insert(name.clone(), ty);
            self.c.param_map.insert(name, py.clone());
            out.push(format!("{p}for {py} in {}:", it.s));
            self.body(ind + 1, out);
            return;
        }
        self.todo(ind, out, "for 文", &head);
        let _ = self.skip_stmt();
    }

    fn try_stmt(&mut self, ind: usize, out: &mut Vec<String>) {
        let p = Self::pad(ind);
        self.i += 1;
        if self.at("(") {
            // try-with-resources: 資源を普通の代入として先に出す(close は呼ばない)
            let res = self.balanced("(", ")");
            for part in split_top(&res, ";") {
                if part.is_empty() {
                    continue;
                }
                let mut sub = Tr { t: &part, i: 0, c: &mut *self.c };
                sub.stmts(ind, out);
            }
            self.c.notes.push("try-with-resources の close() は呼ばない".into());
        }
        out.push(format!("{p}try:"));
        self.body(ind + 1, out);
        let mut had = false;
        while self.at_id("catch") {
            self.i += 1;
            let head = self.balanced("(", ")");
            let name = head.iter().rev().find_map(|t| if let Tok::Id(s) = t { Some(s.clone()) } else { None }).unwrap_or_else(|| "e".into());
            let types: Vec<String> = head
                .iter()
                .filter_map(|t| if let Tok::Id(s) = t { Some(s.clone()) } else { None })
                .filter(|s| s != "final" && *s != name && s.chars().next().is_some_and(|c| c.is_uppercase()))
                .collect();
            let pyty: Vec<String> = types.iter().map(|t| exc_map(t)).collect();
            let list = if pyty.len() == 1 { pyty[0].clone() } else { format!("({})", pyty.join(", ")) };
            self.c.locals.insert(name.clone(), "Exception".into());
            out.push(format!("{p}except {list} as {}:", snake(&name)));
            self.c.param_map.insert(name, "".into());
            self.body(ind + 1, out);
            had = true;
        }
        if self.at_id("finally") {
            self.i += 1;
            out.push(format!("{p}finally:"));
            self.body(ind + 1, out);
            had = true;
        }
        if !had {
            out.push(format!("{p}finally:"));
            out.push(format!("{}pass", Self::pad(ind + 1)));
        }
    }

    /// 式文: 代入・複合代入・++/--・メソッド呼び出し。変換できなければ None。
    fn expr_stmt(&mut self) -> Option<String> {
        let st = self.i;
        let lhs = self.expr(13);
        // 代入
        if let Some(Tok::P(op)) = self.peek().cloned() {
            if ["=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="].contains(&op.as_str()) {
                self.i += 1;
                let rhs = self.expr(0);
                self.eat(";");
                let pyop = match op.as_str() {
                    "/=" if is_intlike(&lhs.ty) => "//=".to_string(),
                    o => o.to_string(),
                };
                if op == "+=" && lhs.ty == "String" {
                    return Some(format!("{} += {}", lhs.s, str_wrap(&rhs)));
                }
                return Some(format!("{} {pyop} {}", lhs.s, rhs.s));
            }
            if op == "++" || op == "--" {
                self.i += 1;
                self.eat(";");
                return Some(format!("{} {}= 1", lhs.s, if op == "++" { "+" } else { "-" }));
            }
        }
        if self.at(";") || self.done() {
            self.eat(";");
            if let Some((r, v)) = lhs.setter {
                return Some(format!("{r} = {v}"));
            }
            if lhs.s.is_empty() {
                return None;
            }
            if let Some(rest) = lhs.s.strip_prefix('\u{1}') {
                return Some(format!("return {rest}"));
            }
            return Some(lhs.s);
        }
        self.i = st;
        None
    }

    // ---------------- 式 ----------------

    pub fn sub_expr(&mut self, toks: &[Tok]) -> E {
        let mut sub = Tr { t: toks, i: 0, c: &mut *self.c };
        sub.expr(0)
    }

    fn binop(op: &str) -> Option<(u8, &'static str)> {
        Some(match op {
            "||" => (3, "or"),
            "&&" => (4, "and"),
            "|" => (5, "|"),
            "^" => (6, "^"),
            "&" => (7, "&"),
            "==" => (8, "=="),
            "!=" => (8, "!="),
            "<" => (9, "<"),
            ">" => (9, ">"),
            "<=" => (9, "<="),
            ">=" => (9, ">="),
            "<<" => (10, "<<"),
            ">>" => (10, ">>"),
            ">>>" => (10, ">>"),
            "+" => (11, "+"),
            "-" => (11, "-"),
            "*" => (12, "*"),
            "/" => (12, "/"),
            "%" => (12, "%"),
            _ => return None,
        })
    }

    pub fn expr(&mut self, min: u8) -> E {
        let mut left = self.unary();
        loop {
            // 三項演算子(優先度 2)
            if min <= 2 && self.at("?") {
                self.i += 1;
                let a = self.expr(2);
                self.eat(":");
                let b = self.expr(2);
                left = e(format!("({} if {} else {})", a.s, left.s, b.s), &a.ty);
                continue;
            }
            // instanceof
            if min <= 9 && self.at_id("instanceof") {
                self.i += 1;
                let ty = self.qname();
                if self.at("<") {
                    self.skip_generics();
                }
                // パターン変数 `instanceof Foo f`
                if matches!(self.peek(), Some(Tok::Id(_))) && !self.at_id("instanceof") {
                    self.i += 1;
                }
                left = e(format!("isinstance({}, {})", left.s, inst_map(&ty)), "boolean");
                continue;
            }
            let Some(Tok::P(op)) = self.peek().cloned() else { break };
            let Some((prec, py)) = Self::binop(&op) else { break };
            if prec < min {
                break;
            }
            self.i += 1;
            let right = self.expr(prec + 1);
            left = self.combine(left, py, right);
        }
        left
    }

    fn combine(&mut self, l: E, op: &str, r: E) -> E {
        match op {
            "+" => {
                if l.ty == "String" || r.ty == "String" || l.s.starts_with('"') || r.s.starts_with('"') {
                    return e(format!("{} + {}", str_wrap(&l), str_wrap(&r)), "String");
                }
                let ty = if is_floatlike(&l.ty) || is_floatlike(&r.ty) { "double" } else if is_intlike(&l.ty) && is_intlike(&r.ty) { "int" } else { "" };
                e(format!("{} + {}", l.s, r.s), ty)
            }
            "/" => {
                if is_intlike(&l.ty) && is_intlike(&r.ty) {
                    e(format!("{} // {}", l.s, r.s), "int")
                } else if is_floatlike(&l.ty) || is_floatlike(&r.ty) {
                    e(format!("{} / {}", l.s, r.s), "double")
                } else {
                    // 型が分からない: 整数同士なら切り捨てだが、確証が無いので `/` のままにして注記
                    self.c.notes.push("型不明の除算: Java の整数除算なら // へ変更".into());
                    e(format!("{} / {}", l.s, r.s), "")
                }
            }
            "==" | "!=" => {
                // 参照比較の null は is / is not
                if r.s == "None" || l.s == "None" {
                    let w = if op == "==" { "is" } else { "is not" };
                    return e(format!("{} {w} {}", l.s, r.s), "boolean");
                }
                e(format!("{} {op} {}", l.s, r.s), "boolean")
            }
            "and" | "or" => e(format!("{} {op} {}", paren_if_or(&l.s, op), paren_if_or(&r.s, op)), "boolean"),
            "<" | ">" | "<=" | ">=" => e(format!("{} {op} {}", l.s, r.s), "boolean"),
            _ => {
                let ty = if is_floatlike(&l.ty) || is_floatlike(&r.ty) { "double" } else if is_intlike(&l.ty) && is_intlike(&r.ty) { "int" } else { "" };
                e(format!("{} {op} {}", l.s, r.s), ty)
            }
        }
    }

    fn unary(&mut self) -> E {
        let Some(t) = self.peek().cloned() else { return e("", "") };
        match &t {
            Tok::P(p) if p == "!" => {
                self.i += 1;
                let x = self.unary();
                e(format!("not {}", wrap_cmp(&x.s)), "boolean")
            }
            Tok::P(p) if p == "-" => {
                self.i += 1;
                let x = self.unary();
                e(format!("-{}", x.s), &x.ty)
            }
            Tok::P(p) if p == "+" => {
                self.i += 1;
                self.unary()
            }
            Tok::P(p) if p == "~" => {
                self.i += 1;
                let x = self.unary();
                e(format!("~{}", x.s), &x.ty)
            }
            Tok::P(p) if p == "++" || p == "--" => {
                self.i += 1;
                let x = self.unary();
                self.c.notes.push("式の中の前置 ++/--".into());
                x
            }
            Tok::P(p) if p == "(" => {
                // キャスト?
                if let (Some(Tok::Id(ty)), Some(close)) = (self.peek_n(1).cloned(), self.peek_n(2)) {
                    if close.is(")") {
                        let next = self.peek_n(3).cloned();
                        let starts = matches!(next, Some(Tok::Id(_)) | Some(Tok::Num(_)) | Some(Tok::Str(_)) | Some(Tok::Chr(_))) || next.as_ref().is_some_and(|n| n.is("(") || n.is("!"));
                        let prim = PRIMS.contains(&ty.as_str());
                        let upper = ty.chars().next().is_some_and(|c| c.is_uppercase());
                        if starts && (prim || upper) {
                            self.i += 3;
                            let x = self.unary();
                            return match ty.as_str() {
                                "int" | "long" | "short" | "byte" => e(format!("int({})", x.s), "int"),
                                "double" | "float" => e(format!("float({})", x.s), "double"),
                                "char" => e(format!("chr({})", x.s), "String"),
                                "String" => e(format!("str({})", x.s), "String"),
                                _ => x,
                            };
                        }
                    }
                }
                let inner = self.balanced("(", ")");
                if self.at("->") {
                    // 複数引数(または型付き・空)のラムダ `(a, b) -> ...`
                    self.i += 1;
                    let params: Vec<String> = split_top(&inner, ",").iter().filter_map(|p| if let Some(Tok::Id(n)) = p.last() { Some(n.clone()) } else { None }).collect();
                    return self.lambda(params);
                }
                let x = self.sub_expr(&inner);
                let r = e(format!("({})", x.s), &x.ty);
                self.postfix(r)
            }
            _ => {
                let p = self.primary();
                self.postfix(p)
            }
        }
    }

    fn args(&mut self) -> Vec<E> {
        let toks = self.balanced("(", ")");
        split_top(&toks, ",").iter().filter(|x| !x.is_empty()).map(|a| self.sub_expr(a)).collect()
    }

    fn primary(&mut self) -> E {
        let Some(t) = self.peek().cloned() else { return e("", "") };
        self.i += 1;
        match t {
            Tok::Num(n) => {
                let ty = if n.contains('.') || n.to_lowercase().ends_with('d') && !n.starts_with("0x") || n.to_lowercase().ends_with('f') && !n.starts_with("0x") { "double" } else { "int" };
                let clean = n.trim_end_matches(['L', 'l', 'f', 'F', 'd', 'D']).to_string();
                e(clean, ty)
            }
            Tok::Str(s) => e(py_str(&s), "String"),
            Tok::Chr(s) => e(py_str(&s.replace("\\'", "'")), "String"),
            Tok::P(p) if p == "{" => {
                self.i -= 1;
                let inner = self.balanced("{", "}");
                let items: Vec<String> = split_top(&inner, ",").iter().filter(|x| !x.is_empty()).map(|a| self.sub_expr(a).s).collect();
                e(format!("[{}]", items.join(", ")), "")
            }
            Tok::Id(id) => match id.as_str() {
                "true" => e("True", "boolean"),
                "false" => e("False", "boolean"),
                "null" => e("None", ""),
                "this" => e("self", ""),
                "super" => e("super()", ""),
                "new" => self.new_expr(),
                _ => self.name_or_call(id),
            },
            Tok::P(p) => e(p, ""),
        }
    }

    fn new_expr(&mut self) -> E {
        let name = self.qname();
        let short = name.rsplit('.').next().unwrap_or("").to_string();
        if self.at("<") {
            self.skip_generics();
        }
        if self.at("[") {
            // 配列
            let mut dims = vec![];
            while self.at("[") {
                let inner = self.balanced("[", "]");
                dims.push(inner);
            }
            if self.at("{") {
                let inner = self.balanced("{", "}");
                let items: Vec<String> = split_top(&inner, ",").iter().filter(|x| !x.is_empty()).map(|a| self.sub_expr(a).s).collect();
                return e(format!("[{}]", items.join(", ")), "");
            }
            let n = dims.first().filter(|d| !d.is_empty()).map(|d| self.sub_expr(d).s).unwrap_or_else(|| "0".into());
            return e(format!("[{}] * ({})", default_value(&short), n), "");
        }
        let args = if self.at("(") { self.args() } else { vec![] };
        if self.at("{") {
            let inner = self.balanced("{", "}");
            return self.anon_class(&short, &inner);
        }
        let a: Vec<String> = args.iter().map(|x| x.s.clone()).collect();
        let r = match short.as_str() {
            "ArrayList" | "LinkedList" | "Vector" | "ArrayDeque" | "CopyOnWriteArrayList" | "Stack" => {
                if a.is_empty() || args[0].ty == "int" { "[]".to_string() } else { format!("list({})", a[0]) }
            }
            "HashMap" | "LinkedHashMap" | "TreeMap" | "ConcurrentHashMap" | "Hashtable" | "EnumMap" => {
                if a.is_empty() || args[0].ty == "int" { "{}".to_string() } else { format!("dict({})", a[0]) }
            }
            "HashSet" | "LinkedHashSet" | "TreeSet" | "CopyOnWriteArraySet" => {
                if a.is_empty() || args[0].ty == "int" { "set()".to_string() } else { format!("set({})", a[0]) }
            }
            "String" => {
                if a.is_empty() { "\"\"".to_string() } else { format!("str({})", a[0]) }
            }
            "StringBuilder" | "StringBuffer" => {
                self.c.uses.insert("rt");
                format!("StringBuilder({})", if a.is_empty() || args[0].ty == "int" { String::new() } else { a[0].clone() })
            }
            "Object" => "object()".to_string(),
            "AtomicInteger" | "AtomicLong" => {
                self.c.uses.insert("rt");
                format!("AtomicNumber({})", a.first().cloned().unwrap_or_else(|| "0".into()))
            }
            "Random" => {
                self.c.uses.insert("random");
                "random.Random()".to_string()
            }
            "Date" => {
                self.c.uses.insert("datetime");
                "datetime.datetime.now()".to_string()
            }
            "Integer" | "Long" | "Short" | "Byte" => format!("int({})", a.first().cloned().unwrap_or_default()),
            "Double" | "Float" | "BigDecimal" => format!("float({})", a.first().cloned().unwrap_or_default()),
            "Boolean" => format!("bool({})", a.first().cloned().unwrap_or_default()),
            "RuntimeException" | "Exception" | "IllegalArgumentException" | "IllegalStateException" | "UnsupportedOperationException" => format!("{}({})", exc_map(&short), a.first().cloned().unwrap_or_default()),
            _ => format!("{}({})", short, a.join(", ")),
        };
        let ty = match short.as_str() {
            "String" | "StringBuilder" | "StringBuffer" => "String",
            "ArrayList" | "LinkedList" => "List",
            "HashMap" | "LinkedHashMap" | "TreeMap" | "ConcurrentHashMap" => "Map",
            "HashSet" | "LinkedHashSet" | "TreeSet" => "Set",
            _ => "",
        };
        e(r, ty)
    }

    fn name_or_call(&mut self, id: String) -> E {
        // 型付きの静的呼び出しや単純な名前
        if self.at("(") {
            let args = self.args();
            return self.unqualified_call(&id, args);
        }
        if self.at("->") {
            // 引数1つのラムダ
            self.i += 1;
            return self.lambda(vec![id]);
        }
        if self.at("::") {
            self.i += 1;
            let m = if let Some(Tok::Id(x)) = self.peek().cloned() {
                self.i += 1;
                x
            } else {
                String::new()
            };
            return self.method_ref(&id, &m);
        }
        self.var(&id)
    }

    /// メソッド参照 `X::m` をラムダ/関数へ
    fn method_ref(&mut self, id: &str, m: &str) -> E {
        let upper = id.chars().next().is_some_and(|c| c.is_uppercase());
        let py = match (id, m) {
            ("String", "valueOf") | ("Objects", "toString") => "str".to_string(),
            ("Integer", "parseInt") | ("Long", "parseLong") | ("Integer", "valueOf") | ("Integer", "intValue") | ("Long", "valueOf") => "int".to_string(),
            ("Double", "parseDouble") | ("Double", "valueOf") => "float".to_string(),
            ("String", "length") | ("List", "size") | ("Collection", "size") => "len".to_string(),
            ("Math", "abs") => "abs".to_string(),
            ("Math", "max") | ("Integer", "max") => "max".to_string(),
            ("Math", "min") | ("Integer", "min") => "min".to_string(),
            ("Integer", "sum") | ("Long", "sum") | ("Double", "sum") => "(lambda a, b: a + b)".to_string(),
            ("Integer", "compare") | ("Long", "compare") | ("Double", "compare") => "(lambda a, b: (a > b) - (a < b))".to_string(),
            ("String", "toUpperCase") => "(lambda s: s.upper())".to_string(),
            ("String", "toLowerCase") => "(lambda s: s.lower())".to_string(),
            ("String", "trim") => "(lambda s: s.strip())".to_string(),
            ("String", "isEmpty") => "(lambda s: len(s) == 0)".to_string(),
            ("this", _) => format!("self.{}", snake(m)),
            (_, "new") => id.to_string(),
            _ if upper => {
                // 型::インスタンスメソッド → 第1引数がレシーバー
                if let Some(prop) = m.strip_prefix("get").filter(|p| p.chars().next().is_some_and(|c| c.is_uppercase())) {
                    format!("(lambda o: o.{})", lower_first(prop))
                } else {
                    format!("(lambda o, *a: o.{}(*a))", snake(m))
                }
            }
            _ => format!("{}.{}", self.var(id).s, snake(m)),
        };
        e(py, "")
    }

    fn var(&mut self, id: &str) -> E {
        if let Some(py) = self.c.param_map.get(id) {
            if !py.is_empty() {
                let ty = self.c.locals.get(id).cloned().unwrap_or_default();
                return e(py.clone(), &norm_ty(&ty));
            }
        }
        if self.c.lambda_params.iter().any(|x| x == id) {
            return e(snake(id), "");
        }
        if self.c.fields.contains(id) {
            let ty = self.c.locals.get(&format!("this.{id}")).cloned().unwrap_or_default();
            return e(format!("self.{}", snake(id)), &norm_ty(&ty));
        }
        if self.c.statics.contains(id) {
            return e(format!("{}.{}", self.c.class_name, id), "");
        }
        if id.chars().next().is_some_and(|c| c.is_uppercase()) {
            // 型名(静的メンバーの前)や定数
            return e(id.to_string(), "");
        }
        e(snake(id), &norm_ty(&self.c.locals.get(id).cloned().unwrap_or_default()))
    }

    /// 匿名クラス `new I() { 型 m(引数) {本体} }` を、メソッドが1つだけなら関数として出す
    fn anon_class(&mut self, short: &str, inner: &[Tok]) -> E {
        // 最初のメソッド: `(` の直前が名前
        let Some(lp) = inner.iter().position(|t| t.is("(")) else {
            self.c.notes.push(format!("匿名クラス new {short}(...) {{...}} はメソッドが見つからず未対応"));
            return e("None", "");
        };
        let name_ok = lp > 0 && matches!(&inner[lp - 1], Tok::Id(_));
        // 対応する `)`
        let mut d = 0;
        let mut rp = None;
        for (k, t) in inner.iter().enumerate().skip(lp) {
            if t.is("(") {
                d += 1;
            } else if t.is(")") {
                d -= 1;
                if d == 0 {
                    rp = Some(k);
                    break;
                }
            }
        }
        let (Some(rp), true) = (rp, name_ok) else {
            self.c.notes.push(format!("匿名クラス new {short}(...) {{...}} は未対応の形"));
            return e("None", "");
        };
        let Some(lb) = inner.iter().skip(rp).position(|t| t.is("{")).map(|k| k + rp) else {
            return e("None", "");
        };
        let mut depth = 0;
        let mut rb = inner.len();
        for (k, t) in inner.iter().enumerate().skip(lb) {
            if t.is("{") {
                depth += 1;
            } else if t.is("}") {
                depth -= 1;
                if depth == 0 {
                    rb = k;
                    break;
                }
            }
        }
        if inner.iter().skip(rb + 1).any(|t| t.is("(")) {
            self.c.notes.push(format!("匿名クラス new {short}(...) {{...}} はメソッドが複数あり、最初の1つだけを関数にした"));
        }
        let params: Vec<String> = crate::parse::split_top(&inner[lp + 1..rp], ",")
            .iter()
            .filter(|x| !x.is_empty())
            .filter_map(|x| x.iter().rev().find_map(|t| if let Tok::Id(n) = t { Some(n.clone()) } else { None }))
            .collect();
        let body: Vec<Tok> = inner[lb + 1..rb].to_vec();
        // フィールド(最初のメソッドより前の `型 名 = 初期値;`)があれば、状態を持つクラスにする
        let hdr = (0..lp).rev().find(|&k| inner[k].is(";") || inner[k].is("}")).map(|k| k + 1).unwrap_or(0);
        if inner[..hdr].iter().any(|t| t.is(";")) {
            let mut inits: Vec<(String, String)> = vec![];
            let mut names: Vec<String> = vec![];
            for f in split_top(&inner[..hdr], ";") {
                if f.is_empty() {
                    continue;
                }
                let eq = f.iter().position(|t| t.is("="));
                let head = &f[..eq.unwrap_or(f.len())];
                let Some(Tok::Id(n)) = head.last().cloned() else { continue };
                let init = match eq {
                    Some(k) => self.sub_expr(&f[k + 1..]).s,
                    None => "None".to_string(),
                };
                names.push(n.clone());
                inits.push((snake(&n), init));
            }
            let saved = std::mem::replace(&mut self.c.fields, names.into_iter().collect());
            for p in &params {
                self.c.lambda_params.push(p.clone());
            }
            self.c.lam_n += 1;
            let cname = format!("_Anon{}", self.c.lam_n);
            let mname = match &inner[lp - 1] {
                Tok::Id(n) => snake(n),
                _ => "call".to_string(),
            };
            let mut lines: Vec<String> = vec![];
            {
                let mut sub = Tr { t: &body, i: 0, c: &mut *self.c };
                sub.stmts(2, &mut lines);
            }
            for _ in &params {
                self.c.lambda_params.pop();
            }
            self.c.fields = saved;
            if lines.is_empty() {
                lines.push("        pass".into());
            }
            let mut def = vec![format!("class {cname}:"), "    def __init__(self):".to_string()];
            for (n, v) in &inits {
                def.push(format!("        self.{n} = {v}"));
            }
            if inits.is_empty() {
                def.push("        pass".into());
            }
            let plist: String = params.iter().map(|x| format!(", {}", snake(x))).collect();
            def.push(format!("    def {mname}(self{plist}):"));
            def.extend(lines);
            def.push(format!("    __call__ = {mname}"));
            self.c.hoisted.extend(def);
            return e(format!("{cname}()"), "");
        }
        for p in &params {
            self.c.lambda_params.push(p.clone());
        }
        self.c.lam_n += 1;
        let fname = format!("_anon{}", self.c.lam_n);
        let mut lines: Vec<String> = vec![];
        {
            let mut sub = Tr { t: &body, i: 0, c: &mut *self.c };
            sub.stmts(1, &mut lines);
        }
        for _ in &params {
            self.c.lambda_params.pop();
        }
        if lines.is_empty() {
            lines.push("    pass".into());
        }
        let mut def = vec![format!("def {fname}({}):", params.iter().map(|x| snake(x)).collect::<Vec<_>>().join(", "))];
        def.extend(lines);
        self.c.hoisted.extend(def);
        e(fname, "")
    }

    fn lambda(&mut self, params: Vec<String>) -> E {
        // 式のラムダだけ変換、ブロックは未対応(注記して None)
        for p in &params {
            self.c.lambda_params.push(p.clone());
        }
        let r = if self.at("{") {
            let inner = self.balanced("{", "}");
            // 単一の `return expr;` なら式に
            if inner.first().is_some_and(|t| t.id("return")) && inner.iter().filter(|t| t.is(";")).count() == 1 {
                e(format!("lambda {}: {}", params.iter().map(|x| snake(x)).collect::<Vec<_>>().join(", "), self.sub_expr(&inner[1..inner.len() - 1]).s), "")
            } else {
                // 複数の文を持つブロック本体: 直前に `def _lamN(...)` として出し、その名前を返す
                self.c.lam_n += 1;
                let name = format!("_lam{}", self.c.lam_n);
                let mut lines: Vec<String> = vec![];
                {
                    let mut sub = Tr { t: &inner, i: 0, c: &mut *self.c };
                    sub.stmts(1, &mut lines);
                }
                if lines.is_empty() {
                    lines.push("    pass".into());
                }
                let mut def = vec![format!("def {name}({}):", params.iter().map(|x| snake(x)).collect::<Vec<_>>().join(", "))];
                def.extend(lines);
                self.c.hoisted.extend(def);
                e(name, "")
            }
        } else {
            let b = self.expr(2);
            e(format!("lambda {}: {}", params.iter().map(|x| snake(x)).collect::<Vec<_>>().join(", "), b.s), "")
        };
        for _ in &params {
            self.c.lambda_params.pop();
        }
        r
    }

    fn unqualified_call(&mut self, name: &str, args: Vec<E>) -> E {
        let a: Vec<String> = args.iter().map(|x| x.s.clone()).collect();
        if name == "this" {
            return e(format!("self.__init__({})", a.join(", ")), "");
        }
        if name == "super" {
            return e(format!("super().__init__({})", a.join(", ")), "");
        }
        if self.c.methods.contains(name) {
            let recv = if self.c.in_static { "cls" } else { "self" };
            return e(format!("{recv}.{}({})", snake(name), a.join(", ")), "");
        }
        e(format!("{}({})", snake(name), a.join(", ")), "")
    }

    fn postfix(&mut self, mut x: E) -> E {
        loop {
            if self.at(".") || self.at("?.") {
                self.i += 1;
                let Some(Tok::Id(name)) = self.peek().cloned() else { break };
                self.i += 1;
                if self.at("<") {
                    // 明示的な型引数 `.<T>foo()` は読み飛ばし
                }
                if self.at("(") {
                    let args = self.args();
                    x = self.method_call(x, &name, args);
                } else if name == "length" && x.ty.ends_with("[]") {
                    x = e(format!("len({})", x.s), "int");
                } else if name == "length" {
                    x = e(format!("len({})", x.s), "int");
                } else {
                    // フィールド参照: 型名.CONSTANT や obj.field
                    let n = if x.s.chars().next().is_some_and(|c| c.is_uppercase()) && x.s.chars().all(|c| c.is_alphanumeric() || c == '_') { name.clone() } else { name.clone() };
                    x = e(format!("{}.{}", x.s, if n.chars().all(|c| c.is_uppercase() || c == '_' || c.is_ascii_digit()) { n } else { n }), "");
                }
            } else if self.at("[") {
                let inner = self.balanced("[", "]");
                let idx = self.sub_expr(&inner);
                x = e(format!("{}[{}]", x.s, idx.s), "");
            } else if self.at("++") || self.at("--") {
                break; // 文側で扱う
            } else if self.at("->") {
                break;
            } else {
                break;
            }
        }
        x
    }

    fn user_method(&self, name: &str) -> bool {
        self.c.methods.contains(name) || self.c.svc_methods.contains(name)
    }

    fn method_call(&mut self, recv: E, name: &str, args: Vec<E>) -> E {
        let r = recv.s.clone();
        let a: Vec<String> = args.iter().map(|x| x.s.clone()).collect();
        let n = args.len();
        let is_type = r.chars().next().is_some_and(|c| c.is_uppercase()) && !r.contains('(') && !r.contains('.');
        // ---- 静的メソッド ----
        if is_type || r.starts_with("System.") {
            if let Some(x) = self.static_call(&r, name, &args) {
                return x;
            }
        }
        if r == "System.out" || r == "System.err" {
            return e(format!("print({})", a.join(", ")), "");
        }
        // ---- サーブレット / Spark / Javalin の入出力 ----
        if let Some(x) = self.web_call(&r, name, &args) {
            return x;
        }
        // ---- 文字列・コレクション ----
        let ty = recv.ty.as_str();
        let is_map = ty.starts_with("Map") || ty.contains("HashMap") || ty.contains("TreeMap");
        let is_set = ty.starts_with("Set") || ty.contains("HashSet") || ty.contains("TreeSet");
        let out = match (name, n) {
            ("equals", 1) => e(format!("({r} == {})", a[0]), "boolean"),
            ("equalsIgnoreCase", 1) => e(format!("({r}.lower() == {}.lower())", a[0]), "boolean"),
            ("length", 0) | ("size", 0) => e(format!("len({r})"), "int"),
            ("isEmpty", 0) => e(format!("(len({r}) == 0)"), "boolean"),
            ("isBlank", 0) => e(format!("({r}.strip() == \"\")"), "boolean"),
            ("toString", 0) => e(format!("str({r})"), "String"),
            ("toUpperCase", 0) => e(format!("{r}.upper()"), "String"),
            ("toLowerCase", 0) => e(format!("{r}.lower()"), "String"),
            ("trim", 0) | ("strip", 0) => e(format!("{r}.strip()"), "String"),
            ("substring", 1) => e(format!("{r}[{}:]", a[0]), "String"),
            ("substring", 2) => e(format!("{r}[{}:{}]", a[0], a[1]), "String"),
            ("charAt", 1) => e(format!("{r}[{}]", a[0]), "String"),
            ("indexOf", 1) => e(format!("{r}.find({})", a[0]), "int"),
            ("lastIndexOf", 1) => e(format!("{r}.rfind({})", a[0]), "int"),
            ("startsWith", 1) | ("endsWith", 1) => e(format!("{r}.{}({})", name.to_lowercase().replace("with", "with"), a[0]), "boolean"),
            ("replace", 2) | ("replaceAll", 2) => {
                if name == "replaceAll" {
                    self.c.uses.insert("re");
                    e(format!("re.sub({}, {}, {r})", a[0], a[1]), "String")
                } else {
                    e(format!("{r}.replace({}, {})", a[0], a[1]), "String")
                }
            }
            ("split", 1) => {
                self.c.uses.insert("re");
                e(format!("re.split({}, {r})", a[0]), "")
            }
            ("contains", 1) if is_map => e(format!("({} in {r})", a[0]), "boolean"),
            ("contains", 1) | ("containsKey", 1) => e(format!("({} in {r})", a[0]), "boolean"),
            ("containsValue", 1) => e(format!("({} in {r}.values())", a[0]), "boolean"),
            ("add", 1) if is_set => e(format!("{r}.add({})", a[0]), ""),
            ("add", 1) | ("addLast", 1) | ("push", 1) | ("offer", 1) => e(format!("{r}.append({})", a[0]), ""),
            ("add", 2) => e(format!("{r}.insert({}, {})", a[0], a[1]), ""),
            ("addAll", 1) => e(format!("{r}.extend({})", a[0]), ""),
            ("put", 2) => {
                e(format!("{r}.__setitem__({}, {})", a[0], a[1]), "")
            }
            ("putAll", 1) => e(format!("{r}.update({})", a[0]), ""),
            ("get", 1) if is_map => e(format!("{r}.get({})", a[0]), ""),
            ("get", 1) => e(format!("{r}[{}]", a[0]), ""),
            ("getOrDefault", 2) => e(format!("{r}.get({}, {})", a[0], a[1]), ""),
            ("remove", 1) if is_map => e(format!("{r}.pop({}, None)", a[0]), ""),
            ("remove", 1) => e(format!("{r}.remove({})", a[0]), ""),
            ("clear", 0) => e(format!("{r}.clear()"), ""),
            ("keySet", 0) => e(format!("{r}.keys()"), ""),
            ("values", 0) => e(format!("{r}.values()"), ""),
            ("entrySet", 0) => e(format!("{r}.items()"), ""),
            ("getKey", 0) => e(format!("{r}[0]"), ""),
            ("getValue", 0) => e(format!("{r}[1]"), ""),
            ("append", 1) if ty == "String" || r.ends_with("sb") || ty == "StringBuilder" => e(format!("{r}.append({})", a[0]), ""),
            ("orElse", 1) => {
                self.c.uses.insert("rt");
                e(format!("or_else({r}, {})", a[0]), "")
            }
            ("orElseGet", 1) => {
                self.c.uses.insert("rt");
                e(format!("or_else_get({r}, {})", a[0]), "")
            }
            ("orElseThrow", _) => {
                self.c.uses.insert("rt");
                e(format!("or_else_throw({r}{})", if n == 0 { String::new() } else { format!(", {}", a[0]) }), "")
            }
            ("isPresent", 0) => e(format!("({r} is not None)"), "boolean"),
            ("isEmpty", _) => e(format!("({r} is None)"), "boolean"),
            ("hashCode", 0) => e(format!("hash({r})"), "int"),
            ("intValue", 0) | ("longValue", 0) => e(format!("int({r})"), "int"),
            ("doubleValue", 0) => e(format!("float({r})"), "double"),
            ("getMessage", 0) => e(format!("str({r})"), "String"),
            ("printStackTrace", 0) => e("traceback.print_exc()".to_string(), ""),
            ("incrementAndGet", 0) | ("getAndIncrement", 0) => {
                e(format!("{r}.increment_and_get()"), "int")
            }
            // ---- Stream API(中間結果は list として即時に評価する) ----
            ("stream", 0) | ("boxed", 0) | ("toList", 0) if !self.user_method(name) && r != "Collectors" => e(format!("list({r})"), ""),
            ("map", 1) | ("mapToInt", 1) | ("mapToLong", 1) | ("mapToDouble", 1) | ("mapToObj", 1) if !self.user_method(name) => e(format!("list(map({}, {r}))", a[0]), ""),
            ("filter", 1) if !self.user_method(name) => e(format!("list(filter({}, {r}))", a[0]), ""),
            ("forEach", 1) if !self.user_method(name) => e(format!("list(map({}, {r}))", a[0]), ""),
            ("anyMatch", 1) if !self.user_method(name) => e(format!("any(map({}, {r}))", a[0]), "boolean"),
            ("allMatch", 1) if !self.user_method(name) => e(format!("all(map({}, {r}))", a[0]), "boolean"),
            ("noneMatch", 1) if !self.user_method(name) => e(format!("(not any(map({}, {r})))", a[0]), "boolean"),
            ("distinct", 0) if !self.user_method(name) => e(format!("list(dict.fromkeys({r}))"), ""),
            ("limit", 1) if !self.user_method(name) => e(format!("list({r})[:{}]", a[0]), ""),
            ("skip", 1) if !self.user_method(name) => e(format!("list({r})[{}:]", a[0]), ""),
            ("sorted", 0) if !self.user_method(name) => e(format!("sorted({r})"), ""),
            ("sorted", 1) if !self.user_method(name) => e(format!("sorted({r}, key=__import__(\"functools\").cmp_to_key({}))", a[0]), ""),
            ("reversed", 0) if !self.user_method(name) && r.starts_with("(lambda a, b:") => e(format!("(lambda a, b: ({r})(b, a))"), ""),
            ("sum", 0) if !self.user_method(name) => e(format!("sum({r})"), "int"),
            ("count", 0) if !self.user_method(name) => e(format!("len(list({r}))"), "long"),
            ("findFirst", 0) | ("findAny", 0) if !self.user_method(name) => e(format!("next(iter({r}), None)"), ""),
            ("reduce", 2) if !self.user_method(name) => e(format!("__import__(\"functools\").reduce({}, {r}, {})", a[1], a[0]), ""),
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.to_list(") => e(format!("list({r})"), ""),
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.to_set(") => e(format!("set({r})"), ""),
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.joining(") => {
                let sep = a[0].trim_start_matches("Collectors.joining(").trim_end_matches(')').to_string();
                let sep = if sep.is_empty() { "\"\"".to_string() } else { sep };
                e(format!("{sep}.join(map(str, {r}))"), "String")
            }
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.partitioning_by(") => {
                let inner = a[0].trim_start_matches("Collectors.partitioning_by(").strip_suffix(')').unwrap_or("").to_string();
                let (f, counting) = match inner.find(", Collectors.counting(") {
                    Some(i) => (inner[..i].to_string(), true),
                    None => (inner.clone(), false),
                };
                let v = |b: &str| if counting { format!("len([x for x in _l if bool(_f(x)) == {b}])") } else { format!("[x for x in _l if bool(_f(x)) == {b}]") };
                e(format!("(lambda _l, _f: {{False: {}, True: {}}})({r}, {f})", v("False"), v("True")), "")
            }
            ("collect", 1) if !self.user_method(name) && (a[0].starts_with("Collectors.averaging_int(") || a[0].starts_with("Collectors.averaging_double(") || a[0].starts_with("Collectors.averaging_long(")) => {
                let f = a[0].split_once('(').map(|x| x.1).unwrap_or("").strip_suffix(')').unwrap_or("").to_string();
                e(format!("(lambda _l, _f: (sum(map(_f, _l)) / len(_l)) if _l else 0.0)({r}, {f})"), "double")
            }
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.to_map(") => {
                let inner = a[0].trim_start_matches("Collectors.to_map(").strip_suffix(')').unwrap_or("").to_string();
                let parts: Vec<&str> = inner.splitn(3, ", ").collect();
                let (k, v) = (parts.first().copied().unwrap_or("").to_string(), parts.get(1).copied().unwrap_or("").to_string());
                if let Some(m) = parts.get(2) {
                    // 重複キーは結合関数で畳む
                    e(format!("(lambda _l, _k, _v, _m: __import__(\"functools\").reduce(lambda d, x: {{**d, _k(x): (_m(d[_k(x)], _v(x)) if _k(x) in d else _v(x))}}, _l, {{}}))({r}, {k}, {v}, {m})"), "")
                } else {
                    e(format!("(lambda _l, _k, _v: {{_k(x): _v(x) for x in _l}})({r}, {k}, {v})"), "")
                }
            }
            ("collect", 1) if !self.user_method(name) && a[0].starts_with("Collectors.grouping_by(") => {
                let inner = a[0].trim_start_matches("Collectors.grouping_by(").strip_suffix(')').unwrap_or("").to_string();
                let (f, down) = match inner.find(", Collectors.") {
                    Some(i) => (inner[..i].to_string(), inner[i + 2..].to_string()),
                    None => (inner.clone(), String::new()),
                };
                let g = "[x for x in _l if _f(x) == k]";
                let val = if down.is_empty() || down.starts_with("Collectors.to_list(") {
                    g.to_string()
                } else if down.starts_with("Collectors.counting(") {
                    format!("len({g})")
                } else if let Some(rest) = down.strip_prefix("Collectors.summing_int(").or_else(|| down.strip_prefix("Collectors.summing_long(")).or_else(|| down.strip_prefix("Collectors.summing_double(")) {
                    format!("sum(map({}, {g}))", rest.strip_suffix(')').unwrap_or(rest))
                } else if let Some(rest) = down.strip_prefix("Collectors.mapping(") {
                    let m = rest.split(", Collectors.").next().unwrap_or("").to_string();
                                        if rest.contains("Collectors.to_set(") { format!("set(map({m}, {g}))") } else { format!("list(map({m}, {g}))") }
                } else if let Some(rest) = down.strip_prefix("Collectors.averaging_int(").or_else(|| down.strip_prefix("Collectors.averaging_long(")).or_else(|| down.strip_prefix("Collectors.averaging_double(")) {
                    let m = rest.strip_suffix(')').unwrap_or(rest);
                    format!("(lambda _g: sum(map({m}, _g)) / len(_g))({g})")
                } else if let Some(rest) = down.strip_prefix("Collectors.joining(") {
                    let sep = rest.strip_suffix(')').unwrap_or(rest);
                    format!("{}.join(map(str, {g}))", if sep.is_empty() { "\"\"" } else { sep })
                } else if let Some(rest) = down.strip_prefix("Collectors.to_set(") {
                    let _ = rest;
                    format!("set({g})")
                } else if let Some(rest) = down.strip_prefix("Collectors.reducing(") {
                    // reducing(identity, op) / reducing(identity, mapper, op)
                    let args = split_py_args(rest.strip_suffix(')').unwrap_or(rest));
                    match args.as_slice() {
                        [id, op] => format!("__import__('functools').reduce({op}, {g}, {id})"),
                        [id, m, op] => format!("__import__('functools').reduce({op}, map({m}, {g}), {id})"),
                        _ => {
                            self.c.notes.push(format!("groupingBy の下流コレクター {down} は未対応"));
                            g.to_string()
                        }
                    }
                } else {
                    self.c.notes.push(format!("groupingBy の下流コレクター {down} は未対応"));
                    g.to_string()
                };
                e(format!("(lambda _l, _f: {{k: {val} for k in dict.fromkeys(map(_f, _l))}})({r}, {f})"), "")
            }
            ("thenComparing", 1) | ("thenComparingInt", 1) | ("thenComparingDouble", 1) | ("thenComparingLong", 1) if r.starts_with("(lambda a, b:") => {
                let head = a[0].split(':').next().unwrap_or("");
                let k = if head.contains(',') { format!("({})(a, b)", a[0]) } else { format!("(lambda _k: (_k(a) > _k(b)) - (_k(a) < _k(b)))({})", a[0]) };
                e(format!("(lambda a, b: ({r})(a, b) or {k})"), "")
            }
            ("sorted", _) | ("reduce", _) | ("collect", 1) | ("stream", 0) => {
                self.c.notes.push(format!("Stream API(.{name}) のこの形は未対応"));
                e(format!("{r}.{}({})", snake(name), a.join(", ")), "")
            }
            _ => {
                // get/is/set アクセサ: 他のクラスのメソッドとして定義がなければ属性へ
                if !self.c.svc_methods.contains(name) && !self.c.methods.contains(name) {
                    if let Some(prop) = name.strip_prefix("get").filter(|p| p.chars().next().is_some_and(|c| c.is_uppercase())) {
                        if n == 0 {
                            return e(format!("{r}.{}", lower_first(prop)), "");
                        }
                    }
                    if let Some(prop) = name.strip_prefix("is").filter(|p| p.chars().next().is_some_and(|c| c.is_uppercase())) {
                        if n == 0 {
                            return e(format!("{r}.{}", lower_first(prop)), "boolean");
                        }
                    }
                    if let Some(prop) = name.strip_prefix("set").filter(|p| p.chars().next().is_some_and(|c| c.is_uppercase())) {
                        if n == 1 {
                            let target = format!("{r}.{}", lower_first(prop));
                            return E { s: format!("setattr({r}, \"{}\", {})", lower_first(prop), a[0]), ty: String::new(), setter: Some((target, a[0].clone())) };
                        }
                    }
                }
                if r == "self" && self.c.methods.contains(name) {
                    return e(format!("self.{}({})", snake(name), a.join(", ")), "");
                }
                e(format!("{r}.{}({})", snake(name), a.join(", ")), "")
            }
        };
        out
    }

    fn static_call(&mut self, r: &str, name: &str, args: &[E]) -> Option<E> {
        let a: Vec<String> = args.iter().map(|x| x.s.clone()).collect();
        let n = a.len();
        let out = match (r, name) {
            ("Comparator", "naturalOrder") => e("(lambda a, b: (a > b) - (a < b))".to_string(), ""),
            ("Comparator", "reverseOrder") => e("(lambda a, b: (b > a) - (b < a))".to_string(), ""),
            ("Comparator", "comparing") | ("Comparator", "comparingInt") | ("Comparator", "comparingLong") | ("Comparator", "comparingDouble") if n == 1 => {
                e(format!("(lambda a, b: (({f})(a) > ({f})(b)) - (({f})(a) < ({f})(b)))", f = a[0]), "")
            }
            ("Math", "max") => e(format!("max({})", a.join(", ")), &args.first().map(|x| x.ty.clone()).unwrap_or_default()),
            ("Math", "min") => e(format!("min({})", a.join(", ")), &args.first().map(|x| x.ty.clone()).unwrap_or_default()),
            ("Math", "abs") => e(format!("abs({})", a[0]), &args[0].ty),
            ("Math", "pow") => e(format!("({} ** {})", a[0], a[1]), "double"),
            ("Math", "sqrt") => {
                self.c.uses.insert("math");
                e(format!("math.sqrt({})", a[0]), "double")
            }
            ("Math", "floor") | ("Math", "ceil") => {
                self.c.uses.insert("math");
                e(format!("float(math.{}({}))", name, a[0]), "double")
            }
            ("Math", "round") => e(format!("int(round({}))", a[0]), "int"),
            ("Math", "random") => {
                self.c.uses.insert("random");
                e("random.random()".to_string(), "double")
            }
            ("Integer", "parseInt") | ("Integer", "valueOf") | ("Long", "parseLong") | ("Long", "valueOf") | ("Short", "parseShort") => e(format!("int({})", a[0]), "int"),
            ("Double", "parseDouble") | ("Double", "valueOf") | ("Float", "parseFloat") => e(format!("float({})", a[0]), "double"),
            ("Boolean", "parseBoolean") => e(format!("({}.lower() == \"true\")", a[0]), "boolean"),
            ("Integer", "toString") | ("String", "valueOf") | ("Long", "toString") | ("Double", "toString") | ("Objects", "toString") => e(format!("str({})", a[0]), "String"),
            ("Integer", "compare") | ("Long", "compare") | ("Double", "compare") => e(format!("(({0} > {1}) - ({0} < {1}))", a[0], a[1]), "int"),
            ("Integer", "MAX_VALUE") => e("2147483647", "int"),
            ("String", "format") => {
                self.c.uses.insert("rt");
                e(format!("jformat({})", a.join(", ")), "String")
            }
            ("String", "join") => e(format!("{}.join(str(x) for x in {})", a[0], if n == 2 { a[1].clone() } else { format!("[{}]", a[1..].join(", ")) }), "String"),
            ("Objects", "equals") => e(format!("({} == {})", a[0], a[1]), "boolean"),
            ("Objects", "isNull") => e(format!("({} is None)", a[0]), "boolean"),
            ("Objects", "nonNull") => e(format!("({} is not None)", a[0]), "boolean"),
            ("Objects", "requireNonNull") => e(a[0].clone(), ""),
            ("Objects", "hash") | ("Objects", "hashCode") => e(format!("hash(({},))", a.join(", ")), "int"),
            ("System", "currentTimeMillis") => {
                self.c.uses.insert("time");
                e("int(time.time() * 1000)".to_string(), "int")
            }
            ("System", "nanoTime") => {
                self.c.uses.insert("time");
                e("time.perf_counter_ns()".to_string(), "int")
            }
            ("System", "getenv") => {
                self.c.uses.insert("os");
                e(format!("os.environ.get({})", a[0]), "String")
            }
            ("System", "exit") => e(format!("sys.exit({})", a[0]), ""),
            ("UUID", "randomUUID") => {
                self.c.uses.insert("uuid");
                e("str(uuid.uuid4())".to_string(), "String")
            }
            ("List", "of") | ("Arrays", "asList") | ("Set", "of") | ("Stream", "of") => e(
                if n == 1 && (r == "Arrays" || args[0].ty.ends_with("[]")) { format!("list({})", a[0]) } else if r == "Set" { format!("{{{}}}", a.join(", ")) } else { format!("[{}]", a.join(", ")) },
                if r == "Set" { "Set" } else { "List" },
            ),
            ("Map", "of") => {
                let pairs: Vec<String> = a.chunks(2).map(|c| format!("{}: {}", c[0], c.get(1).cloned().unwrap_or_default())).collect();
                e(format!("{{{}}}", pairs.join(", ")), "Map")
            }
            ("Collections", "emptyList") => e("[]", "List"),
            ("Collections", "emptyMap") => e("{}", "Map"),
            ("Collections", "emptySet") => e("set()", "Set"),
            ("Collections", "sort") => e(format!("{}.sort()", a[0]), ""),
            ("Collections", "unmodifiableList") | ("Collections", "unmodifiableMap") | ("Collections", "unmodifiableSet") => e(a[0].clone(), ""),
            ("Optional", "of") | ("Optional", "ofNullable") => e(a[0].clone(), ""),
            ("Optional", "empty") => e("None", ""),
            ("ResponseEntity", "ok") | ("Response", "ok") => e(a.first().cloned().unwrap_or_else(|| "None".into()), ""),
            ("ResponseEntity", "notFound") | ("ResponseEntity", "noContent") | ("ResponseEntity", "badRequest") | ("ResponseEntity", "accepted") | ("ResponseEntity", "created") | ("ResponseEntity", "status") | ("Response", "status") | ("Response", "created") | ("Response", "noContent") => {
                self.c.uses.insert("rt");
                e(format!("ResponseBuilder(\"{name}\", {})", if a.is_empty() { "None".to_string() } else { a[0].clone() }), "")
            }
            ("LocalDate", "now") | ("LocalDateTime", "now") | ("Instant", "now") => {
                self.c.uses.insert("datetime");
                e("datetime.datetime.now()".to_string(), "")
            }
            ("Thread", "sleep") => {
                self.c.uses.insert("time");
                e(format!("time.sleep({} / 1000)", a[0]), "")
            }
            _ => return None,
        };
        Some(out)
    }

    /// サーブレット(request/response)・Spark(req/res)・Javalin(ctx)の入出力。
    fn web_call(&mut self, r: &str, name: &str, args: &[E]) -> Option<E> {
        let lit = |i: usize| -> Option<String> { args.get(i).map(|x| x.s.trim_matches('"').to_string()) };
        let out = match name {
            "getParameter" | "queryParams" | "queryParam" | "getParam" => {
                let q = lit(0)?;
                let v = self.c.query_vars.get(&q).cloned().unwrap_or_else(|| snake(&q));
                e(v, "String")
            }
            "pathParam" | "params" if !args.is_empty() => {
                let q = lit(0)?;
                let q = q.trim_start_matches(':').to_string();
                e(self.c.path_vars.get(&q).cloned().unwrap_or_else(|| snake(&q)), "String")
            }
            "body" if args.is_empty() && (self.c.lambda_params.iter().any(|p| snake(p) == r) || r == "request") => e(self.c.body_var.clone().unwrap_or_else(|| "body".into()), "String"),
            "bodyAsClass" => e(self.c.body_var.clone().unwrap_or_else(|| "body".into()), ""),
            "getWriter" if args.is_empty() => e("_out", ""),
            "println" | "print" | "write" | "append" if r == "_out" || r.starts_with("out") || r.ends_with("writer") || r.ends_with("_out") => {
                let nl = if name == "println" { "\\n" } else { "" };
                e(format!("_out.append(str({}) + \"{}\")", args.first().map(|x| x.s.clone()).unwrap_or_else(|| "\"\"".into()), nl), "")
            }
            "result" | "json" | "html" | "send" if self.c.lambda_params.iter().any(|p| snake(p) == r) => e(format!("\u{1}{}", args.first().map(|x| x.s.clone()).unwrap_or_default()), ""),
            "status" if self.c.lambda_params.iter().any(|p| snake(p) == r) => e("None", ""),
            "setContentType" | "setCharacterEncoding" | "setHeader" | "type" | "header" | "addHeader" | "setStatus" if r == "response" || r == "resp" || self.c.lambda_params.iter().any(|p| snake(p) == r) => e("None", ""),
            "getRequestDispatcher" | "forward" | "sendRedirect" => {
                self.c.notes.push(format!("{name} は未対応(画面遷移)"));
                return None;
            }
            "getSession" | "getAttribute" | "setAttribute" if r == "request" || r == "req" => {
                self.c.notes.push(format!("{name}(セッション)は未対応"));
                return None;
            }
            _ => return None,
        };
        Some(out)
    }
}

fn paren_if_or(s: &str, op: &str) -> String {
    if op == "and" && s.contains(" or ") && !s.starts_with('(') {
        format!("({s})")
    } else {
        s.to_string()
    }
}

fn wrap_cmp(s: &str) -> String {
    if s.contains(' ') && !s.starts_with('(') && !s.starts_with("not ") && (s.contains(" == ") || s.contains(" and ") || s.contains(" or ") || s.contains(" < ") || s.contains(" > ") || s.contains(" is ") || s.contains(" != ") || s.contains(" in ")) {
        format!("({s})")
    } else {
        s.to_string()
    }
}

fn str_wrap(x: &E) -> String {
    if x.ty == "String" || x.s.starts_with('"') && x.s.ends_with('"') && x.s.len() >= 2 {
        x.s.clone()
    } else if x.s.starts_with("str(") {
        x.s.clone()
    } else {
        format!("str({})", x.s)
    }
}

fn norm_ty(t: &str) -> String {
    let base = t.split('<').next().unwrap_or(t).to_string();
    base
}

fn py_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

pub fn default_value(ty: &str) -> String {
    match ty {
        "int" | "long" | "short" | "byte" => "0".into(),
        "double" | "float" => "0.0".into(),
        "boolean" => "False".into(),
        "char" => "\"\"".into(),
        _ => "None".into(),
    }
}

fn exc_map(t: &str) -> String {
    match t {
        "IOException" | "FileNotFoundException" | "UncheckedIOException" => "OSError".into(),
        "NumberFormatException" => "ValueError".into(),
        "IllegalArgumentException" => "ValueError".into(),
        "IllegalStateException" | "RuntimeException" | "Throwable" | "Exception" | "Error" => "Exception".into(),
        "NullPointerException" => "AttributeError".into(),
        "IndexOutOfBoundsException" | "ArrayIndexOutOfBoundsException" => "IndexError".into(),
        "UnsupportedOperationException" => "NotImplementedError".into(),
        "NoSuchElementException" => "LookupError".into(),
        "InterruptedException" => "KeyboardInterrupt".into(),
        o => o.to_string(),
    }
}

fn inst_map(t: &str) -> String {
    match t {
        "String" => "str".into(),
        "Integer" | "Long" | "Short" => "int".into(),
        "Double" | "Float" => "float".into(),
        "Boolean" => "bool".into(),
        "List" | "ArrayList" | "Collection" => "list".into(),
        "Map" | "HashMap" => "dict".into(),
        "Set" | "HashSet" => "set".into(),
        "Object" => "object".into(),
        o => o.to_string(),
    }
}

fn status_from(s: &str) -> u16 {
    let up = s.rsplit('.').next().unwrap_or("").to_uppercase();
    match up.as_str() {
        "NOT_FOUND" => 404,
        "BAD_REQUEST" => 400,
        "UNAUTHORIZED" => 401,
        "FORBIDDEN" => 403,
        "CONFLICT" => 409,
        "NOT_IMPLEMENTED" => 501,
        "UNPROCESSABLE_ENTITY" => 422,
        "NO_CONTENT" => 204,
        "CREATED" => 201,
        "OK" => 200,
        _ => s.parse().unwrap_or(500),
    }
}

/// `for (int i = A; i < B; i++)` を range に。
fn classic_range(init: &[Tok], cond: &[Tok], upd: &[Tok], tr: &mut Tr) -> Option<(String, String, String, i64)> {
    let ids: Vec<&Tok> = init.iter().filter(|t| !t.id("final")).collect();
    let eq = ids.iter().position(|t| t.is("="))?;
    let var = match ids.get(eq.checked_sub(1)?)? {
        Tok::Id(s) => s.clone(),
        _ => return None,
    };
    let a_toks: Vec<Tok> = ids[eq + 1..].iter().map(|t| (*t).clone()).collect();
    let a = tr.sub_expr(&a_toks).s;
    if cond.len() < 3 || !cond[0].id(&var) {
        return None;
    }
    let (op, rest) = match &cond[1] {
        Tok::P(p) if p == "<" || p == "<=" || p == ">" || p == ">=" => (p.clone(), &cond[2..]),
        _ => return None,
    };
    let b = tr.sub_expr(rest).s;
    let u: Vec<String> = upd.iter().map(|t| t.text()).collect();
    let step: i64 = match u.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice() {
        [v, "++"] | ["++", v] if *v == var => 1,
        [v, "--"] | ["--", v] if *v == var => -1,
        [v, "+=", k] if *v == var => k.parse().ok()?,
        [v, "-=", k] if *v == var => -(k.parse::<i64>().ok()?),
        _ => return None,
    };
    let (end, ok) = match (op.as_str(), step > 0) {
        ("<", true) | (">", false) => (b, true),
        ("<=", true) => (format!("{b} + 1"), true),
        (">=", false) => (format!("{b} - 1"), true),
        _ => (String::new(), false),
    };
    ok.then_some((var, a, end, step))
}

/// メソッド本体(`{ }` の内側のトークン)を Python の行へ。
pub fn translate_body(toks: &[Tok], ctx: &mut Ctx, ind: usize) -> Vec<String> {
    let mut out = vec![];
    let mut tr = Tr::new(toks, ctx);
    tr.stmts(ind, &mut out);
    if out.is_empty() {
        out.push(format!("{}pass", "    ".repeat(ind)));
    }
    out
}

/// 1つの式(ラムダ本体など)を変換。
pub fn translate_expr(toks: &[Tok], ctx: &mut Ctx) -> String {
    let mut tr = Tr::new(toks, ctx);
    tr.expr(0).s
}

/// Python 式の引数をトップレベルのカンマで分ける。
fn split_py_args(s: &str) -> Vec<String> {
    let (mut out, mut cur, mut d) = (vec![], String::new(), 0i32);
    let mut q: Option<char> = None;
    let mut in_lambda_head = false;
    for ch in s.chars() {
        if let Some(c) = q {
            cur.push(ch);
            if ch == c {
                q = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                q = Some(ch);
                cur.push(ch);
            }
            '(' | '[' | '{' => {
                d += 1;
                cur.push(ch);
            }
            ')' | ']' | '}' => {
                d -= 1;
                cur.push(ch);
            }
            ':' if d == 0 && in_lambda_head => {
                in_lambda_head = false;
                cur.push(ch);
            }
            ',' if d == 0 && !in_lambda_head => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => {
                cur.push(ch);
                if d == 0 && cur.ends_with("lambda ") && cur[..cur.len() - 7].chars().last().is_none_or(|c| !c.is_alphanumeric() && c != '_') {
                    in_lambda_head = true;
                }
            }
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    fn tb(src: &str) -> String {
        let mut c = Ctx::default();
        translate_body(&lex(src), &mut c, 1).join("\n")
    }

    #[test]
    fn block_lambda_and_stream() {
        let p = tb("List<Integer> xs = new ArrayList<>(); List<Integer> ys = xs.stream().filter(x -> x > 1).map(x -> { int d = x * 2; if (d > 10) { return 10; } return d; }).collect(Collectors.toList()); int total = xs.stream().mapToInt(x -> x).sum(); String s = xs.stream().map(x -> String.valueOf(x)).collect(Collectors.joining(\", \"));");
        assert!(p.contains("def _lam1(x):"), "{p}");
        assert!(p.contains("d = x * 2"), "{p}");
        assert!(p.contains("list(filter(lambda x: x > 1"), "{p}");
        assert!(p.contains("list(map(_lam1"), "{p}");
        assert!(p.contains("sum(list(map(lambda x: x"), "{p}");
        assert!(p.contains(".join(map(str,"), "{p}");
        assert!(p.find("def _lam1").unwrap() < p.find("ys =").unwrap(), "{p}");
    }
    #[test]
    fn method_refs_and_comparators() {
        let p = tb("List<String> xs = new ArrayList<>(); List<Integer> a = xs.stream().map(String::length).collect(Collectors.toList()); List<String> b = xs.stream().sorted(Comparator.comparing(String::length).reversed()).collect(Collectors.toList()); List<String> c = xs.stream().sorted(Comparator.reverseOrder()).collect(Collectors.toList()); List<String> d = xs.stream().map(String::toUpperCase).collect(Collectors.toList()); int t = a.stream().reduce(0, Integer::sum);");
        assert!(p.contains("map(len, list(xs))"), "{p}");
        assert!(p.contains("cmp_to_key"), "{p}");
        assert!(p.contains("(lambda s: s.upper())"), "{p}");
        assert!(p.contains("(lambda a, b: a + b)"), "{p}");
        assert!(!p.contains("未対応"), "{p}");
        // 生成した Python を実際に動かす
        let prog = format!("xs = [\"bb\", \"a\", \"ccc\"]
{}
print(a, b, c, d, t)
", p.replace("
        ", "
").trim_start());
        let out = std::process::Command::new("python").args(["-c", &prog]).output();
        if let Ok(o) = out {
            if o.status.success() {
                assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "[2, 1, 3] ['ccc', 'bb', 'a'] ['ccc', 'bb', 'a'] ['BB', 'A', 'CCC'] 6", "{prog}");
            }
        }
    }
    #[test]
    fn grouping_and_then_comparing() {
        let p = tb("List<String> xs = new ArrayList<>(); Map<Integer, List<String>> g = xs.stream().collect(Collectors.groupingBy(String::length)); Map<Integer, Long> n = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.counting())); List<String> s = xs.stream().sorted(Comparator.comparing(String::length).thenComparing(Comparator.naturalOrder())).collect(Collectors.toList());");
        assert!(!p.contains("未対応"), "{p}");
        let p = p.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("\n");
        let prog = format!("{}
print(g, n, s)
", p.replacen("xs = []", "xs = [\"bb\", \"a\", \"cc\", \"ddd\", \"b\"]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "{2: ['bb', 'cc'], 1: ['a', 'b'], 3: ['ddd']} {2: 2, 1: 2, 3: 1} ['a', 'b', 'bb', 'cc', 'ddd']", "{prog}");
        }
    }
    #[test]
    fn to_map_and_downstream() {
        let p = tb("List<String> xs = new ArrayList<>(); Map<String, Integer> m = xs.stream().collect(Collectors.toMap(x -> x, String::length)); Map<Integer, Integer> t = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.summingInt(String::length))); Map<Integer, List<String>> u = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.mapping(String::toUpperCase, Collectors.toList())));");
        assert!(!p.contains("未対応"), "{p}");
        let p = p.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("
");
        let prog = format!("{}
print(m, t, u)
", p.replacen("xs = []", "xs = [\"bb\", \"a\", \"cc\"]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "{'bb': 2, 'a': 1, 'cc': 2} {2: 4, 1: 1} {2: ['BB', 'CC'], 1: ['A']}", "{prog}");
        }
    }
    #[test]
    fn grouping_downstream_more() {
        let p = tb("List<String> xs = new ArrayList<>(); Map<Integer, String> j = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.joining(\"-\"))); Map<Integer, Double> a = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.averagingInt(String::length))); Map<Integer, Integer> r = xs.stream().collect(Collectors.groupingBy(String::length, Collectors.reducing(0, String::length, (x, y) -> x + y)));");
        assert!(!p.contains("未対応"), "{p}");
        let p = p.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("
");
        let prog = format!("{}
print(j, a, r)
", p.replacen("xs = []", "xs = [\"bb\", \"a\", \"cc\"]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "{2: 'bb-cc', 1: 'a'} {2: 2.0, 1: 1.0} {2: 4, 1: 1}", "{prog}");
        }
    }
    #[test]
    fn partitioning_and_averaging() {
        let p = tb("List<String> xs = new ArrayList<>(); Map<Boolean, List<String>> a = xs.stream().collect(Collectors.partitioningBy(x -> x.length() > 1)); Map<Boolean, Long> b = xs.stream().collect(Collectors.partitioningBy(x -> x.length() > 1, Collectors.counting())); double c = xs.stream().collect(Collectors.averagingInt(String::length));");
        assert!(!p.contains("未対応"), "{p}");
        let p = p.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("
");
        let prog = format!("{}
print(a, b, c)
", p.replacen("xs = []", "xs = [\"bb\", \"a\", \"cc\"]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "{False: ['a'], True: ['bb', 'cc']} {False: 1, True: 2} 1.6666666666666667", "{prog}");
        }
    }
    #[test]
    fn to_map_merge() {
        let p = tb("List<String> xs = new ArrayList<>(); Map<Integer, Integer> m = xs.stream().collect(Collectors.toMap(String::length, String::length, Integer::sum));");
        assert!(!p.contains("未対応"), "{p}");
        let p = p.lines().map(|l| l.trim_start()).collect::<Vec<_>>().join("
");
        let prog = format!("{}
print(m)
", p.replacen("xs = []", "xs = [\"bb\", \"a\", \"cc\"]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "{2: 4, 1: 1}", "{prog}");
        }
    }
    #[test]
    fn anonymous_class_single_method() {
        let p = tb("List<Integer> xs = new ArrayList<>(); Comparator<Integer> c = new Comparator<Integer>() { @Override public int compare(Integer a, Integer b) { int d = b - a; return d; } }; List<Integer> s = xs.stream().sorted(c).collect(Collectors.toList());");
        assert!(!p.contains("未対応"), "{p}");
        assert!(p.contains("def _anon"), "{p}");
        // hoisted の def は本体と同じ字下げで出る。字下げを揃えて実際に動かす
        let prog = format!("{}
print(s)
", p.lines().map(|l| l.strip_prefix("    ").unwrap_or(l)).collect::<Vec<_>>().join("
").replacen("xs = []", "xs = [1, 3, 2]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "[3, 2, 1]", "{prog}");
        }
    }
    #[test]
    fn anonymous_class_with_state() {
        let p = tb("List<Integer> xs = new ArrayList<>(); Comparator<Integer> c = new Comparator<Integer>() { int calls = 0; public int compare(Integer a, Integer b) { calls++; return b - a; } }; List<Integer> s = xs.stream().sorted(c).collect(Collectors.toList()); int n = c.calls;");
        assert!(p.contains("class _Anon"), "{p}");
        let prog = format!("{}
print(s)
", p.lines().map(|l| l.strip_prefix("    ").unwrap_or(l)).collect::<Vec<_>>().join("
").replacen("xs = []", "xs = [1, 3, 2]", 1));
        if let Ok(o) = std::process::Command::new("python").args(["-c", &prog]).output() {
            assert!(o.status.success(), "{prog}
{}", String::from_utf8_lossy(&o.stderr));
            assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "[3, 2, 1]", "{prog}");
        }
    }
    #[test]
    fn if_for_return() {
        let p = tb("int total = 0; for (int i = 0; i < n; i++) { total += i; } if (total > 10 && flag) { return \"big\"; } else { return \"small\" + total; }");
        assert!(p.contains("total = 0"), "{p}");
        assert!(p.contains("for i in range(n):"), "{p}");
        assert!(p.contains("total += i"), "{p}");
        assert!(p.contains("if total > 10 and flag:"), "{p}");
        assert!(p.contains("return \"small\" + str(total)"), "{p}");
    }
    #[test]
    fn collections_and_try() {
        let p = tb("List<String> xs = new ArrayList<>(); xs.add(\"a\"); for (String s : xs) { System.out.println(s.toUpperCase()); } try { x = Integer.parseInt(v); } catch (NumberFormatException e) { return null; }");
        assert!(p.contains("xs = []"), "{p}");
        assert!(p.contains("xs.append(\"a\")"), "{p}");
        assert!(p.contains("for s in xs:"), "{p}");
        assert!(p.contains("print(s.upper())"), "{p}");
        assert!(p.contains("except ValueError as e:"), "{p}");
    }
    #[test]
    fn throw_maps_to_http() {
        let p = tb("throw new NotFoundException(\"no\");");
        assert!(p.contains("raise HTTPException(status_code=404, detail=\"no\")"), "{p}");
    }
    #[test]
    fn unsupported_is_loud() {
        let p = tb("switch (x) { case 1: y = 1; case 2: y = 2; break; }");
        assert!(p.contains("TODO(Java_to)"), "{p}");
    }
    #[test]
    fn switch_classic_and_arrow() {
        let p = tb("switch (op) { case 1: case 2: r = 1; break; case 3: return \"x\"; default: r = 0; }");
        assert!(p.contains("_sw1 = op"), "{p}");
        assert!(p.contains("if _sw1 in (1, 2):"), "{p}");
        assert!(p.contains("elif _sw1 == 3:"), "{p}");
        assert!(p.contains("else:"), "{p}");
        let q = tb("switch (op) { case 1, 2 -> r = 1; case 3 -> { r = 2; } default -> r = 0; }");
        assert!(q.contains("if _sw1 in (1, 2):"), "{q}");
        assert!(q.contains("elif _sw1 == 3:"), "{q}");
    }
    #[test]
    fn switch_expression() {
        let p = tb("String s = switch (n) { case 1, 2 -> \"low\"; case 3 -> \"mid\"; default -> \"hi\"; };");
        assert!(p.contains("s = \"low\""), "{p}");
        assert!(p.contains("s = \"hi\""), "{p}");
        let q = tb("return switch (n) { case 1 -> \"a\"; default -> \"b\"; };");
        assert!(q.contains("return \"a\""), "{q}");
        assert!(q.contains("else:"), "{q}");
    }
    #[test]
    fn integer_division_and_cast() {
        let p = tb("int a = 7; int b = a / 2; double d = (double) a / 2;");
        assert!(p.contains("b = a // 2"), "{p}");
        assert!(p.contains("d = float(a) / 2"), "{p}");
    }
}
