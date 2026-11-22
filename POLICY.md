# POLICY


## 2026-11-22 現状と完成予定と課題 / Status, plan and issues

### 日本語
**現状**: COBOL・C・C++・Fortran・Java を、選んだ組み合わせ(Axum/Poem/FastAPI/RPoem/RPoRea × HTML5/CSS3/TypeScript/Bootstrap × MySQL/MariaDB/PostgreSQL/SQLite)のウェブアプリにするお助けロボット。`converters/cobol_to` と `converters/java_to` は COBOL_to・Java_to と同じ内容を取り込み(COBOL: SORT/MERGE/SEARCH/EXEC SQL 一式、Java: メソッド参照・Comparator・Collectors)、試験55件成功。`pl2pl c2rust`・`f2rust` で単純な C/C++/Fortran を Rust へ。
**完成予定**: 元の COBOL_to / Java_to の削除(本人確認のうえ最後に)、Mac・iPhone 実機確認、COBOL→他 OS の COBOL の拡張、Kokkos の GPU 実行、メール到達の確認。
**課題**: 取り込み版と元リポジトリを手で同期している(自動同期なし)。C++/Fortran の Rust 化は単純な範囲のみ。Rust 側の EXEC SQL は疑似ハンドラーでの検証にとどまる。

### English
**Status**: A helper robot that turns COBOL, C, C++, Fortran and Java into web apps in the mix you choose (Axum/Poem/FastAPI/RPoem/RPoRea x HTML5/CSS3/TypeScript/Bootstrap x MySQL/MariaDB/PostgreSQL/SQLite). `converters/cobol_to` and `converters/java_to` mirror COBOL_to and Java_to (COBOL: SORT/MERGE/SEARCH/EXEC SQL; Java: method references, Comparator, Collectors); 55 tests pass. `pl2pl c2rust` and `f2rust` rewrite simple C/C++/Fortran into Rust.
**Planned**: deleting the original COBOL_to / Java_to (last, after the owner confirms), real Mac/iPhone checks, extending COBOL to other OSes, Kokkos on a GPU, a mail-arrival check.
**Issues**: The vendored copies are synced by hand (no automatic sync). C++/Fortran to Rust covers only simple code. Rust-side EXEC SQL is verified only with a fake handler.

方針: 新しい REST は作らない(Connect/gRPC)、秘密は書かない、未対応は TODO で残し黙って落とさない、本物で確認してから完了と書く。
Policy: no new REST (Connect/gRPC), never write secrets, keep unsupported items as TODO instead of dropping them, verify on real systems before saying done.
