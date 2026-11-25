> 說明：儲存庫根目錄的日文 README.md 是原文。本文用簡單易懂的話寫成，由 AI 翻譯，並經 AI 依母語水準審校語句是否自然；尚未經真人母語人士確認。

# pl2pl

## pl2pl 是什麼?
程式是用各種「語言」(程式語言)寫成的。
**pl2pl 是把用一種語言寫的程式，改寫成另一種語言的工具。**
例如，可以把用 PHP(Laravel)做的網站，變成 Python(FastAPI)的網站。
支援 Windows、Linux 和 Mac(v0.1.0，還在開發中)。

- 使用時要安裝到自己的電腦上。網站上只有說明和下載 → <https://easy-web.tokyo/pl2pl/>
- 改寫不了的地方，pl2pl 不會偷偷刪掉。它會留下 `TODO`(之後要做的事)，並在報告裡寫明原因。
- 我們的目標是：讓知名的語言之間，盡可能多地互相改寫。

## 現在能做什麼
| 指令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。詳情見 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 查看資料夾，認出 Java、COBOL、PHP、Flask/Django，交給合適的工具 |
| `pl2pl ir` | (試作)把 PHP、Python、Java 的「路由」變成同一種形式，再產生 JSON、OpenAPI 或 FastAPI 的雛形。PHP 的轉換也會先把型別、函式本體、模型、中介軟體和範本放進這種形式，再寫出結果 |
| `pl2pl watch` | 發現語言和工具的新版本，並自動跟上 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 讓 C/C++ 的迴圈跑得更快(gpu 為試作) |
| `pl2pl cport` | 讓 UNIX 用的 C 程式碼在 Windows 和 Linux 上都能執行 |
| `pl2pl pyhpc` | 用 Numba 讓 Python 的運算更快 |
| `pl2pl llm setup` | 安裝 AI 助手(aruaru-llm)。實際安裝還沒有驗證 |

## 自動跟上新版本
每天晚上和早上執行 `pl2pl watch`。
1. 檢查語言和工具有沒有新版本(清單在 `registry/ecosystem.toml`)。
2. 用新版本測試。通過的話，就自動更新「已支援的版本」。
3. 沒通過，就由 version-control(借助 AI)嘗試修復。
4. 還是修不好，就寄通知給已登記的電子郵件信箱和 LINE。

請注意：laravel、symfony、flask、django 等目前只會提醒「有新版本了」。
在 GitHub 上，每天晚上和早上也會自動用最新版本做測試。

## 使用方法
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 請知道
- pl2pl 很謹慎。看起來有風險的地方，會原樣保留並說明原因。我們不保證速度。
- 目前只在小型範例上測試過，還沒有在大型真實程式上試過。

## 接下來
- 把型別和函式的內容也放進共同的形式，讓 FastAPI 的轉換也經過它。
- PHP：用類別撰寫的 Blade 元件。
- 確認真實的電子郵件通知和 AI 自動修復。
- 確認在 GPU 上執行(open-cuda / open-directx)。

## v0.2.0 新功能
- 自由搭配：`pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`。COBOL 會被改寫；C、C++、Fortran 維持原樣執行，由網頁呼叫。
- `pl2pl c2rust`(簡單的 C 與 C++)和 `pl2pl f2rust`(簡單的 Fortran)可產生 Rust。`pl2pl cobol --os <linux|macos|unix|windows>` 產生 GnuCOBOL 的建置腳本。

## v0.2.1 的新內容(2026-11-20)
- 在 `pl2pl web --backend` 中選 `rpoem` / `rporea` 時會顯示提示(日文和英文):RPoem 已包含 Tauri 相容、WunderGraph Cosmo 相容、tokio 和 WASM，因此不要另選；HTML5 和 CSS3 會用，但盡量少；依設計不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 加入的東西若已被包含，會被拒絕(結束碼 2)；已包含但相容尚未成熟的，仍可選擇，並附上提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 透過執行探測測試來判斷成熟度(台帳：`registry/maturity.toml`)。aruaru-llm 的 AI 意見不會單獨改變判斷結果。
- 在 RPoem / RPoRea 以外的組合中，`--also nodejs`(或 `next.js`)會被接受並附上提示：我們不強加「不用 Node.js」的方針，但不推薦。會列出適合所選語言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js，以及我們尚未驗證的其他框架)。
- 一起開發的姊妹倉庫：Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一個有變動，當天檢查其他倉庫。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 以 11 種語言說明 RPoRea / RPoem 的設計思想及其優缺點。
- 原本的 `COBOL_to` / `Java_to` 保留(不刪除);`converters/` 中合併的副本輸出完全一致。

## 新增內容(2026-11-22 至 2026-11-23)
- COBOL(`converters/cobol_to`):新增 `SORT`(`USING` / `GIVING`、`INPUT` / `OUTPUT PROCEDURE`、`RELEASE`、`RETURN`)、`MERGE`、`SEARCH` / `SEARCH ALL`,以及 `INSPECT ... LEADING` / `CHARACTERS`。現在可以讀取 `COMP-3` 和 `COMP` 用法。
- 內嵌 SQL:`EXEC SQL`(游標、`FETCH`、`SELECT ... INTO`)、`WHENEVER`、指示變數(包含 `INDICATOR` 關鍵字形式)、`SQLSTATE`、`EXECUTE IMMEDIATE`,以及 `PREPARE` / `EXECUTE ... USING`。產生的 Python 已實際執行過;Rust 端只用模擬處理器驗證過。
- `EXEC CICS`(最小支援):`SEND TEXT` 用於顯示文字,`RETURN` / `ABEND` 用於結束程式。其他指令保留為 `TODO`。不重現 BMS 畫面對應。
- Java(`converters/java_to`):支援方法參照、`Comparator`、`Collectors` 和匿名類別(保留每一個方法)。Rust 端支援 List 操作(`Vec`、`push`、索引存取、`for`),產生的 Rust 在 spring、jaxrs、servlet、spark 四種框架下都通過了 `cargo check`。
- 在 Spring PetClinic REST(以 OpenAPI 為起點的專案)上的試驗:找到的路由從 1 條增加到 37 條;產生的 Python 語法錯誤為 0;衍生查詢(`Or`、`Like`、比較、`OrderBy` 等)可在記憶體儲存庫上執行;應用程式可以匯入(41 條路由);`this.field` 的名稱已統一,呼叫能依序經過控制器 → 服務 → 儲存庫。
- 尚未完成:儲存庫與 MapStruct 對應器的產生(呼叫會停在 `TODO`)、真實資料庫、真正執行產生的 FastAPI、CICS 的畫面對應與檔案、含 Map 或 Stream 的 Rust 函式本體。pl2pl 測試:55 項通過。

## 2026-11-25 現況與未完成事項

現況：這是把舊程式（COBOL、C、C++、Fortran、Java 等）改寫成你所選語言和框架的工具。最近納入了 Java_to 對 PetClinic 的支援（ResponseEntity、Mapper、父層資料類別），並同步了多語言文件。尚未完成：在實體 Mac 和 iPhone 上確認（這裡沒有實機）。翻譯：日文與英文以外的語言皆為 AI 翻譯，尚未經母語人士確認。

<!-- directive-20261125-1130 -->
## 2026-11-25 指示（11:30 前不需確認）

到 2026-11-25 11:30 為止，未完成的事項不必確認，自動做到完成。即使判斷需求很少、必要性不高，或是得從頭大規模重做，也不要改用別的方式敷衍，而是正面重做。同時開發的對象：COBOL_to、Java_to、MFastAPI、2MFastAPI（把其他語言和框架轉成與 MFastAPI 相容的程式碼；Node.js 和 TypeScript 是例外，可以選用，也可以都不選）、RMojo、RPoRea（RPoem 與 Reaxt 的整合）、2RPoRea（把其他語言和框架轉成 RPoRea）、Reaxt（React + Next.js + Nuxt.js + Nest.js 的整合）。改動其中一項時，要檢查其他項目，並在同一天以日文和英文更新 README、CLAUDE、PORTING 和 POLICY。

<!-- directive-20261125-mojo -->
## 2026-11-25 補充：MFastAPI 必須能從 Mojo 使用

MFastAPI 要維持**可從 Mojo 使用的規格**（Python 語法、Rust 實作、支援 Mojo；在 Windows 上，透過 RMojo 的 `compat/wsl-bridge` 使用 WSL 中的 Mojo）。2MFastAPI 的轉換結果也必須能從 Mojo 使用。現況：`mojo/` 是參考實作，階段 B、C 的伺服器仍屬實驗性質。從 Mojo 呼叫的實機確認，只有實際確認過的範圍才能寫成「已確認」。到 11:30 為止不需確認，自動進行。

## 2026-11-25 補充：c2rust 支援陣列與 break/continue

`pl2pl c2rust` 現在支援固定長度的陣列（`int a[8] = {...};`、`double w[4];`，初始值不足的部分補 0）、以索引讀寫（`a[j + 1]` → `a[(j + 1) as usize]`）、`break;`、`continue;`，以及 `while (1)` → `loop`。我們用 WSL 的 gcc 和 rustc 執行了一個包含氣泡排序和 `while (1)` 的範例，輸出完全一致（`1 2 3 4 5 7 8 9` / `6.00 12.00 9`）。單元測試 56 項通過。尚未支援：指標、結構、二維陣列、把陣列傳給函式，以及整數與小數的隱含型別轉換（`k * 1.5` 必須寫成 `(double)k`）。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(語言清單) / [COMBINATIONS.md](../../COMBINATIONS.md)(組合) / [DESIGN.md](../../DESIGN.md)(設計，由擁有者撰寫) / [components/README.md](../../../components/README.md)(附帶的元件)。這些檔案為日文。
