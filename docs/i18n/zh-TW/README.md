> 說明:儲存庫根目錄的日文 README 是原文。本文是 AI 翻譯,用簡單易懂的話寫成,尚未經母語人士審閱。

# pl2pl

## pl2pl 是什麼?
程式是用各種「語言」(程式語言)寫成的。
**pl2pl 是把用一種語言寫的程式,改寫成另一種語言的工具。**
例如,可以把用 PHP(Laravel)做的網站,變成 Python(FastAPI)的網站。
支援 Windows、Linux 和 Mac(v0.1.0,還在開發中)。

- 使用時要安裝到自己的電腦上。網站上只有說明和下載 → <https://easy-web.tokyo/pl2pl/>
- 改寫不了的地方,pl2pl 不會偷偷刪掉。它會留下 `TODO`(之後要做的事),並在報告裡寫明原因。
- 我們的目標是:讓知名的語言之間,盡可能多地互相改寫。

## 現在能做什麼
| 指令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。詳情見 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 查看資料夾,認出 Java、COBOL、PHP、Flask/Django,交給合適的工具 |
| `pl2pl ir` | (試作)把 PHP、Python、Java 的「路由」變成同一種形式,再產生 JSON、OpenAPI 或 FastAPI 的雛形 |
| `pl2pl watch` | 發現語言和工具的新版本,並自動跟上 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 讓 C/C++ 的迴圈跑得更快(gpu 為試作) |
| `pl2pl cport` | 讓 UNIX 用的 C 程式碼在 Windows 和 Linux 上都能執行 |
| `pl2pl pyhpc` | 用 Numba 讓 Python 的運算更快 |
| `pl2pl llm setup` | 安裝 AI 助手(aruaru-llm)。實際安裝還沒有驗證 |

## 自動跟上新版本
每天晚上和早上執行 `pl2pl watch`。
1. 檢查語言和工具有沒有新版本(清單在 `registry/ecosystem.toml`)。
2. 用新版本測試。通過的話,就自動更新「已支援的版本」。
3. 沒通過,就由 version-control(借助 AI)嘗試修復。
4. 還是修不好,就寄通知給已登記的電子郵件信箱和 LINE。

請注意:laravel、symfony、flask、django 等目前只會提醒「有新版本了」。
在 GitHub 上,每天晚上和早上也會自動用最新版本做測試。

## 使用方法
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 請知道
- pl2pl 很謹慎。看起來有風險的地方,會原樣保留並說明原因。我們不保證速度。
- 目前只在小型範例上測試過,還沒有在大型真實程式上試過。

## 接下來
- 把型別和函式的內容也放進共同的形式,讓 FastAPI 的轉換也經過它。
- PHP:用類別撰寫的 Blade 元件。
- 確認真實的電子郵件通知和 AI 自動修復。
- 確認在 GPU 上執行(open-cuda / open-directx)。

## v0.2.0 新功能
- 自由搭配:`pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`。COBOL 會被改寫;C、C++、Fortran 維持原樣執行,由網頁呼叫。
- `pl2pl c2rust`(簡單的 C 與 C++)和 `pl2pl f2rust`(簡單的 Fortran)可產生 Rust。`pl2pl cobol --os <linux|macos|unix|windows>` 產生 GnuCOBOL 的建置腳本。

## v0.2.1 的新內容(2026-11-20)
- 在 `pl2pl web --backend` 中選 `rpoem` / `rporea` 時會顯示提示(日文和英文):RPoem 已包含 Tauri 相容、WunderGraph Cosmo 相容、tokio 和 WASM,因此不要另選;HTML5 和 CSS3 會用,但盡量少;依設計不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 加入的東西若已被包含,會被拒絕(結束碼 2);已包含但相容尚未成熟的,仍可選擇,並附上提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 透過執行探測測試來判斷成熟度(台帳:`registry/maturity.toml`)。aruaru-llm 的 AI 意見不會單獨改變判斷結果。
- 在 RPoem / RPoRea 以外的組合中,`--also nodejs`(或 `next.js`)會被接受並附上提示:我們不強加「不用 Node.js」的方針,但不推薦。會列出適合所選語言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js,以及我們尚未驗證的其他框架)。
- 一起開發的姊妹倉庫:Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一個有變動,當天檢查其他倉庫。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 以 11 種語言說明 RPoRea / RPoem 的設計思想及其優缺點。
- 原本的 `COBOL_to` / `Java_to` 保留(不刪除);`converters/` 中合併的副本輸出完全一致。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(語言清單) / [COMBINATIONS.md](../../COMBINATIONS.md)(組合) / [DESIGN.md](../../DESIGN.md)(設計,由擁有者撰寫) / [components/README.md](../../../components/README.md)(附帶的元件)。這些檔案為日文。
