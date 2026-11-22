//! COBOL(固定/自由形式)の部分集合 → 共通IR → Python / Rust。
//! 対応: PROGRAM-ID / 01〜49・77・88(PIC, VALUE, OCCURS, REDEFINES) / DISPLAY / ACCEPT / MOVE / ADD / SUBTRACT / MULTIPLY / DIVIDE(REMAINDER) / COMPUTE /
//!       IF-ELSE-END-IF(AND・OR・NOT・比較・条件名・NUMERIC/ALPHABETIC) / EVALUATE / PERFORM(段落・THRU・n TIMES・UNTIL・VARYING・インライン) /
//!       GO TO(DEPENDING ON 含む) / 段落 / STOP RUN・GOBACK / STRING・UNSTRING・INSPECT / INITIALIZE・SET / CALL(外部登録) /
//!       COPY(変換前に展開) / 順編成ファイル(SELECT・FD・OPEN・READ・WRITE・CLOSE) / 組み込み関数(一部)。
//! 数値は PIC の小数桁(V)を持つ固定小数点(i64 を 10^scale 倍した整数)。ROUNDED は四捨五入、省略は切り捨て。
//! 未対応の文(SORT・EXEC SQL/CICS・索引/相対ファイル等)は TODO コメントとして出力し、黙って落とさない。
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(String),
    Str(String),
    Var(String),
    Idx(String, Box<Expr>),          // 添字つき(1 始まり)
    Slice(String, Box<Expr>, Option<Box<Expr>>), // 参照修飾 X(start:len)
    Func(String, Vec<Expr>),         // FUNCTION NAME(args)
    Bin(char, Box<Expr>, Box<Expr>), // + - * / ^
    Neg(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cond {
    Cmp(String, Expr, Expr), // "==" "!=" ">" "<" ">=" "<="
    And(Box<Cond>, Box<Cond>),
    Or(Box<Cond>, Box<Cond>),
    Not(Box<Cond>),
    Class(Expr, String), // NUMERIC / ALPHABETIC
}

#[derive(Debug, Clone, PartialEq)]
pub enum Loop {
    Once,
    Times(Expr),
    Until(Cond),
    Varying { var: String, from: Expr, by: Expr, until: Cond },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inspect {
    Tally(String, Option<Expr>, bool), // カウンタ, パターン(None=CHARACTERS), LEADING か
    Replace { first: bool, lead: bool, from: Expr, to: Expr },
    Convert(Expr, Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Display(Vec<Expr>),
    DisplayNoNl(Vec<Expr>),
    Accept(String),
    AcceptSys(String, String),
    Move(Expr, Vec<String>),
    Assign(String, Expr),      // x = e
    AddTo(String, Expr),       // x += e
    SubFrom(String, Expr),     // x -= e
    MulBy(String, Expr),       // x *= e
    If(Cond, Vec<Stmt>, Vec<Stmt>),
    Perform(Loop, Vec<Stmt>),
    Call(String),              // PERFORM 段落
    Thru(String, String),      // PERFORM a THRU b
    Goto(String),
    Initialize(Vec<String>),
    Str(Vec<(Expr, Option<Expr>)>, String),   // STRING ... INTO
    Unstring(Expr, Expr, bool, Vec<String>),  // UNSTRING src DELIMITED BY [ALL] d INTO ...
    Inspect(String, Inspect),
    CallExt(String, Vec<Expr>),               // CALL "X" USING ...
    Open(String, String),                     // ファイル, input/output/extend/io
    Close(String),
    Read(String, Option<String>, Vec<Stmt>, Vec<Stmt>), // ファイル, INTO, AT END, NOT AT END
    Write(String, Option<Expr>),              // レコード名, FROM
    InspectScoped(String, Inspect, Option<Expr>, Option<Expr>), // INSPECT ... AFTER INITIAL a BEFORE INITIAL b
    SizeErr(Box<Stmt>, Vec<Stmt>, Vec<Stmt>), // 算術文, ON SIZE ERROR, NOT ON SIZE ERROR
    Stop,
    Nop,
    Todo(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Var {
    pub name: String,
    pub numeric: bool,
    pub init: String,
    pub decimal: bool,
    pub undeclared: bool,
    pub scale: u32,
    pub width: usize,
    pub occurs: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileDef {
    pub name: String,
    pub assign: String,
    pub assign_lit: bool,
    pub status: Option<String>,
    pub record: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CondName {
    pub name: String,
    pub parent: String,
    pub values: Vec<(Expr, Option<Expr>)>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Program {
    pub id: String,
    pub vars: Vec<Var>,
    pub stmts: Vec<Stmt>,
    pub paragraphs: Vec<(String, Vec<Stmt>)>,
    pub subs: Vec<Expr>,
    pub alias: Vec<(String, String)>,
    pub files: Vec<FileDef>,
    pub conds: Vec<CondName>,
    pub notes: Vec<String>,
}

fn strip(src: &str, free: bool) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for l in src.lines() {
        let l: String = if free {
            l.to_string()
        } else {
            let c: Vec<char> = l.chars().collect();
            if c.len() <= 6 || c[6] == '*' || c[6] == '/' {
                continue;
            }
            let body: String = c[7.min(c.len())..c.len().min(72)].iter().collect();
            if c[6] == '-' {
                // 継続行: 直前が閉じていない文字定数なら、先頭の引用符を落として空白ごと連結する
                if let Some(prev) = out.last_mut() {
                    let quotes = prev.chars().filter(|&x| x == '"' || x == '\'').count();
                    let t = body.trim_start();
                    if quotes % 2 == 1 && (t.starts_with('"') || t.starts_with('\'')) {
                        while prev.chars().count() < 66 {
                            prev.push(' ');
                        }
                        prev.push_str(&t[1..]);
                    } else {
                        prev.push(' ');
                        prev.push_str(t);
                    }
                }
                continue;
            }
            format!(" {body}")
        };
        if l.trim_start().starts_with("*>") || l.trim().is_empty() {
            continue;
        }
        out.push(l);
    }
    out
}

fn ident(n: &str) -> String {
    n.to_lowercase().replace('-', "_")
}

#[derive(Debug, Clone)]
struct Tok {
    s: String,
    dot: bool,
}

/// 引用符内の空白を保持し、括弧を独立したトークンにし、文末の '.' は dot フラグにする。
fn tokenize(s: &str) -> Vec<Tok> {
    let mut raw: Vec<String> = vec![];
    let mut cur = String::new();
    let mut q: Option<char> = None;
    for c in s.chars() {
        match q {
            Some(qc) => {
                cur.push(c);
                if c == qc {
                    q = None;
                }
            }
            None if c == '"' || c == '\'' => {
                cur.push(c);
                q = Some(c);
            }
            None if c.is_whitespace() || c == ',' || c == ';' => {
                if !cur.is_empty() {
                    raw.push(std::mem::take(&mut cur));
                }
            }
            None if c == '(' || c == ')' => {
                if !cur.is_empty() {
                    raw.push(std::mem::take(&mut cur));
                }
                raw.push(c.to_string());
            }
            None => cur.push(c),
        }
    }
    if !cur.is_empty() {
        raw.push(cur);
    }
    let mut v: Vec<Tok> = vec![];
    for r in raw {
        if r == "." {
            if let Some(l) = v.last_mut() {
                l.dot = true;
            }
            continue;
        }
        if r.ends_with('.') && r.len() > 1 {
            v.push(Tok { s: r[..r.len() - 1].to_string(), dot: true });
        } else {
            v.push(Tok { s: r, dot: false });
        }
    }
    v
}

const STARTERS: &[&str] = &[
    "COPY", "START", "DELETE", "SEARCH", "RETURN", "RELEASE", "MERGE", "INVOKE", "CANCEL", "ALTER", "END-READ", "END-CALL", "END-STRING", "END-UNSTRING", "END-WRITE", "END-SEARCH", "END-ACCEPT", "END-DISPLAY", "END-COMPUTE", "END-ADD", "END-SUBTRACT", "END-MULTIPLY", "END-DIVIDE", "END-EXEC",
    "DISPLAY", "MOVE", "ADD", "SUBTRACT", "MULTIPLY", "DIVIDE", "COMPUTE", "IF", "ELSE", "END-IF", "PERFORM", "END-PERFORM", "STOP", "GOBACK", "EXIT", "CONTINUE", "OPEN", "CLOSE", "READ", "WRITE", "CALL", "EVALUATE", "END-EVALUATE", "WHEN", "ACCEPT", "GO", "INITIALIZE", "SET", "STRING", "UNSTRING", "INSPECT", "SORT", "EXEC", "REWRITE",
];

fn is_starter(t: &str) -> bool {
    STARTERS.contains(&t.to_uppercase().as_str())
}

struct P {
    t: Vec<Tok>,
    i: usize,
    ended: bool, // 直前に消費したトークンが文末(.)だった
    used: BTreeSet<String>,
    subs: Vec<Expr>,
    conds: Vec<CondName>,
    files: Vec<FileDef>,
}

impl P {
    /// 代入先の文字列表現: 名前[\0添字番号][\u{1}=ROUNDED]
    fn lv_of(&mut self, e: Expr) -> Option<String> {
        match e {
            Expr::Var(n) => Some(n),
            Expr::Idx(n, i) => {
                let (base, r) = match n.strip_suffix('\u{1}') {
                    Some(b) => (b.to_string(), true),
                    None => (n, false),
                };
                self.subs.push(*i);
                Some(format!("{base}\0{}{}", self.subs.len() - 1, if r { "\u{1}" } else { "" }))
            }
            _ => None,
        }
    }
    fn cond_of_name(&self, n: &str) -> Option<Cond> {
        let c = self.conds.iter().find(|c| c.name == n)?;
        let mut acc: Option<Cond> = None;
        for (a, b) in &c.values {
            let v = Expr::Var(c.parent.clone());
            let one = match b {
                None => Cond::Cmp("==".into(), v, a.clone()),
                Some(b) => Cond::And(Box::new(Cond::Cmp(">=".into(), v.clone(), a.clone())), Box::new(Cond::Cmp("<=".into(), v, b.clone()))),
            };
            acc = Some(match acc {
                None => one,
                Some(p) => Cond::Or(Box::new(p), Box::new(one)),
            });
        }
        acc
    }
    fn file_named(&self, n: &str) -> Option<&FileDef> {
        self.files.iter().find(|f| f.name == n)
    }
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }
    fn up(&self, k: usize) -> String {
        self.t.get(self.i + k).map(|t| t.s.to_uppercase()).unwrap_or_default()
    }
    fn next(&mut self) -> Option<Tok> {
        let t = self.t.get(self.i).cloned();
        if let Some(x) = &t {
            self.ended = x.dot;
            self.i += 1;
        }
        t
    }
    fn word(&self) -> String {
        self.up(0)
    }
    /// 次が [NOT] [ON] SIZE ERROR の節の頭か(文の中身の読み取りをここで止める)
    fn at_size_clause(&self) -> bool {
        let w = self.up(0);
        (w == "ON" && self.up(1) == "SIZE") || (w == "NOT" && (self.up(1) == "SIZE" || (self.up(1) == "ON" && self.up(2) == "SIZE")))
    }

    // ---- 式 ----
    fn atom(&mut self) -> Result<Expr, String> {
        let t = self.next().ok_or("式が途中で終わった")?;
        let s = t.s;
        if s == "(" {
            let e = self.expr(0)?;
            if self.peek().map(|x| x.s == ")").unwrap_or(false) {
                self.next();
            }
            return Ok(e);
        }
        if s == "-" {
            return Ok(Expr::Neg(Box::new(self.atom()?)));
        }
        let c = s.chars().next().unwrap_or(' ');
        if c == '"' || c == '\'' {
            return Ok(Expr::Str(s[1..s.len().saturating_sub(1)].to_string()));
        }
        if c.is_ascii_digit() || (c == '-' && s.len() > 1) {
            return Ok(Expr::Num(s));
        }
        match s.to_uppercase().as_str() {
            "ZERO" | "ZEROS" | "ZEROES" => return Ok(Expr::Num("0".into())),
            "SPACE" | "SPACES" => return Ok(Expr::Str(String::new())),
            _ => {}
        }
        if s.to_uppercase() == "FUNCTION" {
            let name = self.next().ok_or("FUNCTION 名が無い")?.s.to_uppercase();
            const KNOWN: [&str; 9] = ["LENGTH", "UPPER-CASE", "LOWER-CASE", "REVERSE", "MAX", "MIN", "ABS", "MOD", "NUMVAL"];
            let known = KNOWN.contains(&name.as_str()) || name == "INTEGER";
            if !known {
                return Err(format!("FUNCTION {name} は未対応"));
            }
            let mut args = vec![];
            if self.peek().map(|x| x.s == "(").unwrap_or(false) {
                self.next();
                while self.peek().map(|x| x.s != ")").unwrap_or(false) {
                    args.push(self.expr(1)?);
                }
                self.next();
            }
            return Ok(Expr::Func(name, args));
        }
        let n = ident(&s);
        self.used.insert(n.clone());
        if self.peek().map(|x| x.s == "(").unwrap_or(false) {
            self.next();
            let mut j = self.i;
            let mut joined = String::new();
            while let Some(t) = self.t.get(j) {
                if t.s == ")" {
                    break;
                }
                joined.push_str(&t.s);
                j += 1;
            }
            if joined.contains(':') && self.t.get(j).is_some() {
                let (a, b) = joined.split_once(':').unwrap();
                let mk = |x: &str, me: &mut P| -> Result<Expr, String> {
                    if x.chars().all(|c| c.is_ascii_digit()) {
                        Ok(Expr::Num(x.to_string()))
                    } else if x.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                        let v = ident(x);
                        me.used.insert(v.clone());
                        Ok(Expr::Var(v))
                    } else {
                        Err("参照修飾の位置・長さは数か変数のみ対応".into())
                    }
                };
                let start = mk(a, self)?;
                let len = if b.is_empty() { None } else { Some(Box::new(mk(b, self)?)) };
                self.i = j + 1;
                return Ok(Expr::Slice(n, Box::new(start), len));
            }
            let idx = self.expr(1)?;
            if !self.peek().map(|x| x.s == ")").unwrap_or(false) {
                return Err("多次元の添字は未対応".into());
            }
            self.next();
            return Ok(Expr::Idx(n, Box::new(idx)));
        }
        Ok(Expr::Var(n))
    }
    fn prec(op: &str) -> Option<(u8, char)> {
        match op {
            "+" => Some((1, '+')),
            "-" => Some((1, '-')),
            "*" => Some((2, '*')),
            "/" => Some((2, '/')),
            "**" => Some((3, '^')),
            _ => None,
        }
    }
    fn expr(&mut self, min: u8) -> Result<Expr, String> {
        let mut l = self.atom()?;
        while let Some(t) = self.peek() {
            let Some((p, op)) = Self::prec(&t.s) else { break };
            if p < min || p == 0 {
                break;
            }
            self.next();
            let r = self.expr(p + 1)?;
            l = Expr::Bin(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    /// 演算子を含まない1つ分(ADD/SUBTRACT の項、DISPLAY の項)
    fn operand(&mut self) -> Result<Expr, String> {
        self.atom()
    }

    // ---- 条件 ----
    fn cond(&mut self) -> Result<Cond, String> {
        let mut l = self.cond_and()?;
        while self.word() == "OR" {
            self.next();
            let r = self.cond_and()?;
            l = Cond::Or(Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn cond_and(&mut self) -> Result<Cond, String> {
        let mut l = self.cond_not()?;
        while self.word() == "AND" {
            self.next();
            let r = self.cond_not()?;
            l = Cond::And(Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn cond_not(&mut self) -> Result<Cond, String> {
        if self.word() == "NOT" {
            self.next();
            return Ok(Cond::Not(Box::new(self.cond_not()?)));
        }
        if self.word() == "(" {
            let save = self.i;
            self.next();
            if let Ok(c) = self.cond() {
                if self.peek().map(|x| x.s == ")").unwrap_or(false) {
                    self.next();
                    return Ok(c);
                }
            }
            self.i = save; // 式の括弧だった
        }
        let a = self.expr(1)?;
        if let Expr::Var(n) = &a {
            let cmp_next = matches!(self.word().as_str(), "=" | ">" | "<" | ">=" | "<=" | "EQUAL" | "GREATER" | "LESS" | "IS" | "NOT");
            if !cmp_next {
                if let Some(c) = self.cond_of_name(n) {
                    return Ok(c);
                }
            }
        }
        let mut neg = false;
        let mut w = self.word();
        if w == "IS" {
            self.next();
            w = self.word();
        }
        if w == "NOT" {
            neg = true;
            self.next();
            w = self.word();
        }
        if matches!(w.as_str(), "NUMERIC" | "ALPHABETIC" | "ALPHABETIC-LOWER" | "ALPHABETIC-UPPER") {
            self.next();
            let c = Cond::Class(a, w);
            return Ok(if neg { Cond::Not(Box::new(c)) } else { c });
        }
        let op = match w.as_str() {
            "=" | "EQUAL" => {
                self.next();
                if self.word() == "TO" {
                    self.next();
                }
                "=="
            }
            ">" | "GREATER" => {
                self.next();
                if self.word() == "THAN" {
                    self.next();
                }
                if self.word() == "OR" && self.up(1) == "EQUAL" {
                    self.next();
                    self.next();
                    if self.word() == "TO" {
                        self.next();
                    }
                    ">="
                } else {
                    ">"
                }
            }
            "<" | "LESS" => {
                self.next();
                if self.word() == "THAN" {
                    self.next();
                }
                if self.word() == "OR" && self.up(1) == "EQUAL" {
                    self.next();
                    self.next();
                    if self.word() == "TO" {
                        self.next();
                    }
                    "<="
                } else {
                    "<"
                }
            }
            ">=" => {
                self.next();
                ">="
            }
            "<=" => {
                self.next();
                "<="
            }
            other => return Err(format!("条件の形を解釈できない: {other}")),
        };
        let b = self.expr(1)?;
        let c = Cond::Cmp(op.to_string(), a, b);
        Ok(if neg { Cond::Not(Box::new(c)) } else { c })
    }

    // ---- 文 ----
    fn skip_to_end(&mut self, until: &[&str]) {
        // 未対応の文: 文末(.)、次の動詞、または指定語まで読み飛ばす
        let first = self.next();
        if first.map(|t| t.dot).unwrap_or(true) {
            return;
        }
        while let Some(t) = self.peek() {
            if is_starter(&t.s) || until.contains(&t.s.to_uppercase().as_str()) {
                break;
            }
            let d = t.dot;
            self.next();
            if d {
                break;
            }
        }
    }

    fn names_until(&mut self, stops: &[&str]) -> Vec<String> {
        let mut v: Vec<String> = vec![];
        while let Some(t) = self.peek() {
            let u = t.s.to_uppercase();
            if is_starter(&u) || stops.contains(&u.as_str()) || t.s == ")" || self.at_size_clause() {
                break;
            }
            let (s, d) = (t.s.clone(), t.dot);
            self.next();
            if u == "ROUNDED" {
                if let Some(l) = v.last_mut() {
                    l.push('\u{1}');
                }
            } else {
                let n = ident(&s);
                self.used.insert(n.clone());
                let sub = if !d && self.peek().map(|x| x.s == "(").unwrap_or(false) {
                    let save = self.i;
                    self.next();
                    match self.expr(1) {
                        Ok(ix) if self.peek().map(|x| x.s == ")").unwrap_or(false) => {
                            let dd = self.next().map(|x| x.dot).unwrap_or(false);
                            self.subs.push(ix);
                            if dd {
                                self.ended = true;
                            }
                            Some(self.subs.len() - 1)
                        }
                        _ => {
                            self.i = save;
                            None
                        }
                    }
                } else {
                    None
                };
                match sub {
                    Some(k) => v.push(format!("{n}\0{k}")),
                    None => v.push(n),
                }
                if self.ended && sub.is_some() {
                    break;
                }
            }
            if d {
                break;
            }
        }
        v
    }

    fn operands_until(&mut self, stops: &[&str]) -> Result<Vec<Expr>, String> {
        let mut v = vec![];
        loop {
            let Some(t) = self.peek() else { break };
            let u = t.s.to_uppercase();
            if is_starter(&u) || stops.contains(&u.as_str()) || self.ended && !v.is_empty() || self.at_size_clause() {
                break;
            }
            if self.ended && v.is_empty() {
                // 直前で文が終わっている(空の文)
                break;
            }
            if u == "ROUNDED" {
                self.next();
                match v.pop() {
                    Some(Expr::Var(n)) => v.push(Expr::Var(format!("{n}\u{1}"))),
                    Some(Expr::Idx(n, i)) => v.push(Expr::Idx(format!("{n}\u{1}"), i)),
                    Some(o) => v.push(o),
                    None => {}
                }
                if self.ended {
                    break;
                }
                continue;
            }
            v.push(self.operand()?);
            if self.ended {
                break;
            }
        }
        Ok(v)
    }

    /// 終端語(END-xxx)または文末(.)まで読み飛ばす(文が既に終わっていれば何もしない)
    fn skip_through(&mut self, endw: &str) {
        if self.ended {
            return;
        }
        while let Some(t) = self.peek() {
            let e = t.s.to_uppercase() == endw;
            let d = t.dot;
            self.next();
            if e || d {
                break;
            }
        }
    }

    fn sum(es: Vec<Expr>) -> Expr {
        let mut it = es.into_iter();
        let mut acc = it.next().unwrap_or(Expr::Num("0".into()));
        for e in it {
            acc = Expr::Bin('+', Box::new(acc), Box::new(e));
        }
        acc
    }

    fn block(&mut self, scoped: bool, terms: &[&str]) -> Vec<Stmt> {
        let mut v = vec![];
        loop {
            if scoped && self.ended {
                break;
            }
            let Some(t) = self.peek() else { break };
            let u = t.s.to_uppercase();
            if terms.contains(&u.as_str()) {
                break;
            }
            if !scoped {
                self.ended = false;
            }
            match self.statement() {
                Some(s) => v.push(s),
                None => break,
            }
        }
        v
    }

    fn statement(&mut self) -> Option<Stmt> {
        let t = self.peek()?.clone();
        let u = t.s.to_uppercase();
        let st = match u.as_str() {
            "DISPLAY" => {
                self.next();
                let ops = self.operands_until(&["UPON", "WITH", "NO"]);
                match ops {
                    Ok(o) => {
                        if self.word() == "UPON" {
                            self.next();
                            self.next();
                        }
                        if self.word() == "WITH" && self.up(1) == "NO" && self.up(2) == "ADVANCING" {
                            self.next();
                            self.next();
                            self.next();
                            Stmt::DisplayNoNl(o)
                        } else {
                            Stmt::Display(o)
                        }
                    }
                    Err(e) => {
                        self.skip_to_end(&[]);
                        Stmt::Todo(format!("DISPLAY: {e}"))
                    }
                }
            }
            "MOVE" => {
                self.next();
                match self.operand() {
                    Ok(src) if self.word() == "TO" => {
                        self.next();
                        let tg = self.names_until(&[]);
                        Stmt::Move(src, tg)
                    }
                    _ => {
                        self.skip_to_end(&[]);
                        Stmt::Todo("MOVE".into())
                    }
                }
            }
            "ADD" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let ops = self.operands_until(&["TO", "GIVING", "ON", "NOT", "SIZE"])?;
                    let mut to = vec![];
                    let mut giving = vec![];
                    if self.word() == "TO" {
                        self.next();
                        to = self.operands_until(&["GIVING", "ON", "NOT", "SIZE"])?;
                    }
                    if self.word() == "GIVING" {
                        self.next();
                        giving = self.names_until(&["ON", "NOT", "SIZE"]);
                    }
                    if giving.is_empty() {
                        let total = Self::sum(ops);
                        let mut out = vec![];
                        for t in to {
                            if let Some(n) = self.lv_of(t) {
                                out.push(Stmt::AddTo(n, total.clone()));
                            }
                        }
                        return Ok(if out.len() == 1 { out.remove(0) } else { Stmt::If(Cond::Cmp("==".into(), Expr::Num("1".into()), Expr::Num("1".into())), out, vec![]) });
                    }
                    let mut all = ops;
                    all.extend(to);
                    let total = Self::sum(all);
                    let mut out: Vec<Stmt> = giving.into_iter().map(|g| Stmt::Assign(g, total.clone())).collect();
                    Ok(if out.len() == 1 { out.remove(0) } else { Stmt::If(Cond::Cmp("==".into(), Expr::Num("1".into()), Expr::Num("1".into())), out, vec![]) })
                })();
                r.unwrap_or_else(|e| Stmt::Todo(format!("ADD: {e}")))
            }
            "SUBTRACT" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let ops = self.operands_until(&["FROM"])?;
                    if self.word() != "FROM" {
                        return Err("FROM が無い".into());
                    }
                    self.next();
                    let from = self.operand()?;
                    let sum = Self::sum(ops);
                    if self.word() == "GIVING" {
                        self.next();
                        let g = self.names_until(&["ON", "NOT", "SIZE"]);
                        return Ok(Stmt::Assign(g.into_iter().next().ok_or("GIVING 先が無い")?, Expr::Bin('-', Box::new(from), Box::new(sum))));
                    }
                    match self.lv_of(from) {
                        Some(n) => Ok(Stmt::SubFrom(n, sum)),
                        None => Err("FROM の先が変数ではない".into()),
                    }
                })();
                r.unwrap_or_else(|e| Stmt::Todo(format!("SUBTRACT: {e}")))
            }
            "MULTIPLY" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let a = self.operand()?;
                    if self.word() != "BY" {
                        return Err("BY が無い".into());
                    }
                    self.next();
                    let b = self.operand()?;
                    if self.word() == "GIVING" {
                        self.next();
                        let g = self.names_until(&["ON", "NOT", "SIZE"]);
                        return Ok(Stmt::Assign(g.into_iter().next().ok_or("GIVING 先が無い")?, Expr::Bin('*', Box::new(a), Box::new(b))));
                    }
                    match self.lv_of(b) {
                        Some(n) => Ok(Stmt::MulBy(n, a)),
                        None => Err("BY の先が変数ではない".into()),
                    }
                })();
                r.unwrap_or_else(|e| Stmt::Todo(format!("MULTIPLY: {e}")))
            }
            "DIVIDE" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let a = self.operand()?;
                    let by = self.word() == "BY";
                    if !(by || self.word() == "INTO") {
                        return Err("INTO/BY が無い".into());
                    }
                    self.next();
                    let b = self.operand()?;
                    if self.word() == "GIVING" {
                        self.next();
                        let g = self.names_until(&["REMAINDER", "ON", "NOT", "SIZE"]);
                        let q = g.into_iter().next().ok_or("GIVING 先が無い")?;
                        let (dividend, divisor) = if by { (a, b) } else { (b, a) };
                        let e = Expr::Bin('/', Box::new(dividend.clone()), Box::new(divisor.clone()));
                        if self.word() == "REMAINDER" {
                            self.next();
                            let r = self.names_until(&["ON", "NOT", "SIZE"]).into_iter().next().ok_or("REMAINDER 先が無い")?;
                            let qv = Expr::Var(q.split(['\0', '\u{1}']).next().unwrap_or("").to_string());
                            let rem = Expr::Bin('-', Box::new(dividend), Box::new(Expr::Bin('*', Box::new(qv), Box::new(divisor))));
                            return Ok(Stmt::If(Cond::Cmp("==".into(), Expr::Num("1".into()), Expr::Num("1".into())), vec![Stmt::Assign(q, e), Stmt::Assign(r, rem)], vec![]));
                        }
                        return Ok(Stmt::Assign(q, e));
                    }
                    match self.lv_of(b) {
                        Some(n) if !by => Ok(Stmt::Assign(n.clone(), Expr::Bin('/', Box::new(Expr::Var(n.split(['\0', '\u{1}']).next().unwrap_or("").to_string())), Box::new(a)))),
                        _ => Err("DIVIDE の形が未対応".into()),
                    }
                })();
                r.unwrap_or_else(|e| {
                    self.skip_to_end(&[]);
                    Stmt::Todo(format!("DIVIDE: {e}"))
                })
            }
            "COMPUTE" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let tg = self.names_until(&["="]);
                    if self.word() != "=" {
                        return Err("= が無い".into());
                    }
                    self.next();
                    let e = self.expr(1)?;
                    Ok(Stmt::Assign(tg.into_iter().next().ok_or("代入先が無い")?, e))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_to_end(&[]);
                    Stmt::Todo(format!("COMPUTE: {e}"))
                })
            }
            "IF" => {
                self.next();
                match self.cond() {
                    Ok(c) => {
                        if self.word() == "THEN" {
                            self.next();
                        }
                        let th = self.block(true, &["ELSE", "END-IF"]);
                        let mut el = vec![];
                        if self.word() == "ELSE" {
                            self.next();
                            el = self.block(true, &["END-IF"]);
                        }
                        if self.word() == "END-IF" {
                            let d = self.peek().map(|t| t.dot).unwrap_or(false);
                            self.next();
                            self.ended = d;
                        }
                        Stmt::If(c, th, el)
                    }
                    Err(e) => {
                        // END-IF か文末まで読み飛ばす
                        while let Some(t) = self.peek() {
                            let end_if = t.s.to_uppercase() == "END-IF";
                            let d = t.dot;
                            self.next();
                            if end_if || d {
                                break;
                            }
                        }
                        Stmt::Todo(format!("IF: {e}"))
                    }
                }
            }
            "EVALUATE" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let is_true = self.word() == "TRUE";
                    let subj = if is_true {
                        self.next();
                        None
                    } else {
                        Some(self.expr(1)?)
                    };
                    let mut arms: Vec<(Option<Cond>, Vec<Stmt>)> = vec![];
                    while self.word() == "WHEN" {
                        let mut conds: Vec<Cond> = vec![];
                        let mut other = false;
                        while self.word() == "WHEN" {
                            self.next();
                            if self.word() == "OTHER" {
                                self.next();
                                other = true;
                                continue;
                            }
                            let c = if let Some(s) = &subj {
                                let a = self.expr(1)?;
                                if self.word() == "THRU" || self.word() == "THROUGH" {
                                    self.next();
                                    let b = self.expr(1)?;
                                    Cond::And(
                                        Box::new(Cond::Cmp(">=".into(), s.clone(), a)),
                                        Box::new(Cond::Cmp("<=".into(), s.clone(), b)),
                                    )
                                } else {
                                    Cond::Cmp("==".into(), s.clone(), a)
                                }
                            } else {
                                self.cond()?
                            };
                            if self.word() == "ALSO" {
                                return Err("ALSO(複数主語)は未対応".into());
                            }
                            conds.push(c);
                        }
                        let body = self.block(true, &["WHEN", "END-EVALUATE"]);
                        let c = if other {
                            None
                        } else {
                            let mut it = conds.into_iter();
                            let mut c = it.next().ok_or("WHEN の条件が無い")?;
                            for n in it {
                                c = Cond::Or(Box::new(c), Box::new(n));
                            }
                            Some(c)
                        };
                        arms.push((c, body));
                    }
                    if self.word() == "END-EVALUATE" {
                        let d = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = d;
                    }
                    let mut tail: Vec<Stmt> = vec![];
                    for (c, body) in arms.into_iter().rev() {
                        tail = match c {
                            None => body,
                            Some(c) => vec![Stmt::If(c, body, tail)],
                        };
                    }
                    Ok(if tail.len() == 1 {
                        tail.remove(0)
                    } else {
                        Stmt::If(Cond::Cmp("==".into(), Expr::Num("1".into()), Expr::Num("1".into())), tail, vec![])
                    })
                })();
                r.unwrap_or_else(|e| {
                    while let Some(t) = self.peek() {
                        let end = t.s.to_uppercase() == "END-EVALUATE";
                        let d = t.dot;
                        self.next();
                        if end || d {
                            break;
                        }
                    }
                    Stmt::Todo(format!("EVALUATE: {e}"))
                })
            }
            "PERFORM" => {
                self.next();
                let w = self.word();
                let inline = w == "VARYING" || w == "UNTIL" || (self.up(1) == "TIMES");
                let mut call: Option<String> = None;
                let mut thru: Option<String> = None;
                if !inline && w != "VARYING" && w != "UNTIL" {
                    let n = self.next().unwrap();
                    call = Some(ident(&n.s));
                    if matches!(self.word().as_str(), "THRU" | "THROUGH") {
                        self.next();
                        if let Some(b) = self.next() {
                            thru = Some(ident(&b.s));
                        }
                    }
                }
                let r: Result<Loop, String> = (|| {
                    let w = self.word();
                    if w == "VARYING" {
                        self.next();
                        let v = ident(&self.next().ok_or("VARYING 変数")?.s);
                        self.used.insert(v.clone());
                        if self.word() != "FROM" {
                            return Err("FROM が無い".into());
                        }
                        self.next();
                        let from = self.expr(1)?;
                        if self.word() != "BY" {
                            return Err("BY が無い".into());
                        }
                        self.next();
                        let by = self.expr(1)?;
                        if self.word() != "UNTIL" {
                            return Err("UNTIL が無い".into());
                        }
                        self.next();
                        let until = self.cond()?;
                        Ok(Loop::Varying { var: v, from, by, until })
                    } else if w == "UNTIL" {
                        self.next();
                        Ok(Loop::Until(self.cond()?))
                    } else if self.up(1) == "TIMES" {
                        let n = self.operand()?;
                        self.next();
                        Ok(Loop::Times(n))
                    } else if self.up(1) == "TIMES" || (call.is_some() && matches!(self.up(1).as_str(), "TIMES")) {
                        Ok(Loop::Once)
                    } else if call.is_some() && self.up(0).chars().all(|c| c.is_ascii_digit()) && self.up(1) == "TIMES" {
                        Ok(Loop::Once)
                    } else {
                        Ok(Loop::Once)
                    }
                })();
                match r {
                    Err(e) => {
                        self.skip_to_end(&["END-PERFORM"]);
                        Stmt::Todo(format!("PERFORM: {e}"))
                    }
                    Ok(mut lp) => {
                        // 段落 n TIMES の形(段落名の後ろに数と TIMES)
                        if let (Some(_), Loop::Once) = (&call, &lp) {
                            if self.up(1) == "TIMES" {
                                if let Ok(n) = self.operand() {
                                    self.next();
                                    lp = Loop::Times(n);
                                }
                            } else if self.word() == "UNTIL" {
                                self.next();
                                if let Ok(c) = self.cond() {
                                    lp = Loop::Until(c);
                                }
                            }
                        }
                        let body = match call {
                            Some(c) => match thru {
                                Some(t) => vec![Stmt::Thru(c, t)],
                                None => vec![Stmt::Call(c)],
                            },
                            None => {
                                let b = self.block(false, &["END-PERFORM"]);
                                if self.word() == "END-PERFORM" {
                                    let d = self.peek().map(|t| t.dot).unwrap_or(false);
                                    self.next();
                                    self.ended = d;
                                }
                                b
                            }
                        };
                        Stmt::Perform(lp, body)
                    }
                }
            }
            "STOP" => {
                self.next();
                if self.word() == "RUN" {
                    self.next();
                }
                Stmt::Stop
            }
            "GOBACK" => {
                self.next();
                Stmt::Stop
            }
            "EXIT" | "CONTINUE" => {
                self.next();
                if self.word() == "PROGRAM" {
                    self.next();
                    Stmt::Stop
                } else {
                    Stmt::Nop
                }
            }
            "GO" => {
                self.next();
                if self.word() == "TO" {
                    self.next();
                }
                let mut names = vec![];
                while let Some(t) = self.peek() {
                    let u = t.s.to_uppercase();
                    if u == "DEPENDING" || is_starter(&u) {
                        break;
                    }
                    let d = t.dot;
                    names.push(ident(&t.s));
                    self.next();
                    if d {
                        break;
                    }
                }
                if !self.ended && self.word() == "DEPENDING" {
                    self.next();
                    if self.word() == "ON" {
                        self.next();
                    }
                    match self.expr(1) {
                        Ok(x) => {
                            let mut tail: Vec<Stmt> = vec![];
                            for (k, n) in names.iter().enumerate().rev() {
                                tail = vec![Stmt::If(Cond::Cmp("==".into(), x.clone(), Expr::Num((k + 1).to_string())), vec![Stmt::Goto(n.clone())], tail)];
                            }
                            bundle(tail)
                        }
                        Err(e) => Stmt::Todo(format!("GO TO DEPENDING: {e}")),
                    }
                } else if let Some(n) = names.into_iter().next() {
                    Stmt::Goto(n)
                } else {
                    Stmt::Todo("GO TO".into())
                }
            }
            "INITIALIZE" => {
                self.next();
                let n = self.names_until(&["REPLACING", "WITH", "TO", "ALL"]);
                if !self.ended && matches!(self.word().as_str(), "REPLACING" | "WITH" | "TO" | "ALL") {
                    self.skip_to_end(&[]);
                    Stmt::Todo("INITIALIZE の REPLACING / WITH FILLER は未対応".into())
                } else {
                    Stmt::Initialize(n)
                }
            }
            "SET" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let tg = self.names_until(&["TO", "UP", "DOWN"]);
                    let t = tg.into_iter().next().ok_or("SET 先が無い")?;
                    let w = self.word();
                    if w == "TO" {
                        self.next();
                        if self.word() == "TRUE" {
                            self.next();
                            let base = t.split(['\0', '\u{1}']).next().unwrap_or("").to_string();
                            let c = self.conds.iter().find(|c| c.name == base).ok_or("条件名が未定義")?;
                            let v = c.values.first().ok_or("条件名に値が無い")?.0.clone();
                            return Ok(Stmt::Move(v, vec![c.parent.clone()]));
                        }
                        if self.word() == "FALSE" {
                            return Err("SET ... TO FALSE は未対応".into());
                        }
                        let e = self.expr(1)?;
                        return Ok(Stmt::Move(e, vec![t]));
                    }
                    if w == "UP" || w == "DOWN" {
                        self.next();
                        if self.word() != "BY" {
                            return Err("BY が無い".into());
                        }
                        self.next();
                        let e = self.expr(1)?;
                        return Ok(if w == "UP" { Stmt::AddTo(t, e) } else { Stmt::SubFrom(t, e) });
                    }
                    Err("SET の形が未対応".into())
                })();
                r.unwrap_or_else(|e| {
                    self.skip_to_end(&[]);
                    Stmt::Todo(format!("SET: {e}"))
                })
            }
            "ACCEPT" => {
                self.next();
                let tg = self.names_until(&["FROM"]);
                if !self.ended && self.word() == "FROM" {
                    self.next();
                    let kind = if self.ended { String::new() } else { self.word() };
                    let ok = matches!(kind.as_str(), "DATE" | "DAY" | "TIME" | "DAY-OF-WEEK");
                    if ok && !tg.is_empty() {
                        self.next();
                        let mut k = kind.clone();
                        if !self.ended && self.word() == "YYYYMMDD" && kind == "DATE" {
                            k = "DATE8".into();
                            self.next();
                        } else if !self.ended && self.word() == "YYYYDDD" && kind == "DAY" {
                            k = "DAY7".into();
                            self.next();
                        }
                        Stmt::AcceptSys(tg.into_iter().next().unwrap(), k)
                    } else {
                        self.skip_to_end(&[]);
                        Stmt::Todo("ACCEPT FROM(DATE/DAY/TIME/DAY-OF-WEEK 以外)は未対応".into())
                    }
                } else if let Some(t) = tg.into_iter().next() {
                    Stmt::Accept(t)
                } else {
                    Stmt::Todo("ACCEPT".into())
                }
            }
            "STRING" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let mut srcs: Vec<(Expr, Option<Expr>)> = vec![];
                    loop {
                        let ops = self.operands_until(&["DELIMITED", "INTO"])?;
                        if self.word() == "DELIMITED" {
                            self.next();
                            if self.word() == "BY" {
                                self.next();
                            }
                            let d = if self.word() == "SIZE" {
                                self.next();
                                None
                            } else if matches!(self.word().as_str(), "SPACE" | "SPACES") {
                                self.next();
                                Some(Expr::Str(" ".into()))
                            } else {
                                Some(self.operand()?)
                            };
                            for o in ops {
                                srcs.push((o, d.clone()));
                            }
                            if self.word() == "INTO" {
                                break;
                            }
                        } else {
                            for o in ops {
                                srcs.push((o, None));
                            }
                            break;
                        }
                    }
                    if self.word() != "INTO" {
                        return Err("INTO が無い".into());
                    }
                    self.next();
                    let tg = self.names_until(&["WITH", "ON", "NOT", "POINTER"]).into_iter().next().ok_or("INTO 先が無い")?;
                    if !self.ended && matches!(self.word().as_str(), "WITH" | "ON" | "NOT" | "POINTER") {
                        return Err("POINTER / ON OVERFLOW は未対応".into());
                    }
                    if !self.ended && self.word() == "END-STRING" {
                        let d = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = d;
                    }
                    Ok(Stmt::Str(srcs, tg))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_through("END-STRING");
                    Stmt::Todo(format!("STRING: {e}"))
                })
            }
            "UNSTRING" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let src = self.operand()?;
                    if self.word() != "DELIMITED" {
                        return Err("DELIMITED BY が無い".into());
                    }
                    self.next();
                    if self.word() == "BY" {
                        self.next();
                    }
                    let all = self.word() == "ALL";
                    if all {
                        self.next();
                    }
                    let d = if matches!(self.word().as_str(), "SPACE" | "SPACES") {
                        self.next();
                        Expr::Str(" ".into())
                    } else {
                        self.operand()?
                    };
                    if self.word() != "INTO" {
                        return Err("INTO が無い".into());
                    }
                    self.next();
                    let tg = self.names_until(&["TALLYING", "ON", "NOT", "WITH", "DELIMITER", "COUNT"]);
                    if !self.ended && matches!(self.word().as_str(), "TALLYING" | "ON" | "NOT" | "WITH" | "DELIMITER" | "COUNT") {
                        return Err("TALLYING / COUNT / ON OVERFLOW は未対応".into());
                    }
                    if !self.ended && self.word() == "END-UNSTRING" {
                        let dd = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = dd;
                    }
                    Ok(Stmt::Unstring(src, d, all, tg))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_through("END-UNSTRING");
                    Stmt::Todo(format!("UNSTRING: {e}"))
                })
            }
            "INSPECT" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let tg = ident(&self.next().ok_or("INSPECT 対象が無い")?.s);
                    self.used.insert(tg.clone());
                    let st = match self.word().as_str() {
                        "TALLYING" => {
                            self.next();
                            let c = ident(&self.next().ok_or("カウンタが無い")?.s);
                            self.used.insert(c.clone());
                            if self.word() != "FOR" {
                                return Err("TALLYING は FOR ALL / LEADING / CHARACTERS のみ対応".into());
                            }
                            self.next();
                            match self.word().as_str() {
                                "ALL" | "LEADING" => {
                                    let lead = self.word() == "LEADING";
                                    self.next();
                                    Stmt::Inspect(tg, Inspect::Tally(c, Some(self.operand()?), lead))
                                }
                                "CHARACTERS" => {
                                    self.next();
                                    Stmt::Inspect(tg, Inspect::Tally(c, None, false))
                                }
                                _ => return Err("TALLYING は FOR ALL / LEADING / CHARACTERS のみ対応".into()),
                            }
                        }
                        "REPLACING" => {
                            self.next();
                            let (first, lead) = match self.word().as_str() {
                                "ALL" => (false, false),
                                "FIRST" => (true, false),
                                "LEADING" => (false, true),
                                _ => return Err("REPLACING は ALL / FIRST / LEADING のみ対応".into()),
                            };
                            self.next();
                            let from = self.operand()?;
                            if self.word() != "BY" {
                                return Err("BY が無い".into());
                            }
                            self.next();
                            let to = self.operand()?;
                            Stmt::Inspect(tg, Inspect::Replace { first, lead, from, to })
                        }
                        "CONVERTING" => {
                            self.next();
                            let a = self.operand()?;
                            if self.word() != "TO" {
                                return Err("TO が無い".into());
                            }
                            self.next();
                            let b = self.operand()?;
                            Stmt::Inspect(tg, Inspect::Convert(a, b))
                        }
                        _ => return Err("INSPECT の形が未対応".into()),
                    };
                    let (mut after, mut before) = (None, None);
                    while !self.ended && matches!(self.word().as_str(), "BEFORE" | "AFTER") {
                        let is_after = self.word() == "AFTER";
                        self.next();
                        if self.word() == "INITIAL" {
                            self.next();
                        }
                        let e = self.operand()?;
                        if is_after {
                            after = Some(e);
                        } else {
                            before = Some(e);
                        }
                    }
                    if after.is_some() || before.is_some() {
                        if let Stmt::Inspect(tg, op) = st {
                            return Ok(Stmt::InspectScoped(tg, op, after, before));
                        }
                    }
                    Ok(st)
                })();
                r.unwrap_or_else(|e| {
                    self.skip_to_end(&[]);
                    Stmt::Todo(format!("INSPECT: {e}"))
                })
            }
            "CALL" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let name = match self.operand()? {
                        Expr::Str(s) => s,
                        _ => return Err("動的 CALL(変数でプログラム名を指定)は未対応".into()),
                    };
                    let mut args = vec![];
                    if !self.ended && self.word() == "USING" {
                        self.next();
                        while !self.ended {
                            let w = self.word();
                            if matches!(w.as_str(), "BY" | "REFERENCE" | "CONTENT" | "VALUE") {
                                self.next();
                                continue;
                            }
                            if w.is_empty() || is_starter(&w) || matches!(w.as_str(), "RETURNING" | "ON" | "NOT") {
                                break;
                            }
                            args.push(self.operand()?);
                        }
                    }
                    let mut v = vec![Stmt::CallExt(name, args)];
                    if !self.ended && matches!(self.word().as_str(), "ON" | "NOT" | "RETURNING") {
                        self.skip_through("END-CALL");
                        v.push(Stmt::Todo("CALL の RETURNING / ON EXCEPTION は未対応".into()));
                    } else if !self.ended && self.word() == "END-CALL" {
                        let d = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = d;
                    }
                    Ok(bundle(v))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_through("END-CALL");
                    Stmt::Todo(format!("CALL: {e}"))
                })
            }
            "EXEC" => {
                let kind = self.up(1);
                self.skip_through("END-EXEC");
                Stmt::Todo(format!("EXEC {kind}(埋め込み SQL/CICS)は未対応"))
            }
            "OPEN" => {
                self.next();
                let mut out: Vec<Stmt> = vec![];
                let mut mode = String::new();
                while let Some(t) = self.peek() {
                    let w = t.s.to_uppercase();
                    if matches!(w.as_str(), "INPUT" | "OUTPUT" | "EXTEND" | "I-O") {
                        mode = match w.as_str() {
                            "INPUT" => "input",
                            "OUTPUT" => "output",
                            "EXTEND" => "extend",
                            _ => "io",
                        }
                        .into();
                        self.next();
                        continue;
                    }
                    if is_starter(&w) {
                        break;
                    }
                    let (n, d) = (ident(&t.s), t.dot);
                    self.next();
                    if self.file_named(&n).is_some() && !mode.is_empty() {
                        out.push(Stmt::Open(n, mode.clone()));
                    } else {
                        out.push(Stmt::Todo(format!("OPEN {n}: ファイル定義(SELECT)または入出力モードが無い")));
                    }
                    if d {
                        break;
                    }
                }
                if out.is_empty() {
                    out.push(Stmt::Todo("OPEN".into()));
                }
                bundle(out)
            }
            "CLOSE" => {
                self.next();
                let mut out: Vec<Stmt> = vec![];
                while let Some(t) = self.peek() {
                    if is_starter(&t.s) {
                        break;
                    }
                    let (n, d) = (ident(&t.s), t.dot);
                    self.next();
                    if self.file_named(&n).is_some() {
                        out.push(Stmt::Close(n));
                    } else {
                        out.push(Stmt::Todo(format!("CLOSE {n}: ファイル定義(SELECT)が無い")));
                    }
                    if d {
                        break;
                    }
                }
                if out.is_empty() {
                    out.push(Stmt::Todo("CLOSE".into()));
                }
                bundle(out)
            }
            "READ" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let f = ident(&self.next().ok_or("ファイル名が無い")?.s);
                    if self.file_named(&f).is_none() {
                        return Err(format!("{f}: ファイル定義(SELECT)が無い"));
                    }
                    if !self.ended && self.word() == "NEXT" {
                        self.next();
                    }
                    if !self.ended && self.word() == "RECORD" {
                        self.next();
                    }
                    let mut into = None;
                    if !self.ended && self.word() == "INTO" {
                        self.next();
                        let n = ident(&self.next().ok_or("INTO 先が無い")?.s);
                        self.used.insert(n.clone());
                        into = Some(n);
                    }
                    if !self.ended && matches!(self.word().as_str(), "INVALID" | "KEY") {
                        return Err("INVALID KEY(索引/相対ファイル)は未対応".into());
                    }
                    let (mut at_end, mut not_at_end) = (vec![], vec![]);
                    if !self.ended && self.word() == "AT" && self.up(1) == "END" {
                        self.next();
                        self.next();
                        at_end = self.block(true, &["NOT", "END-READ"]);
                    }
                    if !self.ended && self.word() == "NOT" && self.up(1) == "AT" && self.up(2) == "END" {
                        self.next();
                        self.next();
                        self.next();
                        not_at_end = self.block(true, &["END-READ"]);
                    }
                    if !self.ended && self.word() == "END-READ" {
                        let d = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = d;
                    }
                    Ok(Stmt::Read(f, into, at_end, not_at_end))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_through("END-READ");
                    Stmt::Todo(format!("READ: {e}"))
                })
            }
            "WRITE" => {
                self.next();
                let r: Result<Stmt, String> = (|| {
                    let rec = ident(&self.next().ok_or("レコード名が無い")?.s);
                    let f = self.files.iter().find(|f| f.record == rec).map(|f| f.name.clone()).ok_or_else(|| format!("{rec}: レコードが FD に無い"))?;
                    let mut from = None;
                    if !self.ended && self.word() == "FROM" {
                        self.next();
                        from = Some(self.operand()?);
                    }
                    let mut v = vec![Stmt::Write(f, from)];
                    if !self.ended && matches!(self.word().as_str(), "AFTER" | "BEFORE") {
                        self.skip_through("END-WRITE");
                        v.push(Stmt::Todo("WRITE の ADVANCING(行送り)は無視し改行のみ".into()));
                    } else if !self.ended && matches!(self.word().as_str(), "INVALID" | "AT" | "NOT") {
                        self.skip_through("END-WRITE");
                        v.push(Stmt::Todo("WRITE の INVALID KEY / EOP は未対応".into()));
                    } else if !self.ended && self.word() == "END-WRITE" {
                        let d = self.peek().map(|t| t.dot).unwrap_or(false);
                        self.next();
                        self.ended = d;
                    }
                    Ok(bundle(v))
                })();
                r.unwrap_or_else(|e| {
                    self.skip_through("END-WRITE");
                    Stmt::Todo(format!("WRITE: {e}"))
                })
            }
            "ELSE" | "END-IF" | "END-PERFORM" | "WHEN" | "END-EVALUATE" | "END-READ" | "END-CALL" | "END-STRING" | "END-UNSTRING" | "END-WRITE" | "END-SEARCH" | "END-ACCEPT" | "END-DISPLAY" | "END-COMPUTE" | "END-ADD" | "END-SUBTRACT" | "END-MULTIPLY" | "END-DIVIDE" | "END-EXEC" => {
                // 対応する開始が無い終端語: 読み飛ばす
                self.next();
                Stmt::Nop
            }
            other if is_starter(other) => {
                self.skip_to_end(&[]);
                Stmt::Todo(other.to_string())
            }
            _ => {
                // 段落名(文頭の語+ピリオド)は呼び出し側で処理済み。ここに来るのは未知の語
                self.next();
                Stmt::Todo(format!("未知の語 {}", t.s))
            }
        };
        let st = if matches!(u.as_str(), "ADD" | "SUBTRACT" | "MULTIPLY" | "DIVIDE" | "COMPUTE") && !matches!(st, Stmt::Todo(_)) { self.size_wrap(st, &u) } else { st };
        Some(st)
    }

    /// 算術文の後ろの [ON] SIZE ERROR ... [NOT [ON] SIZE ERROR ...] [END-xxx] を読み、SizeErr に包む
    fn size_wrap(&mut self, s: Stmt, verb: &str) -> Stmt {
        if self.ended {
            return s;
        }
        let on_size = |p: &P, k: usize| p.up(k) == "SIZE" || (p.up(k) == "ON" && p.up(k + 1) == "SIZE");
        let has_err = on_size(self, 0);
        let has_not = self.up(0) == "NOT" && on_size(self, 1);
        if !has_err && !has_not {
            return s;
        }
        let end = format!("END-{verb}");
        let terms: [&str; 2] = ["NOT", end.as_str()];
        let eat_size = |p: &mut P| {
            if p.word() == "ON" {
                p.next();
            }
            p.next(); // SIZE
            if p.word() == "ERROR" {
                p.next();
            }
        };
        let (mut err, mut ok) = (vec![], vec![]);
        if has_err {
            eat_size(self);
            err = self.block(true, &terms);
        }
        if self.up(0) == "NOT" && !self.ended {
            self.next();
            eat_size(self);
            ok = self.block(true, &terms);
        }
        if self.word() == end && !self.ended {
            self.next();
        }
        Stmt::SizeErr(Box::new(s), err, ok)
    }
}

/// 複数の文を1つの文として束ねる(1個ならそのまま)
fn bundle(v: Vec<Stmt>) -> Stmt {
    if v.len() == 1 {
        v.into_iter().next().unwrap()
    } else {
        Stmt::If(Cond::Cmp("==".into(), Expr::Num("1".into()), Expr::Num("1".into())), v, vec![])
    }
}

/// PIC 文字列 → (数値か, 桁数, 小数桁)
fn pic_info(pc: &str) -> (bool, usize, u32) {
    let u = pc.trim_end_matches('.').to_uppercase();
    let c: Vec<char> = u.chars().collect();
    let mut exp: Vec<char> = vec![];
    let mut i = 0;
    while i < c.len() {
        if c[i] == '(' {
            let mut j = i + 1;
            let mut n = 0usize;
            while j < c.len() && c[j] != ')' {
                n = n * 10 + c[j].to_digit(10).unwrap_or(0) as usize;
                j += 1;
            }
            if let Some(&last) = exp.last() {
                for _ in 1..n {
                    exp.push(last);
                }
            }
            i = j + 1;
        } else {
            exp.push(c[i]);
            i += 1;
        }
    }
    let numeric = u.starts_with('9') || u.starts_with("S9") || u.starts_with("V9");
    let (mut width, mut scale, mut after) = (0usize, 0u32, false);
    for ch in exp {
        match ch {
            'V' | '.' => after = true,
            'S' => {}
            _ => {
                width += 1;
                if after && numeric {
                    scale += 1;
                }
            }
        }
    }
    (numeric, width, scale)
}

fn level(t: &str) -> bool {
    t.len() == 2 && t.chars().all(|c| c.is_ascii_digit()) && (t == "77" || t.parse::<u8>().map(|n| (1..=49).contains(&n)).unwrap_or(false))
}

pub fn parse(src: &str, free: bool) -> Program {
    let text = strip(src, free).join(" ");
    let toks = tokenize(&text);
    let mut p = Program::default();
    let proc_at = (0..toks.len()).find(|&i| toks[i].s.to_uppercase() == "PROCEDURE" && toks.get(i + 1).map(|t| t.s.to_uppercase() == "DIVISION").unwrap_or(false));
    let head_end = proc_at.unwrap_or(toks.len());
    // データ部
    let mut i = 0;
    let mut cur_fd: Option<usize> = None;
    let mut last_var: Option<String> = None;
    let mut group_occurs: Option<(u8, usize)> = None;
    let mut filler_n = 0usize;
    let lit = |s: &str| -> Expr {
        let u = s.to_uppercase();
        match u.as_str() {
            "ZERO" | "ZEROS" | "ZEROES" => Expr::Num("0".into()),
            "SPACE" | "SPACES" => Expr::Str(String::new()),
            _ if s.starts_with('"') || s.starts_with('\'') => Expr::Str(s[1..s.len().saturating_sub(1)].to_string()),
            _ => Expr::Num(s.to_string()),
        }
    };
    while i < head_end {
        let u = toks[i].s.to_uppercase();
        // この項目の終わり(最初に文末ドットを持つトークン)
        let mut k = i;
        while k < head_end && !toks[k].dot {
            k += 1;
        }
        let end = (k + 1).min(head_end);
        if u == "PROGRAM-ID" {
            p.id = toks.get(i + 1).map(|t| t.s.clone()).unwrap_or_default();
            i += 2;
        } else if u == "SELECT" {
            let seg: Vec<String> = toks[i..end].iter().map(|t| t.s.to_uppercase()).collect();
            let raw: Vec<String> = toks[i..end].iter().map(|t| t.s.clone()).collect();
            let mut f = FileDef { name: ident(&raw.get(1).cloned().unwrap_or_default()), ..Default::default() };
            let mut x = 2;
            while x < seg.len() {
                match seg[x].as_str() {
                    "ASSIGN" => {
                        x += 1;
                        while x < raw.len() && matches!(seg[x].as_str(), "TO" | "DISK" | "EXTERNAL" | "DYNAMIC") {
                            x += 1;
                        }
                        if let Some(a) = raw.get(x) {
                            let a = a.trim_end_matches('.').to_string();
                            f.assign_lit = a.starts_with('"') || a.starts_with('\'');
                            f.assign = if f.assign_lit { a[1..a.len().saturating_sub(1)].to_string() } else { a };
                        }
                    }
                    "STATUS" if x > 0 && seg[x - 1] == "FILE" => {
                        x += 1;
                        if seg.get(x).map(|s| s == "IS").unwrap_or(false) {
                            x += 1;
                        }
                        f.status = raw.get(x).map(|s| ident(s.trim_end_matches('.')));
                    }
                    "INDEXED" | "RELATIVE" => p.notes.push(format!("SELECT {}: ORGANIZATION {} は未対応(順編成として扱う)", raw[1], seg[x])),
                    _ => {}
                }
                x += 1;
            }
            p.files.push(f);
            i = end;
        } else if u == "FD" {
            let n = ident(toks.get(i + 1).map(|t| t.s.trim_end_matches('.')).unwrap_or(""));
            cur_fd = p.files.iter().position(|f| f.name == n);
            if cur_fd.is_none() {
                p.notes.push(format!("FD {n}: 対応する SELECT が無い"));
            }
            i = end;
        } else if matches!(u.as_str(), "WORKING-STORAGE" | "LOCAL-STORAGE" | "LINKAGE") {
            cur_fd = None;
            i += 1;
        } else if u == "COPY" {
            p.notes.push(format!("COPY {} が展開されていない(コピー句が見つからない)", toks.get(i + 1).map(|t| t.s.as_str()).unwrap_or("")));
            i = end;
        } else if toks[i].s == "88" && !toks[i].dot {
            let name = ident(toks.get(i + 1).map(|t| t.s.trim_end_matches('.')).unwrap_or(""));
            let seg: Vec<&Tok> = toks[(i + 2).min(end)..end].iter().collect();
            let mut values: Vec<(Expr, Option<Expr>)> = vec![];
            let mut x = 0;
            while x < seg.len() {
                let w = seg[x].s.to_uppercase();
                if matches!(w.as_str(), "VALUE" | "VALUES" | "IS" | "ARE") {
                    x += 1;
                    continue;
                }
                let a = lit(seg[x].s.trim_end_matches('.'));
                if seg.get(x + 1).map(|t| matches!(t.s.to_uppercase().as_str(), "THRU" | "THROUGH")).unwrap_or(false) {
                    if let Some(b) = seg.get(x + 2) {
                        values.push((a, Some(lit(b.s.trim_end_matches('.')))));
                        x += 3;
                        continue;
                    }
                }
                values.push((a, None));
                x += 1;
            }
            match &last_var {
                Some(parent) if !values.is_empty() => p.conds.push(CondName { name, parent: parent.clone(), values }),
                _ => p.notes.push(format!("88 {name}: 親の項目または値が見つからない")),
            }
            i = end;
        } else if level(&toks[i].s) && !toks[i].dot {
            let lvl: u8 = toks[i].s.parse().unwrap_or(1);
            let name = toks.get(i + 1).map(|t| t.s.trim_end_matches('.').to_string()).unwrap_or_default();
            let seg: Vec<String> = if k >= i + 2 { toks[i + 2..end].iter().map(|t| t.s.clone()).collect() } else { vec![] };
            let mut pic = (false, 0usize, 0u32);
            let mut init = String::new();
            let mut has_pic = false;
            let mut occurs = 0usize;
            let mut redef: Option<String> = None;
            let mut x = 0;
            while x < seg.len() {
                match seg[x].to_uppercase().as_str() {
                    "PIC" | "PICTURE" => {
                        has_pic = true;
                        let mut pc = seg.get(x + 1).cloned().unwrap_or_default();
                        if pc.to_uppercase() == "IS" {
                            pc = seg.get(x + 2).cloned().unwrap_or_default();
                        }
                        let mut y = x + if seg.get(x + 1).map(|s| s.to_uppercase() == "IS").unwrap_or(false) { 3 } else { 2 };
                        loop {
                            if seg.get(y).map(|s| s == "(").unwrap_or(false) {
                                pc.push('(');
                                pc.push_str(seg.get(y + 1).map(|s| s.as_str()).unwrap_or(""));
                                pc.push(')');
                                y += 3;
                            } else if y > 0 && seg.get(y - 1).map(|s| s == ")").unwrap_or(false) && seg.get(y).map(|s| !s.is_empty() && s.chars().all(|c| "9XAVSZB.+-*$,/0".contains(c.to_ascii_uppercase()))).unwrap_or(false) {
                                pc.push_str(&seg[y]);
                                y += 1;
                            } else {
                                break;
                            }
                        }
                        pic = pic_info(&pc);
                    }
                    "VALUE" => {
                        let mut v = seg.get(x + 1).cloned().unwrap_or_default();
                        if v.to_uppercase() == "IS" {
                            v = seg.get(x + 2).cloned().unwrap_or_default();
                        }
                        init = match v.to_uppercase().as_str() {
                            "ZERO" | "ZEROS" | "ZEROES" => "0".into(),
                            "SPACE" | "SPACES" => "\"\"".into(),
                            _ => v.trim_end_matches('.').to_string(),
                        };
                    }
                    "OCCURS" => {
                        occurs = seg.get(x + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
                        if seg.iter().any(|s| s.to_uppercase() == "DEPENDING") {
                            p.notes.push(format!("OCCURS DEPENDING ON ({name}): 最大件数の固定長として扱う"));
                            if seg.get(x + 2).map(|s| s.to_uppercase() == "TO").unwrap_or(false) {
                                occurs = seg.get(x + 3).and_then(|s| s.parse().ok()).unwrap_or(occurs);
                            }
                        }
                    }
                    "REDEFINES" => redef = seg.get(x + 1).map(|s| s.trim_end_matches('.').to_string()),
                    _ => {}
                }
                x += 1;
            }
            if let Some((gl, _)) = group_occurs {
                if lvl <= gl {
                    group_occurs = None;
                }
            }
            if !has_pic && occurs > 0 {
                group_occurs = Some((lvl, occurs));
                occurs = 0;
            }
            if has_pic && occurs == 0 {
                if let Some((_, n)) = group_occurs {
                    occurs = n;
                }
            }
            let is_filler = name.to_uppercase() == "FILLER";
            if redef.is_some() && !has_pic {
                p.notes.push(format!("REDEFINES(群項目){name}: 別領域として扱っている(メモリの重ね合わせは再現しない)"));
            }
            if has_pic && (cur_fd.is_some() || !is_filler) {
                let (numeric, width, scale) = pic;
                let vname = if is_filler {
                    filler_n += 1;
                    format!("filler_{filler_n}")
                } else {
                    name.clone()
                };
                let mut aliased = false;
                if let Some(r) = &redef {
                    let rid = ident(r);
                    if let Some(t) = p.vars.iter().find(|v| ident(&v.name) == rid) {
                        if t.numeric == numeric && t.scale == scale && t.width == width && t.occurs == occurs {
                            p.alias.push((ident(&vname), rid));
                            aliased = true;
                        }
                    }
                    if !aliased {
                        p.notes.push(format!("REDEFINES {r} ({name}): 型や桁が違うため別領域として扱っている(メモリの重ね合わせは再現しない)"));
                    }
                }
                if !aliased && !p.vars.iter().any(|v| v.name == vname) {
                    p.vars.push(Var { name: vname.clone(), numeric, init, decimal: scale > 0, undeclared: false, scale, width, occurs });
                }
                last_var = Some(ident(&vname));
                if let Some(fi) = cur_fd {
                    if !aliased {
                        p.files[fi].fields.push(ident(&vname));
                    }
                }
            }
            if let Some(fi) = cur_fd {
                if lvl == 1 {
                    p.files[fi].record = ident(&name);
                }
            }
            i = end.max(i + 1);
        } else {
            i += 1;
        }
    }
    // 手続き部
    if let Some(pa) = proc_at {
        let mut st = P { t: toks[pa + 2..].to_vec(), i: 0, ended: false, used: BTreeSet::new(), subs: vec![], conds: p.conds.clone(), files: p.files.clone() };
        // PROCEDURE DIVISION [USING ...] . の見出しの残りを読み飛ばす
        if pa + 1 < toks.len() && !toks[pa + 1].dot {
            while let Some(t) = st.peek() {
                let d = t.dot;
                st.next();
                if d {
                    break;
                }
            }
        }
        st.ended = false;
        let mut cur: Option<(String, Vec<Stmt>)> = None;
        while st.peek().is_some() {
            // 段落見出し: 文頭の「語.」で、動詞ではないもの
            let t = st.peek().unwrap().clone();
            if t.dot && !is_starter(&t.s) && t.s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                if let Some(c) = cur.take() {
                    p.paragraphs.push(c);
                }
                st.next();
                st.ended = false;
                cur = Some((ident(&t.s), vec![]));
                continue;
            }
            st.ended = false;
            let before = st.i;
            let s = st.statement();
            if st.i == before {
                st.next();
            }
            if let Some(s) = s {
                match cur.as_mut() {
                    Some((_, v)) => v.push(s),
                    None => p.stmts.push(s),
                }
            }
        }
        if let Some(c) = cur.take() {
            p.paragraphs.push(c);
        }
        // 宣言の無い変数を足す(段落名は除く)
        let paras: Vec<String> = p.paragraphs.iter().map(|x| x.0.clone()).collect();
        p.subs = std::mem::take(&mut st.subs);
        let known: Vec<String> = p.conds.iter().map(|c| c.name.clone()).chain(p.files.iter().map(|f| f.name.clone())).chain(p.alias.iter().map(|a| a.0.clone())).collect();
        for u in st.used {
            if !p.vars.iter().any(|v| ident(&v.name) == u) && !paras.contains(&u) && !known.contains(&u) {
                p.vars.push(Var { name: u, numeric: true, undeclared: true, ..Default::default() });
            }
        }
    }
    p
}

// ---------------- 出力 ----------------

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    Py,
    Rs,
}

fn p10(k: u32) -> String {
    10i64.pow(k).to_string()
}

fn canon(p: &Program, n: &str) -> String {
    match p.alias.iter().find(|a| a.0 == n) {
        Some(a) => a.1.clone(),
        None => n.to_string(),
    }
}

/// 代入先文字列 → (名前, 添字番号, ROUNDED)
fn split_lv(t: &str) -> (String, Option<usize>, bool) {
    let rounded = t.contains('\u{1}');
    let t2 = t.replace('\u{1}', "");
    match t2.split_once('\0') {
        Some((n, k)) => (n.to_string(), k.parse().ok(), rounded),
        None => (t2, None, rounded),
    }
}

fn var_of<'a>(p: &'a Program, n: &str) -> Option<&'a Var> {
    let c = canon(p, n);
    p.vars.iter().find(|v| ident(&v.name) == c)
}

fn scale_of(p: &Program, n: &str) -> u32 {
    var_of(p, n).map(|v| v.scale).unwrap_or(0)
}

fn is_num(p: &Program, name: &str) -> bool {
    let (b, _, _) = split_lv(name);
    var_of(p, &b).map(|v| v.numeric).unwrap_or(false)
}

fn is_numeric(p: &Program, e: &Expr) -> bool {
    match e {
        Expr::Num(_) | Expr::Neg(_) | Expr::Bin(..) => true,
        Expr::Str(_) => false,
        Expr::Slice(..) => false,
        Expr::Var(v) | Expr::Idx(v, _) => {
            let (b, _, _) = split_lv(v);
            var_of(p, &b).map(|x| x.numeric).unwrap_or(true)
        }
        Expr::Func(n, _) => matches!(n.as_str(), "LENGTH" | "MAX" | "MIN" | "ABS" | "MOD" | "INTEGER" | "NUMVAL"),
    }
}

/// 数値リテラル → (整数コード, 小数桁)
fn lit_num(n: &str) -> (String, u32) {
    let (neg, body) = match n.strip_prefix('-') {
        Some(b) => (true, b),
        None => (false, n.strip_prefix('+').unwrap_or(n)),
    };
    let (ip, fp) = body.split_once('.').unwrap_or((body, ""));
    let digits = format!("{ip}{fp}");
    let t = digits.trim_start_matches('0');
    let t = if t.is_empty() { "0" } else { t };
    (format!("{}{}", if neg && t != "0" { "-" } else { "" }, t), fp.len() as u32)
}

fn rescale(code: &str, from: u32, to: u32, rounded: bool, l: Lang) -> String {
    if from == to {
        return code.to_string();
    }
    if to > from {
        return format!("({code} * {})", p10(to - from));
    }
    let d = p10(from - to);
    match (rounded, l) {
        (true, Lang::Py) => format!("_rdiv({code}, {d})"),
        (true, Lang::Rs) => format!("rdiv({code}, {d})"),
        (false, Lang::Py) => format!("_tdiv({code}, {d})"),
        (false, Lang::Rs) => format!("({code} / {d})"),
    }
}

fn disp(p: &Program, e: &Expr, l: Lang) -> String {
    let (c, s) = sx(p, e, l);
    if s > 0 && is_numeric(p, e) {
        match l {
            Lang::Py => format!("_fmt({c}, {s})"),
            Lang::Rs => format!("fmt_dec({c}, {s})"),
        }
    } else {
        c
    }
}

/// 式 → (コード, 小数桁)。数値は 10^桁 倍した整数。
fn sx(p: &Program, e: &Expr, l: Lang) -> (String, u32) {
    match e {
        Expr::Num(n) => lit_num(n),
        Expr::Str(s) => (format!("{s:?}"), 0),
        Expr::Var(v) => {
            let (b, _, _) = split_lv(v);
            (format!("self.{}", canon(p, &b)), scale_of(p, &b))
        }
        Expr::Idx(v, i) => {
            let (b, _, _) = split_lv(v);
            let (ic, _) = sx(p, i, l);
            let c = canon(p, &b);
            (
                match l {
                    Lang::Py => format!("self.{c}[({ic}) - 1]"),
                    Lang::Rs => format!("self.{c}[(({ic}) - 1) as usize]"),
                },
                scale_of(p, &b),
            )
        }
        Expr::Slice(v, a, len) => {
            let (b, _, _) = split_lv(v);
            let c = canon(p, &b);
            let (sa, _) = sx(p, a, l);
            let ln = len.as_ref().map(|x| sx(p, x, l).0);
            (
                match (l, ln) {
                    (Lang::Py, Some(n)) => format!("str(self.{c})[({sa}) - 1:({sa}) - 1 + ({n})]"),
                    (Lang::Py, None) => format!("str(self.{c})[({sa}) - 1:]"),
                    (Lang::Rs, Some(n)) => format!("format!(\"{{}}\", self.{c}).chars().skip((({sa}) - 1) as usize).take(({n}) as usize).collect::<String>()"),
                    (Lang::Rs, None) => format!("format!(\"{{}}\", self.{c}).chars().skip((({sa}) - 1) as usize).collect::<String>()"),
                },
                0,
            )
        }
        Expr::Neg(x) => {
            let (c, s) = sx(p, x, l);
            (format!("(-{c})"), s)
        }
        Expr::Func(n, args) => func(p, n, args, l),
        Expr::Bin(op, a, b) => {
            let (ca, sa) = sx(p, a, l);
            let (cb, sb) = sx(p, b, l);
            let both = is_numeric(p, a) && is_numeric(p, b);
            match op {
                '+' | '-' => {
                    if !both || sa == sb {
                        (format!("({ca} {op} {cb})"), sa.max(sb))
                    } else {
                        let s = sa.max(sb);
                        (format!("({} {op} {})", rescale(&ca, sa, s, false, l), rescale(&cb, sb, s, false, l)), s)
                    }
                }
                '*' => (format!("({ca} * {cb})"), sa + sb),
                '/' => {
                    if sa == 0 && sb == 0 {
                        match l {
                            Lang::Py => (format!("_tdiv({ca}, {cb})"), 0),
                            Lang::Rs => (format!("({ca} / {cb})"), 0),
                        }
                    } else {
                        let k = 4 + sb;
                        match l {
                            Lang::Py => (format!("_tdiv({ca} * {}, {cb})", p10(k)), sa + 4),
                            Lang::Rs => (format!("({ca} * {} / {cb})", p10(k)), sa + 4),
                        }
                    }
                }
                _ => {
                    let sc = match &**b {
                        Expr::Num(n) if sa > 0 => sa * n.parse::<u32>().unwrap_or(1),
                        _ => sa,
                    };
                    match l {
                        Lang::Py => (format!("({ca} ** {cb})"), sc),
                        Lang::Rs => (format!("({ca}).pow(({cb}) as u32)"), sc),
                    }
                }
            }
        }
    }
}

fn func(p: &Program, n: &str, args: &[Expr], l: Lang) -> (String, u32) {
    let a: Vec<(String, u32)> = args.iter().map(|x| sx(p, x, l)).collect();
    let a0 = a.first().cloned().unwrap_or(("0".into(), 0));
    let list = a.iter().map(|x| x.0.clone()).collect::<Vec<_>>().join(", ");
    let py = l == Lang::Py;
    match n {
        "LENGTH" => {
            if let Some(Expr::Var(v) | Expr::Idx(v, _)) = args.first() {
                let (b, _, _) = split_lv(v);
                if let Some(x) = var_of(p, &b) {
                    if x.width > 0 {
                        return (x.width.to_string(), 0);
                    }
                }
            }
            (if py { format!("len(str({}))", a0.0) } else { format!("(format!(\"{{}}\", {}).len() as i64)", a0.0) }, 0)
        }
        "UPPER-CASE" => (if py { format!("str({}).upper()", a0.0) } else { format!("format!(\"{{}}\", {}).to_uppercase()", a0.0) }, 0),
        "LOWER-CASE" => (if py { format!("str({}).lower()", a0.0) } else { format!("format!(\"{{}}\", {}).to_lowercase()", a0.0) }, 0),
        "REVERSE" => (if py { format!("str({})[::-1]", a0.0) } else { format!("format!(\"{{}}\", {}).chars().rev().collect::<String>()", a0.0) }, 0),
        "MAX" => (if py { format!("max({list})") } else { format!("*[{list}].iter().max().unwrap()") }, a0.1),
        "MIN" => (if py { format!("min({list})") } else { format!("*[{list}].iter().min().unwrap()") }, a0.1),
        "ABS" => (if py { format!("abs({})", a0.0) } else { format!("({}).abs()", a0.0) }, a0.1),
        "MOD" => {
            let b = a.get(1).map(|x| x.0.clone()).unwrap_or("1".into());
            (if py { format!("(({}) % ({b}))", a0.0) } else { format!("({}).rem_euclid({b})", a0.0) }, 0)
        }
        "NUMVAL" => (if py { format!("_num({})", a0.0) } else { format!("format!(\"{{}}\", {}).trim().parse::<i64>().unwrap_or(0)", a0.0) }, 0),
        _ => (a0.0, 0), // INTEGER ほか: 整数部のみ(引数をそのまま)
    }
}

fn cond(p: &Program, c: &Cond, l: Lang) -> String {
    match c {
        Cond::Cmp(o, a, b) => {
            let (ca, sa) = sx(p, a, l);
            let (cb, sb) = sx(p, b, l);
            if sa != sb && is_numeric(p, a) && is_numeric(p, b) {
                let s = sa.max(sb);
                format!("{} {o} {}", rescale(&ca, sa, s, false, l), rescale(&cb, sb, s, false, l))
            } else {
                format!("{ca} {o} {cb}")
            }
        }
        Cond::And(a, b) => format!("({} {} {})", cond(p, a, l), if l == Lang::Py { "and" } else { "&&" }, cond(p, b, l)),
        Cond::Or(a, b) => format!("({} {} {})", cond(p, a, l), if l == Lang::Py { "or" } else { "||" }, cond(p, b, l)),
        Cond::Not(a) => format!("{}({})", if l == Lang::Py { "not " } else { "!" }, cond(p, a, l)),
        Cond::Class(e, k) => {
            let (c, _) = sx(p, e, l);
            match (l, k.as_str()) {
                (Lang::Py, "NUMERIC") => format!("str({c}).lstrip(\"-\").isdigit()"),
                (Lang::Py, "ALPHABETIC-LOWER") => format!("str({c}).replace(\" \", \"\").islower()"),
                (Lang::Py, "ALPHABETIC-UPPER") => format!("str({c}).replace(\" \", \"\").isupper()"),
                (Lang::Py, _) => format!("str({c}).replace(\" \", \"\").isalpha()"),
                (Lang::Rs, "NUMERIC") => format!("{{ let s = format!(\"{{}}\", {c}); !s.is_empty() && s.trim_start_matches('-').chars().all(|ch| ch.is_ascii_digit()) }}"),
                (Lang::Rs, "ALPHABETIC-LOWER") => format!("format!(\"{{}}\", {c}).chars().all(|ch| ch.is_lowercase() || ch == ' ')"),
                (Lang::Rs, "ALPHABETIC-UPPER") => format!("format!(\"{{}}\", {c}).chars().all(|ch| ch.is_uppercase() || ch == ' ')"),
                (Lang::Rs, _) => format!("format!(\"{{}}\", {c}).chars().all(|ch| ch.is_alphabetic() || ch == ' ')"),
            }
        }
    }
}

/// 代入先の左辺(添字つきなら配列要素)
fn lv(p: &Program, t: &str, l: Lang) -> String {
    let (b, sub, _) = split_lv(t);
    let c = canon(p, &b);
    match sub.and_then(|k| p.subs.get(k)) {
        Some(ix) => {
            let (ic, _) = sx(p, ix, l);
            match l {
                Lang::Py => format!("self.{c}[({ic}) - 1]"),
                Lang::Rs => format!("self.{c}[(({ic}) - 1) as usize]"),
            }
        }
        None => format!("self.{c}"),
    }
}

/// e を代入先 t の型・桁へ合わせた右辺コード
fn val_for(p: &Program, t: &str, e: &Expr, l: Lang) -> String {
    let (b, _, rounded) = split_lv(t);
    let (c, s) = sx(p, e, l);
    let snum = is_numeric(p, e);
    if is_num(p, &b) {
        if snum {
            rescale(&c, s, scale_of(p, &b), rounded, l)
        } else {
            match l {
                Lang::Py => format!("_num({c})"),
                Lang::Rs => format!("{c}.trim().parse::<i64>().unwrap_or(0)"),
            }
        }
    } else {
        let d = if snum && s > 0 { disp(p, e, l) } else { c };
        match l {
            Lang::Py => d,
            Lang::Rs => format!("{d}.to_string()"),
        }
    }
}

/// 文字列値のコードを代入先 t の型へ合わせる
fn str_for(p: &Program, t: &str, s: String, l: Lang) -> String {
    let (b, _, _) = split_lv(t);
    if is_num(p, &b) {
        match l {
            Lang::Py => format!("_num({s})"),
            Lang::Rs => format!("{s}.trim().parse::<i64>().unwrap_or(0)"),
        }
    } else {
        s
    }
}

fn semi(l: Lang) -> &'static str {
    if l == Lang::Rs {
        ";"
    } else {
        ""
    }
}

/// 式の中の割り算の除数コードを集める(0除算の検出用)
fn divisors(p: &Program, e: &Expr, l: Lang, out: &mut Vec<String>) {
    match e {
        Expr::Bin(op, a, b) => {
            divisors(p, a, l, out);
            divisors(p, b, l, out);
            if *op == '/' {
                out.push(sx(p, b, l).0);
            }
        }
        Expr::Neg(x) => divisors(p, x, l, out),
        _ => {}
    }
}

/// ON SIZE ERROR: 結果が代入先の桁数を超える(または0で割る)ときは代入せず、エラー側の文を実行する
fn size_emit(p: &Program, inner: &Stmt, err: &[Stmt], ok: &[Stmt], l: Lang, ind: usize, o: &mut String) {
    let pad = "    ".repeat(ind);
    let py = l == Lang::Py;
    let (t, e, kind) = match inner {
        Stmt::Assign(t, e) => (t, e, 0),
        Stmt::AddTo(t, e) => (t, e, 1),
        Stmt::SubFrom(t, e) => (t, e, 2),
        Stmt::MulBy(t, e) => (t, e, 3),
        other => {
            // 複数の対象を持つ形などは、あふれの判定をせず元の文だけを出し、見えるように残す
            emit(p, std::slice::from_ref(other), l, ind, o);
            emit(p, &[Stmt::Todo("ON SIZE ERROR(複数の対象は未対応)".into())], l, ind, o);
            return;
        }
    };
    let (b, _, rounded) = split_lv(t);
    let tg = lv(p, t, l);
    let value = match kind {
        0 => val_for(p, t, e, l),
        1 => format!("{tg} + {}", val_for(p, t, e, l)),
        2 => format!("{tg} - {}", val_for(p, t, e, l)),
        _ => {
            let (c, sc) = sx(p, e, l);
            let ts = scale_of(p, &b);
            if ts == 0 && sc == 0 {
                format!("{tg} * {c}")
            } else {
                rescale(&format!("({tg} * {c})"), ts + sc, ts, rounded, l)
            }
        }
    };
    let width = var_of(p, &b).filter(|v| v.numeric && v.width > 0 && v.width <= 18).map(|v| v.width);
    let mut divs = vec![];
    divisors(p, e, l, &mut divs);
    let zero = divs.iter().map(|d| format!("{d} == 0")).collect::<Vec<_>>().join(if py { " or " } else { " || " });
    let over = |v: &str| match width {
        Some(w) => if py { format!("abs({v}) >= 10 ** {w}") } else { format!("{v}.abs() >= 10i64.pow({w})") },
        None => if py { "False".to_string() } else { "false".to_string() },
    };
    if py {
        o.push_str(&format!("{pad}_sv = 0\n"));
        if zero.is_empty() {
            o.push_str(&format!("{pad}_sv = {value}\n{pad}_se = {}\n", over("_sv")));
        } else {
            o.push_str(&format!("{pad}if {zero}:\n{pad}    _se = True\n{pad}else:\n{pad}    _sv = {value}\n{pad}    _se = {}\n", over("_sv")));
        }
        o.push_str(&format!("{pad}if _se:\n"));
        emit(p, err, l, ind + 1, o);
        o.push_str(&format!("{pad}else:\n{pad}    {tg} = _sv\n"));
        emit(p, ok, l, ind + 1, o);
    } else {
        o.push_str(&format!("{pad}let mut size_v: i64 = 0;\n"));
        let inner_calc = format!("{{ size_v = {value}; {} }}", over("size_v"));
        if zero.is_empty() {
            o.push_str(&format!("{pad}let size_e: bool = {inner_calc};\n"));
        } else {
            o.push_str(&format!("{pad}let size_e: bool = if {zero} {{ true }} else {inner_calc};\n"));
        }
        o.push_str(&format!("{pad}if size_e {{\n"));
        emit(p, err, l, ind + 1, o);
        o.push_str(&format!("{pad}}} else {{\n{pad}    {tg} = size_v;\n"));
        emit(p, ok, l, ind + 1, o);
        o.push_str(&format!("{pad}}}\n"));
    }
}

fn emit(p: &Program, ss: &[Stmt], l: Lang, ind: usize, o: &mut String) {
    let pad = "    ".repeat(ind);
    if ss.is_empty() {
        o.push_str(&format!("{pad}{}\n", if l == Lang::Py { "pass" } else { "" }));
        return;
    }
    let py = l == Lang::Py;
    for s in ss {
        match s {
            Stmt::Display(a) | Stmt::DisplayNoNl(a) => {
                let nl = matches!(s, Stmt::Display(_));
                let items: Vec<String> = a.iter().map(|e| disp(p, e, l)).collect();
                match l {
                    Lang::Py => o.push_str(&format!("{pad}self.out += \"\".join(str(x) for x in [{}]){}\n", items.join(", "), if nl { " + \"\\n\"" } else { "" })),
                    Lang::Rs => o.push_str(&format!("{pad}self.out += &format!(\"{}{}\", {});\n", "{}".repeat(a.len()), if nl { "\\n" } else { "" }, items.join(", "))),
                }
            }
            Stmt::Accept(t) => {
                let tg = lv(p, t, l);
                let src = if py { "_accept()".to_string() } else { "read_line()".to_string() };
                o.push_str(&format!("{pad}{tg} = {}{}\n", str_for(p, t, src, l), semi(l)));
            }
            Stmt::AcceptSys(t, kind) => {
                let tg = lv(p, t, l);
                let src = if py { format!("_sys({kind:?})") } else { format!("sys_val({kind:?})") };
                o.push_str(&format!("{pad}{tg} = {}{}\n", str_for(p, t, src, l), semi(l)));
            }
            Stmt::Move(src, tg) => {
                for t in tg {
                    o.push_str(&format!("{pad}{} = {}{}\n", lv(p, t, l), val_for(p, t, src, l), semi(l)));
                }
            }
            Stmt::Assign(t, e) => o.push_str(&format!("{pad}{} = {}{}\n", lv(p, t, l), val_for(p, t, e, l), semi(l))),
            Stmt::AddTo(t, e) | Stmt::SubFrom(t, e) => {
                let op = if matches!(s, Stmt::AddTo(..)) { "+=" } else { "-=" };
                o.push_str(&format!("{pad}{} {op} {}{}\n", lv(p, t, l), val_for(p, t, e, l), semi(l)));
            }
            Stmt::MulBy(t, e) => {
                let (b, _, rounded) = split_lv(t);
                let (c, sc) = sx(p, e, l);
                let ts = scale_of(p, &b);
                if ts == 0 && sc == 0 {
                    o.push_str(&format!("{pad}{} *= {c}{}\n", lv(p, t, l), semi(l)));
                } else {
                    let tg = lv(p, t, l);
                    o.push_str(&format!("{pad}{tg} = {}{}\n", rescale(&format!("({tg} * {c})"), ts + sc, ts, rounded, l), semi(l)));
                }
            }
            Stmt::If(Cond::Cmp(o1, Expr::Num(a), Expr::Num(b)), body, _) if o1 == "==" && a == "1" && b == "1" => {
                // 複数の文の束(ADD ... TO a b / GO TO ... DEPENDING など)
                emit(p, body, l, ind, o);
            }
            Stmt::If(c, th, el) => {
                match l {
                    Lang::Py => o.push_str(&format!("{pad}if {}:\n", cond(p, c, l))),
                    Lang::Rs => o.push_str(&format!("{pad}if {} {{\n", cond(p, c, l))),
                }
                emit(p, th, l, ind + 1, o);
                if !el.is_empty() {
                    o.push_str(&format!("{pad}{}\n", if py { "else:" } else { "} else {" }));
                    emit(p, el, l, ind + 1, o);
                }
                if l == Lang::Rs {
                    o.push_str(&format!("{pad}}}\n"));
                }
            }
            Stmt::Perform(lp, body) => match (lp, l) {
                (Loop::Once, _) => emit(p, body, l, ind, o),
                (Loop::Times(n), Lang::Py) => {
                    o.push_str(&format!("{pad}for _ in range({}):\n", sx(p, n, l).0));
                    emit(p, body, l, ind + 1, o);
                }
                (Loop::Times(n), Lang::Rs) => {
                    o.push_str(&format!("{pad}for _ in 0..({}) {{\n", sx(p, n, l).0));
                    emit(p, body, l, ind + 1, o);
                    o.push_str(&format!("{pad}}}\n"));
                }
                (Loop::Until(c), Lang::Py) => {
                    o.push_str(&format!("{pad}while not ({}):\n", cond(p, c, l)));
                    emit(p, body, l, ind + 1, o);
                }
                (Loop::Until(c), Lang::Rs) => {
                    o.push_str(&format!("{pad}while !({}) {{\n", cond(p, c, l)));
                    emit(p, body, l, ind + 1, o);
                    o.push_str(&format!("{pad}}}\n"));
                }
                (Loop::Varying { var, from, by, until }, Lang::Py) => {
                    o.push_str(&format!("{pad}{} = {}\n{pad}while not ({}):\n", lv(p, var, l), val_for(p, var, from, l), cond(p, until, l)));
                    emit(p, body, l, ind + 1, o);
                    o.push_str(&format!("{pad}    {} += {}\n", lv(p, var, l), val_for(p, var, by, l)));
                }
                (Loop::Varying { var, from, by, until }, Lang::Rs) => {
                    o.push_str(&format!("{pad}{} = {};\n{pad}while !({}) {{\n", lv(p, var, l), val_for(p, var, from, l), cond(p, until, l)));
                    emit(p, body, l, ind + 1, o);
                    o.push_str(&format!("{pad}    {} += {};\n{pad}}}\n", lv(p, var, l), val_for(p, var, by, l)));
                }
            },
            Stmt::Call(n) => match l {
                Lang::Py => o.push_str(&format!("{pad}_r = self.p_{n}()\n{pad}if _r:\n{pad}    return _r\n")),
                Lang::Rs => o.push_str(&format!("{pad}{{ let r = self.p_{n}(); if r != 0 {{ return r; }} }}\n")),
            },
            Stmt::Thru(a, b) => {
                let ia = p.paragraphs.iter().position(|x| &x.0 == a);
                let ib = p.paragraphs.iter().position(|x| &x.0 == b);
                match (ia, ib) {
                    (Some(x), Some(y)) if x <= y => {
                        let calls: Vec<Stmt> = p.paragraphs[x..=y].iter().map(|q| Stmt::Call(q.0.clone())).collect();
                        emit(p, &calls, l, ind, o);
                    }
                    _ => o.push_str(&format!("{pad}{} TODO(COBOL_to): PERFORM {a} THRU {b}: 段落が見つからない/順序が逆\n", if py { "#" } else { "//" })),
                }
            }
            Stmt::Goto(n) => match l {
                Lang::Py => o.push_str(&format!("{pad}self.goto = {n:?}\n{pad}return 2\n")),
                Lang::Rs => o.push_str(&format!("{pad}self.goto = {n:?}.to_string();\n{pad}return 2;\n")),
            },
            Stmt::Initialize(names) => {
                for n in names {
                    let (b, _, _) = split_lv(n);
                    match var_of(p, &b) {
                        Some(v) => {
                            let z = if v.numeric { Expr::Num("0".into()) } else { Expr::Str(String::new()) };
                            emit(p, &[Stmt::Move(z, vec![n.clone()])], l, ind, o);
                        }
                        None => o.push_str(&format!("{pad}{} TODO(COBOL_to): INITIALIZE {b}: 群項目は未対応\n", if py { "#" } else { "//" })),
                    }
                }
            }
            Stmt::Str(srcs, tg) => {
                let items: Vec<String> = srcs
                    .iter()
                    .map(|(e, d)| match (d, l) {
                        (None, Lang::Py) => format!("str({})", disp(p, e, l)),
                        (None, Lang::Rs) => format!("format!(\"{{}}\", {})", disp(p, e, l)),
                        (Some(d), Lang::Py) => format!("_upto(str({}), str({}))", disp(p, e, l), sx(p, d, l).0),
                        (Some(d), Lang::Rs) => format!("upto(&format!(\"{{}}\", {}), &format!(\"{{}}\", {}))", disp(p, e, l), sx(p, d, l).0),
                    })
                    .collect();
                let joined = if py { format!("\"\".join([{}])", items.join(", ")) } else { format!("[{}].concat()", items.join(", ")) };
                o.push_str(&format!("{pad}{} = {}{}\n", lv(p, tg, l), str_for(p, tg, joined, l), semi(l)));
            }
            Stmt::Unstring(src, d, all, tgs) => {
                let (cs, _) = sx(p, src, l);
                let (cd, _) = sx(p, d, l);
                match l {
                    Lang::Py => {
                        o.push_str(&format!("{pad}_u = str({cs}).split(str({cd}))\n"));
                        if *all {
                            o.push_str(&format!("{pad}_u = [x for x in _u if x]\n"));
                        }
                        for (k, t) in tgs.iter().enumerate() {
                            o.push_str(&format!("{pad}if len(_u) > {k}:\n{pad}    {} = {}\n", lv(p, t, l), str_for(p, t, format!("_u[{k}]"), l)));
                        }
                    }
                    Lang::Rs => {
                        o.push_str(&format!("{pad}{{\n{pad}    let ss = format!(\"{{}}\", {cs});\n{pad}    let dd = format!(\"{{}}\", {cd});\n"));
                        o.push_str(&format!("{pad}    let u: Vec<String> = ss.split(dd.as_str()).filter(|x| {}).map(|x| x.to_string()).collect();\n", if *all { "!x.is_empty()" } else { "true" }));
                        for (k, t) in tgs.iter().enumerate() {
                            o.push_str(&format!("{pad}    if u.len() > {k} {{ {} = {}; }}\n", lv(p, t, l), str_for(p, t, format!("u[{k}].clone()"), l)));
                        }
                        o.push_str(&format!("{pad}}}\n"));
                    }
                }
            }
            Stmt::Inspect(tg, op) => {
                let t = format!("self.{}", canon(p, tg));
                let s = |e: &Expr| sx(p, e, l).0;
                match (op, l) {
                    (Inspect::Tally(c, pat, lead), Lang::Py) => o.push_str(&format!("{pad}self.{} += {}\n", canon(p, c), tally_expr(true, &format!("str({t})"), pat.as_ref().map(|e| s(e)).as_deref(), *lead))),
                    (Inspect::Tally(c, pat, lead), Lang::Rs) => o.push_str(&format!("{pad}self.{} += {};\n", canon(p, c), tally_expr(false, &format!("format!(\"{{}}\", {t})"), pat.as_ref().map(|e| s(e)).as_deref(), *lead))),
                    (Inspect::Replace { lead: true, from, to, .. }, Lang::Py) => o.push_str(&format!("{pad}{t} = {}\n", replace_lead(true, &format!("str({t})"), &s(from), &s(to)))),
                    (Inspect::Replace { lead: true, from, to, .. }, Lang::Rs) => o.push_str(&format!("{pad}{t} = {};\n", replace_lead(false, &format!("{t}.clone()"), &s(from), &s(to)))),
                    (Inspect::Replace { first, from, to, .. }, Lang::Py) => o.push_str(&format!("{pad}{t} = str({t}).replace(str({}), str({}){})\n", s(from), s(to), if *first { ", 1" } else { "" })),
                    (Inspect::Replace { first, from, to, .. }, Lang::Rs) => o.push_str(&format!(
                        "{pad}{t} = {t}.{}(format!(\"{{}}\", {}).as_str(), format!(\"{{}}\", {}).as_str(){});\n",
                        if *first { "replacen" } else { "replace" },
                        s(from),
                        s(to),
                        if *first { ", 1" } else { "" }
                    )),
                    (Inspect::Convert(a, b), Lang::Py) => o.push_str(&format!("{pad}{t} = str({t}).translate(str.maketrans(str({}), str({})))\n", s(a), s(b))),
                    (Inspect::Convert(a, b), Lang::Rs) => o.push_str(&format!(
                        "{pad}{{ let a: Vec<char> = format!(\"{{}}\", {}).chars().collect(); let b: Vec<char> = format!(\"{{}}\", {}).chars().collect(); {t} = {t}.chars().map(|c| match a.iter().position(|x| *x == c) {{ Some(i) => b.get(i).copied().unwrap_or(c), None => c }}).collect(); }}\n",
                        s(a),
                        s(b)
                    )),
                }
            }
            Stmt::InspectScoped(tg, op, after, before) => {
                let t = format!("self.{}", canon(p, tg));
                let s = |e: &Expr| sx(p, e, l).0;
                if py {
                    o.push_str(&format!("{pad}_t = str({t}); _s = 0; _e = len(_t)\n"));
                    if let Some(a) = after {
                        o.push_str(&format!("{pad}_a = str({}); _i = _t.find(_a); _s = _i + len(_a) if _i >= 0 else len(_t)\n", s(a)));
                    }
                    if let Some(b) = before {
                        o.push_str(&format!("{pad}_j = _t.find(str({}), _s); _e = _j if _j >= 0 else len(_t)\n", s(b)));
                    }
                    o.push_str(&format!("{pad}_m = _t[_s:_e]\n"));
                    match op {
                        Inspect::Tally(c, pat, lead) => o.push_str(&format!("{pad}self.{} += {}\n", canon(p, c), tally_expr(true, "_m", pat.as_ref().map(|e| s(e)).as_deref(), *lead))),
                        Inspect::Replace { lead: true, from, to, .. } => {
                            o.push_str(&format!("{pad}_m = {}\n", replace_lead(true, "_m", &s(from), &s(to))));
                            o.push_str(&format!("{pad}{t} = _t[:_s] + _m + _t[_e:]\n"));
                        }
                        Inspect::Replace { first, from, to, .. } => {
                            o.push_str(&format!("{pad}_m = _m.replace(str({}), str({}){})\n", s(from), s(to), if *first { ", 1" } else { "" }));
                            o.push_str(&format!("{pad}{t} = _t[:_s] + _m + _t[_e:]\n"));
                        }
                        Inspect::Convert(a, b) => {
                            o.push_str(&format!("{pad}_m = _m.translate(str.maketrans(str({}), str({})))\n", s(a), s(b)));
                            o.push_str(&format!("{pad}{t} = _t[:_s] + _m + _t[_e:]\n"));
                        }
                    }
                } else {
                    o.push_str(&format!("{pad}{{\n{pad}    let t0 = format!(\"{{}}\", {t}); let mut s0 = 0usize; let mut e0 = t0.len();\n"));
                    if let Some(a) = after {
                        o.push_str(&format!("{pad}    let a0 = format!(\"{{}}\", {}); s0 = match t0.find(a0.as_str()) {{ Some(i) => i + a0.len(), None => t0.len() }};\n", s(a)));
                    }
                    if let Some(b) = before {
                        o.push_str(&format!("{pad}    let b0 = format!(\"{{}}\", {}); e0 = match t0[s0..].find(b0.as_str()) {{ Some(i) => s0 + i, None => t0.len() }};\n", s(b)));
                    }
                    o.push_str(&format!("{pad}    let m0 = &t0[s0..e0];\n"));
                    match op {
                        Inspect::Tally(c, pat, lead) => o.push_str(&format!("{pad}    self.{} += {};\n", canon(p, c), tally_expr(false, "m0.to_string()", pat.as_ref().map(|e| s(e)).as_deref(), *lead))),
                        Inspect::Replace { lead: true, from, to, .. } => o.push_str(&format!(
                            "{pad}    let m1 = {}; {t} = format!(\"{{}}{{}}{{}}\", &t0[..s0], m1, &t0[e0..]);\n",
                            replace_lead(false, "m0.to_string()", &s(from), &s(to))
                        )),
                        Inspect::Replace { first, from, to, .. } => o.push_str(&format!(
                            "{pad}    let m1 = m0.{}(format!(\"{{}}\", {}).as_str(), format!(\"{{}}\", {}).as_str(){}); {t} = format!(\"{{}}{{}}{{}}\", &t0[..s0], m1, &t0[e0..]);\n",
                            if *first { "replacen" } else { "replace" },
                            s(from),
                            s(to),
                            if *first { ", 1" } else { "" }
                        )),
                        Inspect::Convert(a, b) => o.push_str(&format!(
                            "{pad}    let a: Vec<char> = format!(\"{{}}\", {}).chars().collect(); let b: Vec<char> = format!(\"{{}}\", {}).chars().collect(); let m1: String = m0.chars().map(|c| match a.iter().position(|x| *x == c) {{ Some(i) => b.get(i).copied().unwrap_or(c), None => c }}).collect(); {t} = format!(\"{{}}{{}}{{}}\", &t0[..s0], m1, &t0[e0..]);\n",
                            s(a),
                            s(b)
                        )),
                    }
                    o.push_str(&format!("{pad}}}\n"));
                }
            }
            Stmt::CallExt(name, args) => match l {
                Lang::Py => o.push_str(&format!("{pad}self.call_ext({name:?}, [{}])\n", args.iter().map(|e| disp(p, e, l)).collect::<Vec<_>>().join(", "))),
                Lang::Rs => o.push_str(&format!(
                    "{pad}match self.ext {{ Some(f) => f({name:?}, vec![{}]), None => panic!(\"COBOL_to: CALL {name} は未登録(run_with で登録する)\") }}\n",
                    args.iter().map(|e| format!("format!(\"{{}}\", {})", disp(p, e, l))).collect::<Vec<_>>().join(", ")
                )),
            },
            Stmt::Open(f, mode) => match l {
                Lang::Py => o.push_str(&format!("{pad}self.f_open({f:?}, {mode:?})\n")),
                Lang::Rs => o.push_str(&format!("{pad}self.fo_{f}({mode:?});\n")),
            },
            Stmt::Close(f) => match l {
                Lang::Py => o.push_str(&format!("{pad}self.f_close({f:?})\n")),
                Lang::Rs => o.push_str(&format!("{pad}self.fc_{f}();\n")),
            },
            Stmt::Read(f, into, at_end, not_at_end) => {
                let mut ok: Vec<Stmt> = vec![];
                if let Some(i) = into {
                    match p.files.iter().find(|x| &x.name == f) {
                        Some(fd) if fd.fields.len() == 1 => ok.push(Stmt::Move(Expr::Var(fd.fields[0].clone()), vec![i.clone()])),
                        _ => ok.push(Stmt::Todo("READ INTO: 複数項目のレコードは未対応".into())),
                    }
                }
                ok.extend(not_at_end.iter().cloned());
                match l {
                    Lang::Py => {
                        o.push_str(&format!("{pad}if self.f_read({f:?}):\n"));
                        emit(p, &ok, l, ind + 1, o);
                        if !at_end.is_empty() {
                            o.push_str(&format!("{pad}else:\n"));
                            emit(p, at_end, l, ind + 1, o);
                        }
                    }
                    Lang::Rs => {
                        o.push_str(&format!("{pad}if self.rd_{f}() {{\n"));
                        emit(p, &ok, l, ind + 1, o);
                        o.push_str(&format!("{pad}}} else {{\n"));
                        emit(p, at_end, l, ind + 1, o);
                        o.push_str(&format!("{pad}}}\n"));
                    }
                }
            }
            Stmt::Write(f, from) => {
                if let Some(e) = from {
                    match p.files.iter().find(|x| &x.name == f) {
                        Some(fd) if fd.fields.len() == 1 => emit(p, &[Stmt::Move(e.clone(), vec![fd.fields[0].clone()])], l, ind, o),
                        _ => o.push_str(&format!("{pad}{} TODO(COBOL_to): WRITE FROM: 複数項目のレコードは未対応\n", if py { "#" } else { "//" })),
                    }
                }
                match l {
                    Lang::Py => o.push_str(&format!("{pad}self.f_write({f:?})\n")),
                    Lang::Rs => o.push_str(&format!("{pad}self.wr_{f}();\n")),
                }
            }
            Stmt::SizeErr(inner, err, ok) => size_emit(p, inner, err, ok, l, ind, o),
            Stmt::Stop => o.push_str(&match l {
                Lang::Py => format!("{pad}return 1\n"),
                Lang::Rs => format!("{pad}return 1;\n"),
            }),
            Stmt::Nop => {}
            Stmt::Todo(t) => o.push_str(&format!("{pad}{} TODO(COBOL_to): 未対応の文 {t}\n", if py { "#" } else { "//" })),
        }
    }
}

fn init_value(v: &Var) -> String {
    let base = if v.numeric {
        if v.init.is_empty() {
            "0".to_string()
        } else {
            let (code, s) = lit_num(&v.init);
            let n: i64 = code.parse().unwrap_or(0);
            if s <= v.scale {
                (n * 10i64.pow(v.scale - s)).to_string()
            } else {
                (n / 10i64.pow(s - v.scale)).to_string()
            }
        }
    } else if v.init.is_empty() {
        "\"\"".to_string()
    } else if v.init.starts_with('\'') {
        format!("{:?}", &v.init[1..v.init.len().saturating_sub(1)])
    } else {
        v.init.clone()
    };
    base
}

fn note_lines(p: &Program, c: &str) -> String {
    p.notes.iter().map(|n| format!("{c} TODO(COBOL_to): {n}\n")).collect()
}

fn order(p: &Program) -> Vec<String> {
    let mut v = vec!["_entry".to_string()];
    v.extend(p.paragraphs.iter().map(|x| x.0.clone()));
    v
}

pub fn to_python(p: &Program) -> String {
    let mut o = String::from("# COBOL_to generated(数値は 10^小数桁 倍した整数。割り算は 0 方向への切り捨て)\nimport os\n");
    o += &note_lines(p, "#");
    o += "\nEXTERNAL = {}  # CALL \"NAME\" の呼び先: EXTERNAL[\"NAME\"] = callable(*args)\n\n\n";
    o += "def _tdiv(a, b):\n    q = abs(a) // abs(b)\n    return q if (a < 0) == (b < 0) else -q\n\n\n";
    o += "def _rdiv(a, b):\n    q = (abs(a) * 2 + abs(b)) // (abs(b) * 2)\n    return q if (a < 0) == (b < 0) else -q\n\n\n";
    o += "def _fmt(v, s):\n    d = str(abs(v)).rjust(s + 1, \"0\")\n    return (\"-\" if v < 0 else \"\") + d[:-s] + \".\" + d[-s:]\n\n\n";
    o += "def _num(x):\n    try:\n        return int(str(x).strip())\n    except ValueError:\n        return 0\n\n\n";
    o += "def _accept():\n    try:\n        return input()\n    except EOFError:\n        return \"\"\n\n\n";
    o += "def _sys(kind):\n    import datetime as _d\n    n = _d.datetime.now(_d.timezone.utc)\n    if kind == 'DATE':\n        return n.strftime('%y%m%d')\n    if kind == 'DATE8':\n        return n.strftime('%Y%m%d')\n    if kind == 'DAY':\n        return n.strftime('%y%j')\n    if kind == 'DAY7':\n        return n.strftime('%Y%j')\n    if kind == 'DAY-OF-WEEK':\n        return str(n.isoweekday())\n    return n.strftime('%H%M%S') + '%02d' % (n.microsecond // 10000)\n\n\n";
    o += "def _upto(s, d):\n    return s.split(d)[0] if d else s\n\n\n";
    if !p.files.is_empty() {
        o += "_FILES = {\n";
        for f in &p.files {
            let path = if f.assign_lit { format!("{:?}", f.assign) } else { format!("os.environ.get({:?}, {:?})", f.assign.to_uppercase(), f.assign.to_lowercase()) };
            let fields: Vec<String> = f
                .fields
                .iter()
                .map(|n| {
                    let v = var_of(p, n);
                    format!("({:?}, {}, {})", n, v.map(|x| x.width.max(1)).unwrap_or(1), if v.map(|x| x.numeric).unwrap_or(false) { "True" } else { "False" })
                })
                .collect();
            o += &format!("    {:?}: {{\"path\": {path}, \"status\": {}, \"fields\": [{}]}},\n", f.name, f.status.as_ref().map(|s| format!("{s:?}")).unwrap_or("None".into()), fields.join(", "));
        }
        o += "}\n\n\n";
    }
    o += "class State:\n    def __init__(self):\n        self.out = \"\"\n        self.goto = \"\"\n        self._fh = {}\n";
    for v in &p.vars {
        let init = init_value(v);
        let val = if v.occurs > 0 { format!("[{init}] * {}", v.occurs) } else { init };
        o += &format!("        self.{} = {val}{}\n", ident(&v.name), if v.undeclared { "  # TODO(COBOL_to): 宣言が見つからない変数" } else { "" });
    }
    o += "\n    def call_ext(self, name, args):\n        f = EXTERNAL.get(name)\n        if f is None:\n            raise NotImplementedError(\"COBOL_to: CALL \" + name + \" は EXTERNAL に登録されていない\")\n        return f(*args)\n";
    if !p.files.is_empty() {
        o += "\n    def _fs(self, name, code):\n        st = _FILES[name][\"status\"]\n        if st:\n            setattr(self, st, int(code) if isinstance(getattr(self, st), int) else code)\n";
        o += "\n    def f_open(self, name, mode):\n        try:\n            self._fh[name] = open(_FILES[name][\"path\"], {\"input\": \"r\", \"output\": \"w\", \"extend\": \"a\", \"io\": \"r\"}[mode], encoding=\"utf-8\", newline=\"\")\n            self._fs(name, \"00\")\n        except OSError:\n            self._fs(name, \"35\")\n";
        o += "\n    def f_close(self, name):\n        fh = self._fh.pop(name, None)\n        if fh:\n            fh.close()\n";
        o += "\n    def f_read(self, name):\n        fh = self._fh.get(name)\n        line = fh.readline() if fh else \"\"\n        if not line:\n            self._fs(name, \"10\")\n            return False\n        line = line.rstrip(\"\\r\\n\")\n        pos = 0\n        for attr, w, num in _FILES[name][\"fields\"]:\n            seg = line[pos:pos + w]\n            pos += w\n            setattr(self, attr, _num(seg) if num else seg.rstrip())\n        self._fs(name, \"00\")\n        return True\n";
        o += "\n    def f_write(self, name):\n        fh = self._fh.get(name)\n        buf = \"\"\n        for attr, w, num in _FILES[name][\"fields\"]:\n            v = getattr(self, attr)\n            if num:\n                buf += (\"-\" + str(-v).rjust(w - 1, \"0\")) if v < 0 else str(v).rjust(w, \"0\")\n            else:\n                buf += str(v)[:w].ljust(w)\n        if fh:\n            fh.write(buf + \"\\n\")\n            self._fs(name, \"00\")\n";
    }
    let ord = order(p);
    o += &format!("\n    def main(self):\n        order = [{}]\n        i = 0\n        while i < len(order):\n            r = getattr(self, \"p_\" + order[i])()\n            if r == 1:\n                return 1\n            if r == 2:\n                if self.goto not in order:\n                    return 2\n                i = order.index(self.goto)\n                continue\n            i += 1\n        return 0\n", ord.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(", "));
    o += "\n    def p__entry(self):\n";
    emit(p, &p.stmts, Lang::Py, 2, &mut o);
    o += "        return 0\n";
    for (n, ss) in &p.paragraphs {
        o += &format!("\n    def p_{n}(self):\n");
        emit(p, ss, Lang::Py, 2, &mut o);
        o += "        return 0\n";
    }
    o + "\n\ndef run() -> str:\n    s = State()\n    s.main()\n    for f in list(s._fh.values()):\n        f.close()\n    return s.out\n"
}

pub fn to_rust(p: &Program) -> String {
    let mut o = String::from("// COBOL_to generated. 数値は i64(10^小数桁 倍した整数)、文字は String。\n#![allow(unreachable_code, unused_parens, unused_assignments, unused_mut, unused_variables, dead_code)]\n");
    o += &note_lines(p, "//");
    o += "\n/// CALL \"NAME\" の呼び先(引数は文字列化して渡す)\npub type Ext = fn(&str, Vec<String>);\n\n";
    o += "fn rdiv(a: i64, b: i64) -> i64 {\n    let q = (a.abs() * 2 + b.abs()) / (b.abs() * 2);\n    if (a < 0) == (b < 0) { q } else { -q }\n}\n\n";
    o += "fn fmt_dec(v: i64, s: u32) -> String {\n    let d = format!(\"{:0>w$}\", v.abs(), w = s as usize + 1);\n    let k = d.len() - s as usize;\n    format!(\"{}{}.{}\", if v < 0 { \"-\" } else { \"\" }, &d[..k], &d[k..])\n}\n\n";
    o += "fn pad_num(v: i64, w: usize) -> String {\n    if v < 0 { format!(\"-{:0>w$}\", -v, w = w.saturating_sub(1)) } else { format!(\"{:0>w$}\", v, w = w) }\n}\n\n";
    o += "fn pad_str(s: &str, w: usize) -> String {\n    let t: String = s.chars().take(w).collect();\n    format!(\"{:<w$}\", t, w = w)\n}\n\n";
    o += "fn upto(s: &str, d: &str) -> String {\n    if d.is_empty() { s.to_string() } else { s.split(d).next().unwrap_or(\"\").to_string() }\n}\n\n";
    o += "fn sys_val(kind: &str) -> String {\n    let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();\n    let secs = d.as_secs() as i64;\n    let days = secs.div_euclid(86400);\n    let rem = secs.rem_euclid(86400);\n    let z = days + 719468;\n    let era = z.div_euclid(146097);\n    let doe = z.rem_euclid(146097);\n    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;\n    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);\n    let mp = (5 * doy + 2) / 153;\n    let dd = doy - (153 * mp + 2) / 5 + 1;\n    let mm = if mp < 10 { mp + 3 } else { mp - 9 };\n    let yy = yoe + era * 400 + if mm <= 2 { 1 } else { 0 };\n    let leap = (yy % 4 == 0 && yy % 100 != 0) || yy % 400 == 0;\n    let cum = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];\n    let yday = cum[(mm - 1) as usize] + dd + if leap && mm > 2 { 1 } else { 0 };\n    match kind {\n        \"DATE\" => format!(\"{:02}{:02}{:02}\", yy % 100, mm, dd),\n        \"DATE8\" => format!(\"{:04}{:02}{:02}\", yy, mm, dd),\n        \"DAY\" => format!(\"{:02}{:03}\", yy % 100, yday),\n        \"DAY7\" => format!(\"{:04}{:03}\", yy, yday),\n        \"DAY-OF-WEEK\" => format!(\"{}\", (days + 3).rem_euclid(7) + 1),\n        _ => format!(\"{:02}{:02}{:02}{:02}\", rem / 3600, rem % 3600 / 60, rem % 60, d.subsec_millis() / 10),\n    }\n}\n\n";
    o += "fn read_line() -> String {\n    let mut s = String::new();\n    let _ = std::io::stdin().read_line(&mut s);\n    s.trim_end_matches(['\\r', '\\n']).to_string()\n}\n\n";
    o += "pub struct State {\n    out: String,\n    goto: String,\n    ext: Option<Ext>,\n";
    for v in &p.vars {
        let t = if v.numeric { "i64" } else { "String" };
        o += &format!("    {}: {},\n", ident(&v.name), if v.occurs > 0 { format!("Vec<{t}>") } else { t.to_string() });
    }
    for f in &p.files {
        o += &format!("    fr_{0}: Option<std::io::BufReader<std::fs::File>>,\n    fw_{0}: Option<std::fs::File>,\n", f.name);
    }
    o += "}\n\nimpl State {\n    fn new(ext: Option<Ext>) -> Self {\n        State {\n            out: String::new(),\n            goto: String::new(),\n            ext,\n";
    for v in &p.vars {
        let init = init_value(v);
        let one = if v.numeric { init } else { format!("{init}.to_string()") };
        let val = if v.occurs > 0 { format!("vec![{one}; {}]", v.occurs) } else { one };
        o += &format!("            {}: {val}, {}\n", ident(&v.name), if v.undeclared { "// TODO(COBOL_to): 宣言が見つからない変数" } else { "" });
    }
    for f in &p.files {
        o += &format!("            fr_{0}: None,\n            fw_{0}: None,\n", f.name);
    }
    o += "        }\n    }\n";
    let ord = order(p);
    o += "\n    fn main(&mut self) -> u8 {\n";
    o += &format!("        let order: &[&str] = &[{}];\n", ord.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(", "));
    o += "        let mut i = 0usize;\n        while i < order.len() {\n            let r = match order[i] {\n";
    for n in &ord {
        o += &format!("                {n:?} => self.p_{n}(),\n");
    }
    o += "                _ => 0,\n            };\n            if r == 1 {\n                return 1;\n            }\n            if r == 2 {\n                let g = self.goto.clone();\n                match order.iter().position(|x| *x == g) {\n                    Some(k) => {\n                        i = k;\n                        continue;\n                    }\n                    None => return 2,\n                }\n            }\n            i += 1;\n        }\n        0\n    }\n";
    o += "\n    fn p__entry(&mut self) -> u8 {\n";
    emit(p, &p.stmts, Lang::Rs, 2, &mut o);
    o += "        0\n    }\n";
    for (n, ss) in &p.paragraphs {
        o += &format!("\n    fn p_{n}(&mut self) -> u8 {{\n");
        emit(p, ss, Lang::Rs, 2, &mut o);
        o += "        0\n    }\n";
    }
    for f in &p.files {
        let n = &f.name;
        // 状態コード
        o += &format!("\n    fn fs_{n}(&mut self, code: &str) {{\n");
        match f.status.as_ref().map(|s| (s.clone(), var_of(p, s).map(|v| v.numeric).unwrap_or(false))) {
            Some((s, true)) => o += &format!("        self.{s} = code.parse().unwrap_or(0);\n"),
            Some((s, false)) => o += &format!("        self.{s} = code.to_string();\n"),
            None => o += "        let _ = code;\n",
        }
        o += "    }\n";
        let path = if f.assign_lit { format!("{:?}.to_string()", f.assign) } else { format!("std::env::var({:?}).unwrap_or_else(|_| {:?}.to_string())", f.assign.to_uppercase(), f.assign.to_lowercase()) };
        o += &format!("\n    fn fo_{n}(&mut self, mode: &str) {{\n        let path = {path};\n        if mode == \"input\" || mode == \"io\" {{\n            match std::fs::File::open(&path) {{\n                Ok(f) => {{ self.fr_{n} = Some(std::io::BufReader::new(f)); self.fs_{n}(\"00\"); }}\n                Err(_) => self.fs_{n}(\"35\"),\n            }}\n        }} else {{\n            match std::fs::OpenOptions::new().write(true).create(true).truncate(mode == \"output\").append(mode == \"extend\").open(&path) {{\n                Ok(f) => {{ self.fw_{n} = Some(f); self.fs_{n}(\"00\"); }}\n                Err(_) => self.fs_{n}(\"35\"),\n            }}\n        }}\n    }}\n");
        o += &format!("\n    fn fc_{n}(&mut self) {{\n        self.fr_{n} = None;\n        self.fw_{n} = None;\n    }}\n");
        o += &format!("\n    fn rd_{n}(&mut self) -> bool {{\n        use std::io::BufRead;\n        let mut line = String::new();\n        let got = match self.fr_{n}.as_mut() {{ Some(r) => r.read_line(&mut line).unwrap_or(0), None => 0 }};\n        if got == 0 {{\n            self.fs_{n}(\"10\");\n            return false;\n        }}\n        let chars: Vec<char> = line.trim_end_matches(['\\r', '\\n']).chars().collect();\n        let mut pos = 0usize;\n");
        for fld in &f.fields {
            let v = var_of(p, fld);
            let w = v.map(|x| x.width.max(1)).unwrap_or(1);
            let num = v.map(|x| x.numeric).unwrap_or(false);
            o += &format!("        {{\n            let seg: String = chars.iter().skip(pos).take({w}).collect();\n            pos += {w};\n            self.{fld} = {};\n        }}\n", if num { "seg.trim().parse::<i64>().unwrap_or(0)" } else { "seg.trim_end().to_string()" });
        }
        o += &format!("        self.fs_{n}(\"00\");\n        true\n    }}\n");
        o += &format!("\n    fn wr_{n}(&mut self) {{\n        use std::io::Write;\n        let mut s = String::new();\n");
        for fld in &f.fields {
            let v = var_of(p, fld);
            let w = v.map(|x| x.width.max(1)).unwrap_or(1);
            let num = v.map(|x| x.numeric).unwrap_or(false);
            o += &format!("        s += &{};\n", if num { format!("pad_num(self.{fld}, {w})") } else { format!("pad_str(&self.{fld}, {w})") });
        }
        o += &format!("        s.push('\\n');\n        if let Some(w) = self.fw_{n}.as_mut() {{\n            let _ = w.write_all(s.as_bytes());\n        }}\n        self.fs_{n}(\"00\");\n    }}\n");
    }
    o + "}\n\npub fn run() -> String {\n    run_with(None)\n}\n\npub fn run_with(ext: Option<Ext>) -> String {\n    let mut s = State::new(ext);\n    s.main();\n    s.out\n}\n"
}


// ---------------- COPY 展開 ----------------

/// `COPY name [REPLACING ==a== BY ==b== ...].` をコピー句の内容で置き換える(解析の前に呼ぶ)。
/// 探す場所は dirs(拡張子なし・.cpy・.cbl・.cob・.copy、大文字小文字は両方)。見つからない行はそのまま残す
/// (解析側が `TODO(COBOL_to)` として表示する)。
pub fn expand_copy(src: &str, free: bool, dirs: &[std::path::PathBuf]) -> String {
    expand_rec(src, free, dirs, 0)
}

fn expand_rec(src: &str, free: bool, dirs: &[std::path::PathBuf], depth: usize) -> String {
    if depth > 8 {
        return src.to_string();
    }
    let mut out = String::new();
    for line in src.lines() {
        let code: String = if free {
            line.trim_start().to_string()
        } else {
            let c: Vec<char> = line.chars().collect();
            if c.len() <= 7 || c[6] == '*' || c[6] == '/' {
                String::new()
            } else {
                c[7..c.len().min(72)].iter().collect::<String>().trim_start().to_string()
            }
        };
        let up = code.to_uppercase();
        if up.starts_with("COPY ") {
            let toks: Vec<&str> = code.split_whitespace().collect();
            let name = toks.get(1).map(|t| t.trim_end_matches('.').trim_matches(|c| c == '"' || c == '\'')).unwrap_or("");
            // REPLACING ==a== BY ==b==
            let mut reps: Vec<(String, String)> = vec![];
            if let Some(pos) = up.find("REPLACING") {
                let rest = code[pos + "REPLACING".len()..].trim_end().trim_end_matches('.');
                let parts: Vec<&str> = rest.split("==").collect();
                // parts: ["", a, " BY ", b, ""...]
                let mut k = 1;
                while k + 2 < parts.len() {
                    reps.push((parts[k].to_string(), parts[k + 2].to_string()));
                    k += 4;
                }
            }
            let mut found: Option<String> = None;
            'outer: for d in dirs {
                for nm in [name.to_string(), name.to_lowercase(), name.to_uppercase()] {
                    for ext in ["", ".cpy", ".CPY", ".cbl", ".CBL", ".cob", ".COB", ".copy", ".COPY"] {
                        let path = d.join(format!("{nm}{ext}"));
                        if path.is_file() {
                            if let Ok(b) = std::fs::read(&path) {
                                found = Some(String::from_utf8_lossy(&b).into_owned());
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if let Some(mut text) = found {
                for (a, b) in &reps {
                    text = text.replace(a.trim(), b.trim());
                }
                out.push_str(&expand_rec(&text, free, dirs, depth + 1));
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// `INSPECT ... TALLYING` の数え上げ式(`x` は文字列式、`pat` が None なら CHARACTERS)
fn tally_expr(py: bool, x: &str, pat: Option<&str>, lead: bool) -> String {
    match (py, pat, lead) {
        (true, None, _) => format!("len({x})"),
        (true, Some(p), false) => format!("{x}.count(str({p}))"),
        (true, Some(p), true) => format!("next((_k for _k in range(len({x}) // max(len(str({p})), 1) + 1) if not {x}.startswith(str({p}), _k * len(str({p})))), 0)"),
        (false, None, _) => format!("({x}).chars().count() as i64"),
        (false, Some(p), false) => format!("({x}).matches(format!(\"{{}}\", {p}).as_str()).count() as i64"),
        (false, Some(p), true) => format!("{{ let x0 = {x}; let p0 = format!(\"{{}}\", {p}); let mut n = 0i64; let mut i = 0usize; while !p0.is_empty() && x0[i..].starts_with(p0.as_str()) {{ n += 1; i += p0.len(); }} n }}"),
    }
}

/// `INSPECT ... REPLACING LEADING a BY b`: 先頭から連続する a を b に置き換えた文字列の式
fn replace_lead(py: bool, x: &str, from: &str, to: &str) -> String {
    if py {
        let n = tally_expr(true, x, Some(from), true);
        format!("(str({to}) * {n} + {x}[{n} * len(str({from})):])")
    } else {
        format!("{{ let x0 = {x}; let f0 = format!(\"{{}}\", {from}); let t0 = format!(\"{{}}\", {to}); let mut n = 0usize; let mut i = 0usize; while !f0.is_empty() && x0[i..].starts_with(f0.as_str()) {{ n += 1; i += f0.len(); }} format!(\"{{}}{{}}\", t0.repeat(n), &x0[i..]) }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello_roundtrip() {
        let src = include_str!("../tests/samples/hello.cbl");
        let p = parse(src, false);
        assert_eq!(p.id, "HELLO");
        assert_eq!(p.vars.len(), 2);
        let r = to_rust(&p);
        assert!(r.contains("format!(\"{}{}\\n\", \"HELLO \", self.ws_name)"), "{r}");
        assert!(r.contains("self.ws_count += 2;"), "{r}");
        assert!(to_python(&p).contains("self.ws_count += 2"));
    }
    const LOOP: &str = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. LOOPY.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 I PIC 9(3) VALUE 0.\n       01 TOTAL PIC 9(5) VALUE 0.\n       01 MSG PIC X(10).\n       PROCEDURE DIVISION.\n       MAIN-PARA.\n           PERFORM VARYING I FROM 1 BY 1 UNTIL I > 5\n               ADD I TO TOTAL\n           END-PERFORM.\n           COMPUTE TOTAL = TOTAL * 2 + (I - 1).\n           IF TOTAL > 20 AND NOT TOTAL = 99\n               MOVE \"BIG\" TO MSG\n           ELSE\n               MOVE \"SMALL\" TO MSG\n           END-IF.\n           PERFORM SHOW 2 TIMES.\n           STOP RUN.\n       SHOW.\n           DISPLAY \"TOTAL=\" TOTAL \" MSG=\" MSG.\n";
    #[test]
    fn structured_program() {
        let p = parse(LOOP, false);
        assert_eq!(p.paragraphs.len(), 2);
        let py = to_python(&p);
        assert!(py.contains("while not (self.i > 5):"), "{py}");
        assert!(py.contains("self.total = ((self.total * 2) + (self.i - 1))"), "{py}");
        assert!(py.contains("if (self.total > 20 and not (self.total == 99)):"), "{py}");
        assert!(py.contains("for _ in range(2):"), "{py}");
        let rs = to_rust(&p);
        assert!(rs.contains("while !(self.i > 5) {"), "{rs}");
        assert!(rs.contains("fn p_show(&mut self) -> u8"), "{rs}");
    }
    #[test]
    fn accept_from_date_time() {
        let src = "       PROGRAM-ID. D.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 D8 PIC 9(8).
       01 TM PIC 9(8).
       01 DW PIC 9.
       PROCEDURE DIVISION.
           ACCEPT D8 FROM DATE YYYYMMDD.
           ACCEPT TM FROM TIME.
           ACCEPT DW FROM DAY-OF-WEEK.
           STOP RUN.
";
        let p = parse(src, false);
        let py = to_python(&p);
        assert!(py.contains("_sys(\"DATE8\")"), "{py}");
        assert!(py.contains("_sys(\"TIME\")"), "{py}");
        assert!(py.contains("_sys(\"DAY-OF-WEEK\")"), "{py}");
        assert!(!py.contains("TODO(COBOL_to)"), "{py}");
        let rs = to_rust(&p);
        assert!(rs.contains("sys_val(\"DATE8\")"), "{rs}");
    }
    #[test]
    fn evaluate_arms() {
        let src = "       PROGRAM-ID. E.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 N PIC 9 VALUE 2.
       PROCEDURE DIVISION.
           EVALUATE N
               WHEN 1 DISPLAY \"ONE\"
               WHEN 2 WHEN 3 DISPLAY \"TWO-THREE\"
               WHEN OTHER DISPLAY \"X\"
           END-EVALUATE.
           STOP RUN.
";
        let py = to_python(&parse(src, false));
        assert!(py.contains("if self.n == 1:") && py.contains("elif") == false && py.contains("self.n == 3"), "{py}");
    }
    fn todo_free(src: &str) -> Program {
        let p = parse(src, false);
        let py = to_python(&p);
        assert!(!py.contains("未対応の文"), "{py}");
        p
    }
    #[test]
    fn continuation_line_pads_literal() {
        let src = "       PROGRAM-ID. C.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 X PIC X(80) VALUE \"ABC\n      -    \"DEF\".\n";
        let p = parse(src, false);
        assert_eq!(p.vars[0].init, format!("\"ABC{}DEF\"", " ".repeat(40)), "{:?}", p.vars[0].init);
    }
    #[test]
    fn decimals_are_scaled() {
        let src = "       PROGRAM-ID. D.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 P PIC 9(3)V99 VALUE 12.5.\n       01 T PIC 9(5)V9.\n       PROCEDURE DIVISION.\n           COMPUTE T ROUNDED = P * 3.\n           DISPLAY T.\n           STOP RUN.\n";
        let p = parse(src, false);
        assert_eq!(p.vars[0].scale, 2);
        assert_eq!(p.vars[0].width, 5);
        let py = to_python(&p);
        assert!(py.contains("self.p = 1250"), "{py}");
        assert!(py.contains("_rdiv((self.p * 3), 10)") || py.contains("_rdiv("), "{py}");
        assert!(py.contains("_fmt(self.t, 1)"), "{py}");
        let rs = to_rust(&p);
        assert!(rs.contains("rdiv("), "{rs}");
    }
    #[test]
    fn inspect_before_after_scope() {
        let src = "       PROGRAM-ID. I.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 T PIC X(9) VALUE \"AXBXCXDXE\".\n       01 N PIC 99 VALUE 0.\n       PROCEDURE DIVISION.\n           INSPECT T REPLACING ALL \"X\" BY \"-\" AFTER INITIAL \"B\" BEFORE INITIAL \"D\".\n           STOP RUN.\n";
        let p = parse(src, false);
        assert!(p.stmts.iter().any(|s| matches!(s, Stmt::InspectScoped(..))), "{:?}", p.stmts);
        assert!(to_python(&p).contains("_t.find(_a)"));
        assert!(to_rust(&p).contains("t0.find(a0.as_str())"));
    }

    #[test]
    fn inspect_leading_and_characters() {
        let src = "       PROGRAM-ID. I.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 T PIC X(9) VALUE \"000A00B00\".\n       01 N PIC 99 VALUE 0.\n       PROCEDURE DIVISION.\n           INSPECT T TALLYING N FOR LEADING \"0\".\n           INSPECT T TALLYING N FOR CHARACTERS.\n           INSPECT T REPLACING LEADING \"0\" BY \" \".\n           STOP RUN.\n";
        let p = parse(src, false);
        assert!(!p.stmts.iter().any(|s| matches!(s, Stmt::Todo(_))), "{:?}", p.stmts);
        let py = to_python(&p);
        assert!(py.contains("startswith") && py.contains("len(str("), "{py}");
        assert!(to_rust(&p).contains("starts_with"));
    }
    #[test]
    fn on_size_error_branches() {
        let src = "       PROGRAM-ID. S.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 A PIC 99 VALUE 90.\n       01 B PIC 99 VALUE 0.\n       PROCEDURE DIVISION.\n           ADD 20 TO A\n               ON SIZE ERROR DISPLAY \"OVER\"\n               NOT ON SIZE ERROR DISPLAY \"OK\"\n           END-ADD.\n           DIVIDE 10 BY B GIVING A\n               ON SIZE ERROR DISPLAY \"ZERO\"\n           END-DIVIDE.\n           DISPLAY A.\n           STOP RUN.\n";
        let p = parse(src, false);
        assert!(!p.stmts.iter().any(|s| matches!(s, Stmt::Todo(_))), "{:?}", p.stmts);
        assert_eq!(p.stmts.iter().filter(|s| matches!(s, Stmt::SizeErr(..))).count(), 2, "{:?}", p.stmts);
        let py = to_python(&p);
        assert!(py.contains("abs(_sv) >= 10 ** 2"), "{py}");
        assert!(py.contains("self.b == 0"), "{py}");
        let rs = to_rust(&p);
        assert!(rs.contains("size_v.abs() >= 10i64.pow(2)"), "{rs}");
    }
    #[test]
    fn occurs_goto_88_redefines_string() {
        let src = "       PROGRAM-ID. O.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 T PIC 9 OCCURS 3 TIMES.\n       01 I PIC 9 VALUE 1.\n       01 F PIC 9 VALUE 0.\n          88 ON1 VALUE 1.\n       01 A PIC 9(4).\n       01 B REDEFINES A PIC 9(4).\n       01 S PIC X(20).\n       PROCEDURE DIVISION.\n       M.\n           MOVE 5 TO T(I).\n           SET ON1 TO TRUE.\n           IF ON1 GO TO E.\n           STRING \"a\" DELIMITED BY SIZE \"b\" DELIMITED BY SIZE INTO S.\n       E.\n           STOP RUN.\n";
        let p = todo_free(src);
        assert!(p.alias.iter().any(|a| a.0 == "b" && a.1 == "a"), "{:?}", p.alias);
        assert_eq!(p.vars.iter().filter(|v| v.name == "B").count(), 0);
        let py = to_python(&p);
        assert!(py.contains("self.t[(self.i) - 1] = 5"), "{py}");
        assert!(py.contains("self.goto = \"e\""), "{py}");
        assert!(py.contains("self.t = [0] * 3"), "{py}");
    }
    #[test]
    fn file_io_definitions() {
        let src = include_str!("../tests/samples/fileio.cbl");
        let p = todo_free(src);
        assert_eq!(p.files.len(), 2);
        assert_eq!(p.files[0].fields, vec!["o_name", "o_amt"]);
        assert_eq!(p.files[0].status.as_deref(), Some("fs"));
        let rs = to_rust(&p);
        assert!(rs.contains("fn rd_in_file(&mut self) -> bool"), "{rs}");
    }
    #[test]
    fn copy_is_expanded() {
        let dir = std::env::temp_dir().join("cobol_to_copy_test");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("ITEMS.cpy"), "       01 PRICE PIC 9(3) VALUE 7.\n").unwrap();
        let src = "       PROGRAM-ID. X.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       COPY ITEMS.\n";
        let out = expand_copy(src, false, &[dir.clone()]);
        assert!(out.contains("01 PRICE PIC 9(3) VALUE 7."), "{out}");
        assert_eq!(parse(&out, false).vars.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn reference_modification_reads() {
        let src = "       PROGRAM-ID. R.\n       DATA DIVISION.\n       WORKING-STORAGE SECTION.\n       01 S PIC X(10) VALUE \"ABCDEFGH\".\n       01 K PIC 9 VALUE 3.\n       PROCEDURE DIVISION.\n           DISPLAY S(2:3) \"-\" S(K:2) \"-\" S(5:).\n           STOP RUN.\n";
        let p = parse(src, false);
        let py = to_python(&p);
        assert!(!py.contains("未対応の文"), "{py}");
        assert!(py.contains("str(self.s)[(2) - 1:(2) - 1 + (3)]"), "{py}");
        assert!(to_rust(&p).contains(".skip(((5) - 1) as usize).collect::<String>()"));
    }
    #[test]
    fn unsupported_stays_visible() {
        let src = "       PROGRAM-ID. T.\n       PROCEDURE DIVISION.\n           OPEN INPUT F.\n           STOP RUN.\n";
        let p = parse(src, false);
        assert!(to_python(&p).contains("TODO(COBOL_to): 未対応の文 OPEN"));
    }
}
