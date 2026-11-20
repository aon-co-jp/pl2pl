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
## 世界の言語での 2026-11-21 更新(ON SIZE ERROR・INSPECT BEFORE/AFTER) / 2026-11-21 update in 10 more languages (AI翻訳+AI確認、人間のネイティブ確認はまだ)

### 简体中文 — 更新 (2026-11-21)

*以下为 AI 翻译,并已由 AI 按母语水平审校;尚未经人类母语者校对。*

现已支持 `ADD`、`SUBTRACT`、`MULTIPLY`、`DIVIDE`、`COMPUTE` 的 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`:当结果超出目标 PICTURE 的位数,或除数为 0 时,不会赋值,而是执行错误分支;否则赋值并执行 `NOT ON SIZE ERROR` 分支。同时支持在 `INSPECT ... TALLYING / REPLACING / CONVERTING` 后加 `AFTER INITIAL a` 和 `BEFORE INITIAL b`(可只写其中一个):作用范围从第一个 a 之后开始,到第一个 b 之前结束;找不到 a 时范围为空,找不到 b 时一直到末尾。我们实际运行了生成的 Python 和 Rust:`ADD 5` 得到 `OK1`,`ADD 20`(溢出)得到 `OVER2`,`DIVIDE 10 BY 0` 得到 `ZERO`;对 `"AXBXCXDXE"` 执行 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` 得到 `AXB-C-DXE`,两种语言的输出完全一致。单元测试 13 项通过。**尚未支持**:有多个目标的形式(如 `ADD a b TO c d`)不做溢出判断,仍保留 `TODO(COBOL_to)`;`FOR CHARACTERS`、`LEADING` 仍为 `TODO(COBOL_to)`。其余计划:SORT/MERGE/SEARCH、索引文件与相对文件、COMP-3、EXEC SQL/CICS。上面各语言章节中的“计划”清单已经过时,将由每日翻译更新来修正。

### 繁體中文(台灣) — 更新 (2026-11-21)

*以下為 AI 翻譯,並已由 AI 以母語水準審校;尚未經人類母語者校對。*

現已支援 `ADD`、`SUBTRACT`、`MULTIPLY`、`DIVIDE`、`COMPUTE` 的 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`:當結果超出目標 PICTURE 的位數,或除數為 0 時,不會賦值,而是執行錯誤分支;否則賦值並執行 `NOT ON SIZE ERROR` 分支。同時支援在 `INSPECT ... TALLYING / REPLACING / CONVERTING` 後加上 `AFTER INITIAL a` 與 `BEFORE INITIAL b`(可只寫其中一個):作用範圍從第一個 a 之後開始,到第一個 b 之前結束;找不到 a 時範圍為空,找不到 b 時則一直到結尾。我們實際執行了產生的 Python 與 Rust:`ADD 5` 得到 `OK1`,`ADD 20`(溢位)得到 `OVER2`,`DIVIDE 10 BY 0` 得到 `ZERO`;對 `"AXBXCXDXE"` 執行 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` 得到 `AXB-C-DXE`,兩種語言的輸出完全相同。單元測試 13 項通過。**尚未支援**:有多個目標的形式(如 `ADD a b TO c d`)不做溢位判斷,仍保留 `TODO(COBOL_to)`;`FOR CHARACTERS`、`LEADING` 仍為 `TODO(COBOL_to)`。其餘計畫:SORT/MERGE/SEARCH、索引檔案與相對檔案、COMP-3、EXEC SQL/CICS。上方各語言章節中的「計畫」清單已經過時,將由每日翻譯更新來修正。

### 한국어 — 업데이트 (2026-11-21)

*아래는 AI가 번역하고 AI가 원어민 수준으로 검수한 것이며, 사람 원어민의 검수는 아직 받지 않았습니다.*

이제 `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`, `COMPUTE`의 `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx`를 변환합니다. 결과가 대상 PICTURE의 자릿수를 넘치거나 0으로 나누면 값을 대입하지 않고 오류 쪽 문장을 실행하며, 그렇지 않으면 값을 대입한 뒤 `NOT ON SIZE ERROR` 쪽을 실행합니다. 또한 `INSPECT ... TALLYING / REPLACING / CONVERTING`에 `AFTER INITIAL a`와 `BEFORE INITIAL b`(한쪽만 써도 됨)를 붙인 형태도 지원합니다. 범위는 처음 나타난 a 바로 뒤부터 처음 나타난 b 바로 앞까지이며, a가 없으면 범위가 비고 b가 없으면 끝까지입니다. 생성된 Python과 Rust를 실제로 실행해 확인했습니다. `ADD 5`는 `OK1`, `ADD 20`(넘침)은 `OVER2`, `DIVIDE 10 BY 0`은 `ZERO`였고, `"AXBXCXDXE"`에 `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"`를 실행하면 `AXB-C-DXE`가 되어 두 언어의 출력이 같았습니다. 단위 테스트 13건이 통과했습니다. **아직 지원하지 않는 것**: `ADD a b TO c d`처럼 대상이 여러 개인 형태는 넘침 판정을 하지 않고 `TODO(COBOL_to)`로 남기며, `FOR CHARACTERS`와 `LEADING`도 `TODO(COBOL_to)`입니다. 남은 계획: SORT/MERGE/SEARCH, 인덱스 파일과 상대 파일, COMP-3, EXEC SQL/CICS. 위의 다른 언어 절에 있는 "계획" 목록은 지금은 오래된 내용이며, 매일의 번역 갱신에서 고칩니다.

### Deutsch — Aktualisierung (2026-11-21)

*Die folgenden Texte wurden von einer KI übersetzt und von einer KI auf muttersprachliche Natürlichkeit geprüft; eine Prüfung durch menschliche Muttersprachler steht noch aus.*

Jetzt werden `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` bei `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` und `COMPUTE` umgewandelt: Läuft das Ergebnis über die PICTURE-Stellen des Ziels hinaus oder ist der Divisor 0, wird nichts zugewiesen und stattdessen der Fehlerzweig ausgeführt; andernfalls wird zugewiesen und der Zweig `NOT ON SIZE ERROR` ausgeführt. Ebenso werden `INSPECT ... TALLYING / REPLACING / CONVERTING` mit `AFTER INITIAL a` und `BEFORE INITIAL b` (einzeln oder zusammen) unterstützt: Der Bereich reicht von direkt nach dem ersten a bis direkt vor dem ersten b; fehlt a, ist der Bereich leer, fehlt b, reicht er bis zum Ende. Wir haben das erzeugte Python und Rust tatsächlich ausgeführt: `ADD 5` ergibt `OK1`, `ADD 20` (Überlauf) ergibt `OVER2`, `DIVIDE 10 BY 0` ergibt `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` auf `"AXBXCXDXE"` ergibt `AXB-C-DXE`, und beide Sprachen liefern dieselbe Ausgabe. 13 Unit-Tests bestehen. **Noch nicht unterstützt**: Formen mit mehreren Zielen wie `ADD a b TO c d` erhalten keine Überlaufprüfung und bleiben als `TODO(COBOL_to)` stehen; `FOR CHARACTERS` und `LEADING` bleiben ebenfalls `TODO(COBOL_to)`. Weiter geplant: SORT/MERGE/SEARCH, indizierte und relative Dateien, COMP-3, EXEC SQL/CICS. Die Listen „Geplant“ in den übrigen Sprachabschnitten oben sind damit veraltet und werden durch die tägliche Übersetzungsaktualisierung korrigiert.

### Français — Mise à jour (2026-11-21)

*Les textes suivants ont été traduits par une IA et relus par une IA pour leur naturel ; ils n'ont pas encore été relus par des locuteurs natifs humains.*

`ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` est désormais converti pour `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` et `COMPUTE` : si le résultat dépasse le nombre de chiffres de la PICTURE de la cible, ou si le diviseur est 0, rien n'est affecté et la branche d'erreur s'exécute ; sinon la valeur est affectée et la branche `NOT ON SIZE ERROR` s'exécute. Les formes `INSPECT ... TALLYING / REPLACING / CONVERTING` avec `AFTER INITIAL a` et `BEFORE INITIAL b` (l'un ou l'autre, ou les deux) sont aussi prises en charge : la zone va juste après le premier a jusqu'à juste avant le premier b ; si a est introuvable, la zone est vide, et si b est introuvable, elle va jusqu'à la fin. Nous avons réellement exécuté le Python et le Rust générés : `ADD 5` donne `OK1`, `ADD 20` (dépassement) donne `OVER2`, `DIVIDE 10 BY 0` donne `ZERO` ; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sur `"AXBXCXDXE"` donne `AXB-C-DXE`, et les deux langages produisent la même sortie. 13 tests unitaires réussis. **Pas encore pris en charge** : les formes à plusieurs cibles, comme `ADD a b TO c d`, n'ont pas de contrôle de dépassement et restent signalées par `TODO(COBOL_to)` ; `FOR CHARACTERS` et `LEADING` restent aussi `TODO(COBOL_to)`. Reste prévu : SORT/MERGE/SEARCH, fichiers indexés et relatifs, COMP-3, EXEC SQL/CICS. Les listes « Prévu » des autres sections de langue ci-dessus sont donc périmées ; la mise à jour quotidienne des traductions les corrigera.

### Español — Actualización (2026-11-21)

*Los textos siguientes fueron traducidos por una IA y revisados por una IA en cuanto a naturalidad; aún no los ha revisado un hablante nativo humano.*

Ahora se convierte `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` en `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` y `COMPUTE`: si el resultado desborda los dígitos de la PICTURE del destino, o si el divisor es 0, no se asigna nada y se ejecuta la rama de error; en caso contrario se asigna el valor y se ejecuta la rama `NOT ON SIZE ERROR`. También se admiten `INSPECT ... TALLYING / REPLACING / CONVERTING` con `AFTER INITIAL a` y `BEFORE INITIAL b` (uno solo o ambos): la zona va desde justo después de la primera a hasta justo antes de la primera b; si no se encuentra a, la zona queda vacía, y si no se encuentra b, llega hasta el final. Ejecutamos realmente el Python y el Rust generados: `ADD 5` da `OK1`, `ADD 20` (desbordamiento) da `OVER2`, `DIVIDE 10 BY 0` da `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sobre `"AXBXCXDXE"` da `AXB-C-DXE`, y ambos lenguajes producen la misma salida. Pasan 13 pruebas unitarias. **Aún no admitido**: las formas con varios destinos, como `ADD a b TO c d`, no comprueban el desbordamiento y se quedan como `TODO(COBOL_to)`; `FOR CHARACTERS` y `LEADING` también siguen como `TODO(COBOL_to)`. Sigue previsto: SORT/MERGE/SEARCH, archivos indexados y relativos, COMP-3, EXEC SQL/CICS. Las listas «Previsto» de las demás secciones de idioma de arriba han quedado desactualizadas; la actualización diaria de traducciones las corregirá.

### Português (BR) — Atualização (2026-11-21)

*Os textos a seguir foram traduzidos por uma IA e revisados por uma IA quanto à naturalidade; ainda não foram revisados por falantes nativos humanos.*

Agora `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` é convertido em `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` e `COMPUTE`: se o resultado estourar os dígitos da PICTURE do destino, ou se o divisor for 0, nada é atribuído e o ramo de erro é executado; caso contrário, o valor é atribuído e o ramo `NOT ON SIZE ERROR` é executado. Também são suportados `INSPECT ... TALLYING / REPLACING / CONVERTING` com `AFTER INITIAL a` e `BEFORE INITIAL b` (um deles ou os dois): a região vai de logo depois do primeiro a até logo antes do primeiro b; se a não for encontrado, a região fica vazia, e se b não for encontrado, vai até o fim. Executamos de fato o Python e o Rust gerados: `ADD 5` dá `OK1`, `ADD 20` (estouro) dá `OVER2`, `DIVIDE 10 BY 0` dá `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` sobre `"AXBXCXDXE"` dá `AXB-C-DXE`, e as duas linguagens produzem a mesma saída. 13 testes unitários passam. **Ainda não suportado**: formas com vários destinos, como `ADD a b TO c d`, não têm verificação de estouro e continuam como `TODO(COBOL_to)`; `FOR CHARACTERS` e `LEADING` também continuam como `TODO(COBOL_to)`. Continua planejado: SORT/MERGE/SEARCH, arquivos indexados e relativos, COMP-3, EXEC SQL/CICS. As listas «Planejado» das demais seções de idioma acima estão desatualizadas; a atualização diária das traduções vai corrigi-las.

### Русский — Обновление (2026-11-21)

*Следующие тексты переведены ИИ и проверены ИИ на естественность звучания; носители языка — люди — их пока не проверяли.*

Теперь преобразуется `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` для `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` и `COMPUTE`: если результат не помещается в число разрядов PICTURE получателя или делитель равен 0, значение не присваивается, а выполняется ветка ошибки; иначе значение присваивается и выполняется ветка `NOT ON SIZE ERROR`. Также поддерживаются `INSPECT ... TALLYING / REPLACING / CONVERTING` с `AFTER INITIAL a` и `BEFORE INITIAL b` (с одним из них или с обоими): область начинается сразу после первого a и заканчивается сразу перед первым b; если a не найдено, область пуста, а если не найдено b — идёт до конца. Мы действительно запустили сгенерированные Python и Rust: `ADD 5` даёт `OK1`, `ADD 20` (переполнение) даёт `OVER2`, `DIVIDE 10 BY 0` даёт `ZERO`; `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` для `"AXBXCXDXE"` даёт `AXB-C-DXE`, и вывод обоих языков совпадает. Проходят 13 модульных тестов. **Пока не поддерживается**: формы с несколькими получателями, например `ADD a b TO c d`, не проверяются на переполнение и остаются как `TODO(COBOL_to)`; `FOR CHARACTERS` и `LEADING` тоже остаются `TODO(COBOL_to)`. По-прежнему планируется: SORT/MERGE/SEARCH, индексированные и относительные файлы, COMP-3, EXEC SQL/CICS. Списки «Планируется» в остальных языковых разделах выше устарели; их исправит ежедневное обновление переводов.

### العربية — تحديث (2026-11-21)

<div dir="rtl" lang="ar">

*النص التالي ترجمه الذكاء الاصطناعي وراجعه الذكاء الاصطناعي من حيث سلامة اللغة، ولم يراجعه بعدُ متحدث أصلي من البشر.*

أصبح `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` مدعومًا الآن في `ADD` و`SUBTRACT` و`MULTIPLY` و`DIVIDE` و`COMPUTE`: إذا تجاوزت النتيجة عدد خانات PICTURE الخاصة بالهدف، أو كان المقسوم عليه 0، فلا تُسند القيمة ويُنفَّذ فرع الخطأ؛ وإلا تُسند القيمة ويُنفَّذ فرع `NOT ON SIZE ERROR`. ويُدعَم كذلك `INSPECT ... TALLYING / REPLACING / CONVERTING` مع `AFTER INITIAL a` و`BEFORE INITIAL b` (أحدهما أو كلاهما): يبدأ النطاق مباشرةً بعد أول a وينتهي مباشرةً قبل أول b؛ فإن لم يوجد a كان النطاق فارغًا، وإن لم يوجد b امتد إلى النهاية. شغّلنا فعليًا شيفرتَي Python وRust المولَّدتين: `ADD 5` يعطي `OK1`، و`ADD 20` (تجاوز) يعطي `OVER2`، و`DIVIDE 10 BY 0` يعطي `ZERO`؛ وتنفيذ `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` على `"AXBXCXDXE"` يعطي `AXB-C-DXE`، وكان ناتج اللغتين متطابقًا. نجح 13 اختبارًا من اختبارات الوحدة. **غير مدعوم بعدُ**: الصيغ ذات الأهداف المتعددة مثل `ADD a b TO c d` لا يُفحَص فيها التجاوز وتبقى موسومة بـ `TODO(COBOL_to)`؛ كما يبقى `FOR CHARACTERS` و`LEADING` موسومَين بـ `TODO(COBOL_to)`. ولا يزال مخططًا: SORT/MERGE/SEARCH، والملفات المفهرسة والنسبية، وCOMP-3، وEXEC SQL/CICS. وقوائم «المخطَّط له» في أقسام اللغات الأخرى أعلاه صارت قديمة، وسيصحّحها التحديث اليومي للترجمات.

</div>

### فارسی (ایران) — به‌روزرسانی (2026-11-21)

<div dir="rtl" lang="fa">

*متن زیر را هوش مصنوعی ترجمه و از نظر روانی زبان بازبینی کرده است؛ هنوز هیچ فارسی‌زبانِ بومیِ انسانی آن را بررسی نکرده است.*

اکنون `ON SIZE ERROR ... NOT ON SIZE ERROR ... END-xxx` در `ADD`، `SUBTRACT`، `MULTIPLY`، `DIVIDE` و `COMPUTE` تبدیل می‌شود: اگر نتیجه از تعداد ارقام PICTURE مقصد بیشتر شود یا مقسوم‌علیه 0 باشد، مقداری انتساب داده نمی‌شود و شاخهٔ خطا اجرا می‌شود؛ در غیر این صورت مقدار انتساب داده می‌شود و شاخهٔ `NOT ON SIZE ERROR` اجرا می‌گردد. همچنین `INSPECT ... TALLYING / REPLACING / CONVERTING` همراه با `AFTER INITIAL a` و `BEFORE INITIAL b` (یکی از آن‌ها یا هر دو) پشتیبانی می‌شود: محدوده درست بعد از اولین a آغاز می‌شود و درست پیش از اولین b پایان می‌یابد؛ اگر a پیدا نشود محدوده خالی است و اگر b پیدا نشود تا انتها ادامه می‌یابد. کدهای تولیدشدهٔ Python و Rust را واقعاً اجرا کردیم: `ADD 5` خروجی `OK1`، `ADD 20` (سرریز) خروجی `OVER2` و `DIVIDE 10 BY 0` خروجی `ZERO` می‌دهد؛ اجرای `REPLACING ALL "X" BY "-" AFTER INITIAL "B" BEFORE INITIAL "D"` روی `"AXBXCXDXE"` نتیجهٔ `AXB-C-DXE` را می‌دهد و خروجی هر دو زبان یکسان بود. ۱۳ آزمون واحد موفق است. **هنوز پشتیبانی نمی‌شود**: شکل‌های دارای چند مقصد، مانند `ADD a b TO c d`، بررسی سرریز ندارند و با `TODO(COBOL_to)` باقی می‌مانند؛ `FOR CHARACTERS` و `LEADING` نیز همچنان `TODO(COBOL_to)` هستند. برنامهٔ باقی‌مانده: SORT/MERGE/SEARCH، فایل‌های اندیس‌دار و نسبی، COMP-3 و EXEC SQL/CICS. فهرست‌های «برنامه‌ریزی‌شده» در بخش‌های زبانی دیگرِ بالا اکنون قدیمی است و به‌روزرسانی روزانهٔ ترجمه‌ها آن را اصلاح خواهد کرد.

</div>
