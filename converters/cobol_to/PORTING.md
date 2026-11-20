# COBOL_to — お引越し・再開 / Porting and resume

**日本語 → English**

## 日本語

### 別の PC・別アカウントで再開する手順
1. `git clone https://github.com/aon-co-jp/COBOL_to` して `cd COBOL_to`。
2. Rust(最新の stable)を入れる。`cargo build --release` と `cargo test`。
3. Windows のインストーラーを作るなら Inno Setup 6 を入れ、`ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\cobol_to.iss`(出力は `dist\`)。Linux の `.deb` は `installer/build-deb.sh`、macOS の `.pkg` は macOS 上で `installer/build-pkg.sh`。
4. 動作確認: `cobol_to --src tests/samples --target rust-poem --out <出力先>`。`src/programs/hello.rs` ができ、`COBOL_to-REPORT.md` に判定が出る。
5. 更新通知は version-control の `config.toml`(git 管理外)の `[notify] email_to` に自分のメールを 2 件書く。`root` も自分のドライブ/フォルダにする。秘密(トークン等)はリポジトリに入れない。

### 未了(優先順)
1. **(完了)リリース CI**: `v0.1.2`(run 44168310290)が全6ジョブ成功、Release に deb×2・tar.gz×2・pkg×2・exe×2・`SHA256SUMS.txt`。残るのは実機での確認(Linux arm64・macOS・Windows ARM64)と macOS の署名。以下は経緯(旧記述)。**(旧)**タグ `v0.1.1`、run 44166948447。v0.1.0 は Linux のビルド失敗と macOS x64 の廃止 runner で Release 無し(Windows は成功)。v0.1.1 で Linux arm64・macOS の通し試験が初確認。失敗があればタグを付け直さず `v0.1.2` で出す。成功したら Release の添付物と SHA256SUMS を確認し、README の検証状況を更新。
2. **変換できる構文の拡充**: `IF`/`EVALUATE`/`PERFORM`(`UNTIL`・`VARYING`)/`COMPUTE`/`SUBTRACT`/`MULTIPLY`/`DIVIDE`/`STRING`/`INSPECT`/`COPY`・`REPLACING`/ファイル I/O/`CALL`/`EXEC SQL`・`EXEC CICS`、数値の小数・符号・`COMP-3`、CONTINUATION 行。テストは `tests/samples` に実プログラムを足して追加。
3. **生成物の実サーバー接続**: FastAPI/MFastAPI/Poem/RPoem に `POST /cobol_to.Program/Run`(Connect)を実装。Next.js(TypeScript+Bootstrap)の画面の自動生成。RPoem は Rust→WASM の画面。
4. **最新版への自動追従**: 生成物の依存は固定していない。`cargo add` と PyPI/npm の最新取得、version-control の登録。
5. **通知メールが届かない件**: VPS では `e-mail: sent`(SMTP は受理、コードのバグではない)だが、`norukia.jp@gmail.com` と `cloud9slack@gmail.com` には**届いていない**(利用者報告)。`aon.co.jp` に SPF(TXT)も `_dmarc` も無いのが有力な原因。迷惑メール欄と送信元(`inf…@aon.co.jp`)の返送を確認 → value-domain で SPF(Lolipop 指定値を確認)・DMARC を追加(利用者の操作)。直らない場合は別の送信経路を検討。LINE は未設定。
6. **GitHub の owner 配下を起点にした走査**: version-control には未実装(`root` のみ)。
7. **macOS の署名・公証**(Apple のシークレットが必要)、EBCDIC の自動変換、Linux の `.rpm`/AppImage。
8. **version-control による COBOL_to の実走査確認**(`root = "F:/"` で拾われるはず。走査が長いので `--dry-run --no-fetch --no-api` を時間に余裕のあるときに)。

### 注意
- この PC の Bash に `cargo` は無い(PowerShell で PATH を前置き)。`cargo test` は debug の exe を更新しない。
- 履歴は新規。旧名(`COBOL2`/`COBOL-to`)のリポジトリは作っていない。

### 再開用メッセージ(そのまま貼り付け)
> `F:\COBOL_to`(GitHub: aon-co-jp/COBOL_to)を再開してください。まず `README.md`・`CLAUDE.md`・`PORTING.md` を読むこと。COBOL の方言・版・書式を自動判定し、FastAPI / MFastAPI / Rust+Poem / Rust+RPoem(RPoem は Bootstrap・TypeScript・Next.js・Node.js なし)へ変換するツールで、API は REST ではなく Connect(`POST /cobol_to.Program/Run`)。v0.1.2 の Release は公開済みで CI は全6ジョブ成功しています(v0.1.0/v0.1.1 は失敗のため Release 無し。タグは動かさない)。次に直す点があれば v0.1.3 以降で出してください。通知メールが利用者に届いていない件(`aon.co.jp` に SPF/DMARC が無い疑い)も `PORTING.md` の 5 を見て進めてください。次に、`PORTING.md` の未了の優先順(構文の拡充 → 実サーバー接続 → 最新版への自動追従)を進めてください。注意: Bash に cargo は無いので PowerShell で PATH を前置きする、`cargo test` は debug の exe を更新しない、未対応の構文は黙って落とさず `TODO(COBOL_to)` にする、メールアドレスやトークンはリポジトリに入れない、`git pull` 前に `git status -sb` で分岐を確認する。

## English

### Resume on another PC or account
1. `git clone https://github.com/aon-co-jp/COBOL_to` and `cd COBOL_to`.
2. Install the latest stable Rust. `cargo build --release` and `cargo test`.
3. For the Windows installer install Inno Setup 6 and run `ISCC /DAppVersion=0.1.0 /DArch=x64 /DExeDir=..\target\release installer\cobol_to.iss` (output in `dist\`). The Linux `.deb` is `installer/build-deb.sh`; the macOS `.pkg` is `installer/build-pkg.sh` on macOS.
4. Check: `cobol_to --src tests/samples --target rust-poem --out <out>` creates `src/programs/hello.rs` and the detection in `COBOL_to-REPORT.md`.
5. For update notifications put your own two addresses into `[notify] email_to` in version-control's `config.toml` (git-ignored) and set `root` to your drive/folder. Never commit secrets.

### Open items (priority order)
1. **(Done) Release CI**: `v0.1.2` (run 44168310290) passed all 6 jobs; the Release holds 2 debs, 2 tar.gz, 2 pkg, 2 exe and `SHA256SUMS.txt`. Still open: real-machine checks (Linux arm64, macOS, Windows ARM64) and macOS signing. The rest is history (old text). **(Old)** tag `v0.1.1`, run 44166948447. v0.1.0 had no Release: Linux failed to build and the macOS x64 runner was retired (Windows passed). v0.1.1 is the first real check of the Linux arm64 and macOS smoke tests. On failure ship `v0.1.2` rather than moving the tag. On success verify the Release assets and `SHA256SUMS`, then update the README status.
2. **Widen the supported syntax**: `IF`/`EVALUATE`/`PERFORM` (`UNTIL`, `VARYING`)/`COMPUTE`/`SUBTRACT`/`MULTIPLY`/`DIVIDE`/`STRING`/`INSPECT`/`COPY`+`REPLACING`/file I/O/`CALL`/`EXEC SQL`+`EXEC CICS`, decimals/signs/`COMP-3`, continuation lines. Add real programs to `tests/samples` as tests.
3. **Wire the output into real servers**: implement `POST /cobol_to.Program/Run` (Connect) in FastAPI/MFastAPI/Poem/RPoem; generate the Next.js (TypeScript + Bootstrap) UI; a Rust→WASM UI for RPoem.
4. **Automatic latest versions**: generated dependencies are not pinned. Use `cargo add` and the PyPI/npm latest lookups; register in version-control.
5. **Notification mail is not arriving**: the VPS logs `e-mail: sent` (SMTP accepted; not a code bug), yet the user reports nothing reached `norukia.jp@gmail.com` or `cloud9slack@gmail.com`. The likely cause is that `aon.co.jp` has no SPF (TXT) record and no `_dmarc`. Check the spam folder and bounces in the sender mailbox (`inf…@aon.co.jp`), then add SPF (confirm Lolipop's value) and DMARC at value-domain (the user's action). If that does not help, consider another sending path. LINE is not configured.
6. **Scanning from a GitHub owner**: not implemented in version-control (only `root`).
7. **macOS signing/notarization** (Apple secrets needed), EBCDIC auto-conversion, Linux `.rpm`/AppImage.
8. **Live version-control scan of COBOL_to** (should be picked up via `root = "F:/"`; the scan is long, so run `--dry-run --no-fetch --no-api` when there is time).

### Notes
- No `cargo` in this PC's Bash (prepend PATH in PowerShell). `cargo test` does not refresh the debug exe.
- History is fresh. No repositories were created under the old names (`COBOL2`/`COBOL-to`).

### Resume message (paste as is)
> Please resume `F:\COBOL_to` (GitHub: aon-co-jp/COBOL_to). Read `README.md`, `CLAUDE.md` and `PORTING.md` first. It auto-detects the COBOL dialect, standard and source format and converts to FastAPI / MFastAPI / Rust+Poem / Rust+RPoem (RPoem without Bootstrap, TypeScript, Next.js or Node.js); the API is Connect (`POST /cobol_to.Program/Run`), not REST. The Release v0.1.2 is published and all 6 CI jobs passed (v0.1.0/v0.1.1 failed and have no Release; never move a tag). Ship any further fix as v0.1.3 or later. Also work on the notification mail not reaching the user (suspected missing SPF/DMARC for `aon.co.jp`), see item 5 in `PORTING.md`. Then work through the open items in `PORTING.md` in order (syntax coverage → real server wiring → automatic latest versions). Cautions: Bash has no cargo, so prepend PATH in PowerShell; `cargo test` does not refresh the debug exe; never drop unsupported syntax silently, emit `TODO(COBOL_to)`; keep e-mail addresses and tokens out of the repository; check `git status -sb` for divergence before any `git pull`.

## 2026-11-18 追記2: 未対応の構文を一括対応 / Remaining syntax implemented
**日本語**: 未了の2(構文の拡充)の大半を実装した。`GO TO`(DEPENDING ON・段落の状態機械)・`PERFORM THRU`・小数(固定小数点・ROUNDED)・`OCCURS`(添字)・`REDEFINES`(同型は別名、他は別領域+注記)・`88` 条件名・`NUMERIC`/`ALPHABETIC`・`STRING`/`UNSTRING`/`INSPECT`・`INITIALIZE`/`SET`/`ACCEPT`・`DIVIDE REMAINDER`・`FUNCTION`(9種)・継続行・`COPY`(変換前に展開)・`CALL`(`EXTERNAL`/`run_with` に登録する外部呼び出し)・順編成ファイル I/O(SELECT/FD/OPEN/READ/WRITE/CLOSE/FILE STATUS)。`tests/samples/features.cbl`・`fileio.cbl`・`misc.cbl` を Python と Rust の両方で実行し同一出力を確認、`cargo test` 9件成功。**未対応(TODO表示)**: SORT/MERGE/SEARCH、EXEC SQL/CICS、索引・相対ファイル・REWRITE、COMP-3 の表現、参照修飾、多次元添字、STRING POINTER/ON OVERFLOW、ON SIZE ERROR、BEFORE/AFTER、動的 CALL、ACCEPT FROM DATE、メモリ重ね合わせ。**未確認**: 実際のメインフレーム由来の大きなプログラムでの通し(サンプルは自作の小さなもの)、CI(Linux arm64・macOS)上での新サンプル変換。次は v0.1.3 として出すか、上記の TODO を順に。
**English**: Most of open item 2 (syntax coverage) is implemented: `GO TO` (incl. `DEPENDING ON`; paragraphs run on a small state machine), `PERFORM THRU`, decimals (fixed point, `ROUNDED`), `OCCURS` subscripts, `REDEFINES` (aliased when types match, otherwise a separate area plus a note), level-88 condition names, `NUMERIC`/`ALPHABETIC`, `STRING`/`UNSTRING`/`INSPECT`, `INITIALIZE`/`SET`/`ACCEPT`, `DIVIDE REMAINDER`, nine `FUNCTION`s, continuation lines, `COPY` (expanded before parsing), `CALL` (external calls registered in `EXTERNAL` / `run_with`), and sequential file I/O. `tests/samples/features.cbl`, `fileio.cbl`, `misc.cbl` give identical output in generated Python and Rust; 9 unit tests pass. **Not supported (shown as TODO)**: SORT/MERGE/SEARCH, EXEC SQL/CICS, indexed/relative files and REWRITE, COMP-3 representation, reference modification, multi-dimensional subscripts, STRING POINTER / ON OVERFLOW, ON SIZE ERROR, BEFORE/AFTER, dynamic CALL, ACCEPT FROM DATE, true memory overlay. **Unverified**: large real-world mainframe programs (samples are small and hand-written) and converting the new samples on CI (Linux arm64, macOS). Next: ship as v0.1.3, or work through the TODO list above.

## 2026-11-18 追記: 変換を「小さく→多機能」へ / Conversion grown from minimal to richer
**日本語**: `src/convert.rs` を構造化(式・条件の構文解析)。対応: IF/ELSE/END-IF(AND/OR/NOT・比較語)、PERFORM(段落・n TIMES・UNTIL・VARYING・インライン)、段落、COMPUTE、ADD/SUBTRACT/MULTIPLY/DIVIDE(GIVING 含む)、STOP RUN/GOBACK。未対応の文(OPEN/READ/WRITE/CALL/EVALUATE/GO TO 等)は `TODO(COBOL_to)` として残す。数値は整数(小数 PICは近似+TODO)。`tests/samples/calc.cbl` を Python と Rust の両方で実行し、同じ出力(`TOTAL=35 MSG=BIG`)を確認。次: EVALUATE・ファイル I/O・小数(固定小数点)・REDEFINES/OCCURS。
**EVALUATE(WHEN 値・OR・OTHER・TRUE)も対応**。
**English**: EVALUATE is supported too. `convert.rs` now parses expressions/conditions. Supported: IF/ELSE, PERFORM (paragraph, TIMES, UNTIL, VARYING, inline), paragraphs, COMPUTE, ADD/SUBTRACT/MULTIPLY/DIVIDE (GIVING), STOP RUN/GOBACK. Unsupported statements stay as `TODO(COBOL_to)`. Numbers are integers. `calc.cbl` gives identical output in generated Python and Rust. Next: EVALUATE, file I/O, fixed-point decimals, REDEFINES/OCCURS.


## 2026-11-21 更新: ON SIZE ERROR に対応 / Update: ON SIZE ERROR is supported

**日本語**: `ADD`・`SUBTRACT`・`MULTIPLY`・`DIVIDE`・`COMPUTE` の `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` を変換できるようになりました。結果が代入先の PICTURE の桁数をあふれる、または 0 で割るときは、代入せずエラー側の文を実行します(そうでなければ代入して `NOT ON SIZE ERROR` 側を実行)。Python と Rust の両方で、`ADD 5`→`OK1`、`ADD 20`(あふれ)→`OVER2`、`DIVIDE 10 BY 0`→`ZERO`、最終値 `95` と同じ出力になることを実際に動かして確認しました。単体試験は 12 件成功です。**まだ**: `ADD a b TO c d` のように対象が複数ある形は、あふれの判定をせず `TODO(COBOL_to)` を残します。他の言語の節にある「完成予定」の一覧は、毎朝の翻訳更新で順に直します(それまでは古い記述が残ります)。残りの完成予定: INSPECT BEFORE/AFTER、SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` on `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` and `COMPUTE` is now converted. When the result overflows the target's PICTURE digits, or the divisor is zero, the target is left unchanged and the error branch runs (otherwise the value is stored and the `NOT ON SIZE ERROR` branch runs). We ran the generated Python and Rust: `ADD 5` -> `OK1`, `ADD 20` (overflow) -> `OVER2`, `DIVIDE 10 BY 0` -> `ZERO`, final value `95`, identical in both. 12 unit tests pass. **Not yet**: forms with several targets, such as `ADD a b TO c d`, get no overflow check and keep a `TODO(COBOL_to)`. The "planned" lists in the other language sections will be corrected by the daily translation update (until then they are stale). Still planned: INSPECT BEFORE/AFTER, SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-21 更新: INSPECT の BEFORE/AFTER INITIAL に対応 / Update: INSPECT BEFORE/AFTER INITIAL is supported

**日本語**: `INSPECT ... TALLYING / REPLACING / CONVERTING` に `AFTER INITIAL a` と `BEFORE INITIAL b`(両方でも可)を付けた形を変換できます。対象は「a が最初に現れた直後から、b が最初に現れる直前まで」で、a が見つからなければ範囲は空、b が見つからなければ末尾までです。`"AXBXCXDXE"` に `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` を実行すると `AXB-C-DXE` になり、`TALLYING ... AFTER INITIAL "B"` の数は 3 で、Python と Rust の出力が一致することを実際に動かして確認しました。単体試験は 13 件成功です。**まだ**: `FOR CHARACTERS` / `LEADING` / `REPLACING LEADING` などは `TODO(COBOL_to)` のままです。残りの完成予定: SORT/MERGE/SEARCH、索引・相対ファイル、COMP-3、EXEC SQL/CICS。

**English**: `INSPECT ... TALLYING / REPLACING / CONVERTING` with `AFTER INITIAL a` and `BEFORE INITIAL b` (either or both) is now converted. The affected region runs from just after the first `a` to just before the first `b`; if `a` is not found the region is empty, and if `b` is not found it runs to the end. `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` on `"AXBXCXDXE"` gives `AXB-C-DXE`, and `TALLYING ... AFTER INITIAL "B"` counts 3; the generated Python and Rust were run and give identical output. 13 unit tests pass. **Not yet**: `FOR CHARACTERS`, `LEADING` and `REPLACING LEADING` still stay as `TODO(COBOL_to)`. Still planned: SORT/MERGE/SEARCH, indexed and relative files, COMP-3, EXEC SQL/CICS.