# COBOL_to

**日本語 → English**

COBOL の方言・版・書式を**自動判定**し、選んだスタックのプロジェクトへ変換するツールです。
*Detects the COBOL dialect, standard and source format automatically and converts to the stack you choose.*

- リポジトリ / Repository: https://github.com/aon-co-jp/COBOL_to
- ライセンス / License: MIT

## 日本語

### できること
- **自動判定**: IBM Enterprise(メインフレーム)/ Micro Focus / GnuCOBOL / ACUCOBOL-GT / Fujitsu NetCOBOL / 日立 COBOL、規格は COBOL-68・74・85・2002/2014、書式は固定・自由。根拠つきで `COBOL_to-REPORT.md` に出力します。
- **変換先を選択**(4種):
  1. 最新 Python + 最新 FastAPI + Next.js/Node.js + HTML5/CSS3/TypeScript/Bootstrap
  2. 最新 Python + 最新 MFastAPI + 同上
  3. 最新 Rust + 最新 Poem
  4. 最新 Rust + 最新 RPoem(Bootstrap・TypeScript・Next.js・Node.js は使わない。RPoem は Rust→WASM 方針)
- **フォルダ選択**: 引数なしで起動すると、エクスプローラー(ファイル選択ダイアログ)が開き、プログラムや複数プロジェクトの上位フォルダを選べます。出力先は既存フォルダの選択も、新しい名前の入力(空欄は `<元>-converted`)もできます。
- **API**: Connect 形式 `POST /cobol_to.Program/Run`(REST は使いません)。

### 使い方
```
cobol_to                                  # フォルダ選択ダイアログが開く
cobol_to --src <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir>
```

### 現在の制限(正直な状態)
- 変換できる構文: `PROGRAM-ID` / `01〜49・77・88`(PIC・VALUE・OCCURS・REDEFINES〈同型のみ別名、他は別領域+注記〉)/ `DISPLAY`(`WITH NO ADVANCING`)・`ACCEPT` / `MOVE`・`ADD`・`SUBTRACT`・`MULTIPLY`・`DIVIDE`(`REMAINDER`)・`COMPUTE`(小数=固定小数点、`ROUNDED`)/ `IF`(条件名・`NUMERIC`/`ALPHABETIC`)・`EVALUATE`(`THRU`)/ `PERFORM`(段落・`THRU`・`TIMES`・`UNTIL`・`VARYING`・インライン)・`GO TO`(`DEPENDING ON`)/ `STRING`・`UNSTRING`・`INSPECT`(`TALLYING FOR ALL`・`REPLACING ALL/FIRST`・`CONVERTING`)/ `INITIALIZE`・`SET` / `CALL "名前" USING`(生成物の `EXTERNAL`/`run_with` に登録)/ `COPY`(変換前に展開、`REPLACING ==a== BY ==b==` 可)/ 順編成ファイル(`SELECT`・`FD`・`OPEN`・`READ`・`WRITE`・`CLOSE`・`FILE STATUS`)/ `FUNCTION`(`LENGTH`・`UPPER-CASE`・`LOWER-CASE`・`REVERSE`・`MAX`・`MIN`・`ABS`・`MOD`・`NUMVAL`)。`samples/features.cbl`・`fileio.cbl`・`misc.cbl` を Python と Rust の両方で実行し、同じ出力になることを確認済み。
- **未対応(`TODO(COBOL_to)` として表示、黙って落とさない)**: `SORT`/`MERGE`/`SEARCH`、`EXEC SQL`/`EXEC CICS`、索引ファイル・相対ファイル(`INDEXED`/`RELATIVE`)・`REWRITE`/`START`/`DELETE`、`COMP-3` 等のバイナリ表現(桁だけ扱う)、参照修飾の書き込み側(`MOVE ... TO X(a:b)`)、多次元の添字、`STRING ... WITH POINTER`・`ON OVERFLOW`、`ON SIZE ERROR`、`SET ... TO FALSE`、`INSPECT ... BEFORE/AFTER`、動的 `CALL`、`ACCEPT ... FROM DATE/TIME`、`WRITE ... ADVANCING`(改行のみ)、`REDEFINES` のメモリ重ね合わせ。`DISPLAY` は小数を小数点つきで表示(COBOL の暗黙小数点とは異なる)。`CALL` の引数は値渡しで、呼び先が変えた値は戻らない。
- メインフレームの EBCDIC ファイルは変換せず、レポートに載せるだけです(UTF-8/ASCII へ変換してから実行)。
- 生成物は「プログラムの登録表 + 雛形」までで、FastAPI/MFastAPI/RPoem 実サーバーへの接続、Next.js 画面の自動生成は未実装です。
- 生成プロジェクトの依存は**固定しません**(最新版は version-control の通知に従う)。

### インストーラー
| OS | 形式 | 作り方 |
|---|---|---|
| Windows x64 / ARM64 | `COBOL_to-Setup-x64.exe` / `-arm64.exe`(Inno Setup、ユーザー単位、PATH 追加は任意) | `installer/cobol_to.iss` |
| Linux amd64 / arm64 | `cobol-to_<版>_<arch>.deb`、`.tar.gz` | `installer/build-deb.sh` |
| macOS arm64 / x64 | `COBOL_to-<版>-macos-<arch>.pkg`(未署名) | `installer/build-pkg.sh` |

タグ `v*` の push で `.github/workflows/release.yml` が全 OS 分を作り、各 OS で install→実行→uninstall の通し試験をしてから Release に添付します。
検証状況: **v0.1.2 の CI で全6ジョブ成功**(Windows x64/ARM64・Linux amd64/arm64・macOS arm64/x64 で install→実行→uninstall の通し試験を通過)し、[Release v0.1.2](https://github.com/aon-co-jp/COBOL_to/releases/tag/v0.1.2) に 8 個のインストーラーと `SHA256SUMS.txt` を添付。実機での確認は Windows x64 と Linux amd64(WSL)のみで、他は CI のランナー上。macOS の署名・公証は未対応(未署名の .pkg は警告が出る)。

### 更新通知
自動更新しきれないものは、登録した 2 つのメールへ Claude 向けメッセージとして送ります → [NOTIFY.md](NOTIFY.md)(日英独露)。

## English

### What it does
- **Auto-detection**: IBM Enterprise (mainframe) / Micro Focus / GnuCOBOL / ACUCOBOL-GT / Fujitsu NetCOBOL / Hitachi COBOL; standards COBOL-68, 74, 85, 2002/2014; fixed or free format. Evidence goes to `COBOL_to-REPORT.md`.
- **Choose the target** (4 stacks): (1) latest Python + latest FastAPI + Next.js/Node.js + HTML5/CSS3/TypeScript/Bootstrap; (2) latest Python + latest MFastAPI + the same; (3) latest Rust + latest Poem; (4) latest Rust + latest RPoem (no Bootstrap, TypeScript, Next.js or Node.js; RPoem is Rust→WASM).
- **Folder picker**: started without arguments, a native file dialog opens so you can pick the top folder of a program or a multi-project tree. For the output you can pick an existing folder or type a new name (empty = `<source>-converted`).
- **API**: Connect style `POST /cobol_to.Program/Run` (no REST).

### Usage
```
cobol_to                                  # opens the folder picker
cobol_to --src <dir> --target <fastapi|mfastapi|rust-poem|rpoem> --out <dir>
```

### Current limits (honest status)
- Converted: `PROGRAM-ID`; `01–49/77/88` (PIC, VALUE, OCCURS, REDEFINES — aliased only when the types match, otherwise a separate area plus a note); `DISPLAY` (`WITH NO ADVANCING`), `ACCEPT`; `MOVE`, `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` (`REMAINDER`), `COMPUTE` (decimals as fixed point, `ROUNDED`); `IF` (condition names, `NUMERIC`/`ALPHABETIC`), `EVALUATE` (`THRU`); `PERFORM` (paragraph, `THRU`, `TIMES`, `UNTIL`, `VARYING`, inline), `GO TO` (`DEPENDING ON`); `STRING`, `UNSTRING`, `INSPECT` (`TALLYING FOR ALL`, `REPLACING ALL/FIRST`, `CONVERTING`); `INITIALIZE`, `SET`; `CALL "name" USING` (register the callee in the generated `EXTERNAL` / `run_with`); `COPY` (expanded before conversion, `REPLACING ==a== BY ==b==`); sequential files (`SELECT`, `FD`, `OPEN`, `READ`, `WRITE`, `CLOSE`, `FILE STATUS`); `FUNCTION` (`LENGTH`, `UPPER-CASE`, `LOWER-CASE`, `REVERSE`, `MAX`, `MIN`, `ABS`, `MOD`, `NUMVAL`). `samples/features.cbl`, `fileio.cbl` and `misc.cbl` run in both generated Python and Rust with identical output.
- **Not supported (shown as `TODO(COBOL_to)`, never dropped silently)**: `SORT`/`MERGE`/`SEARCH`, `EXEC SQL`/`EXEC CICS`, indexed/relative files (`INDEXED`/`RELATIVE`) and `REWRITE`/`START`/`DELETE`, binary representations such as `COMP-3` (only the digit count is used), reference modification as a target (`MOVE ... TO X(a:b)`; reads work), multi-dimensional subscripts, `STRING ... WITH POINTER` / `ON OVERFLOW`, `ON SIZE ERROR`, `SET ... TO FALSE`, `INSPECT ... BEFORE/AFTER`, dynamic `CALL`, `ACCEPT ... FROM DATE/TIME`, `WRITE ... ADVANCING` (newline only), true memory overlay for `REDEFINES`. `DISPLAY` prints decimals with a decimal point (COBOL's implied point is not shown). `CALL` arguments are passed by value; changes made by the callee are not returned.
- EBCDIC mainframe files are not converted, only listed in the report (convert to UTF-8/ASCII first).
- Output is a program registry plus a skeleton; wiring into a real FastAPI/MFastAPI/RPoem server and generating the Next.js UI are not implemented.
- Dependencies of generated projects are **not pinned** (latest versions follow version-control notifications).

### Installers
See the table above. Tagging `v*` builds every installer in `.github/workflows/release.yml`, smoke-tests install → run → uninstall on each OS, then attaches them to the Release. Status: **all 6 jobs of the v0.1.2 CI passed** (install → run → uninstall on Windows x64/ARM64, Linux amd64/arm64, macOS arm64/x64) and [Release v0.1.2](https://github.com/aon-co-jp/COBOL_to/releases/tag/v0.1.2) carries 8 installers plus `SHA256SUMS.txt`. Real-machine checks exist only for Windows x64 and Linux amd64 (WSL); the rest ran on CI runners. macOS signing/notarization is not done (the unsigned .pkg shows a warning).

### Update notifications
What cannot be updated automatically is mailed to two registered addresses as a Claude message → [NOTIFY.md](NOTIFY.md) (JA/EN/DE/RU).


## 2026-11-21 更新: ON SIZE ERROR に対応 / Update: ON SIZE ERROR is supported

**日本語**: `ADD`・`SUBTRACT`・`MULTIPLY`・`DIVIDE`・`COMPUTE` の `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` を変換できるようになりました。結果が代入先の PICTURE の桁数をあふれる、または 0 で割るときは、代入せずエラー側の文を実行します(そうでなければ代入して `NOT ON SIZE ERROR` 側を実行)。Python と Rust の両方で、`ADD 5`→`OK1`、`ADD 20`(あふれ)→`OVER2`、`DIVIDE 10 BY 0`→`ZERO`、最終値 `95` と同じ出力になることを実際に動かして確認しました。単体試験は 12 件成功です。**まだ**: `ADD a b TO c d` のように対象が複数ある形は、あふれの判定をせず `TODO(COBOL_to)` を残します。他の言語の節にある「完成予定」の一覧は、毎朝の翻訳更新で順に直します(それまでは古い記述が残ります)。残りの完成予定: INSPECT BEFORE/AFTER、SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` on `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` and `COMPUTE` is now converted. When the result overflows the target's PICTURE digits, or the divisor is zero, the target is left unchanged and the error branch runs (otherwise the value is stored and the `NOT ON SIZE ERROR` branch runs). We ran the generated Python and Rust: `ADD 5` -> `OK1`, `ADD 20` (overflow) -> `OVER2`, `DIVIDE 10 BY 0` -> `ZERO`, final value `95`, identical in both. 12 unit tests pass. **Not yet**: forms with several targets, such as `ADD a b TO c d`, get no overflow check and keep a `TODO(COBOL_to)`. The "planned" lists in the other language sections will be corrected by the daily translation update (until then they are stale). Still planned: INSPECT BEFORE/AFTER, SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-21 更新: INSPECT の BEFORE/AFTER INITIAL に対応 / Update: INSPECT BEFORE/AFTER INITIAL is supported

**日本語**: `INSPECT ... TALLYING / REPLACING / CONVERTING` に `AFTER INITIAL a` と `BEFORE INITIAL b`(両方でも可)を付けた形を変換できます。対象は「a が最初に現れた直後から、b が最初に現れる直前まで」で、a が見つからなければ範囲は空、b が見つからなければ末尾までです。`"AXBXCXDXE"` に `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` を実行すると `AXB-C-DXE` になり、`TALLYING ... AFTER INITIAL "B"` の数は 3 で、Python と Rust の出力が一致することを実際に動かして確認しました。単体試験は 13 件成功です。**まだ**: `FOR CHARACTERS` / `LEADING` / `REPLACING LEADING` などは `TODO(COBOL_to)` のままです。残りの完成予定: SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `INSPECT ... TALLYING / REPLACING / CONVERTING` with `AFTER INITIAL a` and `BEFORE INITIAL b` (either or both) is now converted. The affected region runs from just after the first `a` to just before the first `b`; if `a` is not found the region is empty, and if `b` is not found it runs to the end. `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` on `"AXBXCXDXE"` gives `AXB-C-DXE`, and `TALLYING ... AFTER INITIAL "B"` counts 3; the generated Python and Rust were run and give identical output. 13 unit tests pass. **Not yet**: `FOR CHARACTERS`, `LEADING` and `REPLACING LEADING` still stay as `TODO(COBOL_to)`. Still planned: SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.