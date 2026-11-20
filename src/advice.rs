//! 組み合わせ選択の案内と注意(日本語 / English)。
//! (1) RPoem / RPoRea を選んだときの説明: Tauri 互換・WunderGraph Cosmo 互換・tokio + WASM、HTML5/CSS3 は最小限、JavaScript/TypeScript/Node.js/REST API は使わない設計。
//! (2) `--also` で足そうとした物が、選んだ物に既に含まれていれば「既に〇〇に含んでいますので、ご選択できません」と断る。

/// 名前の別名 → 正規名
fn canon(s: &str) -> Option<&'static str> {
    Some(match s.trim().to_lowercase().replace(['_', ' '], "-").as_str() {
        "tauri" => "tauri",
        "wundergraph-cosmo" | "wundergraph" | "cosmo" => "wundergraph-cosmo",
        "node" | "nodejs" | "node.js" => "nodejs",
        "next" | "nextjs" | "next.js" => "nextjs",
        "javascript" | "js" => "javascript",
        "typescript" | "ts" => "typescript",
        "rest" | "rest-api" | "restapi" => "rest",
        "tokio" => "tokio",
        "wasm" | "webassembly" => "wasm",
        "html5" | "html" => "html5",
        "css3" | "css" => "css3",
        "bootstrap" => "bootstrap",
        "poem" => "poem",
        "rpoem" => "rpoem",
        "reaxt" => "reaxt",
        "fastapi" => "fastapi",
        "python" => "python",
        "mojo" => "mojo",
        "open-cosmo" => "open-cosmo",
        "rust" => "rust",
        _ => return None,
    })
}

fn label(n: &str) -> &'static str {
    match n {
        "tauri" => "Tauri",
        "wundergraph-cosmo" => "WunderGraph Cosmo",
        "nodejs" => "Node.js",
        "nextjs" => "Next.js",
        "javascript" => "JavaScript",
        "typescript" => "TypeScript",
        "rest" => "REST API",
        "tokio" => "tokio",
        "wasm" => "WASM",
        "html5" => "HTML5",
        "css3" => "CSS3",
        "bootstrap" => "Bootstrap",
        "poem" => "Poem",
        "rpoem" => "RPoem",
        "reaxt" => "Reaxt",
        "fastapi" => "FastAPI",
        "python" => "Python",
        "mojo" => "Mojo",
        "open-cosmo" => "open-cosmo",
        _ => "Rust",
    }
}

/// 選んだ物(backend / frontend)が、すでに中に含んでいるもの
fn includes(choice: &str) -> &'static [&'static str] {
    match choice {
        "rust-axum" => &["rust", "tokio"],
        "rust-poem" => &["rust", "poem", "tokio"],
        // RPoem = Poem の Rust 版 + Tauri 互換 + WunderGraph Cosmo 互換(open-cosmo)+ tokio + WASM
        "rpoem" => &["rust", "poem", "rpoem", "tokio", "wasm", "tauri", "wundergraph-cosmo", "open-cosmo", "html5", "css3"],
        "rporea" => &["rust", "poem", "rpoem", "reaxt", "tokio", "wasm", "tauri", "wundergraph-cosmo", "open-cosmo", "html5", "css3"],
        "fastapi" => &["python", "fastapi"],
        "mfastapi" => &["python", "fastapi", "mojo"],
        "bootstrap-ts" => &["html5", "css3", "javascript", "typescript", "bootstrap"],
        "rust-wasm" => &["rust", "wasm", "html5", "css3"],
        _ => &[],
    }
}

/// 含んではいるが、互換の完成度がまだ高くない(部分対応)ので、別に選んでも構わない物。選ぶと注意だけ出す
fn partial(holder: &str, item: &str) -> bool {
    crate::maturity::is_partial(holder, item)
}

/// RPoem / RPoRea では設計上使わないもの
fn not_used(backend: &str) -> &'static [&'static str] {
    if matches!(backend, "rpoem" | "rporea") {
        &["nodejs", "nextjs", "javascript", "typescript", "rest"]
    } else {
        &[]
    }
}

/// RPoem / RPoRea を選んだときの案内(日本語と English)
pub fn backend_notice(backend: &str) -> Option<String> {
    if !matches!(backend, "rpoem" | "rporea") {
        return None;
    }
    let name = if backend == "rporea" { "RPoRea(RPoem + Reaxt)" } else { "RPoem" };
    Some(format!(
        "【{name} について / About {name}】\n\
         日本語: RPoem は Poem の Rust 版で、Tauri 互換・WunderGraph Cosmo 互換の機能と、tokio、WASM を含んでいます。ただし互換の完成度はまだ高くないため、Tauri や WunderGraph Cosmo を別に選ぶこともできます(その場合は注意を表示します)。画面は HTML5 と CSS3 を使いますが、使う量はなるべく少なくします。JavaScript・TypeScript・Node.js・REST API は、設計上、使いません。\n\
         English: RPoem is the Rust edition of Poem. It already includes Tauri-compatible and WunderGraph Cosmo-compatible features, plus tokio and WASM, but that compatibility is not yet complete, so you may still choose Tauri or WunderGraph Cosmo separately (a caution is shown if you do). The UI uses HTML5 and CSS3, kept as small as possible. By design, JavaScript, TypeScript, Node.js and REST APIs are not used.\n{}\n{}\n{}\n",
        crate::info::hint("ui"), crate::info::hint("nojs"), crate::info::hint("connect")
    ))
}


/// Node.js(Next.js)を選んだときに添える「代わりになりうる物」。選んだ言語(backend)に合わせた当方の物を先に、他の言語の物を「このようなものもございます」として添える。
/// 当方の物は、リポジトリの記載・実装で確認できた範囲だけ書く。他の言語の物は一般に知られた例で、当方では未検証。
pub fn node_alternatives(backend: &str) -> String {
    let (ja_own, en_own) = match backend {
        "rust-axum" | "rust-poem" | "rpoem" | "rporea" => (
            "【選んだ言語(Rust)の代わり】RPoem / RPoRea + Reaxt: 画面は Rust から WASM(wasm-bindgen-cli)にするため Node.js が要りません。`open-runo-view`(React 相当の仮想 DOM と SSR)、`open-runo-crossui`(Windows・macOS・Linux・Android・Web 向けの画面を1つの Rust から生成)、通信は Connect / gRPC-web(REST や Express が要りません)。",
            "[For your language (Rust)] RPoem / RPoRea + Reaxt: the UI is built from Rust to WASM (wasm-bindgen-cli), so Node.js is not needed. `open-runo-view` (a React-like virtual DOM with SSR), `open-runo-crossui` (generates UIs for Windows, macOS, Linux, Android and Web from one Rust codebase), and Connect / gRPC-web for communication (no REST or Express needed).",
        ),
        "fastapi" | "mfastapi" => (
            "【選んだ言語(Python)の代わり】FastAPI / MFastAPI は静的ファイルの配信(Starlette)を内蔵しており、Jinja2 のテンプレートで HTML を返せるので、画面のビルドに Node.js は要りません(この変換の出力も `web/` を FastAPI が配信します)。",
            "[For your language (Python)] FastAPI / MFastAPI serve static files (Starlette) out of the box and can return HTML from Jinja2 templates, so the UI needs no Node.js build (this converter's output also serves `web/` from FastAPI).",
        ),
        _ => ("", ""),
    };
    let ja_other = "【他の言語では、このようなフレームワークもございます】Rust: Leptos・Dioxus・Yew(Rust から WASM)/ Python: htmx + Jinja2、NiceGUI / Go: templ + htmx / C#: Blazor(WASM)/ Elixir: Phoenix LiveView。いずれも Node.js を使わずに画面を作れる例ですが、当方では未検証です。";
    let en_other = "[Frameworks in other languages] Rust: Leptos, Dioxus, Yew (Rust to WASM) / Python: htmx with Jinja2, NiceGUI / Go: templ with htmx / C#: Blazor (WASM) / Elixir: Phoenix LiveView. These are examples that build a UI without Node.js, but we have not verified them.";
    // Node.js 互換が必要な場合の、Rust 製の実行環境(開発中。確認できた範囲は RS-Node.js の README に記載)
    let ja_rn = "【Node.js 互換が必要な場合】RNode.js(RS-Node.js): Rust 製の Node.js 互換の実行環境で、JavaScript は Rust 製エンジン Boa、TypeScript は RS-TypeScript で動かします。本物の Node.js と出力を比べて互換を確かめながら開発中で、対応範囲はまだ限られます(console・process・require の path / fs など)。";
    let en_rn = "[If you need Node.js compatibility] RNode.js (RS-Node.js): a Rust runtime aiming at Node.js compatibility; JavaScript runs on the Rust engine Boa and TypeScript through RS-TypeScript. It is under development and checked against real Node.js output, so its coverage is still limited (console, process, and require of path / fs, for now).";
    format!("{ja_own}\n{en_own}\n{ja_rn}\n{en_rn}\n{ja_other}\n{en_other}")
}

/// `--also tauri,nodejs` のような追加の選択を調べる。戻り値: 断る理由(日本語と English)の一覧。空なら全部受け付け。
pub fn check_also(backend: &str, frontend: &str, also: &str) -> (Vec<String>, Vec<String>) {
    let (mut out, mut notes) = (vec![], vec![]);
    for raw in also.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let Some(c) = canon(raw) else {
            out.push(format!("「{raw}」は選択肢にありません。/ \"{raw}\" is not a known option."));
            continue;
        };
        let l = label(c);
        let holder = [backend, frontend].into_iter().find(|ch| includes(ch).contains(&c));
        if let Some(h) = holder.filter(|h| partial(h, c)) {
            let hl = if h == "rporea" { "RPoRea" } else { "RPoem" };
            notes.push(format!("注意: {hl} は {l} 互換の機能を含んでいますが、互換の完成度はまだ高くないため、別に選ぶこともできます。選んだ場合は、重複や動作の違いにご注意ください。/ Note: {hl} includes {l}-compatible features, but the compatibility is not yet complete, so you may still select {l} separately. If you do, please watch for overlap and behavioural differences."));
        } else if let Some(h) = holder {
            let hl = match h {
                "rpoem" => "RPoem",
                "rporea" => "RPoRea",
                "rust-axum" => "Axum",
                "rust-poem" => "Poem",
                "fastapi" => "FastAPI",
                "mfastapi" => "MFastAPI",
                "bootstrap-ts" => "Bootstrap + TypeScript",
                _ => "Rust + WASM",
            };
            out.push(format!("既に {hl} に {l} を含んでいますので、ご選択できません。/ {l} is already included in {hl}, so it cannot be selected separately."));
        } else if not_used(backend).contains(&c) {
            out.push(format!("{l} は、設計上、使用しない仕様ですので、ご選択できません。/ {l} is not used by design, so it cannot be selected."));
        } else if matches!(c, "nodejs" | "nextjs") {
            // 方針を押し付けない: 選べる。ただし推薦はしない、と伝える
            notes.push(format!("お知らせ: {l} を選択されました。私たちは「Node.js を使わない」方針を利用者に押し付けませんので、そのままお使いいただけますが、推薦はしておりません。/ Notice: you selected {l}. We do not impose our no-Node.js policy on you, so you may use it as you wish, but we do not recommend it.\n{}\n{}", node_alternatives(backend), crate::info::hint("nojs")));
        }
    }
    (out, notes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rpoem_notice_mentions_everything() {
        let n = backend_notice("rpoem").unwrap();
        for k in ["Tauri", "WunderGraph Cosmo", "tokio", "WASM", "HTML5", "CSS3", "Node.js", "REST"] {
            assert!(n.contains(k), "{k}");
        }
        assert!(backend_notice("fastapi").is_none());
    }
    #[test]
    fn included_is_declined() {
        let (r, n) = check_also("rporea", "rust-wasm", "tauri,cosmo,wasm,tokio");
        assert_eq!(r.len(), 2); // 構造上いっしょに選べない物だけ断る
        assert!(r[0].contains("既に RPoRea に WASM を含んでいますので、ご選択できません"));
        assert!(r[1].contains("tokio is already included in RPoRea"));
        assert_eq!(n.len(), 2); // 互換が部分的な Tauri / Cosmo は選べて、注意だけ
        assert!(n[0].contains("Tauri") && n[1].contains("not yet complete"));
    }
    #[test]
    fn not_used_by_design() {
        let (r, _) = check_also("rpoem", "rust-wasm", "nodejs,rest,typescript,js");
        assert_eq!(r.len(), 4);
        assert!(r.iter().all(|x| x.contains("設計上") && x.contains("not used by design")));
        let (r2, n2) = check_also("fastapi", "bootstrap-ts", "nodejs,next.js");
        assert!(n2[0].contains("Python") && n2[0].contains("Leptos"));
        assert!(r2.is_empty() && n2.len() == 2 && n2[0].contains("推薦はしておりません") && n2[1].contains("do not recommend"));
    }
    #[test]
    fn mfastapi_includes_fastapi_and_unknown() {
        let (r, _) = check_also("mfastapi", "bootstrap-ts", "fastapi,mojo,bootstrap,foo");
        assert_eq!(r.len(), 4);
        assert!(r[3].contains("foo"));
        assert!(check_also("rust-axum", "none", "tauri").0.is_empty());
    }
}
