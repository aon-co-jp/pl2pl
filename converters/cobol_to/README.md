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
