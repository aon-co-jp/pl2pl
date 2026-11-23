mod body;
mod endpoints;
mod frameworks;
mod lexer;
mod parse;
mod pygen;
mod rsblock;
mod rsgen;

use frameworks::{Found, Support};
use std::fs;
use std::path::{Path, PathBuf};

const USAGE: &str = "java_to — Java のフレームワークを自動判定し、FastAPI / MFastAPI / Poem / RPoem へ変換する

使い方:
  java_to scan <ディレクトリ>                                     判定結果(フレームワーク・ルート)を表示
  java_to convert <ディレクトリ> --target <fastapi|mfastapi|rust-poem|rpoem> --out <出力先> [--name <名前>] [--rpoem-path ../RPoem]
  java_to frameworks                                              判定できるフレームワークの台帳(2000年頃〜2026-11)
  java_to --version
";

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

pub struct Loaded {
    pub units: Vec<parse::Unit>,
    pub extra: Vec<(String, String)>,
    pub skipped: Vec<String>,
    pub openapi: Vec<endpoints::ApiOp>, // openapi.yml の操作(OpenAPI 先行のプロジェクト用)
}

fn load(dir: &Path) -> Loaded {
    let mut l = Loaded { units: vec![], extra: vec![], skipped: vec![], openapi: vec![] };
    let walker = walkdir::WalkDir::new(dir).into_iter().filter_entry(|e| {
        let n = e.file_name().to_string_lossy();
        !(e.file_type().is_dir() && matches!(n.as_ref(), "target" | "build" | "node_modules" | ".git" | ".gradle" | ".idea" | "out"))
    });
    for ent in walker.filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
        let p = ent.path();
        let name = p.to_string_lossy().to_string();
        let lower = name.to_lowercase();
        let Ok(bytes) = fs::read(p) else { continue };
        if bytes.len() > 4_000_000 {
            l.skipped.push(format!("{name}: 4MB を超えるため読みませんでした"));
            continue;
        }
        let text = String::from_utf8(bytes.clone()).unwrap_or_else(|_| {
            // Shift_JIS/EUC-JP など(2000年代の日本の資産に多い)は読めないので、ASCII 部分だけ拾う
            bytes.iter().map(|b| if b.is_ascii() { *b as char } else { '?' }).collect()
        });
        let base = lower.replace('\\', "/");
        let base = base.rsplit('/').next().unwrap_or("").to_string();
        if matches!(base.as_str(), "openapi.yml" | "openapi.yaml" | "swagger.yml" | "swagger.yaml") {
            l.openapi.extend(endpoints::parse_openapi(&text));
        }
        if lower.ends_with(".java") {
            l.units.push(parse::parse_unit(&name, &text));
        } else if lower.ends_with("pom.xml") || lower.ends_with(".gradle") || lower.ends_with(".gradle.kts") || lower.ends_with("web.xml") || lower.ends_with("struts.xml") || lower.ends_with("struts-config.xml") || lower.ends_with("faces-config.xml") || lower.replace('\\', "/").ends_with("conf/routes") || lower.ends_with("applicationcontext.xml") || lower.ends_with("persistence.xml") || lower.ends_with("seasar.dicon") || lower.ends_with(".dicon") {
            l.extra.push((name, text));
        } else if lower.ends_with(".groovy") || lower.ends_with(".kt") || lower.ends_with(".scala") {
            l.skipped.push(format!("{name}: Java 以外(Groovy/Kotlin/Scala)は対象外"));
        }
    }
    l
}

fn print_found(f: &[Found]) {
    println!("判定したフレームワーク/ライブラリ({} 件):", f.len());
    for x in f {
        let s = if x.support == Support::Convert { "変換対象" } else { "報告のみ" };
        println!("  [{s}] {} ({}年〜) → Python: {} / Rust: {}", x.name, x.since, x.python, x.rust);
        for e in &x.evidence {
            println!("        根拠: {e}");
        }
    }
}

fn report(src: &Path, target: &str, found: &[Found], eps: &[endpoints::Endpoint], warnings: &[String], skipped: &[String], todo: usize, files: usize) -> String {
    let mut r = String::new();
    r.push_str(&format!("# Java_to 変換レポート / Conversion report\n\n- 入力 / Input: `{}`\n- 変換先 / Target: `{}`\n- Java ファイル数 / Java files: {}\n- ルート数 / Routes: {}\n- 手作業が要る箇所 / TODO(Java_to) notes: {}\n\n", src.display(), target, files, eps.len(), todo));
    r.push_str("## 判定したフレームワーク / Detected frameworks\n\n| 名前 | 公開年(目安) | 扱い | Python での置き換え | Rust での置き換え | 根拠 |\n|---|---|---|---|---|---|\n");
    for x in found {
        r.push_str(&format!("| {} | {} | {} | {} | {} | {} |\n", x.name, x.since, if x.support == Support::Convert { "変換する / converted" } else { "報告のみ / report only" }, x.python, x.rust, x.evidence.first().cloned().unwrap_or_default().replace('|', "/")));
    }
    r.push_str("\n## ルート / Routes\n\n| 動詞 | パス | クラス.メソッド | フレームワーク | 引数 |\n|---|---|---|---|---|\n");
    for e in eps {
        let ps: Vec<String> = e.params.iter().map(|p| format!("{}:{:?}", p.java, p.kind)).collect();
        r.push_str(&format!("| {} | `{}` | {}.{} | {} | {} |\n", e.verb.to_uppercase(), e.path, e.class, e.method, e.framework, ps.join(", ")));
    }
    if !warnings.is_empty() || !skipped.is_empty() {
        r.push_str("\n## 注意・未対応 / Notes and unsupported\n\n");
        for w in warnings.iter().chain(skipped.iter()) {
            r.push_str(&format!("- {w}\n"));
        }
    }
    let report_only: Vec<&Found> = found.iter().filter(|f| f.support == Support::Report && matches!(f.kind, "web" | "view" | "soap" | "gui" | "mobile")).collect();
    if !report_only.is_empty() {
        r.push_str("\n## 変換しなかった画面・サーバー系 / Not converted (UI, SOAP, GUI)\n\n");
        for f in report_only {
            r.push_str(&format!("- {}: 画面・設定の移行は手作業(Python: {} / Rust: {})\n", f.name, f.python, f.rust));
        }
    }
    r
}

fn write_all(out: &Path, files: &[(String, String)]) -> std::io::Result<()> {
    for (n, c) in files {
        let p: PathBuf = out.join(n);
        if let Some(d) = p.parent() {
            fs::create_dir_all(d)?;
        }
        fs::write(p, c.replace("\r\n", "\n"))?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    match cmd {
        "--version" | "-V" => println!("java_to {}", env!("CARGO_PKG_VERSION")),
        "frameworks" => {
            println!("Java フレームワーク/ライブラリ台帳({} 件)", frameworks::CATALOG.len());
            for f in frameworks::CATALOG {
                println!("{:>5}  {:<10} {}  [{}]", f.since, f.kind, f.name, if f.support == Support::Convert { "変換" } else { "報告" });
            }
        }
        "scan" | "convert" => {
            let Some(dir) = args.get(2).filter(|a| !a.starts_with("--")) else {
                eprintln!("{USAGE}");
                std::process::exit(2);
            };
            let dir = PathBuf::from(dir);
            if !dir.is_dir() {
                eprintln!("java_to: ディレクトリではありません: {}", dir.display());
                std::process::exit(2);
            }
            let l = load(&dir);
            let found = frameworks::detect(&l.units, &l.extra);
            let mut warnings = vec![];
            let eps = endpoints::extract_with(&l.units, &mut warnings, &l.openapi);
            println!("Java ファイル {} 件、ルート {} 本", l.units.len(), eps.len());
            print_found(&found);
            for e in &eps {
                println!("  {:<6} {:<30} {}.{} ({})", e.verb.to_uppercase(), e.path, e.class, e.method, e.framework);
            }
            if cmd == "scan" {
                for w in warnings.iter().chain(l.skipped.iter()) {
                    println!("注意: {w}");
                }
                return;
            }
            let target = arg(&args, "--target").unwrap_or_else(|| "fastapi".into());
            let Some(out) = arg(&args, "--out") else {
                eprintln!("java_to: --out <出力先> が必要です\n{USAGE}");
                std::process::exit(2);
            };
            let out = PathBuf::from(out);
            let name = arg(&args, "--name").unwrap_or_else(|| "converted-app".into());
            let result = match target.as_str() {
                "fastapi" | "mfastapi" => pygen::generate(&pygen::Project { units: &l.units, endpoints: eps.clone() }, &target, &name),
                "rust-poem" | "poem" => rsgen::generate(&l.units, &eps, "poem", &name, ""),
                "rpoem" => rsgen::generate(&l.units, &eps, "rpoem", &name, &arg(&args, "--rpoem-path").unwrap_or_else(|| "../RPoem".into())),
                o => {
                    eprintln!("java_to: 不明な --target: {o}(fastapi|mfastapi|rust-poem|rpoem)");
                    std::process::exit(2);
                }
            };
            warnings.extend(result.warnings.iter().cloned());
            let mut files = result.files;
            let rep = report(&dir, &target, &found, &eps, &warnings, &l.skipped, result.todo_count, l.units.len());
            files.push(("JAVA_to-REPORT.md".into(), rep));
            if let Err(e) = write_all(&out, &files) {
                eprintln!("java_to: 書き出せません: {e}");
                std::process::exit(1);
            }
            println!("書き出しました: {}({} ファイル、手作業が要る箇所 {})", out.display(), files.len(), result.todo_count);
        }
        _ => {
            println!("{USAGE}");
        }
    }
}
