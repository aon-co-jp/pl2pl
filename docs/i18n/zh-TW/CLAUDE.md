> 說明:儲存庫根目錄的日文 CLAUDE.md 是原文。本文是 AI 翻譯,用簡單易懂的話寫成,尚未經母語人士審閱。

# pl2pl 開發筆記
- **任務**:在語言、框架、作業系統之間改寫程式碼的工具(用 Rust 撰寫,位於 `src/`)。來源語言清單見 `docs/LANGUAGES.md`,組合見 `docs/COMBINATIONS.md`。設計由擁有者在 `docs/DESIGN.md` 中一點一點地寫。**不要擅自替他們訂規格。**
- **方向**:在知名語言之間做「多對多」的改寫。不一次做完,先做選定的。網站只有說明和下載,改寫在 PC 應用程式裡進行。
- **規則**:改寫不了的地方留 `TODO` 並回報(不要偷偷刪掉)。要謹慎(可能有相依或副作用就不改)。真正執行過再說「完成」。不新增 REST。回覆只用日語。
- **共同的形式(IR)**:`src/ir.rs`。輸入端(PHP、Python、Java)和輸出端(JSON、OpenAPI、FastAPI)分開,方便增加新組合。沒找到路由的檔案要回報。輸出要真正檢查,例如用 `py_compile`。
- **PHP**:`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端對端測試:`tests/run-php.sh`(真實的 FastAPI、SQLite、Jinja2)。單元測試:`cargo test --release`。WSL 中:`wsl -d Ubuntu -- bash <檔案>`。
- **自動跟上**:`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。檢查指令為空的項目只顯示「未檢查」,絕不自動更新。通知透過 version-control 的 `--notify-file`。
- **部署**:介紹頁是 `webpage/index.html`。VPS 上是 `/root/easy-web.tokyo/pl2pl`(用 `git pull` 更新,是靜態頁面,無需重啟)、`pl2pl-web.service` 連接埠 8142,已寫入 `domains.toml`。
- **陷阱**:含日文的檔案用 Python `-X utf8` 寫入。shell 腳本保持 LF。Git Bash 會改寫 `/mnt/...` 路徑。引號或 `\` 很多的文字,用 Write / Edit 工具,不要用 heredoc。
- **COBOL_to / Java_to**:已併入 `converters/`。刪除原儲存庫無法復原,須先取得最終確認。
- **翻譯系統**(人類語言)與轉換器分開。轉換器不依賴 LLM。
- **文件語言**:日文檔案是原文。英文檔案用簡單的話重寫過。`docs/i18n/` 裡的其他語言是 AI 翻譯(尚未經母語人士審閱)。更早的經過見 `docs/HISTORY-ja.md`。

## v0.2.1 的新內容(2026-11-20)
- 在 `pl2pl web --backend` 中選 `rpoem` / `rporea` 時會顯示提示(日文和英文):RPoem 已包含 Tauri 相容、WunderGraph Cosmo 相容、tokio 和 WASM,因此不要另選;HTML5 和 CSS3 會用,但盡量少;依設計不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 加入的東西若已被包含,會被拒絕(結束碼 2);已包含但相容尚未成熟的,仍可選擇,並附上提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 透過執行探測測試來判斷成熟度(台帳:`registry/maturity.toml`)。aruaru-llm 的 AI 意見不會單獨改變判斷結果。
- 在 RPoem / RPoRea 以外的組合中,`--also nodejs`(或 `next.js`)會被接受並附上提示:我們不強加「不用 Node.js」的方針,但不推薦。會列出適合所選語言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js,以及我們尚未驗證的其他框架)。
- 一起開發的姊妹倉庫:Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一個有變動,當天檢查其他倉庫。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 以 11 種語言說明 RPoRea / RPoem 的設計思想及其優缺點。
- 原本的 `COBOL_to` / `Java_to` 保留(不刪除);`converters/` 中合併的副本輸出完全一致。
