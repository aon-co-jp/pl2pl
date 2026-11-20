> 說明:儲存庫根目錄的日文 PORTING.md 是原文。本文是 AI 翻譯,用簡單易懂的話寫成,尚未經母語人士審閱。

# pl2pl 交接
(更早的詳細紀錄在 `docs/HISTORY-ja.md`,日文。)

## 現在的狀態(2026-11-19)
- 已發布 v0.1.0,有 Windows、Linux、Mac 的安裝程式和 CI 測試。
- PHP → FastAPI 已用真實的 FastAPI 做過端對端測試:路由、Eloquent、查詢建構器、Blade、中介軟體、關聯和 `@can` 系列。
- `pl2pl web` 現在也能認出 PHP。
- `pl2pl watch` 發現新版本、測試、修復,實在不行就發通知。GitHub 每天晚上和早上也會用最新版本測試。
- `pl2pl ir`(試作)把 PHP、Python、Java(Spring、JAX-RS、Servlet、Spark)的路由變成共同的形式,輸出 JSON、OpenAPI 或 FastAPI 雛形。已確認輸出真的能被 Python 讀取。
- 介紹和下載頁面已上線:<https://easy-web.tokyo/pl2pl/>(在 VPS 上,靜態頁面)。
- COBOL_to 和 Java_to 已併入 `converters/`。

## 待辦(重要的在前)
1. **確認**:真實電子郵件是否送達(`version-control --test-notify`;這台電腦沒有 SMTP 設定)、透過 version-control `--loop` 自動修復、用最新的 fastapi 測試(GitHub 每晚的執行會確認)。
2. 把型別和函式內容放進 IR。讓現在的 FastAPI 輸出(`pl2pl php`)經過 IR。如果找路由時有遺漏,再考慮 tree-sitter。
3. 已用真正的新版本試過 `watch`:fastapi 0.165.0(starlette 1.10)下 `tests/run-php.sh` 通過 49 項,所以 registry 的已測版本升到 0.165。尚未完成:新的 laravel 13 / django 6.1 / spring-boot 4.1 仍為「未檢查」,因為通關測試不使用真正的框架(在做出使用真框架的測試前只回報)。
4. 在電腦上登記每晚和每早的自動執行(未做:這台電腦上檢查指令跑不起來,會誤報;範例見 `scripts/pl2pl-watch.cmd` 開頭)。
5. 在 easy-web.tokyo 的首頁加上 pl2pl 的介紹卡片。
6. PHP:用類別撰寫的 Blade 元件、`$attributes`、`@php`、`morphToMany`。
7. 與 aruaru-llm 整合(`llm setup` 的實際安裝未驗證)。Kokkos 的 GPU 執行;在 open-cuda / open-directx 上執行核心。
8. 刪除原本的 COBOL_to 和 Java_to(**無法復原,須先取得最終確認**)。
9. `docs/DESIGN.md` 由擁有者撰寫。確認 `docs/LANGUAGES.md` 沒有過時。

## 測試
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端對端)/ `cargo test --release`

## 重新開始用的訊息
「繼續 F:\pl2pl。從 PORTING.md 的『待辦』由上往下做。改寫不了的地方不要偷偷刪掉,留 TODO。真正執行確認後再說完成。回覆只用日語。每個階段都提交並 push,VPS 也要更新。」

## v0.2.1 的新內容(2026-11-20)
- 在 `pl2pl web --backend` 中選 `rpoem` / `rporea` 時會顯示提示(日文和英文):RPoem 已包含 Tauri 相容、WunderGraph Cosmo 相容、tokio 和 WASM,因此不要另選;HTML5 和 CSS3 會用,但盡量少;依設計不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 加入的東西若已被包含,會被拒絕(結束碼 2);已包含但相容尚未成熟的,仍可選擇,並附上提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 透過執行探測測試來判斷成熟度(台帳:`registry/maturity.toml`)。aruaru-llm 的 AI 意見不會單獨改變判斷結果。
- 在 RPoem / RPoRea 以外的組合中,`--also nodejs`(或 `next.js`)會被接受並附上提示:我們不強加「不用 Node.js」的方針,但不推薦。會列出適合所選語言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js,以及我們尚未驗證的其他框架)。
- 一起開發的姊妹倉庫:Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一個有變動,當天檢查其他倉庫。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 以 11 種語言說明 RPoRea / RPoem 的設計思想及其優缺點。
- 原本的 `COBOL_to` / `Java_to` 保留(不刪除);`converters/` 中合併的副本輸出完全一致。
