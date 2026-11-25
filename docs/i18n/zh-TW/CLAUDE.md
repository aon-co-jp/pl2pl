> 說明：儲存庫根目錄的日文 CLAUDE.md 是原文。本文用簡單易懂的話寫成，由 AI 翻譯，並經 AI 依母語水準審校語句是否自然；尚未經真人母語人士確認。

# pl2pl 開發筆記
- **任務**：在語言、框架、作業系統之間改寫程式碼的工具(用 Rust 撰寫，位於 `src/`)。來源語言清單見 `docs/LANGUAGES.md`，組合見 `docs/COMBINATIONS.md`。設計由擁有者在 `docs/DESIGN.md` 中一點一點地寫。**不要擅自替他們訂規格。**
- **方向**：在知名語言之間做「多對多」的改寫。不一次做完，先做選定的。網站只有說明和下載，改寫在 PC 應用程式裡進行。
- **規則**：改寫不了的地方留 `TODO` 並回報(不要偷偷刪掉)。要謹慎(可能有相依或副作用就不改)。真正執行過再說「完成」。不新增 REST。回覆只用日語。
- **共同的形式(IR)**：`src/ir.rs`(型別、函式本體、模型、中介軟體、範本)和寫出端 `src/ir_emit.rs`。輸入端(PHP、Python、Java)和輸出端(JSON、OpenAPI、FastAPI)分開，方便增加新組合。`pl2pl php` 也走同一個寫出端；不應改變輸出的修改，要用修改前後輸出的 diff 為空來證明。沒找到路由的檔案要回報。輸出要真正檢查，例如用 `py_compile`。
- **PHP**：`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端對端測試：`tests/run-php.sh`(真實的 FastAPI、SQLite、Jinja2)。單元測試：`cargo test --release`。WSL 中：`wsl -d Ubuntu -- bash <file>`。
- **自動跟上**：`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。檢查指令為空的項目只顯示「未檢查」，絕不自動更新。通知透過 version-control 的 `--notify-file`。
- **部署**：介紹頁是 `webpage/index.html`。VPS 上是 `/root/easy-web.tokyo/pl2pl`(用 `git pull` 更新，是靜態頁面，無需重啟)、`pl2pl-web.service` 連接埠 8142，已寫入 `domains.toml`。
- **陷阱**：含日文的檔案用 Python `-X utf8` 寫入。shell 腳本保持 LF。Git Bash 會改寫 `/mnt/...` 路徑。引號或 `\` 很多的文字，用 Write / Edit 工具，不要用 heredoc。
- **COBOL_to / Java_to**：已併入 `converters/`。刪除原儲存庫無法復原，須先取得最終確認。
- **翻譯系統**(人類語言)與轉換器分開。轉換器不依賴 LLM。
- **文件語言**：日文檔案是原文。英文檔案用簡單的話重寫過。`docs/i18n/` 裡的其他八種語言，是把同樣內容由 AI 翻譯、再由 AI 審校語句是否自然的版本(尚未經真人母語人士審閱)。更早的經過見 `docs/HISTORY-ja.md`。

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

## 2026-11-25 補充：轉換器同步

在 `converters/cobol_to` 納入了 COBOL_to 的 EXEC CICS LINK/XCTL → CALL（`7d67bbf`），在 `converters/java_to` 納入了 Java_to 的三元運算子（Rust 函式本體，`e9ac300`）。測試：Java_to 39 項、COBOL_to 25 項、pl2pl 55 項，全部通過。尚未完成：Mac 和 iPhone 實機，以及用真實資料庫啟動所產生的 FastAPI 來確認。

## 2026-11-25 補充：用測試固定「有多個實作時的選擇方式」

當儲存庫介面有多個實作時，產生器本來就會優先選擇在記憶體中運作的那個（名稱不含 Jdbc、Jpa），沒有的話就選第一個（先前寫的「尚未支援」已經過時）。現在用單元測試 `multiple_impls_prefer_non_db_one` 固定了這個行為（Java_to：40 項通過）。尚未完成：MapStruct 的複雜對應器，以及用真實資料庫啟動來確認。

## 2026-11-25 補充：支援 MapStruct 的 @Mapping

對於對應器方法上的 `@Mapping(target=..., source="a.b")`（巢狀的來源屬性）、`ignore=true` 和 `constant="..."`，現在會傳給所產生 Python 的 `MapperBase._to(cls, src, rename, ignore, const)`。已用單元測試 `mapstruct_mapping_annotations`（Java_to：41 項通過）和執行時的實際操作（`owner.name` → `fullName`、ignore、常數）確認。尚未完成：巢狀寫法 `@Mappings({...})`、`expression=`、`@MappingTarget`，以及用真實資料庫啟動來確認。

<!-- directive-20261125-1130 -->
## 2026-11-25 指示（11:30 前不需確認）

到 2026-11-25 11:30 為止，未完成的事項不必確認，自動做到完成。即使判斷需求很少、必要性不高，或是得從頭大規模重做，也不要改用別的方式敷衍，而是正面重做。同時開發的對象：COBOL_to、Java_to、MFastAPI、2MFastAPI（把其他語言和框架轉成與 MFastAPI 相容的程式碼；Node.js 和 TypeScript 是例外，可以選用，也可以都不選）、RMojo、RPoRea（RPoem 與 Reaxt 的整合）、2RPoRea（把其他語言和框架轉成 RPoRea）、Reaxt（React + Next.js + Nuxt.js + Nest.js 的整合）。改動其中一項時，要檢查其他項目，並在同一天以日文和英文更新 README、CLAUDE、PORTING 和 POLICY。

<!-- directive-20261125-mojo -->
## 2026-11-25 補充：MFastAPI 必須能從 Mojo 使用

MFastAPI 要維持**可從 Mojo 使用的規格**（Python 語法、Rust 實作、支援 Mojo；在 Windows 上，透過 RMojo 的 `compat/wsl-bridge` 使用 WSL 中的 Mojo）。2MFastAPI 的轉換結果也必須能從 Mojo 使用。現況：`mojo/` 是參考實作，階段 B、C 的伺服器仍屬實驗性質。從 Mojo 呼叫的實機確認，只有實際確認過的範圍才能寫成「已確認」。到 11:30 為止不需確認，自動進行。

## 2026-11-25 補充：c2rust 支援陣列與 break/continue

`pl2pl c2rust` 現在支援固定長度的陣列（`int a[8] = {...};`、`double w[4];`，初始值不足的部分補 0）、以索引讀寫（`a[j + 1]` → `a[(j + 1) as usize]`）、`break;`、`continue;`，以及 `while (1)` → `loop`。我們用 WSL 的 gcc 和 rustc 執行了一個包含氣泡排序和 `while (1)` 的範例，輸出完全一致（`1 2 3 4 5 7 8 9` / `6.00 12.00 9`）。單元測試 56 項通過。尚未支援：指標、結構、二維陣列、把陣列傳給函式，以及整數與小數的隱含型別轉換（`k * 1.5` 必須寫成 `(double)k`）。

## Versionless API 方針（2026-11-25）

本儲存庫的 API 採用 **Versionless**（網址和套件名稱不加 `v1` 之類的版本，只靠「新增」來演進，不破壞相容性）。欄位只增不減；新增的欄位要可省略；不認得的欄位直接忽略；改名時新增新名稱，並把舊名稱標為不建議使用（期限預設為 6 個月以上，確認使用量為 0 之後才刪除）；列舉（enum）要容許未知的值；安全方面的緊急修正是例外。既有的 `.v1` 為了不讓呼叫端壞掉，暫時保留，並新增不含版本的別名，再分階段收掉。詳情與出處見 [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md) 末尾。之後會依序套用。

## API 方針：全面廢除 REST，改用 gRPC / Connect

本儲存庫**不再新增 REST API**。從瀏覽器或 `curl` 呼叫的 API 使用 **Connect 的簡單呼叫**（`POST /<套件>.<Service>/<Method>`，`content-type: application/json`），服務之間則使用 **gRPC / gRPC-web**。GraphQL 僅供讀取。既有的 REST 在動到時就遷移，遷移後刪除舊路徑。JSON 用 RS-JSON（`from_str_secure` / `parse_secure`）讀取。例外只限於格式由對方標準決定的情況（ACME、OAuth/OIDC 的回呼、第三方 Webhook、健康檢查、靜態檔案、OTLP）。拿不定主意時就用 Connect。以上方連結為準。
