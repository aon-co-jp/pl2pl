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