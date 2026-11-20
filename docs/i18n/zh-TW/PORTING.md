> 說明：儲存庫根目錄的日文 PORTING.md 是原文。本文用簡單易懂的話寫成，由 AI 翻譯，並經 AI 依母語水準審校語句是否自然；尚未經真人母語人士審閱。

# pl2pl 交接
(更早的詳細紀錄在 `docs/HISTORY-ja.md`，日文。)

## 現在的狀態(2026-11-19)
- 已發布 v0.1.0，有 Windows、Linux、Mac 的安裝程式和 CI 測試。
- PHP → FastAPI 已用真實的 FastAPI 做過端對端測試：路由、Eloquent、查詢建構器、Blade、中介軟體、關聯和 `@can` 系列。
- `pl2pl web` 現在也能認出 PHP。
- `pl2pl watch` 發現新版本、測試、修復，實在不行就發通知。GitHub 每天晚上和早上也會用最新版本測試。
- `pl2pl ir`(試作)把 PHP、Python、Java(Spring、JAX-RS、Servlet、Spark)的路由變成共同的形式，輸出 JSON、OpenAPI 或 FastAPI 雛形。IR 還帶有有型別的參數、函式本體、模型、中介軟體和範本，`pl2pl php` 現在透過 `src/ir_emit.rs` 裡的共同寫出器輸出(輸出與以前相同；49 項端到端檢查通過)。
- 介紹和下載頁面已上線：<https://easy-web.tokyo/pl2pl/>(在 VPS 上，靜態頁面)。easy-web.tokyo 的首頁上也有一張卡片。
- COBOL_to 和 Java_to 已併入 `converters/`。

## 待辦(重要的在前)
1. **確認**：真實電子郵件是否送達(`version-control --test-notify`；這台電腦沒有 SMTP 設定)、透過 version-control `--loop` 自動修復、用最新的 fastapi 測試(GitHub 每晚的執行會確認)。
2. 把型別和函式內容放進 IR。讓現在的 FastAPI 輸出(`pl2pl php`)經過 IR。如果找路由時有遺漏，再考慮 tree-sitter。
3. 已用真正的新版本試過 `watch`:fastapi 0.165.0(starlette 1.10)下 `tests/run-php.sh` 通過 49 項，所以 registry 的已測版本升到 0.165。尚未完成：新的 laravel 13 / django 6.1 / spring-boot 4.1 仍為「未檢查」，因為通關測試不使用真正的框架(在做出使用真框架的測試前只回報)。
4. 在電腦上登記每晚和每早的自動執行(未做：這台電腦上檢查指令跑不起來，會誤報；範例見 `scripts/pl2pl-watch.cmd` 開頭)。
5. (已完成)easy-web.tokyo 首頁上的 pl2pl 卡片已加好，並已在正式環境上線(open-easy-web `95cb1e7`；舊的執行檔是 `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`)。
6. PHP(已完成，有限制):Blade 的 `@php` 現在會把指派以及 `++` `--` `+=` `-=` `*=` `/=` `.=` 轉成 `{% set %}`;`$attributes` 可作為輸出，`->merge`(字串值或可轉換的運算式)、`->class([...])`(字串清單)、`->only` 和 `->except` 都能用；`morphToMany`(中介表裡有型別欄)；還有用類別撰寫的 Blade 元件(`app/View/Components/*.php`：建構函式參數變成預設值和 `_props`,`render()` 裡的 `view('x.y')` 也會跟進)(55 項端到端檢查，41 項單元測試)。尚未完成：元件類別的公開方法(範本裡的 `{{ $method() }}` 仍留 TODO，報告裡會統計數量)、帶鍵(有條件)的 `->class()`，以及含 if/foreach 或函式呼叫的 `@php`(仍留 TODO)。
7. 與 aruaru-llm 整合(已完成):2026-11-19 我們引入了 aruaru-llm v0.2.5，並實際執行了 `pl2pl llm setup`。自動選中的 3B 失敗了，因為它的權重被拆成多個分片(shard)，所以現在安裝會退回到小一級的模型(手動指定模型時除外)。`GenerateQwen` 用 qwen2.5-1.5b-instruct(約 2.9 GB,CPU 上)可以產生文字。**`pl2pl php --explain`**(選用；`--explain-max N`，預設 3)會在報告的每個 TODO 項下加一條 AI 說明。實測：在 CPU 上每項約 2 分鐘，而且**品質差，有時是錯的**(2 項裡有 1 項離題)，所以預設關閉。重新啟動後的 aruaru-llm 沒有選定模型(400)，所以會先選一個已安裝的。**更大的模型(約 12 GB)**：aruaru-llm 目錄裡最大的是 3B(約 6.2 GB)，而 3B 和 7B 是以多個分片檔發布的，現在的安裝程式下載不了。首先要有分片支援(在 aruaru-llm 裡)和 CPU 速度(1.5B 就已經這麼慢了)。Kokkos 的 GPU 執行，以及在 open-cuda / open-directx 上執行核心，仍未驗證。**GPU(2026-11-19 已確認)**：在 GeForce GT 730(2 GB,sm_35，驅動 475 = CUDA 11.4 等級)上，我們不需要管理員權限，透過 micromamba(NVIDIA 的 conda 頻道)裝上了 CUDA 11.8 的 nvcc，在真實 GPU 上執行了一個簡單的 CUDA 核心和 `pl2pl gpu --target open-cuda` 的輸出(`vec_loop12.cu`)，與 CPU 相比有 0 處不一致(`tests/gpu-check.ps1`)。注意點：MSVC 14.44 需要 `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` 和 `-allow-unsupported-compiler`;PyPI 上沒有 Windows 版 nvcc;CUDA 12 不能以 sm_35 為目標。**仍未驗證**：Kokkos 的 CUDA 後端，以及在 open-cuda(Rust 實作)和 open-directx 上執行。
8. 刪除原本的 COBOL_to 和 Java_to(**無法復原，須先取得最終確認**)。
9. `docs/DESIGN.md` 由擁有者撰寫。2026-11-19 我們重新讀了 `docs/LANGUAGES.md`，沒有發現與現況不符的地方(Java 和 COBOL 已有轉換器)。它仍是憑已有知識寫成的清單，沒有用最新的網路搜尋核對過。

## 測試
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端對端)/ `cargo test --release`

## 重新開始用的訊息
「繼續 F:\pl2pl。從 PORTING.md 的『待辦』由上往下做。改寫不了的地方不要偷偷刪掉，留 TODO。真正執行確認後再說完成。回覆只用日語。每個階段都提交並 push,VPS 也要更新。」

## v0.2.1 的新內容(2026-11-20)
- 在 `pl2pl web --backend` 中選 `rpoem` / `rporea` 時會顯示提示(日文和英文):RPoem 已包含 Tauri 相容、WunderGraph Cosmo 相容、tokio 和 WASM，因此不要另選；HTML5 和 CSS3 會用，但盡量少；依設計不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 加入的東西若已被包含，會被拒絕(結束碼 2)；已包含但相容尚未成熟的，仍可選擇，並附上提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 透過執行探測測試來判斷成熟度(台帳：`registry/maturity.toml`)。aruaru-llm 的 AI 意見不會單獨改變判斷結果。
- 在 RPoem / RPoRea 以外的組合中，`--also nodejs`(或 `next.js`)會被接受並附上提示：我們不強加「不用 Node.js」的方針，但不推薦。會列出適合所選語言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js，以及我們尚未驗證的其他框架)。
- 一起開發的姊妹倉庫：Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一個有變動，當天檢查其他倉庫。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 以 11 種語言說明 RPoRea / RPoem 的設計思想及其優缺點。
- 原本的 `COBOL_to` / `Java_to` 保留(不刪除);`converters/` 中合併的副本輸出完全一致。
