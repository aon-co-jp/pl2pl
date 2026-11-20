//! 設計思想の説明(日本語・English・中文(简/繁)・한국어・Deutsch・Русский・Français・Italiano・العربية・فارسی)。
//! 元データは `registry/info.tsv`(1行 = 言語¦話題¦見出し¦内容¦メリット¦デメリット)。
//! `pl2pl info [話題] [--lang 言語]` で表示、`pl2pl info --html` で画面(webpage/info.html)用の HTML を作る。
//! 注意や案内の文の最後に出す「詳しく見る」の入口は [`hint`]。

const DATA: &str = include_str!("../registry/info.tsv");

pub const LANGS: &[(&str, &str)] = &[
    ("ja", "日本語"),
    ("en", "English"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文(台灣)"),
    ("ko", "한국어"),
    ("de", "Deutsch"),
    ("ru", "Русский"),
    ("fr", "Français"),
    ("it", "Italiano"),
    ("ar", "العربية"),
    ("fa", "فارسی"),
];
pub const TOPICS: &[&str] = &["ui", "nojs", "json", "connect", "shared"];

#[derive(Clone, Debug)]
pub struct Entry {
    pub lang: String,
    pub topic: String,
    pub title: String,
    pub what: String,
    pub pros: String,
    pub cons: String,
}

pub fn entries() -> Vec<Entry> {
    DATA.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let f: Vec<&str> = l.trim_end_matches('¦').splitn(6, '¦').collect();
            (f.len() == 6).then(|| Entry { lang: f[0].into(), topic: f[1].into(), title: f[2].into(), what: f[3].into(), pros: f[4].into(), cons: f[5].trim_end_matches('¦').into() })
        })
        .collect()
}

pub fn find(lang: &str, topic: &str) -> Option<Entry> {
    entries().into_iter().find(|e| e.lang == lang && e.topic == topic)
}

fn labels(lang: &str) -> (&'static str, &'static str, &'static str) {
    match lang {
        "ja" => ("内容", "メリット", "デメリット"),
        "zh-CN" => ("内容", "优点", "缺点"),
        "zh-TW" => ("內容", "優點", "缺點"),
        "ko" => ("내용", "장점", "단점"),
        "de" => ("Inhalt", "Vorteile", "Nachteile"),
        "ru" => ("Суть", "Преимущества", "Недостатки"),
        "fr" => ("Principe", "Avantages", "Inconvénients"),
        "it" => ("Descrizione", "Vantaggi", "Svantaggi"),
        "ar" => ("الوصف", "المزايا", "العيوب"),
        "fa" => ("توضیح", "مزایا", "معایب"),
        _ => ("What", "Pros", "Cons"),
    }
}

/// 1 つの話題を、指定の言語でテキスト表示にする
pub fn render(lang: &str, topic: &str) -> Option<String> {
    let e = find(lang, topic)?;
    let (a, b, c) = labels(lang);
    Some(format!("■ {}\n{a}: {}\n{b}: {}\n{c}: {}\n", e.title, e.what, e.pros, e.cons))
}

/// 案内や注意の文の最後に付ける「詳しく見る」の入口(日本語と English)
pub fn hint(topic: &str) -> String {
    format!("詳しく(メリット・デメリット、11言語) / Details with pros and cons in 11 languages: pl2pl info {topic}  (例: pl2pl info {topic} --lang ko)")
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// 画面(Web ページ)用の HTML。話題ごとの見出しをクリックすると、言語を選んで詳細(内容・メリット・デメリット)が開く。JavaScript は使わず `<details>` だけ。
pub fn html() -> String {
    let mut h = String::from("<!doctype html>\n<html lang=\"ja\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>pl2pl 設計思想 / Design principles</title>\n<style>body{font-family:system-ui,sans-serif;max-width:52rem;margin:1rem auto;padding:0 1rem;line-height:1.6}details{border:1px solid #ccc;border-radius:.4rem;margin:.4rem 0;padding:.4rem .8rem}summary{cursor:pointer;font-weight:600}.pros{color:#0a6b2d}.cons{color:#a33}</style></head><body>\n<h1>pl2pl 設計思想 / Design principles</h1>\n<p>RPoRea・RPoem などの設計思想です。話題と言語を選ぶと、内容・メリット・デメリットが開きます。翻訳は AI が作成したもので、ネイティブの確認前です。 / The design principles behind RPoRea, RPoem and friends. Open a topic and a language for details, pros and cons. Translations are AI-made and not yet checked by native speakers.</p>\n");
    for t in TOPICS {
        let head = find("ja", t).map(|e| e.title).unwrap_or_default();
        let en = find("en", t).map(|e| e.title).unwrap_or_default();
        h.push_str(&format!("<details id=\"{t}\"><summary>{} / {}</summary>\n", esc(&head), esc(&en)));
        for (code, name) in LANGS {
            if let Some(e) = find(code, t) {
                let (a, b, c) = labels(code);
                let dir = if matches!(*code, "ar" | "fa") { " dir=\"rtl\"" } else { "" };
                h.push_str(&format!("<details lang=\"{code}\"{dir}><summary>{name}</summary><p><b>{}</b></p><p><b>{a}:</b> {}</p><p class=\"pros\"><b>{b}:</b> {}</p><p class=\"cons\"><b>{c}:</b> {}</p></details>\n", esc(&e.title), esc(&e.what), esc(&e.pros), esc(&e.cons)));
            }
        }
        h.push_str("</details>\n");
    }
    h.push_str("</body></html>\n");
    h
}

pub fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--html") {
        print!("{}", html());
        return 0;
    }
    let pos: Vec<&String> = args.iter().skip(2).take_while(|a| !a.starts_with("--")).collect();
    let lang = args.iter().position(|a| a == "--lang").and_then(|i| args.get(i + 1)).map(String::as_str).unwrap_or("ja");
    if !LANGS.iter().any(|(c, _)| *c == lang) {
        eprintln!("言語は {} から選んでください / choose a language from: {}", LANGS.iter().map(|l| l.0).collect::<Vec<_>>().join(" "), LANGS.iter().map(|l| l.0).collect::<Vec<_>>().join(" "));
        return 2;
    }
    let topics: Vec<&str> = match pos.first() {
        Some(t) if TOPICS.contains(&t.as_str()) => vec![t.as_str()],
        Some(t) => {
            eprintln!("話題は {} から選んでください / unknown topic \"{t}\"; choose from: {}", TOPICS.join(" | "), TOPICS.join(" | "));
            return 2;
        }
        None => TOPICS.to_vec(),
    };
    for t in topics {
        if let Some(s) = render(lang, t) {
            println!("{s}");
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_language_has_every_topic_with_all_fields() {
        for (code, _) in LANGS {
            for t in TOPICS {
                let e = find(code, t).unwrap_or_else(|| panic!("missing {code}/{t}"));
                assert!(!e.title.is_empty() && !e.what.is_empty() && !e.pros.is_empty() && !e.cons.is_empty(), "{code}/{t}");
            }
        }
    }
    #[test]
    fn html_has_all_and_no_script() {
        let h = html();
        assert!(h.contains("id=\"nojs\"") && h.contains("lang=\"fa\" dir=\"rtl\"") && !h.contains("<script"));
        assert!(render("ko", "json").unwrap().contains("RS-JSON"));
    }
}
