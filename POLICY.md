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


## 2026-11-22 更新: Collectors 対応の完了と残課題 / Update: Collectors done, what remains

### 日本語
**現状**: Collectors の対応が揃いました: `toList`/`toSet`/`joining`/`toMap`(重複キーの結合関数つき)/`groupingBy`(下流 `counting`・`summingInt`・`mapping`)/`partitioningBy`(`counting` つき)/`averagingInt`。メソッド参照、`Comparator`(`comparing`/`thenComparing`/`reversed`)も対応済み。生成した Python を単体試験で実際に動かして結果を確認(Java_to 23件・pl2pl 55件成功)。
**未了(残る課題)**: 匿名クラス、Rust 側のメソッド本体(今は単純な return のみ)、Spring PetClinic など実プロジェクトでの通し試験、`groupingBy` のその他の下流コレクター(`summingDouble` 以外の `reducing` など)。
**課題**: 中間結果を list で即時評価するため、無限・遅延ストリームは対象外。実プロジェクトでの検証が未了。

### English
**Status**: Collectors coverage is now complete for the common cases: `toList`/`toSet`/`joining`/`toMap` (with a merge function)/`groupingBy` (downstream `counting`, `summingInt`, `mapping`)/`partitioningBy` (with `counting`)/`averagingInt`. Method references and `Comparator` (`comparing`/`thenComparing`/`reversed`) are supported too. Unit tests run the generated Python and check results (Java_to 23, pl2pl 55 pass).
**Still open**: anonymous classes, Rust-side method bodies (currently a simple return only), end-to-end runs on real projects such as Spring PetClinic, and other `groupingBy` downstream collectors (`reducing`, etc.).
**Issues**: intermediate results are evaluated eagerly as lists, so infinite or lazy streams are out of scope; real-project verification is not done.

<!-- directive-20261125-1130 -->
## 2026-11-25 指示(11:30 まで確認不要) / Directive (no confirmation until 11:30)
**日本語**: 2026-11-25 11:30 まで、未了は確認なしで完成まで自動で進める。需要が少ない・必要性が低いと判断される、あるいは1から大規模に作り直す必要があっても、別の方式で済ませず、正面から作り直す。同時に開発する対象: COBOL_to・Java_to・MFastAPI・2MFastAPI(他の言語・フレームワークから MFastAPI 互換へ。Node.js・TypeScript は例外的に選べ、選ばない選択も可能)・RMojo・RPoRea(RPoem と Reaxt の統合)・2RPoRea(他の言語・フレームワークから RPoRea へ)・Reaxt(React+Next.js+Nuxt.js+Nest.js の統合)。1つ変えたら他を確認し、README・CLAUDE・PORTING・POLICY を日英で同じ日に更新する。
**English**: Until 2026-11-25 11:30, carry unfinished work through to completion automatically without asking. Even if demand looks low or a large from-scratch rewrite is needed, do not substitute another approach; do the real work. Developed together: COBOL_to, Java_to, MFastAPI, 2MFastAPI (converts other languages and frameworks to MFastAPI-compatible code; Node.js and TypeScript are an optional exception and choosing neither is allowed), RMojo, RPoRea (RPoem and Reaxt integrated), 2RPoRea (converts other languages and frameworks to RPoRea) and Reaxt (React + Next.js + Nuxt.js + Nest.js integrated). When one changes, check the others and update README, CLAUDE, PORTING and POLICY in Japanese and English the same day.
