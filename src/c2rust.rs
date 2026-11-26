//! `pl2pl c2rust`: 単純な C を Rust へ書き換える。行ごとの変換で、式は型つきの小さな構文解析器で読む。
//! わからない行は黙って捨てず `// TODO(c2rust): 元の行` として残し、レポートにも載せる。
//! 対応: 整数・小数の変数、四則と比較(整数と小数を混ぜた式は C と同じく小数へ暗黙変換)、if / while / for、
//! 関数、printf、固定長の配列(1次元・2次元)、配列を関数に渡す(`int a[]` → `&mut [i32]`)、
//! 単純な構造体(`struct P { int x; double y; };`、変数・初期化・配列・`.`/`->`)、
//! 出力用のポインタ引数(`void f(int *x)` → `x: &mut i32`、呼び出し `f(&v)` → `f(&mut v)`)、break / continue。
//! ポインタ変数(`int *p` → 生ポインタ `*mut i32`、読み書きは `unsafe`)、malloc/calloc/free、
//! 文字列(`char s[N]`・`char *`、strlen/strcpy/strcat/strcmp/atoi、printf の %s・%c・幅指定)、
//! `#define 名前 値`、ファイル入出力(fopen/fprintf/fputs/fgets/fclose)、`int a, b;` の複数宣言、
//! sqrt などの数学関数、`f(a, a[0])` の借用エラー回避(引数を先に一時変数へ)。
//! 未対応(TODO のまま): 関数形式のマクロ・ポインタのポインタ・scanf など。
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
    Char,
    File,
    Raw(Box<T>),
    Null,
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
        T::Char => "u8".into(),
        T::File => "CFile".into(),
        T::Raw(e) => format!("*mut {}", rs(e)),
        T::Null => "*mut u8".into(),
    }
}

fn zero(t: &T) -> String {
    match t {
        T::I32 | T::I64 | T::Char => "0".into(),
        T::Raw(_) | T::Null => "std::ptr::null_mut()".into(),
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
        "char" | "unsigned char" => Some(T::Char),
        "FILE" => Some(T::File),
        _ => None,
    }
}

fn is_float(t: &T) -> bool {
    matches!(t, T::F32 | T::F64)
}
fn is_num(t: &T) -> bool {
    matches!(t, T::I32 | T::I64 | T::F32 | T::F64 | T::Bool | T::Char)
}
fn is_int(t: &T) -> bool {
    matches!(t, T::I32 | T::I64 | T::Char)
}

thread_local! {
    static VARS: RefCell<HashMap<String, T>> = Default::default();
    static STRUCTS: RefCell<HashMap<String, Vec<(String, T)>>> = Default::default();
    static FUNCS: RefCell<HashMap<String, (Vec<T>, Option<T>)>> = Default::default();
    static ALIAS: RefCell<HashMap<String, String>> = Default::default();
    static DEFINES: RefCell<HashMap<String, String>> = Default::default();
    static UNSAFE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn mark() {
    UNSAFE.with(|u| u.set(true));
}
fn take_unsafe() -> bool {
    UNSAFE.with(|u| u.replace(false))
}
fn us(e: String) -> String {
    if take_unsafe() { format!("unsafe {{ {e} }}") } else { e }
}

/// C の標準関数(文字列・メモリ・ファイル)を、出力の末尾に付ける小さな関数へ対応づける
fn builtins() {
    let c = || T::Raw(Box::new(T::Char));
    let f = || T::Raw(Box::new(T::File));
    let list: Vec<(&str, &str, Vec<T>, Option<T>)> = vec![
        ("strlen", "c_strlen", vec![c()], Some(T::I32)),
        ("strcpy", "c_strcpy", vec![c(), c()], Some(c())),
        ("strcat", "c_strcat", vec![c(), c()], Some(c())),
        ("strcmp", "c_strcmp", vec![c(), c()], Some(T::I32)),
        ("atoi", "c_atoi", vec![c()], Some(T::I32)),
        ("malloc", "c_malloc", vec![T::I32], Some(c())),
        ("calloc", "c_calloc", vec![T::I32, T::I32], Some(c())),
        ("free", "c_free", vec![c()], None),
        ("fopen", "c_fopen", vec![c(), c()], Some(f())),
        ("fclose", "c_fclose", vec![f()], Some(T::I32)),
        ("fgets", "c_fgets", vec![c(), T::I32, f()], Some(c())),
        ("fputs", "c_fputs_p", vec![c(), f()], Some(T::I32)),
    ];
    for (n, a, ps, r) in list {
        FUNCS.with(|m| m.borrow_mut().insert(n.into(), (ps, r)));
        ALIAS.with(|m| m.borrow_mut().insert(n.into(), a.into()));
    }
}

/// 出力の末尾に付ける実行時の小さな関数(C の標準ライブラリの代わり)
const RUNTIME: &str = r#"
// ---- pl2pl c2rust の実行時関数(C の標準ライブラリの代わり) ----
fn c_strlen(p: *mut u8) -> i32 { let mut n = 0; unsafe { while *p.add(n) != 0 { n += 1; } } n as i32 }
fn c_strcpy(d: *mut u8, s: *mut u8) -> *mut u8 { unsafe { let mut i = 0; loop { let c = *s.add(i); *d.add(i) = c; if c == 0 { break; } i += 1; } } d }
fn c_strcat(d: *mut u8, s: *mut u8) -> *mut u8 { let n = c_strlen(d) as usize; c_strcpy(d.wrapping_add(n), s); d }
fn c_strcmp(a: *mut u8, b: *mut u8) -> i32 { unsafe { let mut i = 0; loop { let (x, y) = (*a.add(i), *b.add(i)); if x != y || x == 0 { return x as i32 - y as i32; } i += 1; } } }
fn c_atoi(p: *mut u8) -> i32 { let s = c_str(p).trim_start(); let mut n: i32 = 0; let mut neg = false; for (k, c) in s.chars().enumerate() { if k == 0 && (c == '-' || c == '+') { neg = c == '-'; continue; } match c.to_digit(10) { Some(d) => n = n * 10 + d as i32, None => break } } if neg { -n } else { n } }
fn c_altz(v: u32, k: char, w: usize) -> String { let s = c_alt(v, k); if v == 0 || s.len() >= w { return s; } let pl = if k == 'o' { 1 } else { 2 }; format!("{}{}{}", &s[..pl], "0".repeat(w - s.len()), &s[pl..]) }
fn c_alt(v: u32, k: char) -> String { if v == 0 { return "0".to_string(); } match k { 'x' => format!("0x{:x}", v), 'X' => format!("0X{:X}", v), _ => format!("0{:o}", v) } }
thread_local! { static C_SCAN_Q: std::cell::RefCell<std::collections::VecDeque<String>> = std::cell::RefCell::new(std::collections::VecDeque::new()); }
fn c_scan_tok() -> String { loop { if let Some(t) = C_SCAN_Q.with(|q| q.borrow_mut().pop_front()) { return t; } let mut l = String::new(); if std::io::stdin().read_line(&mut l).unwrap_or(0) == 0 { return String::new(); } C_SCAN_Q.with(|q| q.borrow_mut().extend(l.split_whitespace().map(String::from))); } }
fn c_scan_i32() -> i32 { c_scan_tok().parse().unwrap_or(0) }
fn c_scan_i64() -> i64 { c_scan_tok().parse().unwrap_or(0) }
fn c_scan_f64() -> f64 { c_scan_tok().parse().unwrap_or(0.0) }
fn c_scan_f32() -> f32 { c_scan_tok().parse().unwrap_or(0.0) }
fn c_str(p: *mut u8) -> &'static str { unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(p, c_strlen(p) as usize)) } }
extern "C" { fn malloc(n: usize) -> *mut u8; fn calloc(n: usize, m: usize) -> *mut u8; fn free(p: *mut u8); }
fn c_malloc(n: i32) -> *mut u8 { unsafe { malloc(n as usize) } }
fn c_calloc(n: i32, m: i32) -> *mut u8 { unsafe { calloc(n as usize, m as usize) } }
fn c_free(p: *mut u8) { unsafe { free(p) } }
struct CFile { r: Option<std::io::BufReader<std::fs::File>>, w: Option<std::fs::File> }
fn c_fopen(path: *mut u8, mode: *mut u8) -> *mut CFile {
    let (p, m) = (c_str(path), c_str(mode));
    let f = if m.starts_with('r') { std::fs::File::open(p).map(|f| CFile { r: Some(std::io::BufReader::new(f)), w: None }) }
        else if m.starts_with('a') { std::fs::OpenOptions::new().append(true).create(true).open(p).map(|f| CFile { r: None, w: Some(f) }) }
        else { std::fs::File::create(p).map(|f| CFile { r: None, w: Some(f) }) };
    match f { Ok(c) => Box::into_raw(Box::new(c)), Err(_) => std::ptr::null_mut() }
}
fn c_fclose(f: *mut CFile) -> i32 { if !f.is_null() { unsafe { drop(Box::from_raw(f)); } } 0 }
fn c_fputs(f: *mut CFile, s: &str) -> i32 { use std::io::Write; match unsafe { (*f).w.as_mut() } { Some(w) => { let _ = w.write_all(s.as_bytes()); 0 } None => -1 } }
fn c_fputs_p(s: *mut u8, f: *mut CFile) -> i32 { c_fputs(f, c_str(s)) }
fn c_fgets(buf: *mut u8, n: i32, f: *mut CFile) -> *mut u8 {
    use std::io::BufRead;
    let r = match unsafe { (*f).r.as_mut() } { Some(r) => r, None => return std::ptr::null_mut() };
    let (mut k, max) = (0usize, (n - 1).max(0) as usize);
    while k < max {
        let b = match r.fill_buf() { Ok(b) if !b.is_empty() => b[0], _ => break };
        r.consume(1);
        unsafe { *buf.add(k) = b; }
        k += 1;
        if b == b'\n' { break; }
    }
    if k == 0 { return std::ptr::null_mut(); }
    unsafe { *buf.add(k) = 0; }
    buf
}
"#;

/// C の文字列リテラル(引用符なし)の中身のバイト数(エスケープは1バイト)
fn unescape_len(s: &str) -> usize {
    let mut n = 0;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            it.next();
            n += 1;
        } else {
            n += c.len_utf8();
        }
    }
    n
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
    match (from, to) {
        (T::Null, T::Raw(_)) => return Some("std::ptr::null_mut()".into()),
        (T::Arr(a, _) | T::Slice(a), T::Raw(b)) if a == b => return Some(format!("{e}.as_mut_ptr()")),
        (T::Ptr(a), T::Raw(b)) if a == b => return Some(format!("({e} as *mut {})", rs(b))),
        (T::Raw(_), T::Raw(b)) => return Some(format!("({e} as *mut {})", rs(b))),
        (T::Raw(a), T::Ptr(b)) if a == b => {
            mark();
            return Some(format!("&mut *{e}"));
        }
        _ => {}
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
        (T::I32 | T::I64, T::Char) if is_lit_int(&e) => e,
        (a, b) if is_num(a) && is_num(b) && *b != T::Bool => format!("(({e}) as {})", rs(b)),
        _ => return None,
    })
}

fn to_bool(e: &str, t: &T) -> Option<String> {
    Some(match t {
        T::Bool => e.to_string(),
        T::F32 | T::F64 => format!("({e} != 0.0)"),
        T::I32 | T::I64 | T::Char => format!("({e} != 0)"),
        T::Raw(_) => format!("!({e}).is_null()"),
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
        if ch == '"' || ch == '\'' {
            let st = i;
            i += 1;
            while i < c.len() && c[i] != ch {
                if c[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            if i >= c.len() {
                return None;
            }
            i += 1;
            v.push(c[st..i].iter().collect());
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
        if "+-*/%<>!&()[].,?:".contains(ch) {
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
    let isp = |t: &T| matches!(t, T::Raw(_) | T::Null);
    if isp(&l.1) || isp(&r.1) {
        return match (op, &l.1, &r.1) {
            ("+" | "-", T::Raw(_), x) if is_int(x) => Some((format!("{}.wrapping_offset({}(({}) as isize))", l.0, if op == "-" { "-" } else { "" }, r.0), l.1.clone())),
            ("+", x, T::Raw(_)) if is_int(x) => Some((format!("{}.wrapping_offset((({}) as isize))", r.0, l.0), r.1.clone())),
            ("-", T::Raw(_), T::Raw(_)) => {
                mark();
                Some((format!("({}.offset_from({}) as i32)", l.0, r.0), T::I32))
            }
            ("==" | "!=", _, T::Null) => Some((format!("{}({}).is_null()", if op == "!=" { "!" } else { "" }, l.0), T::Bool)),
            ("==" | "!=", T::Null, _) => Some((format!("{}({}).is_null()", if op == "!=" { "!" } else { "" }, r.0), T::Bool)),
            ("==" | "!=" | "<" | ">" | "<=" | ">=", T::Raw(_), T::Raw(_)) => Some((format!("{} {op} {}", l.0, r.0), T::Bool)),
            _ => None,
        };
    }
    if !is_num(&l.1) || !is_num(&r.1) {
        return None;
    }
    let cmp = matches!(op, "==" | "!=" | "<" | ">" | "<=" | ">=");
    if l.1 == T::Char && r.1 == T::Char && cmp {
        return Some((format!("{} {op} {}", l.0, r.0), T::Bool));
    }
    let target = if is_float(&l.1) || is_float(&r.1) {
        if l.1 == T::F64 || r.1 == T::F64 || !(is_float(&l.1) && is_float(&r.1)) { T::F64 } else { T::F32 }
    } else if l.1 == T::I64 || r.1 == T::I64 {
        T::I64
    } else {
        T::I32
    };
    let (a, b) = (coerce(&l.0, &l.1, &target)?, coerce(&r.0, &r.1, &target)?);
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
        // 三項演算子 `c ? a : b`(数値どうし、または同じ型どうし)
        if minp == 0 && self.peek() == Some("?") {
            self.i += 1;
            let a = self.expr(0)?;
            if !self.eat(":") {
                return None;
            }
            let b = self.expr(0)?;
            let c = to_bool(&l.0, &l.1)?;
            let t = if a.1 == b.1 {
                a.1.clone()
            } else if is_num(&a.1) && is_num(&b.1) {
                if is_float(&a.1) || is_float(&b.1) {
                    T::F64
                } else if a.1 == T::I64 || b.1 == T::I64 {
                    T::I64
                } else {
                    T::I32
                }
            } else {
                return None;
            };
            let (x, y) = (coerce(&a.0, &a.1, &t)?, coerce(&b.0, &b.1, &t)?);
            return Some((format!("(if {c} {{ {x} }} else {{ {y} }})"), t));
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
                T::Raw(inner) => {
                    mark();
                    Some((format!("(*{e})"), *inner))
                }
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
                let ptr = words.last().map(|w| w == "*").unwrap_or(false);
                let ct = if ptr { base_ty(&words[..words.len() - 1].join(" ")).map(|b| T::Raw(Box::new(b))) } else { base_ty(&words.join(" ")).filter(is_num) };
                if let Some(ct) = ct {
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
                if let T::Raw(el) = t {
                    mark();
                    e = format!("(*{e}.wrapping_offset(({ix}) as isize))");
                    t = *el;
                    continue;
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
                    (T::Raw(b), true) => match &**b {
                        T::St(s) => {
                            mark();
                            e = format!("(*{e})");
                            s.clone()
                        }
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
        if let Some(body) = tk.strip_prefix('"').and_then(|x| x.strip_suffix('"')) {
            return Some((format!("(b\"{body}\\0\".as_ptr() as *mut u8)"), T::Raw(Box::new(T::Char))));
        }
        if tk.starts_with('\'') {
            return Some((format!("b{tk}"), T::Char));
        }
        if tk == "NULL" {
            return Some(("std::ptr::null_mut()".into(), T::Null));
        }
        if tk == "sizeof" {
            if !self.eat("(") {
                return None;
            }
            let save = self.i;
            let mut words = vec![];
            while let Some(w) = self.peek() {
                if w == ")" {
                    break;
                }
                words.push(w.to_string());
                self.i += 1;
            }
            let ptr = words.last().map(|w| w == "*").unwrap_or(false);
            let bt = if ptr { base_ty(&words[..words.len() - 1].join(" ")).map(|b| T::Raw(Box::new(b))) } else { base_ty(&words.join(" ")) };
            if let Some(bt) = bt {
                if self.eat(")") {
                    return Some((format!("(std::mem::size_of::<{}>() as i32)", rs(&bt)), T::I32));
                }
            }
            self.i = save;
            let (e, _) = self.expr(0)?;
            return self.eat(")").then(|| (format!("(std::mem::size_of_val(&{e}) as i32)"), T::I32));
        }
        let math1 = ["sqrt", "fabs", "floor", "ceil", "sin", "cos", "tan", "exp", "log", "log10", "round"];
        if (math1.contains(&tk.as_str()) || tk == "pow" || tk == "abs") && self.peek() == Some("(") && !FUNCS.with(|m| m.borrow().contains_key(&tk)) {
            self.i += 1;
            let a = self.expr(0)?;
            if tk == "pow" {
                if !self.eat(",") {
                    return None;
                }
                let b = self.expr(0)?;
                if !self.eat(")") {
                    return None;
                }
                return Some((format!("f64::powf({}, {})", coerce(&a.0, &a.1, &T::F64)?, coerce(&b.0, &b.1, &T::F64)?), T::F64));
            }
            if !self.eat(")") {
                return None;
            }
            if tk == "abs" {
                return Some((format!("i32::abs({})", coerce(&a.0, &a.1, &T::I32)?), T::I32));
            }
            let m = if tk == "fabs" { "abs" } else { tk.as_str() };
            return Some((format!("f64::{m}({})", coerce(&a.0, &a.1, &T::F64)?), T::F64));
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
            let fname = ALIAS.with(|m| m.borrow().get(&tk).cloned()).unwrap_or(tk.clone());
            // `f(&mut a, a[0])` は Rust の借用の規則で通らないので、ほかの引数を先に一時変数へ移す
            if args.iter().any(|a| a.starts_with("&mut ")) && args.len() > 1 {
                let mut pre = String::new();
                let mut fin = vec![];
                for (k, a) in args.iter().enumerate() {
                    if a.starts_with("&mut ") || is_lit_int(a) || is_lit_float(a) {
                        fin.push(a.clone());
                    } else {
                        pre.push_str(&format!("let _a{k} = {a}; "));
                        fin.push(format!("_a{k}"));
                    }
                }
                if !pre.is_empty() {
                    return Some((format!("{{ {pre}{fname}({}) }}", fin.join(", ")), ret.unwrap_or(T::I32)));
                }
            }
            return Some((format!("{fname}({})", args.join(", ")), ret.unwrap_or(T::I32)));
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

/// printf の書式を Rust の `{}` へ(%d %f %s %c %u、幅・左寄せ・精度)。
fn fmt(args: &str) -> Option<String> {
    let parts = split_args(args);
    let f = parts.first()?.trim();
    if !(f.starts_with('"') && f.ends_with('"') && f.len() >= 2) {
        return None;
    }
    let spec = Regex::new(r"%([-#]*)(\d*)(\.\d+)?(?:ll|l|h)?([dfscuixXo])|%%").unwrap();
    let body = &f[1..f.len() - 1];
    let esc = |x: &str| x.replace('{', "{{").replace('}', "}}");
    let (mut out, mut last, mut k, mut vals) = (String::new(), 0, 0, vec![]);
    for c in spec.captures_iter(body) {
        let m = c.get(0).unwrap();
        out.push_str(&esc(&body[last..m.start()]));
        last = m.end();
        if m.as_str() == "%%" {
            out.push('%');
            continue;
        }
        let a = parts.get(k + 1)?;
        k += 1;
        let (e, t) = ex(a.trim())?;
        let letter = &c[4];
        vals.push(match letter {
            "s" => match &t {
                T::Arr(el, _) | T::Slice(el) if **el == T::Char => format!("c_str({e}.as_mut_ptr())"),
                T::Raw(el) if **el == T::Char => format!("c_str({e})"),
                _ => return None,
            },
            "c" => format!("(({e}) as u8 as char)"),
            "x" | "X" | "o" if c[1].contains('#') && c[2].starts_with('0') && c[2].len() > 1 && !c[1].contains('-') => format!("c_altz(({e}) as u32, '{letter}', {})", &c[2][1..]),
            "x" | "X" | "o" if c[1].contains('#') => format!("c_alt(({e}) as u32, '{letter}')"),
            _ if t == T::Bool => format!("(({e}) as i32)"),
            _ => e,
        });
        let alt = c[1].contains('#') && matches!(letter, "x" | "X" | "o");
        let (minus, w, pr) = (c[1].contains('-'), &c[2], c.get(3).map(|x| x.as_str()).unwrap_or(""));
        let align = if minus { "<" } else if !w.is_empty() && (letter == "s" || letter == "c" || alt) { ">" } else { "" };
        let ty = if alt {
            ""
        } else {
            match letter {
                "x" => "x",
                "X" => "X",
                "o" => "o",
                _ => "",
            }
        };
        let zalt = alt && w.starts_with('0') && w.len() > 1 && !minus;
        let sp = if zalt { String::new() } else { format!("{align}{w}{pr}{ty}") };
        out.push_str(&if sp.is_empty() { "{}".to_string() } else { format!("{{:{sp}}}") });
    }
    out.push_str(&esc(&body[last..]));
    if k != parts.len() - 1 {
        return None;
    }
    Some(if vals.is_empty() { format!("\"{out}\"") } else { format!("\"{out}\", {}", vals.join(", ")) })
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
        T::Arr(el, n) if **el == T::Char && s.starts_with('"') => {
            let body = s.strip_prefix('"')?.strip_suffix('"')?;
            let len = unescape_len(body);
            if len > *n {
                return None;
            }
            Some(format!("{{ let mut b = [0u8; {n}]; b[..{len}].copy_from_slice(b\"{body}\"); b }}"))
        }
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

/// `[2][3]`・`[N]`(#define)を寸法の並びへ(空は None)
fn dims(s: &str) -> Option<Vec<Option<usize>>> {
    Regex::new(r"\[\s*(\w*)\s*\]")
        .unwrap()
        .captures_iter(s)
        .map(|c| {
            let w = &c[1];
            if w.is_empty() {
                Some(None)
            } else if let Ok(n) = w.parse() {
                Some(Some(n))
            } else {
                DEFINES.with(|d| d.borrow().get(w).and_then(|v| v.trim().parse().ok())).map(Some)
            }
        })
        .collect()
}

fn wrap(base: T, ds: &[Option<usize>]) -> Option<T> {
    let mut t = base;
    for d in ds.iter().rev() {
        t = T::Arr(Box::new(t), (*d)?);
    }
    Some(t)
}

const TYPES: &str = r"struct\s+\w+|long\s+long|unsigned\s+int|unsigned\s+char|int|long|double|float|short|unsigned|char|FILE";

/// 関数の引数 1 つ: (名前, 型)
fn param(p: &str) -> Option<(String, T)> {
    let re = Regex::new(&format!(r"^({TYPES})(?:\s+|\s*(\*)\s*)(\w+)((?:\s*\[\s*\w*\s*\])*)$")).unwrap();
    let c = re.captures(p.trim())?;
    let b = base_ty(&c[1])?;
    let ds = dims(&c[4])?;
    let t = if c.get(2).is_some() {
        if !ds.is_empty() {
            return None;
        }
        if matches!(b, T::Char | T::File) { T::Raw(Box::new(b)) } else { T::Ptr(Box::new(b)) }
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

/// 宣言 1 つ(`int x = 1;`・`int *p = &v;`・`char s[] = "ab";`・`struct P ps[3];` など)
fn decl_one(l: &str) -> Option<String> {
    let re = Regex::new(&format!(r"^({TYPES})(?:\s+|\s*(\*)\s*)(\w+)((?:\s*\[\s*\w*\s*\])*)\s*(?:=\s*(.+))?;$")).unwrap();
    let c = re.captures(l)?;
    let mut b = base_ty(&c[1])?;
    let mut ds = dims(&c[4])?;
    let ix = c.get(5).map(|m| m.as_str().trim().to_string());
    if c.get(2).is_some() {
        if !ds.is_empty() {
            return None;
        }
        b = T::Raw(Box::new(b));
    } else if b == T::File {
        return None;
    }
    if ds.first() == Some(&None) {
        let it = ix.as_ref()?;
        ds[0] = Some(match it.strip_prefix('"').and_then(|x| x.strip_suffix('"')) {
            Some(q) => unescape_len(q) + 1,
            None => split_args(it.strip_prefix('{')?.strip_suffix('}')?).iter().filter(|x| !x.trim().is_empty()).count(),
        });
    }
    let t = wrap(b, &ds)?;
    take_unsafe();
    let v = match &ix {
        Some(x) => init(x, &t)?,
        None => zero(&t),
    };
    let v = us(v);
    set_var(&c[3], t.clone());
    Some(format!("let mut {}: {} = {v};", &c[3], rs(&t)))
}

/// `//` から後ろを捨てる(文字列の中の `//` は残す)
fn strip_comment(l: &str) -> &str {
    let b = l.as_bytes();
    let mut q: u8 = 0;
    let mut k = 0;
    while k < b.len() {
        let c = b[k];
        if q != 0 {
            if c == b'\\' {
                k += 1;
            } else if c == q {
                q = 0;
            }
        } else if c == b'"' || c == b'\'' {
            q = c;
        } else if c == b'/' && k + 1 < b.len() && b[k + 1] == b'/' {
            return &l[..k];
        }
        k += 1;
    }
    l
}

/// `/* ... */` を取り除く(文字列の中は残す。改行は保って行番号を変えない)。
fn strip_block_comments(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let (mut o, mut i, mut q) = (String::new(), 0, false);
    while i < b.len() {
        let c = b[i];
        if q {
            o.push(c);
            if c == '\\' && i + 1 < b.len() {
                o.push(b[i + 1]);
                i += 1;
            } else if c == '"' {
                q = false;
            }
        } else if c == '"' {
            q = true;
            o.push(c);
        } else if c == '/' && i + 1 < b.len() && b[i + 1] == '*' {
            i += 2;
            while i + 1 < b.len() && !(b[i] == '*' && b[i + 1] == '/') {
                if b[i] == '\n' {
                    o.push('\n');
                }
                i += 1;
            }
            i += 1;
            o.push(' ');
        } else {
            o.push(c);
        }
        i += 1;
    }
    o
}

/// 引数つきマクロ `#define SQ(x) ((x)*(x))` を、呼び出し箇所へ文字列として展開する
/// (引数は括弧で包む。定義の行は空行にして行番号を保つ。マクロの中で別のマクロは呼べるが、自己再帰は展開しない)。
fn expand_fn_macros(src: &str) -> String {
    let def = Regex::new(r"^\s*#define\s+(\w+)\(([^)]*)\)\s+(.+)$").unwrap();
    let mut macros: Vec<(String, Vec<String>, String)> = vec![];
    let mut lines: Vec<String> = vec![];
    for l in src.lines() {
        if let Some(c) = def.captures(l) {
            let ps = c[2].split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect();
            macros.push((c[1].to_string(), ps, c[3].trim().to_string()));
            lines.push(String::new());
        } else {
            lines.push(l.to_string());
        }
    }
    if macros.is_empty() {
        return src.to_string();
    }
    let expand = |text: &str| -> String {
        let mut s = text.to_string();
        for _ in 0..6 {
            let mut changed = false;
            for (name, ps, body) in &macros {
                let mut out = String::new();
                let mut rest = s.as_str();
                while let Some(pos) = rest.find(name.as_str()) {
                    let before_ok = rest[..pos].chars().last().map_or(true, |c| !(c.is_alphanumeric() || c == '_'));
                    let after = &rest[pos + name.len()..];
                    if before_ok && after.starts_with('(') {
                        // 対応する閉じ括弧を探す
                        let (mut d, mut end) = (0i32, None);
                        for (i, c) in after.char_indices() {
                            if c == '(' {
                                d += 1;
                            } else if c == ')' {
                                d -= 1;
                                if d == 0 {
                                    end = Some(i);
                                    break;
                                }
                            }
                        }
                        if let Some(e) = end {
                            let args = split_args(&after[1..e]);
                            if args.len() == ps.len() {
                                let mut b = body.clone();
                                for (p, a) in ps.iter().zip(&args) {
                                    let re = Regex::new(&format!(r"\b{}\b", regex::escape(p))).unwrap();
                                    b = re.replace_all(&b, regex::NoExpand(&format!("({})", a.trim()))).to_string();
                                }
                                out.push_str(&rest[..pos]);
                                out.push_str(&format!("({b})"));
                                rest = &after[e + 1..];
                                changed = true;
                                continue;
                            }
                        }
                    }
                    out.push_str(&rest[..pos + name.len()]);
                    rest = after;
                }
                out.push_str(rest);
                s = out;
            }
            if !changed {
                break;
            }
        }
        s
    };
    let mut res = lines.iter().map(|l| expand(l)).collect::<Vec<_>>().join("\n");
    if src.ends_with('\n') {
        res.push('\n');
    }
    res
}

/// 1行に収まった `if (x) { a; b; }`・`do { i++; } while (c);`・`int f(int v) { return v; }` を、複数行に開く
/// (本体は1行ずつ変換するため)。初期化子の `= { ... }` や struct・enum には触らない。
fn split_one_liners(src: &str) -> String {
    fn open_close(l: &str) -> Option<(usize, usize)> {
        let (mut q, mut esc, mut d, mut first) = (None::<char>, false, 0i32, None);
        for (i, ch) in l.char_indices() {
            if let Some(qc) = q {
                if esc { esc = false } else if ch == '\\' { esc = true } else if ch == qc { q = None }
                continue;
            }
            match ch {
                '"' | '\'' => q = Some(ch),
                '{' => {
                    if d == 0 { first = Some(i); }
                    d += 1;
                }
                '}' => {
                    d -= 1;
                    if d == 0 { return first.map(|f| (f, i)); }
                }
                _ => {}
            }
        }
        None
    }
    fn stmts(inner: &str) -> Vec<String> {
        let (mut v, mut cur, mut d, mut q, mut esc) = (vec![], String::new(), 0i32, None::<char>, false);
        for ch in inner.chars() {
            cur.push(ch);
            if let Some(qc) = q {
                if esc { esc = false } else if ch == '\\' { esc = true } else if ch == qc { q = None }
                continue;
            }
            match ch {
                '"' | '\'' => q = Some(ch),
                '(' | '[' | '{' => d += 1,
                ')' | ']' | '}' => d -= 1,
                ';' if d == 0 => { v.push(cur.trim().to_string()); cur.clear(); }
                _ => {}
            }
        }
        if !cur.trim().is_empty() { v.push(cur.trim().to_string()); }
        v
    }
    fn one(l: &str, out: &mut Vec<String>, depth: usize) {
        let t = l.trim();
        // `} else 文;`(波括弧なしの else)
        if let Some(r) = t.strip_prefix("} else ") {
            let r = r.trim();
            if !r.is_empty() && !r.starts_with('{') && !r.starts_with("if") && !r.starts_with("//") && depth < 6 {
                out.push("} else {".to_string());
                one(r, out, depth + 1);
                out.push("}".to_string());
                return;
            }
        }
        // 波括弧なしの `for (...) 文;`・`if (...) 文;`・`while (...) 文;` は波括弧つきに直す
        for kw in ["for", "while", "if", "} else if"] {
            if let Some(r) = t.strip_prefix(kw) {
                let r = r.trim_start();
                if r.starts_with('(') {
                    let (mut d, mut end) = (0i32, None);
                    for (i, ch) in r.char_indices() {
                        match ch {
                            '(' => d += 1,
                            ')' => {
                                d -= 1;
                                if d == 0 {
                                    end = Some(i);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    if let Some(e) = end {
                        let rest = r[e + 1..].trim();
                        if !rest.is_empty() && !rest.starts_with('{') && rest != ";" && !rest.contains(" else") && !rest.starts_with("//") && depth < 6 {
                            out.push(format!("{kw} {} {{", &r[..=e]));
                            one(rest, out, depth + 1);
                            out.push("}".to_string());
                            return;
                        }
                    }
                }
            }
        }
        if depth < 6 && !t.starts_with('#') && !t.contains("//") && !t.starts_with("struct") && !t.starts_with("enum") && !t.starts_with("typedef") {
            let (pre, t2) = if let Some(r) = t.strip_prefix('}') { ("} ", r.trim_start()) } else { ("", t) };
            if let Some((a, b)) = open_close(t2) {
                let core = t2[..a].trim_end();
                if core.ends_with(')') || core.ends_with("else") || core == "do" {
                    out.push(format!("{pre}{core} {{"));
                    for st in stmts(&t2[a + 1..b]) {
                        one(&st, out, depth + 1);
                    }
                    let tail = t2[b + 1..].trim();
                    one(&format!("}} {tail}").trim_end().to_string(), out, depth + 1);
                    return;
                }
            }
        }
        out.push(t.to_string());
    }
    let mut out: Vec<String> = vec![];
    for l in src.lines() {
        let t = l.trim();
        if t.starts_with("else") && out.last().map(|x| x == "}").unwrap_or(false) {
            out.pop();
            one(&format!("}} {t}"), &mut out, 0);
        } else {
            one(l, &mut out, 0);
        }
    }
    out.join("
")
}

pub fn convert(src: &str, name: &str, rep: &mut Vec<String>) -> Vec<(String, String)> {
    let src_owned = split_one_liners(&expand_fn_macros(&strip_block_comments(src)));
    let src: &str = &src_owned;
    let func = Regex::new(&format!(r"^({TYPES}|void)(?:\s+|\s*(\*)\s*)(\w+)\s*\(([^)]*)\)\s*\{{?\s*$")).unwrap();
    let multi = Regex::new(&format!(r"^({TYPES})\s+(.+);$")).unwrap();
    let define = Regex::new(r"^#define\s+(\w+)\s+(.+)$").unwrap();
    let sdef = Regex::new(r"^struct\s+(\w+)\s*\{$").unwrap();
    let forl = Regex::new(r"^for\s*\(\s*(?:int|long)?\s*(\w+)\s*=\s*([^;]+);\s*(\w+)\s*(<=|<)\s*([^;]+);\s*(\w+)(\+\+|\s*\+=\s*1)\s*\)\s*\{$").unwrap();
    let ifl = Regex::new(r"^(\}\s*else\s+)?if\s*\((.*)\)\s*\{$").unwrap();
    let whl = Regex::new(r"^while\s*\((.*)\)\s*\{$").unwrap();
    let call = Regex::new(r"^(printf|puts)\s*\((.*)\)\s*;$").unwrap();
    let fpr = Regex::new(r"^fprintf\s*\((.*)\)\s*;$").unwrap();
    let scanf_re = Regex::new(r"^scanf\s*\((.*)\)\s*;$").unwrap();
    let cout = Regex::new(r"^(?:std::)?cout\s*<<\s*(.*);$").unwrap();
    let incp = Regex::new(r"^(.+?)\s*(\+\+|--)\s*;$").unwrap();
    let incq = Regex::new(r"^(\+\+|--)\s*(.+);$").unwrap();
    let callst = Regex::new(r"^\w+\s*\(.*\)\s*;$").unwrap();
    VARS.with(|v| v.borrow_mut().clear());
    STRUCTS.with(|v| v.borrow_mut().clear());
    FUNCS.with(|v| v.borrow_mut().clear());
    ALIAS.with(|v| v.borrow_mut().clear());
    DEFINES.with(|v| v.borrow_mut().clear());
    take_unsafe();
    builtins();
    let ret_of = |c: &regex::Captures| -> Option<T> {
        if &c[1] == "void" || &c[3] == "main" {
            return None;
        }
        let b = base_ty(&c[1])?;
        Some(if c.get(2).is_some() { T::Raw(Box::new(b)) } else { b })
    };
    // 先に #define・構造体・関数の形を集める(定義の順番によらず呼べるように)
    {
        let mut cur: Option<(String, Vec<(String, T)>)> = None;
        let field = Regex::new(&format!(r"^({TYPES})\s+(\w+)((?:\s*\[\s*\w+\s*\])*)\s*;$")).unwrap();
        for raw in src.lines() {
            let l = strip_comment(raw).trim();
            if let Some(c) = define.captures(l) {
                DEFINES.with(|d| d.borrow_mut().insert(c[1].to_string(), c[2].trim().to_string()));
                continue;
            }
            if let Some(c) = sdef.captures(l) {
                cur = Some((c[1].to_string(), vec![]));
                STRUCTS.with(|m| m.borrow_mut().insert(c[1].to_string(), vec![]));
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
                let ps = c[4].trim();
                let pv: Option<Vec<T>> = if ps.is_empty() || ps == "void" { Some(vec![]) } else { ps.split(',').map(|p| param(p).map(|x| x.1)).collect() };
                if let Some(pv) = pv {
                    FUNCS.with(|m| m.borrow_mut().insert(c[3].to_string(), (pv, ret_of(&c))));
                    ALIAS.with(|m| m.borrow_mut().remove(&c[3]));
                }
            }
        }
    }
    let mut out = String::from("// pl2pl c2rust で変換。TODO(c2rust) の行は手で直してください。\n#![allow(unused_mut, unused_variables, unused_parens, unused_assignments, unused_unsafe, dead_code, non_snake_case, non_upper_case_globals)]\n\n");
    let mut in_main = false;
    let mut ret_ty: Option<T> = None;
    let mut stack: Vec<bool> = vec![];
    let mut dos: Vec<usize> = vec![];
    let dowhile = Regex::new(r"^\}\s*while\s*\((.*)\)\s*;$").unwrap();
    let mut in_struct = false;
    let mut todo = 0;
    for (i, raw) in src.lines().enumerate() {
        let l = strip_comment(raw).trim();
        if l.is_empty() || l.starts_with("#include") || l.starts_with("using namespace") {
            continue;
        }
        take_unsafe();
        let ind = "    ".repeat(stack.len());
        if let Some(c) = define.captures(l) {
            if let Some((e, t)) = ex(&c[2]).filter(|(_, t)| is_num(t)) {
                set_var(&c[1], t.clone());
                out.push_str(&format!("{ind}const {}: {} = {e};\n", &c[1], rs(&t)));
                continue;
            }
        }
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
            let ps = c[4].trim();
            let pv: Option<Vec<(String, T)>> = if ps.is_empty() || ps == "void" { Some(vec![]) } else { ps.split(',').map(param).collect() };
            if let Some(pv) = pv {
                let is_main = &c[3] == "main";
                let rt = ret_of(&c);
                let r = rt.as_ref().map(|t| format!(" -> {}", rs(t))).unwrap_or_default();
                let args: Vec<String> = pv
                    .iter()
                    .map(|(n, t)| {
                        set_var(n, t.clone());
                        if matches!(t, T::Slice(_) | T::Ptr(_)) { format!("{n}: {}", rs(t)) } else { format!("mut {n}: {}", rs(t)) }
                    })
                    .collect();
                out.push_str(&format!("{ind}fn {}({}){r} {{\n", &c[3], args.join(", ")));
                stack.push(is_main);
                in_main = is_main;
                ret_ty = rt;
                continue;
            }
        }
        if l == "do {" || l == "do{" {
            out.push_str(&format!("{ind}loop {{
"));
            stack.push(false);
            dos.push(stack.len());
            continue;
        }
        if let Some(c) = dowhile.captures(l) {
            if dos.last() == Some(&stack.len()) {
                if let Some(e) = cond(c[1].trim()).map(us) {
                    dos.pop();
                    out.push_str(&format!("{ind}if !({e}) {{ break; }}
"));
                    stack.pop();
                    out.push_str(&format!("{}}}
", "    ".repeat(stack.len())));
                    continue;
                }
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
        let mut wrapped = false;
        if let Some(c) = forl.captures(l) {
            if c[1] == c[3] && c[1] == c[6] {
                set_var(&c[1], T::I32);
                let a = ex_to(c[2].trim(), &T::I32).map(us);
                let b = ex_to(c[5].trim(), &T::I32).map(us);
                if let (Some(a), Some(b)) = (a, b) {
                    out.push_str(&format!("{ind}for {} in {a}..{}{b} {{\n", &c[1], if &c[4] == "<=" { "=" } else { "" }));
                    stack.push(false);
                    continue;
                }
            }
        }
        if let Some(c) = ifl.captures(l) {
            if let Some(e) = cond(c[2].trim()).map(us) {
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
            } else if let Some(e) = cond(cd).map(us) {
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
                if a.starts_with('"') && a.ends_with('"') {
                    Some(format!("println!({a});"))
                } else {
                    match ex(a) {
                        Some((e, T::Arr(el, _))) | Some((e, T::Slice(el))) if *el == T::Char => Some(format!("println!(\"{{}}\", c_str({e}.as_mut_ptr()));")),
                        Some((e, T::Raw(el))) if *el == T::Char => Some(format!("println!(\"{{}}\", c_str({e}));")),
                        _ => None,
                    }
                }
            } else {
                fmt(&c[2]).map(|f| format!("print!({f});"))
            };
        } else if let Some(c) = fpr.captures(l) {
            let parts = split_args(&c[1]);
            if parts.len() >= 2 {
                let fl = parts[0].trim();
                let rest = parts[1..].join(",");
                emit = fmt(&rest).and_then(|f| match fl {
                    "stdout" => Some(format!("print!({f});")),
                    "stderr" => Some(format!("eprint!({f});")),
                    _ => match ex(fl) {
                        Some((e, T::Raw(el))) if *el == T::File => Some(format!("c_fputs({e}, &format!({f}));")),
                        _ => None,
                    },
                });
            }
        } else if let Some(c) = scanf_re.captures(l) {
            // scanf("%d %lf", &a, &b); → 変数へ代入(`&名前` だけ。空白区切りの数値)
            let parts = split_args(&c[1]);
            let f = parts.first().map(|s| s.trim().to_string()).unwrap_or_default();
            let spec = Regex::new(r"%(?:ll|l|h)?([dfi])").unwrap();
            let specs: Vec<(String, bool)> = spec.captures_iter(&f).map(|m| (m[1].to_string(), m[0].starts_with("%l"))).collect();
            if f.starts_with('"') && specs.len() == parts.len() - 1 && !specs.is_empty() {
                let mut stmts = vec![];
                for ((letter, long), a) in specs.iter().zip(&parts[1..]) {
                    let a = a.trim();
                    let Some(name) = a.strip_prefix('&').map(str::trim).filter(|n| n.chars().all(|ch| ch.is_alphanumeric() || ch == '_')) else {
                        stmts.clear();
                        break;
                    };
                    let f_name = match (ex(name).map(|x| x.1), letter.as_str(), long) {
                        (Some(T::I32), "d" | "i", false) => "c_scan_i32",
                        (Some(T::I64), "d" | "i", _) => "c_scan_i64",
                        (Some(T::F64), "f", true) => "c_scan_f64",
                        (Some(T::F32), "f", false) => "c_scan_f32",
                        _ => {
                            stmts.clear();
                            break;
                        }
                    };
                    stmts.push(format!("{name} = {f_name}();"));
                }
                if !stmts.is_empty() {
                    emit = Some(stmts.join(" "));
                }
            }
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
        } else if (l == "break;" || l == "continue;") && !(l == "continue;" && !dos.is_empty()) {
            emit = Some(l.to_string());
        } else if let Some(c) = multi.captures(l) {
            let parts = split_args(&c[2]);
            wrapped = true;
            emit = if parts.len() > 1 {
                parts.iter().map(|p| decl_one(&format!("{} {};", &c[1], p.trim()))).collect::<Option<Vec<_>>>().map(|v| v.join(&format!("\n{ind}")))
            } else {
                decl_one(l)
            };
        } else if let Some(r) = l.strip_prefix("return").filter(|r| r.starts_with(' ') || r.starts_with(';')) {
            let r = r.trim().trim_end_matches(';').trim();
            wrapped = true;
            emit = if in_main || r.is_empty() {
                Some("return;".into())
            } else {
                ret_ty.as_ref().and_then(|t| init(r, t)).map(|e| format!("return {};", us(e)))
            };
        } else if let Some(a) = assign(l) {
            emit = Some(a);
        } else if let Some(c) = incp.captures(l).map(|c| (c[1].to_string(), c[2].to_string())).or_else(|| incq.captures(l).map(|c| (c[2].to_string(), c[1].to_string()))) {
            let plus = c.1 == "++";
            emit = ex(&c.0).and_then(|(e, t)| match t {
                T::Raw(_) => Some(format!("{e} = {e}.wrapping_offset({});", if plus { "1" } else { "-1" })),
                t if is_num(&t) => Some(format!("{e} {}= {};", if plus { "+" } else { "-" }, if is_float(&t) { "1.0" } else { "1" })),
                _ => None,
            });
        } else if callst.is_match(l) {
            emit = ex(l.trim_end_matches(';')).map(|(e, _)| format!("{e};"));
        }
        if let Some(e) = emit {
            let e = if wrapped {
                take_unsafe();
                e
            } else {
                us(e)
            };
            out.push_str(&format!("{ind}{e}\n"));
            continue;
        }
        todo += 1;
        out.push_str(&format!("{ind}// TODO(c2rust): {l}\n"));
        rep.push(format!("{name}:{}: 未対応 `{l}`", i + 1));
    }
    let rt_used = ["c_strlen(", "c_strcpy(", "c_strcat(", "c_strcmp(", "c_atoi(", "c_str(", "c_malloc(", "c_calloc(", "c_free(", "c_fopen(", "c_fclose(", "c_fputs", "c_fgets(", "CFile", "c_scan_", "c_alt(", "c_altz("];
    if rt_used.iter().any(|k| out.contains(k)) {
        out.push_str(RUNTIME);
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
    fn fn_macro_and_ternary() {
        let (t, r) = conv("#define SQ(x) ((x) * (x))\nint main(void) {\n  int n = 3;\n  int m = n > 2 ? SQ(n) : 0; /* c */\n  printf(\"%x\\n\", m);\n  return 0;\n}\n");
        assert!(t.contains("if n > 2"), "{t}");
        assert!(t.contains("{:x}"), "{t}");
        assert!(!t.contains("// TODO(c2rust):"), "{t}{r:?}");
        assert!(!t.contains("SQ("), "{t}");
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
    #[test]
    fn pointers_strings_files_macros() {
        let src = "#define N 4\nint f(int a[], int x) {\n  return a[0] + x;\n}\nint main() {\n  int a, b = 2, c[N];\n  int *p = &b;\n  *p = 5;\n  int *m = malloc(N * sizeof(int));\n  m[1] = 3;\n  free(m);\n  char s[8] = \"hi\";\n  strcat(s, \"!\");\n  printf(\"%s %c %5d\\n\", s, s[0], strlen(s));\n  double r = sqrt(2.0) + pow(2, 3);\n  int g = f(c, c[0]);\n  FILE *fp = fopen(\"o.txt\", \"w\");\n  fprintf(fp, \"%d\\n\", g);\n  fclose(fp);\n  return 0;\n}\n";
        let (t, r) = conv(src);
        assert!(t.contains("const N: i32 = 4;"), "{t}");
        assert!(t.contains("let mut a: i32 = 0;\n    let mut b: i32 = 2;\n    let mut c: [i32; 4] = [0; 4];"), "{t}");
        assert!(t.contains("let mut p: *mut i32 = (&mut b as *mut i32);"), "{t}");
        assert!(t.contains("unsafe { (*p) = 5; }"), "{t}");
        assert!(t.contains("let mut m: *mut i32 = (c_malloc(N * (std::mem::size_of::<i32>() as i32)) as *mut i32);"), "{t}");
        assert!(t.contains("unsafe { (*m.wrapping_offset((1) as isize)) = 3; }"), "{t}");
        assert!(t.contains("c_free((m as *mut u8));"), "{t}");
        assert!(t.contains("let mut s: [u8; 8] = { let mut b = [0u8; 8]; b[..2].copy_from_slice(b\"hi\"); b };"), "{t}");
        assert!(t.contains("print!(\"{} {} {:5}\\n\", c_str(s.as_mut_ptr()), ((s[0]) as u8 as char), c_strlen(s.as_mut_ptr()));"), "{t}");
        assert!(t.contains("f64::sqrt(2.0) + f64::powf(2.0, 3.0)"), "{t}");
        assert!(t.contains("let mut g: i32 = { let _a1 = c[0]; f(&mut c, _a1) };"), "{t}");
        assert!(t.contains("c_fputs(fp, &format!(\"{}\\n\", g));"), "{t}");
        assert!(t.contains("fn c_fgets("), "{t}");
        assert!(r.is_empty(), "{r:?}");
    }
}
