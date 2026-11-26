
## Versionless API の方針 / Versionless API policy (2026-11-25)
**日本語**: このリポジトリのAPIは **Versionless**(URL・パッケージ名に `v1` 等を付けず、互換性を壊さず足すだけで進化させる)。欄は足すだけ・足す欄は省略可能・知らない欄は無視・名前変更は新名を足して旧を非推奨に(期限は既定で6か月以上、使用0を確認してから削除)・enumは未知の値を許容・安全上の緊急修正は例外。既存の `.v1` は呼び出し側を壊さないよう当面残し、版なしの別名を追加して段階的に畳む。詳細・出典は [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md) 末尾。適用はこれから順に行う。
**English**: This repository's APIs are **Versionless**: no `v1` in URLs or package names; evolve by adding only, without breaking compatibility. Only add fields, new fields are optional, ignore unknown fields, rename by adding the new name and deprecating the old (default deadline 6 months or more, remove only after confirming zero use), tolerate unknown enum values, emergency security fixes are the exception. Existing `.v1` names stay for now so callers do not break; a versionless alias is added and `.v1` is retired in stages. Details and sources: the end of [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md). Rollout is step by step.

## API 方針: REST 全廃・gRPC / Connect へ
**日本語**: このリポジトリでは **REST API を新規に作らない**。ブラウザ・`curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`)、サービス間は **gRPC / gRPC-web**。GraphQL は読み取り専用。既存の REST は触るときに移し、移したら旧経路は消す。JSON は RS-JSON(`from_str_secure` / `parse_secure`)で読む。例外は相手の規格が形を決めるもの(ACME、OAuth/OIDC のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)のみ。迷ったら Connect。正本は [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md)。
**English**: This repository **does not add new REST APIs**. APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`); service-to-service calls use **gRPC / gRPC-web**. GraphQL is read-only. Existing REST is migrated when touched and the old route is removed. Read JSON with RS-JSON (`from_str_secure` / `parse_secure`). Exceptions are only shapes dictated by someone else's standard (ACME, OAuth/OIDC callbacks, third-party webhooks, health checks, static files, OTLP). When in doubt, use Connect. Source of truth: the link above.


## 2026-11-26 追記: pl2pl f2c(単純な Fortran → C) / Added: pl2pl f2c (simple Fortran to C)
**日本語**: `pl2pl f2c <dir|file> --out <dir>` を追加(program・integer/real・do〈刻み付き〉・if/else・`print *`)。わからない行・配列は `TODO(f2c)` で残しレポートへ。試験2件成功、gcc でビルドし `print *` の整形(整数は幅11)を確認。**未確認**: この WSL に gfortran が無く、本物の gfortran との出力一致は未実施(期待値は gfortran の仕様どおりに合わせただけ)。**未了**: c2f、C↔C++、Rust→C/Fortran、配列・subroutine。
**English**: Added `pl2pl f2c <dir|file> --out <dir>` (program, integer/real, do with step, if/else, `print *`). Unsupported lines and arrays stay as `TODO(f2c)` and go to the report. 2 tests pass; the output builds with gcc and matches the expected `print *` layout (integers width 11). **Unverified**: this WSL has no gfortran, so equality with real gfortran output was not checked. **Open**: c2f, C to/from C++, Rust to C/Fortran, arrays and subroutines.

## 2026-11-26 追記: c2f・C↔C++・Rust→C/Fortran / Added: c2f, C to/from C++, Rust to C/Fortran
**日本語**: 5つの変換を追加: `pl2pl c2f`(C→Fortran)、`c2cpp`(C→C++)、`cpp2c`(C++→C)、`rust2c`(Rust→C)、`rust2f`(Rust→Fortran)。c2f・rust2c・rust2f は共通の小さな中間表現(`src/mini.rs`)を通す: main の中の int/long/double/bool 変数、`for`(C は `for (int i = a; i < b; i++)` 形、Rust は `a..b`/`a..=b`)、while、if/else if/else、printf(%d %ld %lld %f %.Nf %g %s)・puts・`println!`/`print!`(`{}` `{:N}` `{:.N}`)。c2cpp はヘッダーを `<cstdio>` 形へ、C++ 予約語の変数名を `名前_` へ、キャスト無しの malloc を報告。cpp2c は `cout <<` を printf へ(double は cout 既定と同じ %g)、`nullptr`→`NULL`、`bool` 用に `<stdbool.h>`。わからない行(関数・配列・ポインタ・構造体・class・template・std::string/vector・Rust の vec!/メソッド呼び出し・同名の let 等)は `TODO(<変換名>)` で残しレポートへ。単体テスト 7 件追加(全体 68 件成功)。**実際に確認したこと**: `tests/samples/xlang/` の見本を変換し、WSL の gcc・g++・rustc でビルド・実行して出力が完全に一致(c2cpp: gcc↔g++、cpp2c: g++↔gcc、rust2c: rustc↔gcc)。手順は `sh tests/xlang-check.sh <出力dir>`。**未確認**: c2f・rust2f の出力は gfortran がこの WSL に無く(導入に sudo のパスワードが必要)、ビルド・出力比較をしていない。f2c の gfortran 比較も未確認のまま。注意: Rust の小数 `{}` は %g / g0 にしており桁が違う場合がある(レポートに出す)。C の for の終端は毎回評価、Fortran の do は1回だけ評価なので、ループ中に終端を変える C は意味が変わる。**未了**: 関数・配列・構造体・ポインタ、Rust の step_by/rev、Fortran 出力の gfortran 確認。
**English**: Added five converters: `pl2pl c2f` (C to Fortran), `c2cpp` (C to C++), `cpp2c` (C++ to C), `rust2c` (Rust to C) and `rust2f` (Rust to Fortran). c2f, rust2c and rust2f share a small intermediate form (`src/mini.rs`): int/long/double/bool variables inside main, `for` (C in the `for (int i = a; i < b; i++)` shape, Rust `a..b`/`a..=b`), while, if/else if/else, printf (%d %ld %lld %f %.Nf %g %s), puts and `println!`/`print!` (`{}` `{:N}` `{:.N}`). c2cpp moves headers to the `<cstdio>` form, renames variables that are C++ keywords to `name_`, and reports malloc without a cast. cpp2c turns `cout <<` into printf (double uses %g, like cout's default), `nullptr` into `NULL`, and adds `<stdbool.h>` for `bool`. Lines it does not understand (functions, arrays, pointers, structs, class, template, std::string/vector, Rust vec!/method calls, shadowing let, etc.) stay as `TODO(<converter>)` and go to the report. 7 unit tests added (68 pass in total). **Actually checked**: the samples in `tests/samples/xlang/` were converted, built and run with gcc, g++ and rustc in WSL, and the outputs match exactly (c2cpp: gcc vs g++, cpp2c: g++ vs gcc, rust2c: rustc vs gcc). Run it with `sh tests/xlang-check.sh <out dir>`. **Unverified**: c2f and rust2f output was not built or compared, because this WSL has no gfortran (installing needs a sudo password). The f2c comparison with gfortran is still unverified too. Note: Rust's float `{}` is written as %g / g0 and may differ in digits (reported). A C for loop re-evaluates its bound every time while a Fortran do evaluates it once, so C code that changes the bound inside the loop changes meaning. **Open**: functions, arrays, structs, pointers, Rust step_by/rev, checking Fortran output with gfortran.

## 2026-11-26 チェックポイント: c2rust の未対応部分の対応(作業中) / Checkpoint: c2rust remaining features (in progress)

**日本語**: 到達点: `src/c2rust.rs` に、ポインタ変数(生ポインタ `*mut T`、読み書きは `unsafe`)、malloc/calloc/free、文字列(`char s[N]`・`char *`・strlen/strcpy/strcat/strcmp/atoi・printf の %s %c と幅指定)、`#define 名前 値`、ファイル入出力(fopen/fprintf/fputs/fgets/fclose、出力の末尾に小さな実行時関数を付ける)、`int a, b;` の複数宣言、sqrt/pow/fabs/abs/floor などの数学関数、`f(a, a[0])` の借用エラー回避(ほかの引数を先に一時変数へ)を実装しました。ビルドは成功しています。**未確認**: 単体テスト(新しいテスト `pointers_strings_files_macros` を追加済み)の実行と、サンプル `F:\_work_pl2pl_x3\s.c` での gcc と rustc の出力比較はまだです。出力の一致はまだ主張しません。
**再開用メッセージ**: 「F:\pl2pl の `cargo test` を通し、`F:\_work_pl2pl_x3\s.c` を `pl2pl c2rust s.c --out out` で変換して、WSL の gcc と rustc で出力を比べ、違いを直してから README・CLAUDE・PORTING・POLICY に日英で確認済みの事実だけを書いて push してください。`src/main.rs` の他人の未コミット変更は触らないこと。」

**English**: Reached: `src/c2rust.rs` now implements:
- pointer variables (raw pointers `*mut T`, with reads and writes inside `unsafe`)
- malloc/calloc/free
- strings (`char s[N]`, `char *`, strlen/strcpy/strcat/strcmp/atoi, printf `%s`/`%c` and widths)
- `#define NAME value`
- file I/O (fopen/fprintf/fputs/fgets/fclose; a small runtime is appended to the output)
- multiple declarations such as `int a, b;`
- math functions (sqrt, pow, fabs, abs, floor)
- avoiding the borrow error in `f(a, a[0])` (the other arguments go into temporaries first)

It builds. **Not yet verified**: running the unit tests (a new test `pointers_strings_files_macros` was added), and comparing gcc and rustc output for the sample `F:\_work_pl2pl_x3\s.c`. No output match is claimed yet.
**Resume message**: "In F:\pl2pl, make `cargo test` pass. Convert `F:\_work_pl2pl_x3\s.c` with `pl2pl c2rust s.c --out out`, compare gcc and rustc output in WSL, and fix any differences. Then write only verified facts in Japanese and English to README, CLAUDE, PORTING and POLICY, and push. Do not touch the other session's uncommitted changes in `src/main.rs`."

## 2026-11-26 追記: c2rust の未対応部分に対応(gcc と出力一致を確認) / c2rust: remaining features done, output matches gcc

**日本語**: 上の「チェックポイント(作業中)」の続きで、確認が終わりました。`pl2pl c2rust` が次のものに対応しました。
- ポインタ変数(`int *p = &v;`、`int *q = c;`、`q[1]`、`q = q + 2`、`*p`)。Rust の生ポインタ `*mut T` に置き換え、読み書きは `unsafe` で囲みます。
- malloc/calloc/free。
- 文字列: `char s[N] = "..."`、`char *`、strlen/strcpy/strcat/strcmp/atoi、printf の `%s` `%c` と幅・左寄せ(`%-8s`、`%5d`)。
- `#define 名前 値`(配列の大きさにも使えます)。
- ファイル入出力: fopen/fprintf/fputs/fgets/fclose。出力の末尾に小さな実行時関数を付けます。
- `int a, b = 7, c[N];` のような複数宣言。
- 数学関数: sqrt/pow/fabs/abs/floor/ceil/sin/cos/exp/log など。
- `f(d, d[2])` の借用エラー回避: ほかの引数を先に一時変数へ移します。

これらを使った C のサンプル(`F:\_work_pl2pl_x3\s.c`)を WSL の gcc と rustc で動かしました。標準出力 13 行と書き出したファイル `out.txt` の中身が、どちらも完全に一致しました。前回のサンプル(構造体・2次元配列など)も引き続き一致しています。単体テストは 69 件すべて成功です。

まだできないこと:
- 関数形式のマクロ、ポインタのポインタ、scanf/fscanf。
- `/* */` 形式のコメント、`%x` などの書式。
- 文字列リテラルを書き換えること(C でも未定義の動作です)。

ポインタを使う部分は `unsafe` になるので、安全な Rust への手直しは人が行う前提です。

**English**: This continues the "checkpoint (in progress)" above; verification is now done. `pl2pl c2rust` now handles:
- Pointer variables (`int *p = &v;`, `int *q = c;`, `q[1]`, `q = q + 2`, `*p`). They become Rust raw pointers `*mut T`, and reads and writes are wrapped in `unsafe`.
- malloc/calloc/free.
- Strings: `char s[N] = "..."`, `char *`, strlen/strcpy/strcat/strcmp/atoi, and printf `%s`, `%c`, widths and left alignment (`%-8s`, `%5d`).
- `#define NAME value`, which also works as an array size.
- File I/O: fopen/fprintf/fputs/fgets/fclose, through a small runtime appended to the output.
- Multiple declarations such as `int a, b = 7, c[N];`.
- Math functions: sqrt/pow/fabs/abs/floor/ceil/sin/cos/exp/log and others.
- Avoiding the borrow error in `f(d, d[2])`: the other arguments go into temporaries first.

A C sample using all of these (`F:\_work_pl2pl_x3\s.c`) was compiled with gcc and rustc in WSL. The 13 lines of standard output and the written file `out.txt` match exactly. The previous sample (structs, 2-D arrays and so on) still matches. All 69 unit tests pass.

Not supported yet:
- Function-like macros, pointers to pointers, scanf/fscanf.
- `/* */` comments and formats such as `%x`.
- Writing to string literals (undefined behavior in C too).

Code that uses pointers becomes `unsafe`; turning it into safe Rust is left to a person.

## 2026-11-27 追記: c2rust の `/* */` コメントと %x %X %o / Update: block comments and %x %X %o in c2rust
**日本語**: `pl2pl c2rust` が `/* ... */` コメント(複数行・文字列の中は残す)と printf の `%x`・`%X`・`%o`(幅・`0`埋め込みつき)に対応。`tests/samples/c2rust/hexcomment.c` を gcc と rustc で動かし出力3行が完全一致、`tests/c2rust-check.sh` で再確認できる。単体テスト69件成功。**未対応**: 引数つきマクロ、ポインターのポインター、scanf、`%#x`(代替形式)。
**English**: `pl2pl c2rust` now handles `/* ... */` comments (multi-line; text inside strings is kept) and printf `%x`, `%X` and `%o` (with width and zero padding). `tests/samples/c2rust/hexcomment.c` run with gcc and rustc gives identical 3-line output; rerun with `tests/c2rust-check.sh`. 69 unit tests pass. **Not yet**: function-like macros, pointer-to-pointer, scanf, and `%#x` (alternate form).

## 2026-11-27 追記(2): c2rust の引数つきマクロと三項演算子 / Update (2): function-like macros and the ternary operator in c2rust
**日本語**: `pl2pl c2rust` が引数つきマクロ(`#define SQ(x) ((x)*(x))`、呼び出し箇所へ文字列として展開、引数は括弧で包む、定義の行は空行にして行番号を保つ)と三項演算子 `c ? a : b`(数値どうし・同じ型どうし。Rust の `if c { a } else { b }` へ)に対応。`tests/samples/c2rust/macro.c`(4行出力)と `hexcomment.c` が gcc と rustc で完全一致、単体テスト70件成功。**限界**: マクロは文字列の置換なので、引数を2回評価するマクロ(`MAX2(a++, b)` など)は C と同じく副作用が重なる。ポインター型どうしや構造体の三項演算子は未対応(TODO で残る)。**未対応**: ポインターのポインター、scanf、`%#x`。
**English**: `pl2pl c2rust` now expands function-like macros (`#define SQ(x) ((x)*(x))` is substituted textually at call sites, arguments wrapped in parentheses, the definition line blanked to keep line numbers) and handles the ternary `c ? a : b` (numeric or same-type operands, emitted as Rust `if c { a } else { b }`). `tests/samples/c2rust/macro.c` (4 output lines) and `hexcomment.c` match gcc vs rustc exactly; 70 unit tests pass. **Limits**: macros are textual, so a macro that evaluates an argument twice (such as `MAX2(a++, b)`) repeats side effects just like in C. Ternaries on pointers or structs are not supported (left as TODO). **Not yet**: pointer-to-pointer, scanf, `%#x`.
