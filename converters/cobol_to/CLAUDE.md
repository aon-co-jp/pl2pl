# COBOL_to — 開発メモ / Development notes

**日本語 → English**

## 日本語

### 役割
COBOL(各方言・各版・固定/自由形式)を自動判定し、Python(FastAPI/MFastAPI)または Rust(Poem/RPoem)のプロジェクトへ変換するツール。作業場所は `F:\COBOL_to`、GitHub は `aon-co-jp/COBOL_to`(公開、MIT)。`COBOL2` → `COBOL-to` → `COBOL_to` と改名した経緯があるが、**履歴は新規**(旧名は残っていない)。

### 構成
- `src/detect.rs` 方言・規格・書式の判定(キーワードと桁位置の根拠つき)
- `src/convert.rs` 字句解析 → 共通 IR(`Program`/`Var`/`Stmt`)→ `to_rust` / `to_python`。単体テストは `hello_roundtrip`
- `src/main.rs` フォルダ選択(`rfd`)、変換先の選択、`.cbl/.cob/.cobol/.pco` の再帰走査、雛形生成、`COBOL_to-REPORT.md`
- `installer/` Windows(`cobol_to.iss`)、Linux(`build-deb.sh`)、macOS(`build-pkg.sh`)。`.github/workflows/release.yml` が全 OS を作って試験
- `NOTIFY.md` / `notify.example.toml` 更新通知(version-control の `[notify] email_to` に 2 件)

### 方針(守ること)
- **REST は作らない**。API は Connect(`POST /cobol_to.Program/Run`)。JSON は RS-JSON に寄せる方針(現状は未使用の小さなコード)。
- **未対応の構文は黙って落とさない**(`TODO(COBOL_to)`)。生成物の依存は**固定しない**。
- RPoem 向けには Bootstrap・TypeScript・Next.js・Node.js を**使わない**(RPoem は Rust→WASM)。
- Windows 配布は `.exe`(Inno Setup)。PowerShell スクリプトを利用者向けの入口にしない。
- 走査の起点(`root`/`owner`)は各ユーザーが指定する。`F:/` や `aon-co-jp` は作者の例。
- メールアドレスは `config.toml`(git 管理外)に置き、リポジトリへ入れない。
- 文書は日本語と英語(通知文書のみ日英独露。ドイツ語・ロシア語は AI 翻訳でネイティブ検証前)。

### 環境の落とし穴
- この PC の Bash では `cargo` が PATH に無い。**PowerShell で `$env:PATH="$env:USERPROFILE\.cargo\bin;$env:PATH"`** を前置きする(WSL の Ubuntu には cargo あり)。
- **`cargo test` では `target\debug\cobol_to.exe` は更新されない**。変更後の実行確認は `cargo build` してから。
- Bash の heredoc に `'` を含む長い内容を入れると構文エラーになる。ファイルは Write ツールで作る。
- PowerShell 5.1 は `git push` の進行表示を赤い「エラー」で出す(終了コードと `git status -sb` で判断)。日本語を含むファイルを PowerShell で書かない(文字化け)。
- Windows で `Select-String` を一時出力フォルダのディレクトリに向けるとアクセス拒否になる(ファイルを個別に指定する)。

### 状態(2026-11-18)
- 手元で確認済み: テスト成功、Windows x64 のインストール/実行/アンインストール、Linux amd64 の `dpkg -i`/実行/`-r`(WSL)。
- **現在: `v0.1.2` の Release 公開済み、CI 全6ジョブ成功**(v0.1.1 は Linux 試験スクリプトの `command -v` の hash 残りと macOS arm64 の runner `macos-14` 未対応で失敗 → 552c5cb で修正)。以下は経緯:
- **v0.1.0 は失敗(Release 無し・タグだけ残る)**: Linux は CI に wayland 開発ライブラリが無くビルド失敗(実行時も `libwayland-client.so.0` に依存)、macOS x64 の runner `macos-13` は廃止で待機のまま。Windows x64/ARM64 は成功。修正(b2180c8・da0cf50)して **`v0.1.1` を push、リリース CI(run 44166948447)を実行中**。Linux arm64・macOS は v0.1.1 が初めての確認。タグは付け直さず次は `v0.1.2` で出す。
- 教訓: 手元(WSL)に開発ライブラリが入っていると CI の欠落に気づけない。`ldd` で実行時依存を調べて `.deb` の `Depends` に入れる。
- version-control は `root = "F:/"` なので `F:\COBOL_to` を自動で走査する(動作確認は走査が長く未了)。通知メールは `email_to` の 2 件(手元と VPS の両方の `config.toml`)。VPS で `version-control.service` を実行して `e-mail: sent` を確認、受信箱への到達は未確認、LINE は未設定。**2 件とも届いていない(利用者報告)**: SMTP は 3 回とも受理(`SMTP send failed` 0 件)でコードのバグではない。送信元は `smtp.lolipop.jp` 経由の `aon.co.jp` だが、`aon.co.jp` には **TXT(SPF)も `_dmarc` も無い**(DNS は value-domain)ため、Gmail に迷惑メール扱い/破棄されている疑い。迷惑メール欄・送信元受信箱の返送を確認し、SPF(Lolipop 指定値を確認)・DMARC を追加する(DNS は利用者の操作)。

## English

### Role
A tool that auto-detects COBOL (dialects, standards, fixed/free format) and converts to Python (FastAPI/MFastAPI) or Rust (Poem/RPoem) projects. Working copy `F:\COBOL_to`; GitHub `aon-co-jp/COBOL_to` (public, MIT). It was renamed `COBOL2` → `COBOL-to` → `COBOL_to`, but the **history is fresh** (no old names remain).

### Layout
- `src/detect.rs` dialect/standard/format detection with evidence
- `src/convert.rs` tokenizer → shared IR (`Program`/`Var`/`Stmt`) → `to_rust` / `to_python`; unit test `hello_roundtrip`
- `src/main.rs` folder picker (`rfd`), target choice, recursive scan of `.cbl/.cob/.cobol/.pco`, scaffolding, `COBOL_to-REPORT.md`
- `installer/` Windows (`cobol_to.iss`), Linux (`build-deb.sh`), macOS (`build-pkg.sh`); `.github/workflows/release.yml` builds and tests every OS
- `NOTIFY.md` / `notify.example.toml` update notifications (two addresses in version-control's `[notify] email_to`)

### Rules
- **No REST.** The API is Connect (`POST /cobol_to.Program/Run`). JSON should move to RS-JSON (currently not used by this small code).
- **Never drop unsupported syntax silently** (`TODO(COBOL_to)`). Do **not pin** dependencies of generated projects.
- No Bootstrap/TypeScript/Next.js/Node.js for the RPoem target (RPoem is Rust→WASM).
- Windows ships as `.exe` (Inno Setup); PowerShell scripts are not a user entry point.
- The scan start points (`root`/`owner`) are specified by each user; `F:/` and `aon-co-jp` are the author's examples.
- E-mail addresses live in `config.toml` (git-ignored), never in the repository.
- Docs are Japanese and English (only the notification doc is JA/EN/DE/RU; German and Russian are AI translations, not native-checked).

### Environment pitfalls
- `cargo` is not on PATH in this PC's Bash. Use **PowerShell with `$env:PATH="$env:USERPROFILE\.cargo\bin;$env:PATH"`** (WSL Ubuntu has cargo).
- **`cargo test` does not refresh `target\debug\cobol_to.exe`**; run `cargo build` before checking behaviour.
- Long Bash heredocs containing `'` fail to parse; create files with the Write tool.
- PowerShell 5.1 shows `git push` progress as a red "error" (judge by exit code and `git status -sb`); do not write Japanese files from PowerShell (mojibake).
- `Select-String` on a directory in a temp output folder gives access-denied (name files individually).

### Status (2026-11-18)
- Verified locally: tests pass, Windows x64 install/run/uninstall, Linux amd64 `dpkg -i`/run/`-r` (WSL).
- **Now: Release `v0.1.2` is published and all 6 CI jobs passed** (v0.1.1 failed on a stale bash hash in the Linux smoke test's `command -v` and on the unsupported macOS arm64 runner `macos-14`; fixed in 552c5cb). History:
- **v0.1.0 failed (no Release; only the tag remains)**: Linux failed to build because CI lacked the wayland dev libraries (the binary also needs `libwayland-client.so.0` at run time), and the macOS x64 runner `macos-13` is retired and stayed queued. Windows x64/ARM64 passed. Fixed (b2180c8, da0cf50), **`v0.1.1` is pushed and its release CI (run 44166948447) is running**. Linux arm64 and macOS are first verified by v0.1.1. Do not move tags; ship the next fix as `v0.1.2`.
- Lesson: dev libraries present locally (WSL) hide gaps in CI. Check run-time deps with `ldd` and list them in the `.deb` `Depends`.
- version-control uses `root = "F:/"`, so `F:\COBOL_to` is scanned automatically (a live check was not finished; the scan is long). Notification mail goes to the two `email_to` addresses (both the local and the VPS `config.toml`). Running `version-control.service` on the VPS logged `e-mail: sent`; delivery to the inboxes is unconfirmed; LINE is not configured. **The user reports neither mail arrived**: SMTP accepted all three sends (0 `SMTP send failed`), so it is not a code bug. The sender is `aon.co.jp` via `smtp.lolipop.jp`, but `aon.co.jp` has **no TXT (SPF) record and no `_dmarc`** (DNS at value-domain), so Gmail probably files it as spam or drops it. Check the spam folder and bounces in the sender mailbox, then add SPF (confirm Lolipop's value) and DMARC (DNS is the user's action).


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
