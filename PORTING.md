# pl2pl 引き継ぎ
(むかしの細かい記録は `docs/HISTORY-ja.md`)

## いまの状態(2026-11-19)
- v0.1.0 を公開済み。Windows・Linux・Mac のインストーラーと、CI のテストがあります。
- PHP → FastAPI は、ルート・Eloquent・クエリビルダ・Blade・ミドルウェア・リレーション・`@can` 系まで、本物の FastAPI で通し試験済みです。
- `pl2pl web` が PHP も見分けます。
- `pl2pl watch`: 新しい版を見つけ、テストし、直し、だめなら通知します。GitHub の毎晩・毎朝の実行でも新しい版でテストしています。
- `pl2pl ir`(試作): PHP・Python・Java(Spring・JAX-RS・Servlet・Spark)のルートを共通の形にして、JSON・OpenAPI・FastAPI のひな形を出します。IR は型つき引数・関数の本体・モデル・ミドルウェア・テンプレートも持ち、`pl2pl php` の出力は `src/ir_emit.rs` の共通の書き出しを通ります(出力は変更前と同一、通し試験 49 件)。
- 紹介・ダウンロードのページを公開: <https://easy-web.tokyo/pl2pl/>(VPS、静的)。easy-web.tokyo のトップにも紹介カードがあります。
- COBOL_to・Java_to は `converters/` に取り込み済みです。

## まだのこと(優先順)
1. **確認**: 実際のメールの着信(`version-control --test-notify`。この PC に SMTP の設定が無い)、version-control の `--loop` による自動修正、fastapi 最新版でのテスト(GitHub の毎晩の実行が確かめる)。
2. (済み・ただし薄い)IR に型つき引数・依存・本体(`Body`)・モデル(`Model`/`Relation`)・ミドルウェア・テンプレート(`Template`)をのせ、`pl2pl php` の出力は `ir_emit.rs` の共通出力を通す(出力は変更前と同一、単体40件・e2e 49件成功)。ただしテンプレートは変換済みの Jinja2 文字列を持つだけで、Blade のノード木ではない(ほかの言語・出力先へ回すなら木にする必要あり)。ルート抽出が取りこぼすなら tree-sitter を検討。
3. `watch` を本物の新しい版で試した: fastapi 0.165.0(starlette 1.10)で `tests/run-php.sh` が 49 件成功したので、registry の tested を 0.165 へ進めた。残り: laravel 13 / django 6.1 / spring-boot 4.1 の新版は、通し試験が実物のフレームワークを使わないため「未検証」のまま(実物を使う試験を作るまで報告のみ)。
4. 毎晩・毎朝の自動実行の登録(この PC では確認コマンドが動かず誤報になるので未登録。登録の例は `scripts/pl2pl-watch.cmd` の先頭)。
5. (済み)easy-web.tokyo のトップページに pl2pl の紹介カードを足し、本番(VPS の open-easy-web)へ反映しました。open-easy-web `95cb1e7`、旧バイナリは `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`。
6. PHP(済み・限りあり): Blade の `@php` は代入と `++` `--` `+=` `-=` `*=` `/=` `.=` まで `{% set %}` に変換、`$attributes` は出力・`->merge`(値は文字列か変換できる式)・`->class([...])`(文字列の並び)・`->only`・`->except`、`morphToMany`(中間表に型列)、クラスで作る Blade の部品(`app/View/Components/*.php` のコンストラクタ引数を既定値と `_props` にし、`render()` の `view('x.y')` も追う)を実装した(通し試験 55 件、単体 41 件)。残り: クラス部品の公開メソッド(テンプレートの `{{ $メソッド() }}`)は TODO のまま(レポートに件数を出す)、`->class()` のキー付き(条件つき)、`@php` の if/foreach や関数呼び出し(TODO のまま)。
7. aruaru-llm との連携(済み): 2026-11-19 に aruaru-llm v0.2.5 を取り込み、`pl2pl llm setup` で実際に導入した。自動選択の 3B は重みが分割されていて 404 で失敗したため、失敗したら一段小さいモデルへ自動で下げる(固定指定のときは下げない)。qwen2.5-1.5b-instruct(約 2.9GB)で `GenerateQwen` が返ることを確認(CPU)。**`pl2pl php --explain`**(任意、`--explain-max N` 既定 3)は、レポートの TODO 項目ごとに AI の説明を「参考」として足す。実測: 1 件に約 2 分(CPU)、**説明の質は低く間違いも出る**(2 件中 1 件は的外れ)ので既定はオフ。起動し直した aruaru-llm はモデルが未選択(400)なので、説明の前に導入済みモデルを選び直す。**もっと大きいモデル(約 12GB)**: aruaru-llm のカタログの最大は 3B(約 6.2GB)で、3B・7B は重みが複数ファイルに分割されていて今の導入処理では取れない。分割重みへの対応(aruaru-llm 側)と、CPU の速度(1.5B でもこの遅さ)の見直しが先。Kokkos の GPU 実行、open-cuda / open-directx でのカーネル実行は未確認。 **GPU(2026-11-19 確認)**: GeForce GT 730(2GB、sm_35、ドライバ 475 = CUDA 11.4 相当)に、管理者権限なしで CUDA 11.8 の nvcc を micromamba(NVIDIA の conda チャンネル)で入れ、素の CUDA カーネルと `pl2pl gpu --target open-cuda` の出力(`vec_loop12.cu`)を実際の GPU で動かし、CPU と不一致 0 を確認した(`tests/gpu-check.ps1`)。落とし穴: MSVC 14.44 には `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` と `-allow-unsupported-compiler` が要る、PyPI には Windows 用の nvcc が無い、CUDA 12 系は sm_35 不可。**未確認のまま**: Kokkos の CUDA バックエンド、open-cuda(Rust 実装)・open-directx 上での実行。
8. 元の COBOL_to・Java_to の削除(**取り返しがつかないので、最後の確認を取ってから**)。
9. `docs/DESIGN.md` は持ち主が書く。`docs/LANGUAGES.md` は 2026-11-19 に中身を見直し、いまの状態(Java・COBOL は変換器あり)と食い違いはなかった(ただし知識ベースの一覧で、検索による最新性の確認はまだ)。


## テスト
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 通し)/ `cargo test --release`

## 再開用メッセージ
「F:\pl2pl を再開。PORTING.md の『まだのこと』を上から進める。書きかえられない所はこっそり消さず TODO で残す。実際に動かして確かめてから完了と報告する。返事は日本語のみ。区切りごとにコミットして push、VPS にも反映。」

## 2026-11-19 いまの状況と、これから作るもの / Where we are now, and what comes next

### 日本語(やさしく)
**いまできること**: pl2pl は、PHP(Laravel など)や Python や Java で書かれたウェブアプリを、Python の FastAPI に作り直すお助けロボットです。道順(ルート)、データの入れ物(モデル)、見た目の部品(Blade の画面)、見張り番(ミドルウェア)まで変えられます。通し試験は 55 項目すべて合格です。
**新しく直したところ**: 「新しい版が出たら自動で確かめる」機能(`pl2pl watch`)が、古い道具のまま試していたので、合格しても本当の確認になっていませんでした。いまは、毎回**まっさらな部屋(新しい venv)に最新版を入れてから**試験します。fastapi 0.165 で本当に通ることを確かめました。Windows では WSL の中で動かすので、まちがった警報も出ません。
**これからやること(確認はいりません。自動で順に進めます)**:
1. 毎晩・毎朝の自動実行の登録(まちがい警報が出ないことを確かめてから)
2. メールが本当に届くかの確認(このPCにはメール送信の設定がありません)
3. aruaru-llm・Kokkos・open-cuda・open-directx での GPU 実行の確認
4. PHP の細かい残り(クラス部品のメソッドなど)
5. 8 か国語の文書を最新に合わせる
**人に聞くこと**: 元の COBOL_to / Java_to を消す作業だけは、元に戻せないので、最後に必ずあなたに聞きます。
**ルール**: 「未了(まだ終わっていないこと)は確認せず、自動で進める」と決まっています。

### English (simple words, easy to read)
**What it can do now**: pl2pl is a helper robot. It rebuilds web apps written in PHP (like Laravel), Python or Java into Python FastAPI. It can change routes, data models, Blade screens and middleware. All 55 end-to-end checks pass.
**What we fixed today**: The "check new versions by itself" feature (`pl2pl watch`) used old tools for its tests. So a pass did not really prove anything. Now every check starts in a **clean new room (a fresh venv), installs the newest version, and then runs the tests**. We confirmed it really passes on fastapi 0.165. On Windows it runs inside WSL, so false alarms are gone.
**What comes next (no need to ask; we go on by ourselves, in order)**:
1. Register the nightly and morning runs (after we are sure there are no false alarms)
2. Check that e-mail really arrives (this PC has no mail settings)
3. Check GPU runs with aruaru-llm, Kokkos, open-cuda and open-directx
4. Small PHP leftovers (class-component methods, and so on)
5. Bring the 8 other language docs up to date
**What we still ask a person**: deleting the original COBOL_to / Java_to cannot be undone, so we always ask you first.
**Rule**: "Unfinished work goes on automatically, without asking." This is the standing rule.

- 2026-11-19 追記 / Update: 夜 03:00・朝 07:00 の自動実行を登録し、手動実行で exit=0(誤警報なし)を確認。/ Nightly (03:00) and morning (07:00) runs are registered; a manual run ended with exit=0 and no false alarm. 次: メール到達・GPU(Kokkos等)・PHP残り。Next: mail arrival, GPU (Kokkos etc.), PHP leftovers.

- 2026-11-19 追記 / Update: Kokkos の CUDA 実行はこのPCでは不可(GT 730=Kepler は WSL の GPU 対象外・Kokkos 新版は sm_35 非対応)。CPU 版(OpenMP/Serial)と Windows の CUDA 11.8 直接実行は確認済み。別の新しい GPU の機械で試す。/ Kokkos CUDA cannot run on this PC (GT 730 Kepler is not supported by WSL GPU, and recent Kokkos drops sm_35). CPU backends and direct CUDA 11.8 on Windows are verified; try on a newer GPU machine. 大規模モデル側: 実3B(2分割)がCPUで動作(約0.6語/秒)。/ Large-model side: the real 3B (2 shards) runs on CPU (about 0.6 tokens/s).

## 最優先機能 / Top-priority feature: 組み合わせを選べる変換 (`pl2pl web --backend ...`) (2026-11-19, 10 languages)

```
pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea>
                [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>
```
COBOL は変換器で、C / C++ / Fortran は「そのままビルドして呼ぶ橋渡し」(`legacy/build.sh`)で、選んだ組み合わせのアプリの骨組みになります。Axum 版は実際に起動して呼び出しの成功を確認済みです(COBOL と C/C++)。RPoem / RPoRea は Bootstrap・TypeScript なし(Rust→WASM)。

### 日本語
やさしく言うと、「古いプログラム(COBOL・C・C++・Fortran)を、今のウェブのしくみに乗せる」機能です。サーバーは Axum・Poem・FastAPI・RPoem・RPoRea から、画面は HTML5・CSS3・TypeScript・Bootstrap から、データベースは MySQL・MariaDB・PostgreSQL・SQLite から、好きな組み合わせを選べます。COBOL は書き換えて変換します。C・C++・Fortran は元のまま動かし、ウェブから呼べるようにします(まだ Rust への書き換えはしません)。メインフレーム・UNIX・Linux・Windows・Mac の COBOL を別の OS の COBOL へ直す機能は `pl2pl cobol`(IBM→GnuCOBOL)にあります。
### English
In simple words: it puts old programs (COBOL, C, C++, Fortran) onto today's web. You pick the server (Axum, Poem, FastAPI, RPoem or RPoRea), the screen (HTML5, CSS3, TypeScript, Bootstrap) and the database (MySQL, MariaDB, PostgreSQL or SQLite) in any allowed mix. COBOL is rewritten. C, C++ and Fortran keep running as they are, and the web calls them (they are not rewritten into Rust yet). Turning COBOL from one OS into COBOL for another OS is `pl2pl cobol` (IBM to GnuCOBOL).
### 简体中文
简单地说:把旧程序(COBOL、C、C++、Fortran)放到今天的网页上。服务器可选 Axum、Poem、FastAPI、RPoem、RPoRea;界面可选 HTML5、CSS3、TypeScript、Bootstrap;数据库可选 MySQL、MariaDB、PostgreSQL、SQLite,自由组合。COBOL 会被改写;C、C++、Fortran 保持原样运行,由网页调用(暂不改写成 Rust)。把 COBOL 从一个系统转到另一个系统用 `pl2pl cobol`。
### 繁體中文
簡單地說:把舊程式(COBOL、C、C++、Fortran)放到現在的網頁上。伺服器可選 Axum、Poem、FastAPI、RPoem、RPoRea;畫面可選 HTML5、CSS3、TypeScript、Bootstrap;資料庫可選 MySQL、MariaDB、PostgreSQL、SQLite,自由搭配。COBOL 會被改寫;C、C++、Fortran 維持原樣執行,由網頁呼叫(暫不改寫成 Rust)。把 COBOL 從一個系統轉到另一個系統用 `pl2pl cobol`。
### 한국어
쉽게 말하면, 오래된 프로그램(COBOL, C, C++, Fortran)을 지금의 웹에 올려 주는 기능입니다. 서버는 Axum, Poem, FastAPI, RPoem, RPoRea 중에서, 화면은 HTML5, CSS3, TypeScript, Bootstrap 중에서, 데이터베이스는 MySQL, MariaDB, PostgreSQL, SQLite 중에서 마음대로 조합할 수 있습니다. COBOL은 다시 써서 변환하고, C·C++·Fortran은 그대로 실행하면서 웹에서 부릅니다(아직 Rust로 다시 쓰지는 않습니다). COBOL을 다른 OS의 COBOL로 바꾸는 일은 `pl2pl cobol`이 합니다.
### Deutsch
Einfach gesagt: Alte Programme (COBOL, C, C++, Fortran) werden ins heutige Web gebracht. Man wählt Server (Axum, Poem, FastAPI, RPoem oder RPoRea), Oberfläche (HTML5, CSS3, TypeScript, Bootstrap) und Datenbank (MySQL, MariaDB, PostgreSQL oder SQLite) frei im erlaubten Rahmen. COBOL wird umgeschrieben; C, C++ und Fortran laufen unverändert und werden vom Web aus aufgerufen (noch nicht nach Rust umgeschrieben). COBOL für ein anderes Betriebssystem erzeugt `pl2pl cobol`.
### Français
En termes simples : on met de vieux programmes (COBOL, C, C++, Fortran) sur le web d'aujourd'hui. On choisit le serveur (Axum, Poem, FastAPI, RPoem ou RPoRea), l'écran (HTML5, CSS3, TypeScript, Bootstrap) et la base de données (MySQL, MariaDB, PostgreSQL ou SQLite) dans les combinaisons permises. Le COBOL est réécrit ; C, C++ et Fortran tournent tels quels et sont appelés par le web (pas encore réécrits en Rust). Pour du COBOL d'un autre système, voir `pl2pl cobol`.
### Español
En palabras sencillas: lleva programas antiguos (COBOL, C, C++, Fortran) a la web de hoy. Eliges el servidor (Axum, Poem, FastAPI, RPoem o RPoRea), la pantalla (HTML5, CSS3, TypeScript, Bootstrap) y la base de datos (MySQL, MariaDB, PostgreSQL o SQLite) con las combinaciones permitidas. El COBOL se reescribe; C, C++ y Fortran siguen funcionando tal cual y la web los llama (todavía no se reescriben en Rust). Para COBOL de otro sistema operativo, usa `pl2pl cobol`.
### Português
Em palavras simples: leva programas antigos (COBOL, C, C++, Fortran) para a web de hoje. Você escolhe o servidor (Axum, Poem, FastAPI, RPoem ou RPoRea), a tela (HTML5, CSS3, TypeScript, Bootstrap) e o banco de dados (MySQL, MariaDB, PostgreSQL ou SQLite) nas combinações permitidas. O COBOL é reescrito; C, C++ e Fortran continuam rodando como estão e a web os chama (ainda não são reescritos em Rust). Para COBOL de outro sistema, use `pl2pl cobol`.
### Русский
Простыми словами: старые программы (COBOL, C, C++, Fortran) переносятся в сегодняшний веб. Сервер выбирается из Axum, Poem, FastAPI, RPoem, RPoRea; интерфейс — из HTML5, CSS3, TypeScript, Bootstrap; база данных — из MySQL, MariaDB, PostgreSQL, SQLite, в допустимых сочетаниях. COBOL переписывается; C, C++ и Fortran работают как есть, а веб их вызывает (в Rust пока не переписываются). COBOL для другой ОС делает `pl2pl cobol`.

**確認したこと / Verified**: Axum(COBOL 7本・C・C++)を起動して `POST /cobol_to.Program/Run` が正しい出力を返す。全組み合わせの検査は単体試験。**まだ / Not yet**: gfortran の実機ビルド(この PC の WSL に無い)、C/C++/Fortran→Rust の書き換え、COBOL→別 OS の COBOL の拡張、Axum 以外の起動確認。**進め方 / How we work**: 未了は確認せず自動で進める(元のリポジトリの削除だけは最後に確認)。 / Unfinished items go on automatically without asking (only deleting the original repos is confirmed first).

## 2026-11-19 大きなAIを小さなパソコンで動かす / Running big AI on a small PC (10 languages, easy words)

### 日本語
**できたこと**: 大きなAIは、重さが「いくつかのファイル」に分かれていることがあります。それを自動でぜんぶダウンロードして、読めるようになりました。本物のAI(Qwen 3B、約6GB)を、GPUのないふつうのパソコンで動かして、文章を作れました。ちょっと遅いけれど、ちゃんと動きます。
**わかったこと**: 東芝や富士通の「最適な組み合わせを探す機械」は、AIを直接速くする道具ではありません。「どの部品を何ビットに縮めるか」を決める手伝いに使えるかもしれません。作ったあとの記憶の圧縮(MLA)は、品質が落ちるので使えませんでした。
**これから**: AIの重さを小さい数で持って、もっと大きなAI(約12GB)も動かします。「確認しないで進める」ので、止まらずに作って、試して、直します。

### English
**Done**: Big AI models often come in several files. The program now downloads every part and loads them. We ran a real AI (Qwen 3B, about 6 GB) on an ordinary PC without a GPU, and it wrote text. It is a bit slow, but it works.
**Learned**: The "best-combination finder" machines from Toshiba and Fujitsu do not make AI faster by themselves. They might help decide how many bits each part gets. Squeezing the memory afterwards (MLA) hurt quality, so we cannot use it.
**Next**: Keep the AI's weights in small numbers so even bigger AI (about 12 GB) can run. We go on without asking each time: build, test and fix again and again.

### 简体中文
**已完成**:大型AI的权重常被拆成好几个文件。现在程序能自动全部下载并读取。我们在没有显卡的普通电脑上运行了真正的AI(Qwen 3B,约6GB),它能写出文字。虽然有点慢,但是能用。
**发现**:东芝和富士通那种"找最佳组合的机器"不能直接让AI变快,也许能帮忙决定每个部分用几位数。事后压缩记忆(MLA)会让质量变差,所以不能用。
**接下来**:用更小的数字保存权重,让更大的AI(约12GB)也能运行。我们不会每次都问,而是不停地做、测、改。

### 繁體中文
**已完成**:大型AI的權重常被拆成好幾個檔案。現在程式能自動全部下載並讀取。我們在沒有顯示卡的一般電腦上執行了真正的AI(Qwen 3B,約6GB),它能寫出文字。雖然有點慢,但可以用。
**發現**:東芝和富士通那種「找最佳組合的機器」不能直接讓AI變快,也許能幫忙決定每個部分用幾位元。事後壓縮記憶(MLA)會讓品質變差,所以不能用。
**接下來**:用更小的數字保存權重,讓更大的AI(約12GB)也能執行。我們不會每次都詢問,而是不停地做、測、改。

### 한국어
**해낸 일**: 큰 AI는 무게(가중치)가 여러 파일로 나뉘어 있을 때가 많아요. 이제 프로그램이 전부 자동으로 내려받아 읽을 수 있어요. 그래픽카드가 없는 보통 컴퓨터에서 진짜 AI(Qwen 3B, 약 6GB)를 돌려 글을 만들었어요. 조금 느리지만 잘 돼요.
**알게 된 것**: 도시바와 후지쯔의 "가장 좋은 조합을 찾는 기계"는 AI를 직접 빠르게 하지는 않아요. 각 부분을 몇 비트로 줄일지 정하는 데 도움이 될 수는 있어요. 나중에 기억을 줄이는 방법(MLA)은 품질이 떨어져서 쓸 수 없었어요.
**앞으로**: 무게를 더 작은 숫자로 들고 있어서 더 큰 AI(약 12GB)도 돌릴 거예요. 매번 묻지 않고 만들고, 시험하고, 고치기를 계속해요.

### Deutsch
**Geschafft**: Große KI-Modelle bestehen oft aus mehreren Dateien. Das Programm lädt jetzt alle Teile selbst herunter und liest sie. Wir haben eine echte KI (Qwen 3B, etwa 6 GB) auf einem normalen PC ohne Grafikkarte laufen lassen, und sie hat Text geschrieben. Etwas langsam, aber es klappt.
**Gelernt**: Die „Maschinen, die die beste Kombination suchen“ von Toshiba und Fujitsu machen eine KI nicht von selbst schneller. Sie könnten helfen zu entscheiden, wie viele Bits jeder Teil bekommt. Das nachträgliche Zusammendrücken des Gedächtnisses (MLA) machte die Qualität schlechter, deshalb nutzen wir es nicht.
**Als Nächstes**: Die Gewichte in kleineren Zahlen speichern, damit noch größere KI (etwa 12 GB) läuft. Wir machen ohne Rückfrage weiter: bauen, testen, verbessern, immer wieder.

### Français
**Fait** : Les grosses IA sont souvent livrées en plusieurs fichiers. Le programme télécharge maintenant toutes les parties et les lit. Nous avons fait tourner une vraie IA (Qwen 3B, environ 6 Go) sur un PC ordinaire sans carte graphique, et elle a écrit du texte. C'est un peu lent, mais ça marche.
**Appris** : Les « machines qui cherchent la meilleure combinaison » de Toshiba et Fujitsu ne rendent pas l'IA plus rapide toutes seules. Elles pourraient aider à choisir combien de bits donner à chaque partie. Compresser la mémoire après coup (MLA) a abîmé la qualité, donc nous ne l'utilisons pas.
**Ensuite** : Garder les poids en plus petits nombres pour faire tourner de plus grosses IA (environ 12 Go). Nous continuons sans demander à chaque fois : construire, tester, corriger, encore et encore.

### Español
**Hecho**: Las IA grandes suelen venir en varios archivos. Ahora el programa descarga todas las partes y las lee. Ejecutamos una IA real (Qwen 3B, unos 6 GB) en un PC normal sin tarjeta gráfica y escribió texto. Es un poco lento, pero funciona.
**Aprendido**: Las «máquinas que buscan la mejor combinación» de Toshiba y Fujitsu no hacen más rápida a la IA por sí solas. Podrían ayudar a decidir cuántos bits da cada parte. Comprimir la memoria después (MLA) empeoró la calidad, así que no lo usamos.
**Siguiente**: Guardar los pesos con números más pequeños para que corran IA aún más grandes (unos 12 GB). Seguimos sin preguntar cada vez: construir, probar y arreglar una y otra vez.

### Português
**Feito**: As IAs grandes costumam vir em vários arquivos. Agora o programa baixa todas as partes e as lê. Rodamos uma IA de verdade (Qwen 3B, cerca de 6 GB) num PC comum sem placa de vídeo, e ela escreveu texto. É um pouco lento, mas funciona.
**Aprendido**: As «máquinas que procuram a melhor combinação» da Toshiba e da Fujitsu não deixam a IA mais rápida sozinhas. Elas podem ajudar a decidir quantos bits cada parte recebe. Comprimir a memória depois (MLA) piorou a qualidade, então não usamos.
**Próximo**: Guardar os pesos em números menores para rodar IAs ainda maiores (cerca de 12 GB). Seguimos sem perguntar toda vez: construir, testar e consertar, de novo e de novo.

### Русский
**Сделано**: Большие ИИ часто поставляются в нескольких файлах. Теперь программа сама скачивает все части и читает их. Мы запустили настоящий ИИ (Qwen 3B, около 6 ГБ) на обычном компьютере без видеокарты, и он написал текст. Немного медленно, но работает.
**Узнали**: «Машины, которые ищут лучшую комбинацию» от Toshiba и Fujitsu сами по себе ИИ не ускоряют. Возможно, они помогут решить, сколько бит дать каждой части. Сжатие памяти задним числом (MLA) ухудшило качество, поэтому мы его не используем.
**Дальше**: Хранить веса в меньших числах, чтобы запускать ещё более крупные ИИ (около 12 ГБ). Мы продолжаем без вопросов каждый раз: строим, проверяем, исправляем снова и снова.

- 2026-11-19 追記 / Update: 選べる組み合わせのうち Axum・Poem・FastAPI を実際に起動して確認(COBOL の hello が HELLO WORLD を返し、存在しない名前は not_found、画面 `/` は 200)。RPoem/RPoRea は起動未確認。/ Axum, Poem and FastAPI were started and checked (COBOL hello returns HELLO WORLD, unknown name gives not_found, the screen `/` returns 200). RPoem/RPoRea not started yet. 次: C/C++/Fortran→Rust の書き換え、COBOL→別OSのCOBOL拡張。/ Next: rewriting C/C++/Fortran into Rust; extending COBOL-to-other-OS COBOL.

- 2026-11-19 追記 / Update: `pl2pl c2rust`(最初の一歩)を追加。単純な C(変数・四則・if/while/for・関数・printf)を Rust へ。fib のサンプルで gcc と同じ出力を確認。ポインタ・構造体・配列は TODO(c2rust) として残す。Fortran/C++ は未対応。/ Added `pl2pl c2rust` (first step): simple C (variables, arithmetic, if/while/for, functions, printf) to Rust; the fib sample prints the same as gcc. Pointers, structs, arrays stay as TODO(c2rust). Fortran and C++ not yet. 軽い文書作業のクラウド自動実行は、GitHub 接続待ち。/ The cloud schedule for light doc work is waiting for the GitHub connection.

## 2026-11-19 到達点と再開用メモ(日本語・リミット前)
**できたこと(pl2pl)**: ①`pl2pl web --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend] [--db]` で、サーバー・画面・データベースの組み合わせを選んで変換(COBOL は変換、C・C++・Fortran は元のままビルドして呼ぶ橋渡し)。Axum・Poem・FastAPI は実際に起動して確認済み。②`pl2pl c2rust`(単純な C → Rust、fib で gcc と同じ出力)。③`pl2pl watch` は新版を「まっさらな venv」で試験(WSL)、毎晩 03:00・毎朝 07:00 の自動実行を登録済み。④テストは単体 46 件すべて成功。
**未了(確認せず自動で進める)**: C++・Fortran の Rust 化と C の対応範囲の拡大(ポインタ・構造体・配列)、gfortran での橋渡しの実機確認、RPoem/RPoRea の起動確認、COBOL を別 OS の COBOL へ直す機能の拡張、メール到達の確認、Kokkos の GPU 実行(新しい GPU の機械が必要)、8 か国語文書の更新。クラウド定期実行は GitHub 接続待ち(https://claude.ai/connect-github)。
**人に確認すること**: 元の COBOL_to / Java_to の削除だけ(元に戻せないので最後に確認)。
**再開の手順**: `F:\pl2pl` の `PORTING.md` 末尾を読む。開発・試験はこの PC(WSL)、VPS は配備と確認だけ。`git pull` は VPS の `/root/easy-web.tokyo/pl2pl` で `--ff-only`。

## 2026-11-19 限界前チェックポイント(大きなモデル対応、別アカウントでの再開用)

**完了(すべて main に push 済み)**
- 分割重み(`model.safetensors.index.json`+複数ファイル)の取得(`aruaru-llm` `model_catalog.rs`、単一ファイルが404のときだけ・ストリーム書き込み・導入判定対応)と読み込み(`open-cuda-llm` `qwen_arch.rs`、mmap)。モックと合成データのテスト成功。
- 実機確認(32GB・GPUなし・CPU): Qwen2.5-3B(2分割・6.17GB)を読み込み生成成功(読込約48秒、約0.6語/秒、ppl 8.5)。1.5Bは ppl 7.08。
- 後付けMLA(PCA較正)は実用外: d_c=96でKV−25%だが ppl 12.7、d_c=64で崩れる(ppl 245)。MLAは学習時から入れる設計でないと効かない。
- FP8読み込み `QwenModel::load_fp8`: 量子化を二分探索化・ホスト並列化し、行列積を融合(`opencuda-blas`)。1.5Bで読込340→21秒、生成1067→69秒、ppl 7.27(f32は7.08)、メモリ約1/4。ただしf32より約2.8倍遅い。
- 調査: 東芝SB・富士通デジタルアニーラは組合せ最適化の道具でLLM推論を直接速めない。LLMでの先行例はTree-Ising(重みのQUBO修復、自己申告値)のみ。エキスパートごとのビット配分は線形/整数計画(GEMQ・BitsMoE)。「折り畳み」はModel Folding(ICLR 2025)・Layer Collapse・Sliding-Window Merging・GPTailor。DeepSeekのMLA・MoE遅延読み込みは `deepseek_arch.rs` に実装済み(実在チェックポイントでの通しは未確認)。
- クラウドセッションに調査文書 `docs/LARGE-MODEL-TECHNIQUES.md`(ブランチ `docs/large-model-techniques`)を依頼済み(終了通知は来ない。`aruaru-llm` のブランチを確認すること)。

**未了(確認なしで順に進める方針)**
1. 層ごとの感度測定(層を飛ばす/隣接層を平均する「折り畳み」)と、ビット配分の自動決定(まず貪欲法、次に焼きなまし・SB)。`QwenModel` に層スキップ機構を足して `qwen_mla_bench` 系の例で ppl と速度を測る。
2. FP8がf32より遅い問題(約2.8倍): SIMD化・f16保持・行単位スケール見直し。
3. 7B級(約12〜15GB)の実読み込み: FP8読み込みで32GB機に載るかを実測(要ダウンロード許可内の公開モデル)。
4. 実3BのFP8読み込み実測、`pl2pl llm setup` への分割モデル・FP8の反映、`aruaru-llm` のカタログに3Bの実機確認の記載。
5. 10言語文書は現状分を追記済み。上記の結果が出たら更新する。翻訳はAI作成でネイティブ検証前。

**注意**: `F:\open-cuda` の作業ブランチは `perf/serial-small-gemm`(`HEAD:main` を指定して push している)。`F:\aruaru-llm` は `Cargo.lock` と `data/` に未コミットの変更がある(別作業のもの、コミットしない)。モデルは `F:\aruaru-llm\models\qwen2.5-{1.5b,3b}-instruct`(サイズが大きいので消さない)。測定スクリプト: `open-cuda/crates/open-cuda-llm/examples/qwen_mla_bench.rs`(`QWEN_FP8=1` でFP8、引数にd_c)。

## 2026-11-19 いまの状況と、つづきのやり方 / Status and how to continue (final checkpoint)

### 日本語(やさしく)
**いまできること**: 古いプログラム(COBOL・C・C++・Fortran)を、いまのウェブのしくみに乗せるお助けロボットです。`pl2pl web --backend ...` で、サーバー(Axum・Poem・FastAPI・RPoem・RPoRea)、画面(HTML5・CSS3・TypeScript・Bootstrap)、データベース(MySQL・MariaDB・PostgreSQL・SQLite)を好きな組み合わせで選べます。COBOL は書き換え、C・C++・Fortran は元のまま動かして呼びます。単純な C は `pl2pl c2rust` で Rust に書き換えられます。新しい版の自動チェック(`pl2pl watch`)は毎晩 03:00・毎朝 07:00 に動きます。
**まだのこと(確認せず、自動で進めます)**: C++・Fortran の Rust 化、C の対応範囲の拡大、Fortran の実機確認、RPoem/RPoRea の起動確認、COBOL から別 OS の COBOL への拡張、メール到達の確認、Kokkos の GPU 実行、8 か国語文書の更新。大きなモデル(3B・7B)の対応は別の記録(`PORTING.md` の大規模モデルの節)にあります。
**人にたずねること**: 元の `COBOL_to` / `Java_to` を消す作業だけです(元に戻せないため)。クラウドの定期実行は、GitHub の接続(https://claude.ai/connect-github)を待っています。
**つづけるには**: `F:\pl2pl\PORTING.md` の末尾から読みます。作る・試すのはこの PC(WSL)で、VPS は入れ替えと確認だけに使います。

### English (plain and friendly)
**What it does now**: pl2pl is a helper robot that puts old programs (COBOL, C, C++, Fortran) onto today's web. With `pl2pl web --backend ...` you pick the server (Axum, Poem, FastAPI, RPoem or RPoRea), the screen (HTML5, CSS3, TypeScript, Bootstrap) and the database (MySQL, MariaDB, PostgreSQL or SQLite) in any allowed mix. COBOL is rewritten; C, C++ and Fortran keep running as they are and the web calls them. Simple C can be rewritten into Rust with `pl2pl c2rust`. The automatic check for new versions (`pl2pl watch`) runs every night at 03:00 and every morning at 07:00.
**Still to do (we go on automatically, no need to ask)**: Rust versions of C++ and Fortran, wider C support, a real Fortran build check, starting RPoem/RPoRea, COBOL to COBOL for another OS, a real mail-arrival check, Kokkos on a GPU, and updating the 8 other language docs. Big-model work (3B and 7B) is recorded separately in the large-model section of `PORTING.md`.
**What we ask a person**: only deleting the original `COBOL_to` / `Java_to` (it cannot be undone). The cloud schedule is waiting for the GitHub connection (https://claude.ai/connect-github).
**To continue**: start from the end of `F:\pl2pl\PORTING.md`. Build and test on this PC (WSL); use the VPS only for swapping binaries and checking.

- 2026-11-19 追記 / Update: `pl2pl c2rust` が単純な C++(`cout << ... << endl`、`using namespace std`)にも対応。生成した Rust が元と同じ出力(i=0..2, done)。クラス・テンプレート・STL は TODO のまま。Fortran は未対応。/ `pl2pl c2rust` now also handles simple C++ (`cout << ... << endl`, `using namespace std`); the generated Rust prints the same as the original. Classes, templates and the STL stay as TODO. Fortran is not supported yet.

## 2026-11-19 まとめ: 大きなAIを小さなパソコンで / Summary: big AI on a small PC

### 日本語(やさしい説明)
**できたこと**
- 重さが「いくつかのファイル」に分かれた大きなAIを、自動でぜんぶダウンロードして読めるようになりました(`aruaru-llm`・`open-cuda`)。
- 本物の Qwen2.5-3B(約6GB)を、GPUのないふつうのパソコンで動かして文章を作れました(読み込み約48秒、1秒に約0.6語)。
- 重さを「FP8」という小さい数で持つ方法を作りました。メモリが約4分の1になり、品質はほぼ同じです(迷い度 7.08→7.27)。速くする工夫で、読み込みは340秒→21秒、文章づくりは1067秒→69秒になりました。まだ普通の方法より約2.8倍遅いので、メモリが足りないときの手段です。

**試してわかったこと(正直な結果)**
- 後から記憶を圧縮する「MLA」は、品質が落ちるので使えません(25%減で迷い度 7.1→12.7、50%減で文が壊れる)。
- 「折り畳み(層を飛ばす)」を1.5Bで試しました。1層ずつ飛ばすと、真ん中あたりの層(5・10・12・15番など)は飛ばしてもほとんど平気でしたが、1番・24番・26番は大事でした。3層飛ばすと迷い度7.19(元7.08)で、速さは約7%だけ上がり、答えが少し不正確になりました。6層以上飛ばすと壊れます。**学び直しなしの層飛ばしは、効果が小さい**とわかりました。
- 東芝の「シミュレーテッド分岐」や富士通の「デジタルアニーラ」は、「いちばん良い組み合わせを探す」道具で、AIを直接速くはしません。使うなら「どの層を飛ばすか」「何ビットにするか」を決める所ですが、まだ試していません。

**これから(確認せず順に進めます)**
1. 層を飛ばす組み合わせを、焼きなましやSBで探して、貪欲法と比べる。
2. FP8を速くする(SIMD、f16保持)。
3. 7B(約12〜15GB)をFP8で32GBのパソコンに載せられるか測る。
4. `pl2pl llm setup` に分割モデルとFP8を反映する。

**約束**: 危なくない未了は、いちいち聞かずに進めます。取り返しのつかない削除や秘密の公開はしません。悪い結果も正直に書きます。

### English (plain and friendly)
**What works now**
- Big AI models are often split into several files. The program now downloads every part and loads them (`aruaru-llm` and `open-cuda`).
- We ran the real Qwen2.5-3B model (about 6 GB) on an ordinary PC with no GPU, and it wrote text (about 48 seconds to load, about 0.6 words per second).
- We built a way to keep the weights as small "FP8" numbers. Memory drops to about a quarter and quality stays almost the same (perplexity 7.08 to 7.27). After speed fixes, loading went from 340 s to 21 s and writing from 1067 s to 69 s. It is still about 2.8 times slower than the normal way, so it is for when memory is short.

**What we tested (honest results)**
- Squeezing the memory afterwards with MLA does not work: it hurts quality (25% smaller gave perplexity 7.1 to 12.7; 50% smaller broke the text).
- We tried "folding" by skipping layers on the 1.5B model. Skipping one layer at a time showed that many middle layers (5, 10, 12, 15 and so on) barely matter, while layers 1, 24 and 26 matter a lot. Skipping three layers gave perplexity 7.19 (from 7.08), only about 7% faster, with slightly less accurate answers. Skipping six or more breaks the model. **Skipping layers without retraining gives only a small gain.**
- Toshiba's Simulated Bifurcation and Fujitsu's Digital Annealer find the best combination among many choices. They do not speed up the AI by themselves. They could help choose which layers to skip or how many bits to use, but we have not tried that yet.

**What comes next (we go on without asking)**
1. Search for the best set of layers to skip with annealing or SB, and compare with the greedy way.
2. Make FP8 faster (SIMD, f16 storage).
3. Measure whether 7B (about 12 to 15 GB) fits a 32 GB PC with FP8.
4. Bring sharded models and FP8 into `pl2pl llm setup`.

**Our promise**: for safe unfinished work we go on without asking each time. We never do irreversible deletes or publish secrets, and we report bad results honestly.

- 2026-11-19 追記 / Update: (1) `pl2pl cobol` に `--os linux|macos|unix|windows` と `--dialect ibm|mf|default` を追加。ソースは変えず、選んだ OS 用のビルド手順(`build.sh` / Windows は `build.bat`)を出す。不正な指定は終了コード 2。(2) RPoem / RPoRea の骨組みを実際にビルド・実行して確認(COBOL の calc などが出力)。外部 CALL が未登録の `misc` で落ちるため、1 本落ちても残りを続けるようにした。RPoem 本体への組み込み(サーバー登録)は未確認。/ (1) `pl2pl cobol` now takes `--os linux|macos|unix|windows` and `--dialect ibm|mf|default`; the source is untouched and a build script for that OS is written (`build.sh`, or `build.bat` on Windows); a bad value exits with code 2. (2) The RPoem / RPoRea scaffold was built and run for real (COBOL programs like calc print their output); because `misc` stops on an unregistered external CALL, one failing program no longer stops the rest. Registering into the real RPoem server is not checked yet. 未確認 / Not verified: Windows/macOS で実際に cobc を動かすこと。/ Running cobc on real Windows/macOS.

## 2026-11-19 最後の追記: 飛ばす層の探し方 / Last note: choosing which layers to skip

### 日本語(やさしい説明)
- **やったこと**: AIの「飛ばしてもよい層」を、3つ選ぶ問題として2通りで探しました。(1)貪欲法=いちばん良い層を1つずつ足す。(2)焼きなまし=選び方を少しずつ入れ替えて探す。同じ試行回数(78回)で比べました。
- **結果(Qwen2.5-1.5B)**: 貪欲法は {5, 12, 15} 番で迷い度 7.187(元は 7.08)。焼きなましは {12, 15, 16} 番で 7.438。**この小さな問題では、焼きなましは貪欲法に勝てませんでした。** 5個の場合の比較は途中です。
- **気をつけること**: 1つの短い文だけで選んでいるので、その文にだけ合った選び方かもしれません。別の文でも確かめる必要があります。
- **ずっと言えること**: 東芝SBや富士通デジタルアニーラは、層がもっと多い大きなAIや、ビット配分のように選択肢が爆発する問題で試す価値があります。今回の小さな問題では、ふつうの方法で足ります。
- **アップロード**: コードと文書は GitHub の `main` に push しました。VPS には、きれいなクローンだけ安全に更新します(再ビルドや再起動はしません)。
- **これから(確認せず進めます)**: 5個の比較を終える、別の文で確かめる、FP8を速くする、7Bを測る、`pl2pl llm setup` に反映する。

### English (plain and friendly)
- **What we did**: We looked for three "layers that are safe to skip" in two ways. (1) Greedy: add the best layer one at a time. (2) Annealing: swap choices little by little. Both got the same number of tries (78).
- **Result (Qwen2.5-1.5B)**: Greedy picked layers {5, 12, 15} with perplexity 7.187 (the original is 7.08). Annealing picked {12, 15, 16} with 7.438. **On this small problem, annealing did not beat greedy.** The comparison for five layers is still running.
- **A caution**: We chose using one short sentence, so the choice may fit only that sentence. We must check with other sentences.
- **What still holds**: Toshiba's Simulated Bifurcation and Fujitsu's Digital Annealer are worth trying on bigger models with many more layers, or on problems like bit allocation where the choices explode. For this small problem, the plain method is enough.
- **Upload**: Code and docs are pushed to `main` on GitHub. On the VPS we only update clean clones safely (no rebuild, no restart).
- **Next (we go on without asking)**: finish the five-layer comparison, check with other sentences, make FP8 faster, measure 7B, and bring it into `pl2pl llm setup`.

- 2026-11-20 追記 / Update: `pl2pl f2rust`(最初の一歩)を追加。単純な Fortran(program・integer/real・do・if/else・print *)を Rust へ。サンプルで期待どおり `sum 10` を出力、単体テスト 48 件成功。配列・subroutine・module・書式付き出力・`**` は TODO(f2rust) のまま。`print *` の書式は近似で、gfortran との出力一致は未確認(この PC に gfortran が無い)。/ Added `pl2pl f2rust` (first step): simple Fortran (program, integer/real, do, if/else, print *) to Rust. The sample prints `sum 10` as expected; 48 unit tests pass. Arrays, subroutines, modules, formatted output and `**` stay as TODO(f2rust). The `print *` layout is approximate and not compared with gfortran (not installed here).

- 2026-11-20 追記 / Update: 本物の gfortran(conda-forge 16.2、`F:\_work\gfnv`)で確認。(1) Fortran の橋渡し: build.sh でビルドし Axum 経由で ` sum          10` を返した。(2) `pl2pl f2rust` の出力が gfortran と完全に一致(`print *` は各項目の前に空白 1 個・整数は幅 11 に合わせた)。小数(real)の桁数は未一致で近似。単体テスト 48 件成功。sudo 無しで入れる方法: micromamba で conda-forge の gfortran。/ Verified with a real gfortran (conda-forge 16.2). (1) The Fortran bridge built with build.sh and returned ` sum          10` through Axum. (2) `pl2pl f2rust` output now matches gfortran exactly (`print *`: one space before each item, integers width 11). Real-number digits are not matched yet (approximate). 48 unit tests pass. No sudo needed: install gfortran with micromamba from conda-forge.

## 2026-11-20 追記: 7Bも動いた / Update: 7B runs too

### 日本語(やさしい説明)
- **7Bが32GBのパソコンで動きました**: Qwen2.5-7B-Instruct(4つに分かれた約15GB)を、FP8で読み込んで文章を作れました。メモリに収まり、読み込みは約3.5分、文章づくりは1語あたり約8秒(とても遅いけれど、動きます)。品質の目安の迷い度は、評価用の文で8.6、別の文で12.2でした。1.5Bの別の文(12.1)とほぼ同じ値です。
- **層を飛ばす方法(折り畳み)の最終結果**: 1つの文だけで飛ばす層を選ぶと、別の文で悪くなりました(5・12・15番を飛ばすと別の文で約15%悪化、5層飛ばすとさらに悪化)。速くなるのは最大でも約14%です。**学び直しなしでは、実用的な効果は小さい**です。貪欲法と焼きなましは、この小さな問題では大差ありませんでした。
- **`pl2pl llm setup` について**: 分割モデルの対応は `aruaru-llm` の `main` に入りました。`pl2pl` が使う配布版(v0.2.5)にはまだ入っていないので、新しい版を出すまで、3Bは今までどおり自動で小さいモデルに下がります。新版を出したら3B・7Bも選べるようにします。
- **これから(確認せず進めます)**: FP8をもっと速くする(SIMDなど)、新しい版の公開、クラウドで頼んだ調査文書の確認。

### English (plain and friendly)
- **7B runs on a 32 GB PC**: We loaded Qwen2.5-7B-Instruct (about 15 GB in four parts) as FP8 and it wrote text. It fits in memory; loading took about 3.5 minutes and writing took about 8 seconds per word (very slow, but it works). The quality score (perplexity) was 8.6 on the test sentence and 12.2 on other sentences, about the same as the 1.5B model on other sentences (12.1).
- **Final result for layer skipping ("folding")**: Choosing layers to skip from a single sentence got worse on other sentences (skipping layers 5, 12 and 15 was about 15% worse on other sentences, and skipping five was worse still). The speed gain is at most about 14%. **Without retraining, the practical gain is small.** On this small problem, greedy search and annealing made little difference.
- **About `pl2pl llm setup`**: Sharded-model support is now on `main` in `aruaru-llm`. The released build that `pl2pl` uses (v0.2.5) does not have it yet, so until a new release, 3B still falls back to a smaller model as before. After the new release, 3B and 7B can be chosen.
- **Next (we go on without asking)**: make FP8 faster (SIMD and more), publish a new release, and check the survey document requested from the cloud.

- 2026-11-20 調査 / Findings (RPoem 実登録・メール・Kokkos): (1) RPoem への実登録は `open-runo-connect-server` の `Service` を実装する形(gRPC/gRPC-web/Connect を 1 つで受ける)。proto の記述子(`cobol_to.Program/Run`)を作って `handle` に渡す必要があり、骨組みの `PROGRAMS` を呼ぶ `Service` 実装が次の仕事(小さくない)。(2) メール到達: この PC に SMTP の設定が無く、確認は SMTP を持つ VPS 側(本人のメール)でしかできない。(3) Kokkos の GPU: この PC の GT 730 では不可、新しい GPU の機械が必要。/ (1) Registering in the real RPoem means implementing `Service` of `open-runo-connect-server` (one handler serves gRPC, gRPC-web and Connect); it needs a proto descriptor for `cobol_to.Program/Run`, and a `Service` that calls the scaffold `PROGRAMS` is the next, non-small task. (2) Mail arrival: this PC has no SMTP settings; it can only be checked on the VPS side with SMTP, to the owner address. (3) Kokkos on a GPU: not possible on this PC (GT 730); needs a machine with a newer GPU.

- 2026-11-20 追記 / Update: RPoem / RPoRea の骨組みを、本物の RPoem の `open-runo-connect-server`(gRPC / gRPC-web / Connect を 1 つの口で受ける)に載せた。生成した `src/main.rs` は `cobol_to.Program/Run` の記述子を持つ `Service` を実装し、`PROGRAMS` を呼ぶ。置き場所は環境変数 `RPOEM_DIR`(既定 `../RPoem`)。実機確認: Connect(JSON)で `hello` が `HELLO WORLD`、未登録名は 404 `not_found`、外部 CALL が未登録の `misc` は 500 で、サーバーは止まらない。gRPC / gRPC-web での実呼び出しは未確認。RPoRea は同じ RPoem のサーバーを使う(RPoRea 専用の部品は未使用)。/ The RPoem / RPoRea scaffold now runs on the real RPoem `open-runo-connect-server` (one handler serves gRPC, gRPC-web and Connect). The generated `src/main.rs` implements a `Service` with a `cobol_to.Program/Run` descriptor and calls `PROGRAMS`; the RPoem location comes from `RPOEM_DIR` (default `../RPoem`). Verified: with Connect (JSON) `hello` returns `HELLO WORLD`, an unknown name gives 404 `not_found`, `misc` (unregistered external CALL) gives 500 and the server keeps running. Real gRPC / gRPC-web calls are not checked yet. RPoRea uses the same RPoem server (no RPoRea-specific parts yet).

- 2026-11-20 追記 / Update: RPoem サーバーへの実呼び出しを gRPC-web(`application/grpc-web+proto`)と Connect(proto)でも確認。`hello` の出力が返り、grpc-status 0。素の gRPC(HTTP/2)は未確認(骨組みは HTTP/1 のみ)。/ Real calls to the RPoem server also verified with gRPC-web (`application/grpc-web+proto`) and Connect (proto): `hello` returns its output with grpc-status 0. Plain gRPC (HTTP/2) is not checked (the scaffold serves HTTP/1 only).

## 2026-11-20 方針: 姉妹リポジトリとして同時開発 / Sister repositories, developed together
**日本語**: pl2pl の変換機能は、`Reaxt`・`RPoRea`・`RPoem`・`RMojo`・`open-cosmo` などを、**変換の前でも後でも使える**ようにします(変換元として読む、変換先の骨組みとして出す、出力を動かす土台にする)。これらは `pl2pl` の**姉妹リポジトリ**として、今後は**いつも同時に一緒に開発**します。1つを変えたら、ほかの姉妹の動作確認と README / CLAUDE / PORTING / POLICY の更新を同じ日に行い、未了は確認せず自動で進めます(元の COBOL_to / Java_to の削除だけは最後に確認)。
**English**: pl2pl's conversion features must work with `Reaxt`, `RPoRea`, `RPoem`, `RMojo` and `open-cosmo` both **before and after conversion** (read as a source, emitted as a target scaffold, used as the runtime for the output). They are **sister repositories** of `pl2pl` and are **always developed together**: when one changes, check the others and update README / CLAUDE / PORTING / POLICY the same day. Unfinished items continue automatically; only deleting the original COBOL_to / Java_to is confirmed last.

## 2026-11-20 選択画面の案内と注意 / Selection notices and warnings
**日本語**: `pl2pl web --backend rpoem|rporea` を選ぶと、RPoem(Poem の Rust 版)は Tauri 互換・WunderGraph Cosmo 互換・tokio・WASM を含むので Tauri と WunderGraph Cosmo は選ばないこと、HTML5・CSS3 は使うがなるべく少量、JavaScript・TypeScript・Node.js・REST API は設計上使わないこと、を日本語と English で表示します。`--also tauri,cosmo,nodejs,...` で足そうとした物が、選んだ物に既に含まれていれば「既に〇〇に含んでいますので、ご選択できません」、使わない仕様の物なら「設計上、使用しない仕様ですので、ご選択できません」と断ります(終了コード 2)。対応は `src/advice.rs`(試験4件)。
**English**: Choosing `pl2pl web --backend rpoem|rporea` prints (in Japanese and English) that RPoem, the Rust edition of Poem, already includes Tauri-compatible and WunderGraph Cosmo-compatible features, tokio and WASM, so Tauri and WunderGraph Cosmo should not be chosen; that HTML5 and CSS3 are used but kept as small as possible; and that JavaScript, TypeScript, Node.js and REST APIs are not used by design. Anything added with `--also tauri,cosmo,nodejs,...` that is already included in the chosen stack is declined ("X is already included in Y, so it cannot be selected separately"), and so are items not used by design (exit code 2). Implemented in `src/advice.rs` (4 tests).

## 2026-11-20 追記: 含んでいても選べる場合 / Included but still selectable
**日本語**: 既に含んでいて構造上いっしょに選べない物(tokio・WASM・Poem など)は「既に〇〇に含んでおりますので、ご選択できません」と断ります。ただし、含んではいても互換の完成度がまだ高くない物(RPoem / RPoRea の Tauri 互換・WunderGraph Cosmo 互換・open-cosmo)は断らず、選べます(重複や動作の違いへの注意を表示)。互換が完成したら `src/advice.rs` の `partial()` から外します。
**English**: Items that are already included and cannot be combined structurally (tokio, WASM, Poem and so on) are declined with "X is already included in Y, so it cannot be selected separately". Items that are included but whose compatibility is not yet mature (Tauri- and WunderGraph Cosmo-compatibility and open-cosmo in RPoem / RPoRea) are not declined: they can be selected, with a caution about overlap and behavioural differences. When the compatibility matures, remove them from `partial()` in `src/advice.rs`.

## 2026-11-20 追記: 互換の完成度の判定を自動化 / Automatic maturity judgement
**日本語**: 「含んでいるが互換が未完成なので選べる」物の台帳は `registry/maturity.toml`(`src/advice.rs` を手で直す必要はなくなりました)。`pl2pl maturity [--repo DIR] [--llm URL] [--apply]` が、台帳の試験(probe)を **open-cpu** で調べた CPU 数の並列で実行し、成功したら完成と判定します。**aruaru-llm**(open-cuda / open-cpu で動く)には参考の一文を聞きますが、AI だけでは判定を変えません。`--apply` で利用者の設定フォルダーの `maturity.toml` に反映し、以後その物は「既に〇〇に含んでおりますのでご選択できません」に変わります。試験がまだ無い物(Tauri 互換)は判定できず、未完成のままです。**Claude(AI)が結果を見て適用してもよい**(`pl2pl maturity --llm http://127.0.0.1:4600` → 結果を確認 → `--apply`)。
**English**: The ledger of "included but immature, so still selectable" items is now `registry/maturity.toml`; no hand-editing of `src/advice.rs` is needed. `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` runs each probe test with parallelism taken from **open-cpu**'s CPU detection and judges an item complete when it passes. **aruaru-llm** (running on open-cuda / open-cpu) gives a one-line opinion, but the AI alone never changes the verdict. `--apply` records it in the user's `maturity.toml`, after which the item is declined with "already included in X, so it cannot be selected". Items with no probe yet (Tauri compatibility) cannot be judged and stay partial. **Claude (the AI) may review the output and apply it** (`pl2pl maturity --llm http://127.0.0.1:4600`, check, then `--apply`).

## 2026-11-20 v0.2.1 と統合の確認 / v0.2.1 and absorption check
**日本語**: v0.2.1: 選択時の案内と注意(`--also`、含んでいて選べない物/互換が未完成で選べる物)、`pl2pl maturity`(互換の完成度の判定、open-cpu・aruaru-llm 利用)。元の `COBOL_to` / `Java_to` は **残します**(削除しません)。確認結果: コードは `converters/` と改行コード以外同一、単体試験は元・取り込み版とも成功(10件・15件)、同じ入力から同じ出力(COBOL 4 ターゲット + Java 4 フレームワーク×4 ターゲット = 20 通り)で完全一致。注意: 元の COBOL_to の FastAPI 系の画面案内は今も Next.js/Node.js を挙げています(Python 側の画面は許可のまま。RPoem 系は使わない)。
**English**: v0.2.1: selection notices and cautions (`--also`; declined when included, selectable when compatibility is immature) and `pl2pl maturity` (maturity judgement using open-cpu and aruaru-llm). The original `COBOL_to` / `Java_to` are **kept** (not deleted). Checks: the code is identical to `converters/` apart from line endings; unit tests pass in both the originals and the vendored copies (10 and 15); the same inputs give byte-identical outputs (COBOL x 4 targets plus Java 4 frameworks x 4 targets = 20 combinations). Note: the original COBOL_to FastAPI-side UI guidance still mentions Next.js/Node.js (Python-side UI stays allowed; the RPoem side uses none).

## 2026-11-20 追記: Node.js を選ばれたとき / When the user picks Node.js
**日本語**: RPoem / RPoRea 以外の組み合わせで `--also nodejs`(または `next.js`)が選ばれたときは、断らずに通し、「『Node.js を使わない』方針を利用者に押し付けませんので、そのままお使いいただけますが、推薦はしておりません」と日英で表示します。RPoem / RPoRea は Rust→WASM の構造上、Node.js を使わない設計のため、これまでどおりお断りします。
**English**: With any stack other than RPoem / RPoRea, if the user selects `--also nodejs` (or `next.js`), it is accepted and a notice says (in Japanese and English) that we do not impose our no-Node.js policy and the user may use it, but we do not recommend it. RPoem and RPoRea are Rust-to-WASM by design and still decline Node.js.

## 2026-11-20 追記: Node.js の代わりになる物の案内 / Node.js alternatives shown
**日本語**: Node.js(Next.js)が選ばれたとき、選んだ言語に合わせて代わりになる物を表示します。Rust 系: RPoem / RPoRea + Reaxt(Rust→WASM で Node.js 不要、`open-runo-view` の仮想 DOM と SSR、`open-runo-crossui` の各 OS 向け画面生成、Connect / gRPC-web で REST・Express 不要。いずれも RPoem・Reaxt のリポジトリの記載で確認)。Python 系: FastAPI / MFastAPI の静的配信(Starlette)と Jinja2。さらに「他の言語では、このようなフレームワークもございます」として Leptos・Dioxus・Yew・htmx・NiceGUI・templ・Blazor・Phoenix LiveView を挙げます(これらは当方では未検証と明記)。実装は `src/advice.rs` の `node_alternatives()`。
**English**: When Node.js (Next.js) is selected, the notice lists alternatives matching the chosen language. Rust: RPoem / RPoRea + Reaxt (Rust to WASM, no Node.js; `open-runo-view` virtual DOM and SSR; `open-runo-crossui` multi-OS UI generation; Connect / gRPC-web instead of REST or Express; all confirmed in the RPoem and Reaxt repositories). Python: FastAPI / MFastAPI static serving (Starlette) with Jinja2. It then adds "frameworks in other languages" (Leptos, Dioxus, Yew, htmx, NiceGUI, templ, Blazor, Phoenix LiveView), clearly marked as not verified by us. Implemented in `node_alternatives()` in `src/advice.rs`.

## 2026-11-20 追記: RNode.js(Rust 版 Node.js 互換)を姉妹に追加 / RNode.js added as a sister
**日本語**: `RS-Node.js`(RNode.js)は、Node.js 互換を目指す Rust 製の実行環境です。JavaScript は Rust 製エンジン Boa、TypeScript は `RS-TypeScript` を通して動き、将来は `RS-JavaScript`(一から作る Rust 製エンジン)に差し替えられます。本物の Node.js(v26.5.0)と同じスクリプトの出力を比べて互換を確かめながら開発しており、現在の対応は `console`・`process`・`require`(`path`・`fs` の同期版)までです。Node.js を選んだ利用者への案内に、代わりの候補として追加しました。姉妹リポジトリ(Reaxt・RPoRea・RPoem・RMojo・open-cosmo・RS-TypeScript・RS-JavaScript・RS-Node.js・pl2pl)は同時に開発します。
**English**: `RS-Node.js` (RNode.js) is a Rust runtime aiming at Node.js compatibility. JavaScript runs on the Rust engine Boa and TypeScript through `RS-TypeScript`; `RS-JavaScript` (our from-scratch Rust engine) can replace Boa later. It is developed by comparing script output against real Node.js (v26.5.0); current coverage is `console`, `process` and `require` (synchronous `path` and `fs`). It is now listed as an alternative in the notice shown to users who pick Node.js. The sister repositories (Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js, pl2pl) are developed together.


## 2026-11-20 設計思想ページ `pl2pl info` / Design-principles page (11 languages)
**日本語**: `pl2pl info [ui|nojs|json|connect|shared] [--lang ja|zh-CN|zh-TW|ko|en|de|ru|fr|it|ar|fa]` と `webpage/info.html` で、RPoRea・RPoem の設計思想(HTML5/CSS3 は最小限・WASM/tokio・Node.js/JS/TS を使わない・JSON は RS-JSON・REST 全廃)をメリット・デメリット付きで表示します。選択時の注意文からもリンクします。翻訳は AI 作成でネイティブ確認前です。Node.js を使うかは利用者の自由で、推薦はしません。
**English**: `pl2pl info [topic] [--lang ...]` and `webpage/info.html` explain the RPoRea/RPoem design principles (minimal HTML5/CSS3, WASM/tokio, no Node.js/JS/TS, RS-JSON, no REST) with pros and cons in 11 languages. Selection notices point to it. Translations are AI-made and not yet native-checked. Using Node.js is the user's free choice; it is not recommended.
## 2026-11-22 追記: INSPECT の LEADING / CHARACTERS / Added INSPECT LEADING and CHARACTERS
**日本語**: `INSPECT ... TALLYING ... FOR LEADING "x"` と `FOR CHARACTERS`、`REPLACING LEADING "x" BY "y"` を変換できるようになりました(`AFTER/BEFORE INITIAL` との併用も可)。`"000A00B00"` と `"AAABAAAAA"` で実際に動かし、Python と Rust がどちらも `3 9 2 3` / `   A00B00` / `---BAAAAA` と同じ出力になることを確認しました(サンプル `tests/samples/inspect2.cbl`、単体試験14件成功)。**まだ**: `REPLACING CHARACTERS`、`FOR` を複数並べる形、`SORT/MERGE/SEARCH`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。pl2pl の `converters/cobol_to` にも同じ変更を反映済みです。他の言語の節の「完成予定」は、毎朝の翻訳更新で順に直します。
**English**: `INSPECT ... TALLYING ... FOR LEADING "x"`, `FOR CHARACTERS` and `REPLACING LEADING "x" BY "y"` are now converted (also together with `AFTER/BEFORE INITIAL`). We ran `"000A00B00"` and `"AAABAAAAA"` and both the generated Python and Rust print the same `3 9 2 3` / `   A00B00` / `---BAAAAA` (sample `tests/samples/inspect2.cbl`, 14 unit tests pass). **Not yet**: `REPLACING CHARACTERS`, several `FOR` clauses in one statement, `SORT/MERGE/SEARCH`, indexed and relative files, COMP-3, EXEC SQL/CICS. The same change is applied to `converters/cobol_to` in pl2pl. The planned lists in the other language sections will be fixed by the daily translation update.

## 2026-11-22 追記: SEARCH / SEARCH ALL / Added SEARCH and SEARCH ALL
**日本語**: `OCCURS ... INDEXED BY ix` の表を `SEARCH 表 [VARYING ix] [AT END ...] WHEN 条件 ... END-SEARCH` で引けるようになりました(`WHEN` は複数可)。`SEARCH ALL` は索引を 1 から始める直線探索として動かします(整列済みの表で一致が1件なら、二分探索と結果は同じです)。表を `PERFORM UNTIL`(範囲外か条件一致まで索引を進める)と `IF` に分解して出すので、Python と Rust の両方で同じ出力になります。サンプル `tests/samples/search1.cbl` で `FOUND 3` / `NONE` / `ALL 5` / `EQ20` を確認、単体試験15件成功。`INDEXED BY` の索引名は変数として自動で宣言します。**まだ**: `SORT`(表・ファイル)、`MERGE`、`SEARCH ALL` の本物の二分探索、複数の `INDEXED BY` 名、`REPLACING CHARACTERS`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: Tables declared with `OCCURS ... INDEXED BY ix` can now be searched with `SEARCH table [VARYING ix] [AT END ...] WHEN cond ... END-SEARCH` (several `WHEN` clauses allowed). `SEARCH ALL` runs as a linear search starting at index 1 (identical results on a sorted table with a single match). It is emitted as a `PERFORM UNTIL` (advance the index until out of range or a condition matches) plus an `IF` chain, so Python and Rust print the same output. In `tests/samples/search1.cbl` we confirmed `FOUND 3` / `NONE` / `ALL 5` / `EQ20`; 15 unit tests pass. The `INDEXED BY` name is declared as a variable automatically. **Not yet**: `SORT` (table and file forms), `MERGE`, a true binary `SEARCH ALL`, several `INDEXED BY` names, `REPLACING CHARACTERS`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: SORT(USING / GIVING)/ Added SORT with USING and GIVING
**日本語**: `SORT 整列用ファイル ON ASCENDING|DESCENDING KEY 項目... [WITH DUPLICATES IN ORDER] USING ファイル... GIVING ファイル...` を変換できるようになりました(`SD` も `FD` と同じ扱いです)。USING のファイルの行をすべて読み、キー項目の位置と幅(SD の項目並びから計算)で、キーを後ろから順に安定ソートし、GIVING のファイルへ書き出します(キーは複数可、昇順・降順の混在可、数値キーは数として比べます)。サンプル `tests/samples/sort1.cbl`(年齢の昇順、同じ年齢は名前の降順)で、Python も Rust も `DI 25` / `AL 25` / `CY 30` / `BOB 30` と一致しました。単体試験16件成功。**まだ**: `INPUT PROCEDURE` / `OUTPUT PROCEDURE`(`RELEASE`/`RETURN`)、表の `SORT`、`MERGE`、`COLLATING SEQUENCE`、複数項目をまたぐレコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `SORT sort-file ON ASCENDING|DESCENDING KEY field... [WITH DUPLICATES IN ORDER] USING file... GIVING file...` is now converted (`SD` is handled like `FD`). All records of the USING files are read, then stably sorted from the last key to the first by the key's position and width (taken from the SD layout), then written to the GIVING files (several keys, mixed ascending/descending, numeric keys compared as numbers). In `tests/samples/sort1.cbl` (ascending age, then descending name) Python and Rust both print `DI 25` / `AL 25` / `CY 30` / `BOB 30`; 16 unit tests pass. **Not yet**: `INPUT PROCEDURE` / `OUTPUT PROCEDURE` (`RELEASE`/`RETURN`), table `SORT`, `MERGE`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: SORT の INPUT / OUTPUT PROCEDURE、RELEASE、RETURN / Added SORT INPUT/OUTPUT PROCEDURE, RELEASE and RETURN
**日本語**: `SORT ... INPUT PROCEDURE IS 段落 [THRU 段落]` と `OUTPUT PROCEDURE IS 段落 [THRU 段落]`、手続きの中の `RELEASE レコード [FROM ...]` と `RETURN 整列用ファイル [INTO ...] AT END ... NOT AT END ... END-RETURN` を変換できるようになりました。`USING` と `OUTPUT PROCEDURE`(またはその逆)の組み合わせも可です。整列用の行は State の中のバッファに溜め、INPUT 側が終わったら安定ソートし、OUTPUT 側の `RETURN` が1行ずつ取り出します。サンプル `tests/samples/sort2.cbl`(20歳の行を INPUT で除き、年齢の降順で取り出す)で、Python も Rust も `CY 31` / `BOB 30` / `AL 25` / `DONE` と一致しました。単体試験17件成功。**まだ**: 表の `SORT`、`MERGE`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO` と `RELEASE/RETURN INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `SORT ... INPUT PROCEDURE IS para [THRU para]` and `OUTPUT PROCEDURE IS para [THRU para]`, plus `RELEASE record [FROM ...]` and `RETURN sort-file [INTO ...] AT END ... NOT AT END ... END-RETURN` inside those procedures, are now converted. `USING` can be combined with `OUTPUT PROCEDURE` and vice versa. Sort records collect in a buffer inside State; when the INPUT side ends they are stably sorted, and `RETURN` on the OUTPUT side takes them one by one. In `tests/samples/sort2.cbl` (INPUT drops the age-20 row, output in descending age) Python and Rust both print `CY 31` / `BOB 30` / `AL 25` / `DONE`; 17 unit tests pass. **Not yet**: table `SORT`, `MERGE`, `COLLATING SEQUENCE`, `READ INTO` and `RELEASE/RETURN INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: MERGE / Added MERGE
**日本語**: `MERGE 整列用ファイル ON ASCENDING|DESCENDING KEY 項目... USING ファイル ファイル... GIVING ファイル...`(`OUTPUT PROCEDURE` でも可)を変換できるようになりました。入力が整列済みなら、USING の全ファイルをつないで安定ソートした結果と同じなので、`SORT` と同じ生成コードを使います(同じキーの行は、USING に書いた順のまま並びます)。COBOL の規則どおり、`USING` が1つだけの `MERGE` や `INPUT PROCEDURE` つきの `MERGE` は変換せず `TODO(COBOL_to)` に残します。サンプル `tests/samples/merge1.cbl`(年齢の昇順)で、Python も Rust も `AL 20` / `BO 20` / `CY 30` / `DI 40` と一致しました。単体試験18件成功。**注意**: 入力が整列済みでないと、本物の MERGE は結果が不定ですが、この変換は整列して出します。**まだ**: 表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO` と `RELEASE/RETURN INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3、EXEC SQL/CICS。
**English**: `MERGE sort-file ON ASCENDING|DESCENDING KEY field... USING file file... GIVING file...` (or `OUTPUT PROCEDURE`) is now converted. When the inputs are already sorted, the result equals a stable sort of all USING files joined together, so it shares the `SORT` generated code (rows with equal keys keep the USING order). As COBOL requires, a `MERGE` with a single `USING` file or with an `INPUT PROCEDURE` is not converted and stays as `TODO(COBOL_to)`. In `tests/samples/merge1.cbl` (ascending age) Python and Rust both print `AL 20` / `BO 20` / `CY 30` / `DI 40`; 18 unit tests pass. **Note**: a real MERGE gives undefined results for unsorted input, while this conversion sorts it. **Not yet**: table `SORT`, `COLLATING SEQUENCE`, `READ INTO` and `RELEASE/RETURN INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, COMP-3, EXEC SQL/CICS.

## 2026-11-22 追記: COMP-3 / COMP(使用句)の扱い / USAGE COMP-3 and COMP handling
**日本語**: `COMP-3`・`COMPUTATIONAL-3`・`PACKED-DECIMAL`(パック10進)と、`COMP`・`COMP-4`・`COMP-5`・`BINARY`(2進)を、`USAGE [IS]` の有無にかかわらず読み取り、変数ごとに記録するようになりました(`Var.usage`)。算術は桁数(PICTURE)で決まるので、元から正しく動いていました。サンプル `tests/samples/comp3.cbl`(`ADD`・`COMPUTE`・`MULTIPLY`・`ON SIZE ERROR` を含む)で、Python も Rust も `OVER` / `12347.00 -5 9999 95 185 3.0 -3.25` と一致しました。単体試験19件成功。**できないこと(明記)**: ファイルのレコードに入った COMP-3 / COMP の項目は、この変換器のファイルが「固定長テキストの行」なので、パックや2進のバイト列ではなく**桁の文字**として入出力します(その旨を変換結果の注意書きに出します)。実際のメインフレームの2進ファイルとは互換がありません。パックのバイト列を扱う機能(2進の順編成ファイル、`REDEFINES` や群項目の `MOVE` でのバイト単位の重ね合わせ)は未実装です。また、`COMP-5` を含む2進の桁あふれは、PICTURE の桁数で判定します(本物の2進の幅は使いません)。**まだ**: 上のバイト列対応、表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、EXEC SQL/CICS。
**English**: `COMP-3`, `COMPUTATIONAL-3`, `PACKED-DECIMAL` (packed decimal) and `COMP`, `COMP-4`, `COMP-5`, `BINARY` (binary) are now read, with or without `USAGE [IS]`, and recorded per variable (`Var.usage`). Arithmetic is driven by the PICTURE digits, so it already worked. In `tests/samples/comp3.cbl` (with `ADD`, `COMPUTE`, `MULTIPLY` and `ON SIZE ERROR`) Python and Rust both print `OVER` / `12347.00 -5 9999 95 185 3.0 -3.25`; 19 unit tests pass. **Limits, stated plainly**: files in this converter are fixed-width text lines, so a COMP-3 / COMP item inside a file record is read and written as **digit characters**, not as packed or binary bytes (the output notes say so); it is not compatible with real mainframe binary files. Byte-level packed handling (binary sequential files, byte overlay through `REDEFINES` or group `MOVE`) is not implemented. Overflow of binary items, `COMP-5` included, is judged by the PICTURE digits, not by the real binary width. **Not yet**: the byte-level support above, table `SORT`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, EXEC SQL/CICS.

## 2026-11-22 追記: EXEC SQL(埋め込み SQL)/ Added EXEC SQL (embedded SQL)
**日本語**: `EXEC SQL ... END-EXEC` を変換できるようになりました。対応: `INCLUDE SQLCA`、`DECLARE カーソル CURSOR FOR SELECT ...`(WORKING-STORAGE に書いても手続き部に書いても可)、`OPEN` / `FETCH ... INTO :a, :b` / `CLOSE`、`SELECT ... INTO :a, :b FROM ...`(先頭の1行を取る)、そのほかの文(`INSERT` / `UPDATE` / `DELETE` / `CREATE` / `COMMIT` / `ROLLBACK` など)。`:変数` は `?` に置き換えて、変数の値を文字列にして渡します。結果は `SQLCODE`(自動で宣言)に入れます: 成功 0、行なし 100、カーソルが無い -501、実行の失敗 -1。**実行先**: Python は標準の `sqlite3`(環境変数 `COBOL_SQL_DB` で DB ファイル、無ければメモリ)で、`SQL_HANDLER` を差し替えれば別の DB にもできます。Rust は DB 部品に依存せず(C 依存の SQLite をやめた方針と同じ)、`set_sql_handler(fn(&str, Vec<String>) -> (SQLCODE, 行のリスト))` で実行先を登録します。未登録なら `SQLCODE` は -1 です。サンプル `tests/samples/sql1.cbl` で、Python(実際の sqlite3)の出力 `COUNT=3 SQLCODE=0` / `BOB 20.25 SQLCODE=0` / `NOROW SQLCODE=100` / `ROW 1 ALICE` / `ROW 2 BOB` / `END SQLCODE=100` を確認し、Rust(試験用の疑似ハンドラー)も同じ出力になりました。単体試験20件成功。**確認していないこと**: Rust は本物の DB につないでいません(受け渡しだけを疑似ハンドラーで確認)。数値の列は文字列で渡すので、DB 側の型変換に任せます。**まだ**: `WHENEVER`、指示子変数(`:変数:指示子`)、`SQLSTATE`・`SQLERRM`、`FETCH` の位置指定(`NEXT` 以外)、動的 SQL(`PREPARE` / `EXECUTE`)、`EXEC CICS`、表の `SORT`、`COLLATING SEQUENCE`、複数項目レコードの `READ INTO`、本物の二分探索の `SEARCH ALL`、索引・相対ファイル、COMP-3 のバイト列。
**English**: `EXEC SQL ... END-EXEC` is now converted. Supported: `INCLUDE SQLCA`; `DECLARE cursor CURSOR FOR SELECT ...` (in WORKING-STORAGE or the procedure division); `OPEN` / `FETCH ... INTO :a, :b` / `CLOSE`; `SELECT ... INTO :a, :b FROM ...` (takes the first row); and other statements (`INSERT` / `UPDATE` / `DELETE` / `CREATE` / `COMMIT` / `ROLLBACK` ...). `:host-vars` become `?` and their values are passed as strings. Results go to `SQLCODE` (declared automatically): 0 on success, 100 for no row, -501 for an unknown cursor, -1 when execution fails. **Where it runs**: Python uses the standard `sqlite3` (env `COBOL_SQL_DB` for a DB file, in-memory otherwise) and `SQL_HANDLER` can be replaced to use another database. Rust depends on no database crate (same policy as dropping C-based SQLite); register the target with `set_sql_handler(fn(&str, Vec<String>) -> (SQLCODE, rows))`. Without one, `SQLCODE` is -1. With `tests/samples/sql1.cbl`, Python (real sqlite3) prints `COUNT=3 SQLCODE=0` / `BOB 20.25 SQLCODE=0` / `NOROW SQLCODE=100` / `ROW 1 ALICE` / `ROW 2 BOB` / `END SQLCODE=100`, and Rust (with a test fake handler) prints the same; 20 unit tests pass. **Not verified**: Rust was not connected to a real database (only the hand-over was checked with the fake handler). Numeric columns are passed as strings and left to the database's type conversion. **Not yet**: `WHENEVER`, indicator variables (`:var:ind`), `SQLSTATE` / `SQLERRM`, positioned `FETCH` other than `NEXT`, dynamic SQL (`PREPARE` / `EXECUTE`), `EXEC CICS`, table `SORT`, `COLLATING SEQUENCE`, `READ INTO` for multi-field records, a true binary `SEARCH ALL`, indexed and relative files, byte-level COMP-3.

## 2026-11-22 追記: EXEC SQL WHENEVER (JA/EN)
**日本語**: `EXEC SQL WHENEVER SQLERROR|SQLWARNING|NOT FOUND` に対応(`CONTINUE`・`GOTO`/`GO TO 段落`・`PERFORM 段落`・`STOP`)。以降の各 SQL 文の直後に SQLCODE の判定(SQLERROR=負、SQLWARNING=正で100以外、NOT FOUND=100)を挿入する。`tests/samples/sql2.cbl` で Python と Rust(疑似ハンドラー)の出力一致を確認(20件成功)。**未了**: 指示変数、SQLSTATE/SQLERRM、NEXT 以外の FETCH、動的SQL、EXEC CICS。WHENEVER の効く範囲はソース上の並び順(COBOL の仕様どおり)。
**English**: Added `EXEC SQL WHENEVER SQLERROR|SQLWARNING|NOT FOUND` (`CONTINUE`, `GOTO`/`GO TO para`, `PERFORM para`, `STOP`). A SQLCODE check follows each later SQL statement (SQLERROR = negative, SQLWARNING = positive but not 100, NOT FOUND = 100). Python and Rust (fake handler) outputs match on `tests/samples/sql2.cbl` (20 tests pass). **Open**: indicator variables, SQLSTATE/SQLERRM, FETCH other than NEXT, dynamic SQL, EXEC CICS. WHENEVER scope follows source order (as in COBOL).

## 2026-11-22 追記: EXEC SQL 指示変数 (JA/EN)
**日本語**: 指示変数(`:変数:指示変数`)に対応。入力は指示変数が負なら NULL を渡し、`INTO` では NULL の列のとき指示変数に -1(それ以外は 0)を入れて、変数の値は 0/空白にする。NULL は文字列 `SQL_NULL`(`\0NULL`)で表す(Python 標準ハンドラーは sqlite3 の None と相互変換、Rust の自作ハンドラーは同じ文字列を受け取り/返す)。`tests/samples/sql3.cbl` で Python と Rust(疑似ハンドラー)の出力一致を確認。**未了**: `INDICATOR` キーワード形、SQLSTATE/SQLERRM、NEXT 以外の FETCH、動的SQL、EXEC CICS。
**English**: Added indicator variables (`:var:ind`). On input a negative indicator passes NULL; on `INTO` a NULL column sets the indicator to -1 (else 0) and the variable to 0/spaces. NULL is the string `SQL_NULL` (`\0NULL`): the Python default handler maps it to/from sqlite3 None, and a custom Rust handler receives/returns the same string. Python and Rust (fake handler) match on `tests/samples/sql3.cbl`. **Open**: the `INDICATOR` keyword form, SQLSTATE/SQLERRM, FETCH other than NEXT, dynamic SQL, EXEC CICS.

## 2026-11-22 追記: メソッド参照と Comparator (JA/EN)
**日本語**: メソッド参照(`String::length`・`Integer::parseInt`・`Integer::sum`・`this::m`・`型::getXxx` など)をラムダ/関数へ変換し、`.sorted(Comparator)`(`Comparator.comparing/comparingInt/naturalOrder/reverseOrder` と `.reversed()`、ラムダの比較関数)に対応。生成した Python を実際に実行して結果が一致することを単体試験で確認(19件成功)。**未了**: `thenComparing`、`Collectors.groupingBy`、匿名クラス、Rust 側のメソッド本体、Spring PetClinic での通し試験。
**English**: Method references (`String::length`, `Integer::parseInt`, `Integer::sum`, `this::m`, `Type::getXxx`, ...) now become lambdas/functions, and `.sorted(Comparator)` works (`Comparator.comparing/comparingInt/naturalOrder/reverseOrder`, `.reversed()`, lambda comparators). The unit test runs the generated Python and checks the result (19 tests pass). **Open**: `thenComparing`, `Collectors.groupingBy`, anonymous classes, Rust-side method bodies, an end-to-end run on Spring PetClinic.

## 2026-11-22 追記: groupingBy と thenComparing (JA/EN)
**日本語**: `Collectors.groupingBy(f)`(値は一覧)と `groupingBy(f, Collectors.counting())`(件数)、`Comparator.thenComparing/thenComparingInt` を Python へ変換。生成した Python を単体試験で実行し結果の一致を確認(20件成功)。**未了**: `groupingBy` の他の下流コレクター(`mapping`・`summingInt` 等)、`toMap`、匿名クラス、Rust 側のメソッド本体、Spring PetClinic での通し試験。
**English**: `Collectors.groupingBy(f)` (lists), `groupingBy(f, Collectors.counting())` (counts) and `Comparator.thenComparing/thenComparingInt` now convert to Python; the unit test runs the generated code and checks the result (20 tests pass). **Open**: other downstream collectors (`mapping`, `summingInt`, ...), `toMap`, anonymous classes, Rust-side method bodies, an end-to-end run on Spring PetClinic.

## 2026-11-22 追記: toMap と groupingBy の下流コレクター (JA/EN)
**日本語**: `Collectors.toMap(k, v)`、`groupingBy` の下流 `counting`/`summingInt`(Long・Double)/`mapping(f, toList()|toSet())` を Python へ変換。生成した Python を単体試験で実行して結果の一致を確認(21件成功)。括弧の数え間違い(ラムダ内の `)` を削っていた)も直した。**未了**: `toMap` の重複キー結合、`averagingInt`、`partitioningBy`、匿名クラス、Rust 側のメソッド本体、Spring PetClinic での通し試験。
**English**: `Collectors.toMap(k, v)` and the `groupingBy` downstream collectors `counting`, `summingInt` (Long/Double) and `mapping(f, toList()|toSet())` now convert to Python; the unit test runs the generated code and checks the result (21 tests pass). Also fixed a paren-stripping bug that ate the `)` of a lambda. **Open**: `toMap` merge function, `averagingInt`, `partitioningBy`, anonymous classes, Rust-side method bodies, an end-to-end run on Spring PetClinic.


## 2026-11-22 現状と完成予定と課題 / Status, plan and issues

### 日本語
**現状**: COBOL・C・C++・Fortran・Java を、選んだ組み合わせ(Axum/Poem/FastAPI/RPoem/RPoRea × HTML5/CSS3/TypeScript/Bootstrap × MySQL/MariaDB/PostgreSQL/SQLite)のウェブアプリにするお助けロボット。`converters/cobol_to` と `converters/java_to` は COBOL_to・Java_to と同じ内容を取り込み(COBOL: SORT/MERGE/SEARCH/EXEC SQL 一式、Java: メソッド参照・Comparator・Collectors)、試験55件成功。`pl2pl c2rust`・`f2rust` で単純な C/C++/Fortran を Rust へ。
**完成予定**: 元の COBOL_to / Java_to の削除(本人確認のうえ最後に)、Mac・iPhone 実機確認、COBOL→他 OS の COBOL の拡張、Kokkos の GPU 実行、メール到達の確認。
**課題**: 取り込み版と元リポジトリを手で同期している(自動同期なし)。C++/Fortran の Rust 化は単純な範囲のみ。Rust 側の EXEC SQL は疑似ハンドラーでの検証にとどまる。

### English
**Status**: A helper robot that turns COBOL, C, C++, Fortran and Java into web apps in the mix you choose (Axum/Poem/FastAPI/RPoem/RPoRea x HTML5/CSS3/TypeScript/Bootstrap x MySQL/MariaDB/PostgreSQL/SQLite). `converters/cobol_to` and `converters/java_to` mirror COBOL_to and Java_to (COBOL: SORT/MERGE/SEARCH/EXEC SQL; Java: method references, Comparator, Collectors); 55 tests pass. `pl2pl c2rust` and `f2rust` rewrite simple C/C++/Fortran into Rust.
**Planned**: deleting the original COBOL_to / Java_to (last, after the owner confirms), real Mac/iPhone checks, extending COBOL to other OSes, Kokkos on a GPU, a mail-arrival check.
**Issues**: The vendored copies are synced by hand (no automatic sync). C++/Fortran to Rust covers only simple code. Rust-side EXEC SQL is verified only with a fake handler.


## 2026-11-22 更新: Collectors 対応の完了と残課題 / Update: Collectors done, what remains

### 日本語
**現状**: Collectors の対応が揃いました: `toList`/`toSet`/`joining`/`toMap`(重複キーの結合関数つき)/`groupingBy`(下流 `counting`・`summingInt`・`mapping`)/`partitioningBy`(`counting` つき)/`averagingInt`。メソッド参照、`Comparator`(`comparing`/`thenComparing`/`reversed`)も対応済み。生成した Python を単体試験で実際に動かして結果を確認(Java_to 23件・pl2pl 55件成功)。
**未了(残る課題)**: 匿名クラス、Rust 側のメソッド本体(今は単純な return のみ)、Spring PetClinic など実プロジェクトでの通し試験、`groupingBy` のその他の下流コレクター(`summingDouble` 以外の `reducing` など)。
**課題**: 中間結果を list で即時評価するため、無限・遅延ストリームは対象外。実プロジェクトでの検証が未了。

### English
**Status**: Collectors coverage is now complete for the common cases: `toList`/`toSet`/`joining`/`toMap` (with a merge function)/`groupingBy` (downstream `counting`, `summingInt`, `mapping`)/`partitioningBy` (with `counting`)/`averagingInt`. Method references and `Comparator` (`comparing`/`thenComparing`/`reversed`) are supported too. Unit tests run the generated Python and check results (Java_to 23, pl2pl 55 pass).
**Still open**: anonymous classes, Rust-side method bodies (currently a simple return only), end-to-end runs on real projects such as Spring PetClinic, and other `groupingBy` downstream collectors (`reducing`, etc.).
**Issues**: intermediate results are evaluated eagerly as lists, so infinite or lazy streams are out of scope; real-project verification is not done.

## 2026-11-22 追記: 匿名クラス (JA/EN)
**日本語**: `new Comparator<T>() { public int compare(...) {...} }` のような匿名クラスを、メソッドが1つなら Python の関数 `_anonN` として直前に出し、その名前に置き換える(関数型インターフェースの代用)。生成した Python を実行して並び替えの結果が一致することを単体試験で確認(24件成功)。メソッドが複数ある場合は最初の1つだけ関数にし、注記を残す。**未了**: 状態(フィールド)を持つ匿名クラス、Rust 側のメソッド本体、Spring PetClinic などでの通し試験。
**English**: Anonymous classes such as `new Comparator<T>() { public int compare(...) {...} }` with a single method are emitted just before as a Python function `_anonN` and replaced by its name (standing in for a functional interface). The unit test runs the generated Python and checks the sort result (24 tests pass). With several methods only the first becomes a function, with a note. **Open**: anonymous classes that hold state (fields), Rust-side method bodies, end-to-end runs on Spring PetClinic and similar.

## 2026-11-22 再開用メモ(pl2pl・日本語)
**現状**: `pl2pl web --backend ...`(COBOL・C・C++・Fortran→Web)、`pl2pl c2rust`・`f2rust`、`pl2pl cobol --os` は動作済み。`converters/cobol_to`(WHENEVER・指示変数まで)と `converters/java_to`(Collectors・匿名クラス・Rust側の本体変換)は元の COBOL_to / Java_to と同期済み(試験: COBOL_to 20・Java_to 24・pl2pl 55)。
**未了(この順、確認不要)**: (1)Java_to の Rust 側(状態を持つ匿名クラス・`reducing`)、(2)COBOL_to の INDICATOR 句・SQLSTATE・動的SQL・EXEC CICS、(3)姉妹(RPoRea・Reaxt・RPoem・RMojo)での通し試験、(4)gfortran 以外の実機確認、Mac・iPhone 実機(実機なし)。
**決まり**: 本物で確認してから「完了」/秘密は書かない/新しいRESTは作らない/元の COBOL_to・Java_to の削除だけ最後に本人確認/VPS用ビルドは `Ubuntu-24.04`/変換器を直したら `converters/` へ同期。
**再開用メッセージ**: 前の続きです。`F:\pl2pl\PORTING.md` 末尾「2026-11-22 再開用メモ」を読んで、上の未了を確認なしで順に進めてください。返答は日本語のみ。

## 2026-11-22 追記: INDICATOR キーワード形 (日本語)
`:変数 INDICATOR :指示変数` の形を、`:変数:指示変数` と同じに扱うようにしました(パラメーターと `INTO` の両方)。`tests/samples/sql4.cbl` で、Python・Rust とも `sql3.cbl` と同じ出力になることを試験しました(21件成功)。固定形式の72桁を超える行は切り捨てられるため、長い EXEC SQL は折り返して書きます。残り: SQLSTATE・動的SQL・EXEC CICS。

## 2026-11-22 追記: SQLSTATE (日本語)
ソースに `SQLSTATE` が出てくるプログラムでは、SQL 文のたびに `SQLSTATE`(PIC X(5)、未宣言なら自動で追加)を SQLCODE から導きます(0→`00000`、100→`02000`、それ以外→`HY000`)。生成した Python を実際に動かし、`00000` と `02000` が出ることを確認しました(`tests/samples/sql5.cbl`、試験22件成功)。DBごとの細かい状態コードの対応は未了です。残り: 動的SQL・EXEC CICS。

## 2026-11-22 追記: EXECUTE IMMEDIATE(動的 SQL・日本語)
- `EXEC SQL EXECUTE IMMEDIATE :変数 END-EXEC` に対応しました。SQL 文を変数の中身(前後の空白は除去)から実行し、SQLCODE を更新します(Python・Rust 両方)。文字列定数を直接渡す形は通常の SQL 文として扱います。
- 試験は23件成功。生成した Python を実行し、CREATE・INSERT・SELECT の結果 `0` と `7` が出ることを確認しました(`tests/samples/sql6.cbl`)。Rust は生成コードの確認のみで、実行は未確認です。
- 未了: PREPARE/EXECUTE(`?` の引数つき)、EXEC CICS。

## 2026-11-23 追記: 複数メソッドを持つ無名クラス(日本語)
- 無名クラスにメソッドが複数あるとき、これまでは最初の1つだけを出していましたが、**すべてのメソッドを Python クラスのメソッドとして出す**ようにしました(フィールドは各メソッドから見えます)。最初のメソッドは従来どおり `__call__` でも呼べます。
- 単体テストは31件成功(`anonymous_class_multiple_methods`)。生成物の文字列確認までで、Python 実行の通し確認は未了です。
- 未了: コレクション操作を含む Rust 本体、Spring PetClinic など実プロジェクトでの通し確認。

## 2026-11-23 追記: PREPARE / EXECUTE ... USING(日本語)
- `EXEC SQL PREPARE 名 FROM :変数` で SQL 文を名前つきで保存し、`EXEC SQL EXECUTE 名 [USING :a, :b]` で `?` に値を渡して実行できます(Python・Rust 両方)。未準備の名前を実行すると SQLCODE は -518 です。
- 試験は24件成功。生成した Python を実行し、`INSERT ... VALUES (?)` に 5 を渡して SELECT で 5 が返ることを確認しました(`tests/samples/sql7.cbl`)。Rust は生成コードの確認のみです。
- 未了: EXEC CICS、`DESCRIBE`/SQLDA を使う完全な動的 SQL。

## 2026-11-23 追記: EXEC CICS の最小対応(日本語)
- **調査(英語・日本語のWeb検索とGitHub)**: EXEC CICS を実際に Python/Rust へ変換するオープンソースは見つかりませんでした。多くのツールは CICS 部分に TODO を付けて人手に任せ、`viper-mf` は CICS を変換せずエミュレーターで動かす方式、`cobol-py` は EXEC CICS を抜き出す解析だけです。
- **実装**: `EXEC CICS SEND TEXT FROM(変数 または '文字列')` は表示に、`RETURN` と `ABEND` は終了にします。`READ FILE`・`XCTL`・`SEND MAP` など、ほかの命令は黙って落とさず「未対応」の TODO として残します。
- 試験は25件成功(`exec_cics_minimal`)。生成した Python の文字列確認までで、画面(BMS マップ)の再現はしていません。
- 未了: SEND/RECEIVE MAP(BMS)、READ/WRITE FILE、XCTL/LINK、COMMAREA。

## 2026-11-23 追記: Rust 本体の List 操作(日本語)
- Rust(Poem / RPoem)側のメソッド本体で、`List<Integer> xs = new ArrayList<>();`(`Vec<i32>` になる。`Integer` などの箱型は基本型に直す)、`xs.add(x);`(`push`)、`xs.get(i)`(添字アクセス)、拡張 for `for (int x : xs)` を変換できるようにしました。`xs.size()` は従来どおり `len()` です。
- 単体テストは32件成功(`list_operations`、生成された Rust の文字列確認)。生成 Rust のコンパイル確認(`tests/run-rust.sh`)はこの追加分では未実施です。
- 未了: Map や Stream を含む Rust 本体、Spring PetClinic など実プロジェクトでの通し確認。

## 2026-11-23 追記: List 操作の生成 Rust をコンパイル確認(日本語)
- 前回の List 操作(宣言・`add`・`get`・拡張 for)を、Spring サンプルに `/squares/{n}` を足して実際に変換し、WSL で `tests/run-rust.sh` を実行しました。生成された Rust(`Vec<i32>`、`push`、`for x in xs.clone()`、`xs.len()`)は、spring・jaxrs・servlet・spark の4つとも `cargo check` に通りました。
- 実行して値を確かめるところまでは未了です。未了: Map や Stream を含む Rust 本体、Spring PetClinic など実プロジェクトでの通し確認。

## 2026-11-23 追記: OpenAPI 先行のプロジェクトに対応(Spring PetClinic REST で確認・日本語)
- **きっかけ(実プロジェクトでの通し確認)**: GitHub の `spring-petclinic/spring-petclinic-rest`(Java 109ファイル)を変換にかけたところ、ルートが **1本しか見つかりませんでした**。このプロジェクトは `openapi.yml` から生成した `XxxApi` インターフェースを `implements` する方式で、コントローラーに `@GetMapping` などが無いためです。
- **対応**: `openapi.yml`(`openapi.yaml`・`swagger.yml`・`swagger.yaml` も)の `paths:` から (operationId・動詞・経路) を読み、`implements XxxApi` のコントローラーでメソッド名が operationId と一致するものをルートにします。引数は、経路の `{名前}` に当たれば Path、単純な型なら Query、それ以外は Body としました。
- **結果**: ルートが **1本 → 37本**(openapi.yml の operationId 37件と一致)になりました。単体テスト34件成功(`openapi_first_routes`)。`tests/run-pc.sh <プロジェクトのパス>` で実プロジェクトを変換できます。
- **まだ**: 本体(`ResponseEntity<...Dto>` を返す部分や MapStruct のマッパー)は変換していません。サーバー URL の接頭辞(`/api` など)、生成した FastAPI/Rust の実行確認、`/v2` 系のページング型は未対応です。

## 2026-11-23 追記: PetClinic の生成 Python を構文エラー0に(日本語)
- 実プロジェクト(Spring PetClinic REST)の変換結果を `python -m py_compile` にかけたところ、2種類の構文エラーが出たため直しました。
  - **クラスリテラル** `Foo.class`(82か所)が Python に無い `.class` のまま出ていたので、`Foo` に変換します。
  - **`e.getClass().getSimpleName()`** が getter の規則で `e.class.simpleName` になっていたので、`type(e).__name__` にします(`getName` も同様)。
- 結果: 生成された Python 7ファイルすべてが構文エラー0になりました。単体テスト36件成功(`class_literal`、`get_class_name`)。`tests/check-py2.sh` で構文確認、`tests/check-py.sh` で TODO の数も見られます。
- **まだ**: 生成物の実行確認(FastAPI を起動して呼ぶ)は未実施です。`services.py` に TODO が108件残っています(JDBC・MapStruct・`ResponseEntity` などの本体)。

## 2026-11-23 追記: PetClinic の TODO の内訳(日本語)
- `tests/todo-top.sh` で生成 Python の TODO を種類ごとに数えました。残りの TODO の大半は、**インターフェースのメソッドの実装**です。`XxxMapper`(MapStruct が実行時に作るもの)と `XxxRepository`(Spring Data / JPA が実行時に作るもの)で、Java のソースには本体がありません。ほかは導出クエリ(`findByLastName` など)と未対応の式が数件です。
- これは「黙って落とさず、TODO を `NotImplementedError` で残す」という既存の方針どおりの出力です。自動で埋めるには、Spring Data のメソッド名からの SQL 生成(導出クエリ)と、MapStruct のフィールド対応からの変換関数の生成が必要で、設計の判断を伴うため、次の課題にしました。
- 未了: 導出クエリの SQL 生成、MapStruct マッパーの生成、生成した FastAPI の起動確認。

## 2026-11-23 追記: 導出クエリの対応を拡大(日本語)
- **調査(英語・日本語のWeb検索とGitHub)**: Spring Data の導出クエリの規則は公式の「Repository query keywords」が正本です(`find…By` の `By` から条件が始まる、`And`/`Or`、`Like`・`Containing`・`StartingWith`・`EndingWith`、`GreaterThan`・`LessThan`・`Between`・`In`・`IsNull`、`OrderBy…Asc/Desc` など)。メソッド名から条件を作る実装は Spring Data Commons の `PartTree` ですが、Python へ移せる既存ライブラリは見つかりませんでした(日本語の解説も公式の英語文書が中心)。
- **実装**: 生成コードの実行時部品 `java_rt.py` の `InMemoryRepository._derived` を拡張しました。`_or_`、`Containing`/`NotContaining`、`Like`/`NotLike`(`%`・`_`)、`StartingWith`/`EndingWith`、`GreaterThan`/`LessThan`(と `Equal` 付き)、`Between`、`In`/`NotIn`、`IsNull`/`IsNotNull`、`Not`、`True`/`False`、`IgnoreCase`、`OrderBy…Asc/Desc` に対応します。
- **確認**: 単体テスト37件成功(`derived_queries_in_memory_repo`、実際に Python でメモリ上のリポジトリに問い合わせ、並びと件数を検証)。
- **まだ**: メモリ上の足場であり、実 DB(SQL 生成)ではありません。`Top`/`First` の件数制限、`Distinct`、入れ子のプロパティ(`AddressCity`)、MapStruct マッパーの生成は未対応です。

## 2026-11-23 追記: PetClinic の生成アプリが起動(import)するようになった(日本語)
- 生成した FastAPI アプリ(`main.py`)を実際に `import` して起動を試したところ、実行時の不具合が2つ見つかったので直しました。
  - `LoggerFactory.getLogger(...)` が未定義だったため、実行時部品 `java_rt.py` に SLF4J 相当の `LoggerFactory`(`info/warn/error/debug/trace`、`{}` の置換つき)を追加。
  - `private static final Logger logger = LoggerFactory.getLogger(X.class)` のように、**自分のクラス名を使う static の初期値**がクラス本体の中で `NameError` になっていたため、クラス定義の後ろで `X.logger = ...` と代入する形にしました。
- 結果: PetClinic の変換結果が `import ok`(ルート41本、うち37本が API)になりました。
- **実際に呼ぶと500のまま**です。例: `GET /pettypes` は `PetTypeRestControllerV1 object has no attribute clinicService`。コンストラクター注入(`ClinicService` やマッパー)の配線が、この形(Lombok や明示コンストラクター)ではまだ生成されていません。リポジトリ・マッパーのインターフェースも TODO のままです。
- 未了: コンストラクター注入の自動配線、MapStruct マッパーの生成、実 DB。

## 2026-11-23 追記: `this.フィールド` の名前を揃えた(PetClinic の呼び出しが先へ進む・日本語)
- 生成した PetClinic アプリを呼ぶと `object has no attribute clinicService` で落ちていた原因は、本体の `this.clinicService` が **camelCase のまま** `self.clinicService` と出る一方、フィールドの宣言は `self.clinic_service`(snake_case)だったことでした。`this.xxx` は snake_case にそろえるよう直しました(単体テスト38件成功、`this_field_is_snake_case`)。
- 結果: `GET /pettypes`・`/owners`・`/vets` の処理が **コントローラー → サービス → リポジトリ** まで進むようになりました。いまはリポジトリのインターフェース(`PetTypeRepository.findAll` など)の `NotImplementedError`(TODO)で止まります。
- **まだ**: リポジトリのインターフェースに実装クラス(Jdbc・JPA・Spring Data JPA の3種類がある)が複数ある場合の選び方と、メモリ上の足場(`InMemoryRepository`)への割り当て、MapStruct マッパーの生成は未対応です。

## 2026-11-25 現況・未了 / Status and open items

### 日本語

現状: 古いプログラム(COBOL・C・C++・Fortran・Java など)を、選んだ言語とフレームワークへ書きかえる道具。最近は Java_to の PetClinic 対応(ResponseEntity・Mapper・親データクラス)を取り込み、多言語の文書を同期した。残り: Mac・iPhone の実機での確認(ここには実機がありません)。翻訳: AI 翻訳(ja/en 以外)、ネイティブの確認はまだです。

### English

Status: a tool that rewrites old programs (COBOL, C, C++, Fortran, Java and more) into the language and framework you choose. Recently it took in Java_to's PetClinic support (ResponseEntity, Mapper, parent data classes) and its translated docs were synced. Open: checks on a real Mac and iPhone (no such devices here). Translations: AI-translated (other than ja/en), not yet checked by native speakers.

## 2026-11-25 追記: 変換器の同期 / Converter sync

**日本語**: `converters/cobol_to` に COBOL_to の EXEC CICS LINK/XCTL→CALL(`7d67bbf`)、`converters/java_to` に Java_to の三項演算子(Rust 本体、`e9ac300`)を取り込みました。試験: Java_to 39・COBOL_to 25・pl2pl 55、すべて成功。未了: Mac・iPhone 実機、生成した FastAPI の実 DB 起動確認。

**English**: Synced `converters/cobol_to` with COBOL_to's EXEC CICS LINK/XCTL to CALL (`7d67bbf`) and `converters/java_to` with Java_to's ternary operator in Rust bodies (`e9ac300`). Tests: Java_to 39, COBOL_to 25, pl2pl 55, all pass. Open: real Mac/iPhone devices and starting generated FastAPI apps against a real DB.

## 2026-11-25 追記: 実装が複数あるときの選び方を試験で固定 / Pinned implementation choice with a test

**日本語**: リポジトリのインターフェースに実装が複数あるとき、メモリ上で動くもの(名前に Jdbc・Jpa を含まない)を優先し、無ければ先頭を選ぶ処理は実装済みでした(以前の「未対応」の記述は古い)。単体テスト `multiple_impls_prefer_non_db_one` で固定しました(Java_to 40件成功)。未了: MapStruct の複雑なマッパー、実 DB での起動確認。

**English**: When a repository interface has several implementations, the generator already prefers one without Jdbc/Jpa in its name and otherwise takes the first (the earlier "unsupported" note was stale). It is now pinned by the unit test `multiple_impls_prefer_non_db_one` (Java_to: 40 pass). Open: complex MapStruct mappers and starting against a real DB.

## 2026-11-25 追記: MapStruct の @Mapping に対応 / MapStruct @Mapping support

**日本語**: マッパーのメソッドに付けた `@Mapping(target=..., source="a.b")`(入れ子の元の属性)、`ignore=true`、`constant="..."` を、生成する Python の `MapperBase._to(cls, src, rename, ignore, const)` へ渡すようにしました。単体テスト `mapstruct_mapping_annotations`(Java_to 41件成功)と、実行時の動作確認(`owner.name`→`fullName`、ignore、定数)をしました。未了: `@Mappings({...})` の入れ子の書き方、`expression=`、`@MappingTarget`、実 DB での起動確認。

**English**: Mapper methods' `@Mapping(target=..., source="a.b")` (nested source), `ignore=true` and `constant="..."` are now passed to the generated Python `MapperBase._to(cls, src, rename, ignore, const)`. Covered by the unit test `mapstruct_mapping_annotations` (Java_to: 41 pass) and a runtime check (`owner.name` to `fullName`, ignore, constant). Open: the nested `@Mappings({...})` form, `expression=`, `@MappingTarget`, and starting against a real DB.

<!-- directive-20261125-1130 -->
## 2026-11-25 指示(11:30 まで確認不要) / Directive (no confirmation until 11:30)
**日本語**: 2026-11-25 11:30 まで、未了は確認なしで完成まで自動で進める。需要が少ない・必要性が低いと判断される、あるいは1から大規模に作り直す必要があっても、別の方式で済ませず、正面から作り直す。同時に開発する対象: COBOL_to・Java_to・MFastAPI・2MFastAPI(他の言語・フレームワークから MFastAPI 互換へ。Node.js・TypeScript は例外的に選べ、選ばない選択も可能)・RMojo・RPoRea(RPoem と Reaxt の統合)・2RPoRea(他の言語・フレームワークから RPoRea へ)・Reaxt(React+Next.js+Nuxt.js+Nest.js の統合)。1つ変えたら他を確認し、README・CLAUDE・PORTING・POLICY を日英で同じ日に更新する。
**English**: Until 2026-11-25 11:30, carry unfinished work through to completion automatically without asking. Even if demand looks low or a large from-scratch rewrite is needed, do not substitute another approach; do the real work. Developed together: COBOL_to, Java_to, MFastAPI, 2MFastAPI (converts other languages and frameworks to MFastAPI-compatible code; Node.js and TypeScript are an optional exception and choosing neither is allowed), RMojo, RPoRea (RPoem and Reaxt integrated), 2RPoRea (converts other languages and frameworks to RPoRea) and Reaxt (React + Next.js + Nuxt.js + Nest.js integrated). When one changes, check the others and update README, CLAUDE, PORTING and POLICY in Japanese and English the same day.

<!-- directive-20261125-mojo -->
## 2026-11-25 追記: MFastAPI は Mojo から使えること / Addendum: MFastAPI must be usable from Mojo
**日本語**: MFastAPI は **Mojo から使用できる仕様**を保つ(Python 構文・Rust 実装・Mojo 対応、Windows では RMojo の `compat/wsl-bridge` 経由で WSL 上の Mojo を使う)。2MFastAPI の変換結果も Mojo から使えること。現状: `mojo/` は参照実装で、段階B・Cのサーバーは実験的。Mojo からの呼び出しの実機確認は、確認済みの範囲だけを「確認済み」と書く。11:30 まで確認なしで自動で進める。
**English**: MFastAPI keeps the **specification of being usable from Mojo** (Python syntax, Rust core, Mojo support; on Windows the real Mojo in WSL is driven through RMojo's `compat/wsl-bridge`). Output of 2MFastAPI must be usable from Mojo too. Current state: `mojo/` is a reference implementation and its stage B/C servers are experimental. Only what was actually run is called "verified". Continue automatically without asking until 11:30.

## 2026-11-25 追記: c2rust の配列・break/continue 対応 / c2rust: arrays, break/continue

**日本語**: `pl2pl c2rust` が固定長の配列(`int a[8] = {...};`・`double w[4];`、足りない初期値は 0 で埋める)、添字の読み書き(`a[j + 1]` → `a[(j + 1) as usize]`)、`break;`・`continue;`、`while (1)` → `loop` に対応しました。泡立て並べ替えと `while (1)` を含むサンプルを WSL の gcc と rustc で動かし、出力が完全に一致しました(`1 2 3 4 5 7 8 9` / `6.00 12.00 9`)。単体テスト 56 件成功。まだ: ポインタ・構造体・2次元配列・配列を関数に渡すこと・整数と小数の暗黙の型変換(`k * 1.5` は `(double)k` と書く必要あり)。

**English**: `pl2pl c2rust` now handles fixed-size arrays (`int a[8] = {...};`, `double w[4];`, missing initializers padded with 0), indexing for reads and writes (`a[j + 1]` becomes `a[(j + 1) as usize]`), `break;`, `continue;`, and `while (1)` as `loop`. A sample with bubble sort and `while (1)` was compiled with gcc and rustc in WSL and the outputs match exactly. 56 unit tests pass. Not yet: pointers, structs, 2-D arrays, passing arrays to functions, and implicit int/float conversion (`k * 1.5` must be written with `(double)k`).


## 2026-11-26 追記: pl2pl f2c(単純な Fortran → C) / Added: pl2pl f2c (simple Fortran to C)
**日本語**: `pl2pl f2c <dir|file> --out <dir>` を追加(program・integer/real・do〈刻み付き〉・if/else・`print *`)。わからない行・配列は `TODO(f2c)` で残しレポートへ。試験2件成功、gcc でビルドし `print *` の整形(整数は幅11)を確認。**未確認**: この WSL に gfortran が無く、本物の gfortran との出力一致は未実施(期待値は gfortran の仕様どおりに合わせただけ)。**未了**: c2f、C↔C++、Rust→C/Fortran、配列・subroutine。
**English**: Added `pl2pl f2c <dir|file> --out <dir>` (program, integer/real, do with step, if/else, `print *`). Unsupported lines and arrays stay as `TODO(f2c)` and go to the report. 2 tests pass; the output builds with gcc and matches the expected `print *` layout (integers width 11). **Unverified**: this WSL has no gfortran, so equality with real gfortran output was not checked. **Open**: c2f, C to/from C++, Rust to C/Fortran, arrays and subroutines.

## 2026-11-26 追記: c2rust の暗黙の型変換・配列引数・2次元配列・構造体・ポインタ引数 / c2rust: promotion, array params, 2-D arrays, structs, pointer params

**日本語**: `pl2pl c2rust` の式の読み方を、行の置き換えから「型を覚える小さな構文解析」に作り直しました。これで次のことができます。
- 整数と小数を混ぜた式は C と同じように小数へ変換します(`k * 1.5`、`int n = x;`、`n += x;`)。`k / 2 + x` の `k / 2` は C と同じく整数のまま割ります。
- 配列を関数に渡せます(`int a[]` → `&mut [i32]`、呼ぶ側は `&mut a`)。
- 2次元配列を使えます(`int m[3][3] = {{...}}`、引数 `int m[][3]`)。
- 単純な構造体を使えます(`struct P { int x; double y; };`、`{1, 2.5}` での初期化、構造体の配列、`.` と `->`)。
- 出力用のポインタ引数を使えます(`void f(int *x)` → `x: &mut i32`、`f(&v)` → `f(&mut v)`)。

これらをまとめた C のサンプルを WSL の gcc と rustc で動かし、7行の出力が完全に一致しました。単体テストは 60 件すべて成功です。
まだできないこと: ポインタ変数(`int *p = ...`)・動的確保・文字列操作・マクロ・ファイル I/O・`int a, b;` のような複数宣言・`sqrt` などの標準関数。また、同じ配列を `&mut` で渡しながら同じ呼び出しの中で読むと、Rust の借用の規則でコンパイルが通りません。

**English**: `pl2pl c2rust` now reads expressions with a small typed parser instead of line-by-line replacement. It now handles:
- Implicit int-to-double promotion as in C (`k * 1.5`, `int n = x;`, `n += x;`). In `k / 2 + x`, `k / 2` stays an integer division, as in C.
- Passing arrays to functions (`int a[]` becomes `&mut [i32]`; the caller passes `&mut a`).
- 2-D arrays (`int m[3][3] = {{...}}`, parameter `int m[][3]`).
- Simple structs (`struct P { int x; double y; };`, `{1, 2.5}` initializers, arrays of structs, `.` and `->`).
- Pointer out-parameters (`void f(int *x)` becomes `x: &mut i32`; `f(&v)` becomes `f(&mut v)`).

A C sample using all of these was compiled with gcc and rustc in WSL, and the 7 output lines match exactly. All 60 unit tests pass.
Not yet supported: pointer variables (`int *p = ...`), dynamic allocation, string handling, macros, file I/O, multiple declarations such as `int a, b;`, and standard functions such as `sqrt`. Also, passing an array as `&mut` while reading it in the same call does not compile under Rust's borrow rules.

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

## 2026-11-27 追記(3): c2rust の scanf / Update (3): scanf in c2rust
**日本語**: `pl2pl c2rust` が `scanf("%d %lf %ld", &a, &b, &c);` の形(変換指定 `%d`・`%i`・`%ld`・`%lf`・`%f`、引数は `&変数名` だけ)に対応。出力には標準入力から空白区切りで読む小さな関数 `c_scan_*` が付く。`tests/samples/c2rust/scan.c`+`scan.in` を gcc と rustc に同じ入力で与えて出力2行が完全一致(`tests/c2rust-check.sh` は同名の `.in` があれば標準入力に渡す)。単体テスト70件成功。**限界**: 読み取りに失敗しても 0 を入れるだけで戻り値は C と同じではない、`%s`・`%c`・幅指定・`&a[i]` は未対応(TODO で残る)。**未対応**: ポインターのポインター、`%#x`。
**English**: `pl2pl c2rust` now handles `scanf("%d %lf %ld", &a, &b, &c);` (conversions `%d`, `%i`, `%ld`, `%lf`, `%f`; arguments must be plain `&variable`). The output gets small `c_scan_*` functions that read whitespace-separated tokens from stdin. `tests/samples/c2rust/scan.c` with `scan.in`, fed the same input to gcc and rustc, gives identical 2-line output (`tests/c2rust-check.sh` pipes a same-named `.in` file to stdin). 70 unit tests pass. **Limits**: a failed read just stores 0 and the return value is not the same as in C; `%s`, `%c`, widths and `&a[i]` are not supported (left as TODO). **Not yet**: pointer-to-pointer and `%#x`.

## 2026-11-27 追記(4): c2rust の %#x %#X %#o / Update (4): %#x, %#X, %#o in c2rust
**日本語**: `pl2pl c2rust` が printf の代替形式 `%#x`・`%#X`・`%#o`(幅・左寄せつき)に対応。C と同じく、値が 0 のときは接頭辞を付けず `0` と出す(Rust の `{:#x}` は `0x0` と出して食い違うため、小さな関数 `c_alt` で補った)。`tests/samples/c2rust/hexalt.c`(出力3行)が gcc と rustc で完全一致、他の見本3つも一致、単体テスト70件成功。**未対応**: ポインターのポインター、`%#` と `0` 埋め込みの組み合わせ(`%#06x`)。
**English**: `pl2pl c2rust` now handles printf's alternate form `%#x`, `%#X` and `%#o` (with width and left-justify). As in C, a value of 0 prints plain `0` with no prefix (Rust's `{:#x}` prints `0x0`, which differs, so the small function `c_alt` covers it). `tests/samples/c2rust/hexalt.c` (3 output lines) matches gcc vs rustc exactly, the other three samples still match, and 70 unit tests pass. **Not yet**: pointer-to-pointer, and `%#` combined with zero padding (`%#06x`).

## 2026-11-27 追記: c2rust が1行に書いた本体・波括弧なしの文・do-while に対応 / One-liners, braceless statements and do-while
**日本語**: `pl2pl c2rust` が、行ごとに変換する前に1行の書き方を複数行へ開くようになった: `int sq(int v) { return v * v; }`、`if (x) { a; } else { b; }`、`for (...) 文;`・`if (...) 文;`・`while (...) 文;`・`else 文;`(波括弧なし)、`}` の次の行の `else`。あわせて `do { ... } while (c);` を `loop { ...; if !(c) { break; } }` へ変換する。初期化子 `= { ... }`・struct・enum・typedef は開かない。**実測**: 新しい `tests/samples/c2rust/oneliner.c` を gcc と rustc で比べて出力が一致(既存4本も一致)、`cargo test` 70件成功。**限界**: do-while の中の `continue;` は C では条件の判定へ飛ぶが、`loop` に直すと意味が変わるため、do-while が開いている間の `continue;` は変換せず TODO のまま残す。`if (x) a; else b;` を1行に書いた形は開かない(未対応のまま)。**未了**: `switch`/`case`、`enum`、匿名 `typedef struct`、`static` のグローバル変数、`<<` などのシフト、ポインタのポインタ。
**English**: `pl2pl c2rust` now expands one-line forms into several lines before its line-by-line conversion: `int sq(int v) { return v * v; }`, `if (x) { a; } else { b; }`, braceless `for (...) stmt;` / `if (...) stmt;` / `while (...) stmt;` / `else stmt;`, and an `else` on the line after `}`. It also converts `do { ... } while (c);` to `loop { ...; if !(c) { break; } }`. Initializers `= { ... }`, struct, enum and typedef are left alone. **Measured**: the new `tests/samples/c2rust/oneliner.c` matches gcc vs rustc output (the four existing samples still match) and `cargo test` passes 70 tests. **Limits**: `continue;` inside a do-while jumps to the condition check in C, but inside a Rust `loop` it would change the meaning, so `continue;` while a do-while is open is not converted and stays as TODO. `if (x) a; else b;` written on one line is not expanded (still unsupported). **Open**: `switch`/`case`, `enum`, anonymous `typedef struct`, `static` globals, shifts such as `<<`, and pointer-to-pointer.

## 2026-11-27 追記(2): c2rust がビット演算と16進数に対応 / Bitwise operators and hex literals
**日本語**: `pl2pl c2rust` が `&`・`|`・`^`・`<<`・`>>`・`~` と複合代入 `&=`・`|=`・`^=`・`<<=`・`>>=`、16進数の定数(`0xFF`)に対応。優先順位は C の表(`*` > `+` > シフト > 比較 > `==` > `&` > `^` > `|` > `&&` > `||`)どおりで、Rust 側へは括弧つきで出すので Rust との優先順位の違い(`a & b == c`)の影響を受けない。シフトの右辺は `u32` に直す。**実測**: 新しい `tests/samples/c2rust/bitops.c` が gcc と rustc で出力一致(既存5本も一致)、`cargo test` 70件成功。**限界**: `int`(と `long`、`char`)だけ。符号なし(`unsigned`)の意味(オーバーフローの折り返し、`>>` の論理シフト)は再現しない(i32 として扱う)。`i32` に収まらない16進数(`0xFFFFFFFF` など)と、比較の結果(bool)を `&` に渡す書き方は未対応で TODO のまま。**未了**: `switch`/`case`、`enum`、匿名 `typedef struct`、`static` のグローバル変数、ポインタのポインタ。
**English**: `pl2pl c2rust` now handles `&`, `|`, `^`, `<<`, `>>`, `~`, the compound assignments `&=`, `|=`, `^=`, `<<=`, `>>=`, and hexadecimal constants (`0xFF`). Precedence follows C's table (`*` > `+` > shifts > relational > `==` > `&` > `^` > `|` > `&&` > `||`), and results are emitted in parentheses so Rust's different precedence (`a & b == c`) cannot change the meaning. The right-hand side of a shift is cast to `u32`. **Measured**: the new `tests/samples/c2rust/bitops.c` matches gcc vs rustc (the five existing samples still match) and `cargo test` passes 70 tests. **Limits**: `int` (and `long`, `char`) only. Unsigned semantics (wrap-around, logical `>>`) are not reproduced (values are treated as i32). Hex constants that do not fit in i32 (such as `0xFFFFFFFF`) and passing a comparison result (bool) to `&` are unsupported and stay as TODO. **Open**: `switch`/`case`, `enum`, anonymous `typedef struct`, `static` globals, pointer-to-pointer.

## 2026-11-27 追記(3): c2rust が switch / case / default に対応 / switch, case and default
**日本語**: `pl2pl c2rust` が `switch (式) { case 1: ... break; ... default: ... }` を Rust の `match` へ変換する。`case 1: case 2:` の並びは `1 | 2 =>`、`case 3: 文; break;` の1行書きも開く。文字の `case 'a':` は、`char` の式なら `b'a'`、整数の式なら文字コードにする。`default` が無いときは `_ => {}` を足す。腕の直下の `break;` は不要なので消す(`for` の中の `break;` は残る)。**実測**: 新しい `tests/samples/c2rust/switch.c`(重ね case・1行 case・腕の中の for と break・`char` と `int` の switch・`default` なし)が gcc と rustc で出力一致(既存6本も一致)、`cargo test` 70件成功。**限界**: break なしで次の case へ抜ける(fallthrough)は Rust の `match` で表せないので、該当行に TODO を入れて「未対応」と報告する(黙って意味を変えない。実測で確認)。腕の中の `if` の中の `break;`(switch を抜ける)は未対応。`enum` の定数の case は enum が未対応のため未対応。**未了**: `enum`、匿名 `typedef struct`、`static` のグローバル変数、ポインタのポインタ。
**English**: `pl2pl c2rust` now converts `switch (expr) { case 1: ... break; ... default: ... }` to a Rust `match`. A run of `case 1: case 2:` becomes `1 | 2 =>`, and one-line forms such as `case 3: stmt; break;` are expanded. A character `case 'a':` becomes `b'a'` for a `char` expression, or the character code for an integer expression. A missing `default` gets `_ => {}`. The `break;` directly inside an arm is dropped (a `break;` in a `for` is kept). **Measured**: the new `tests/samples/c2rust/switch.c` (stacked cases, one-line cases, a for and break inside an arm, `char` and `int` switches, no `default`) matches gcc vs rustc output (the six existing samples still match) and `cargo test` passes 70 tests. **Limits**: fallthrough (a case that runs into the next without `break`) cannot be expressed as a Rust `match`, so a TODO is inserted and the file is reported as unsupported (it does not silently change the meaning; checked by running it). A `break;` inside an `if` inside an arm (leaving the switch) is unsupported. Cases using `enum` constants are unsupported because `enum` is not supported yet. **Open**: `enum`, anonymous `typedef struct`, `static` globals, pointer-to-pointer.

## 2026-11-27 追記(4): c2rust が enum と typedef enum に対応 / enum and typedef enum
**日本語**: `pl2pl c2rust` が `enum Color { RED, GREEN = 5, BLUE };` と `typedef enum { LOW = 1, MID, HIGH } Level;` を、整数の定数(`const RED: i32 = 0;` ...)へ開き、`enum Color` と `Level` の型名は `int` として扱う。値を書かなければ前の値+1、書けば整数のその値。`case RED:` のように switch のラベルにも使える。**実測**: 新しい `tests/samples/c2rust/enum.c`(enum の変数・typedef enum を引数にする関数・switch の case に定数・比較)が gcc と rustc で出力一致(既存7本も一致)、`cargo test` 70件成功。**限界**: 値が式のもの(`A = 1 << 2`、`B = A + 1`)は開かず、そのまま未対応として残る。enum を別の enum 型として区別しない(すべて `int` 扱い、型の安全性は失う)。**未了**: 匿名 `typedef struct`、`static` のグローバル変数、ポインタのポインタ。
**English**: `pl2pl c2rust` now expands `enum Color { RED, GREEN = 5, BLUE };` and `typedef enum { LOW = 1, MID, HIGH } Level;` into integer constants (`const RED: i32 = 0;` and so on), and treats the type names `enum Color` and `Level` as `int`. A value without `=` is the previous value + 1; with `=` it is that integer. The constants also work as `case RED:` labels in a switch. **Measured**: the new `tests/samples/c2rust/enum.c` (enum variables, a function taking a typedef enum, constants as case labels, comparisons) matches gcc vs rustc (the seven existing samples still match) and `cargo test` passes 70 tests. **Limits**: values given as expressions (`A = 1 << 2`, `B = A + 1`) are not expanded and stay unsupported. Distinct enum types are not told apart (everything is `int`, so type safety is lost). **Open**: anonymous `typedef struct`, `static` globals, pointer-to-pointer.
