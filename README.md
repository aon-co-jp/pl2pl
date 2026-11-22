# pl2pl

**Languages**: 日本語(このファイル) / [English](docs/i18n/en/README.md) / [简体中文](docs/i18n/zh-CN/README.md) / [繁體中文](docs/i18n/zh-TW/README.md) / [한국어](docs/i18n/ko/README.md) / [Deutsch](docs/i18n/de/README.md) / [Français](docs/i18n/fr/README.md) / [Español](docs/i18n/es/README.md) / [Русский](docs/i18n/ru/README.md) / [Português](docs/i18n/pt/README.md)
(日本語と英語は、やさしい言葉に書き直しました。ほかの言葉も、同じ内容に AI 翻訳しました。ネイティブの確認はまだです)

## pl2pl ってなに?
プログラムは、いろいろな「言葉」(プログラミング言語)で書かれています。
**pl2pl は、ある言葉で書かれたプログラムを、別の言葉に書きかえる道具です。**
たとえば、PHP(Laravel)で作ったサイトを、Python(FastAPI)のサイトに変えられます。
Windows・Linux・Mac で使えます(v0.1.0、まだ作っている途中です)。

- 使うときは、パソコンに入れます。WEBサイトには、説明とダウンロードだけがあります → <https://easy-web.tokyo/pl2pl/>
- 書きかえられない所は、こっそり消しません。`TODO`(あとでやること)として残して、理由を報告書に書きます。
- ゴールは、有名な言葉どうしを、できるだけたくさんの組み合わせで書きかえられるようにすることです。

## いまできること
| コマンド | 何をするか |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。くわしくは [docs/PHP.md](docs/PHP.md) |
| `pl2pl web` | フォルダの中を見て、Java・COBOL・PHP・Flask/Django を見分けて、合う道具に任せる |
| `pl2pl ir` | (試作)PHP・Python・Java の「道すじ(ルート)」を同じ形にして、JSON・OpenAPI・FastAPI のひな形にする。PHP の変換も、型・関数の本体・モデル・ミドルウェア・画面をこの形にのせてから書き出す |
| `pl2pl watch` | 言葉や道具の新しい版を見つけて、自動で追いつく |
| `pl2pl cobol` | COBOL+JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | C/C++ のくり返しを、はやく動く形にする(gpu は試作) |
| `pl2pl cport` | UNIX 用の C を、Windows でも Linux でも動く形にする |
| `pl2pl pyhpc` | Python の計算を Numba ではやくする |
| `pl2pl llm setup` | AI の助っ人(aruaru-llm)を入れる(実際に入れる所は、まだ確かめていません) |

## 新しい版に、自動で追いつく
`pl2pl watch` を、毎晩と毎朝に動かします。
1. 言葉や道具の新しい版が出ていないか、調べます(一覧は `registry/ecosystem.toml`)。
2. 新しい版でテストします。通れば、「対応している版」を自動で新しくします。
3. 通らなければ、version-control(AI の助けつき)が直そうとします。
4. それでも直らなければ、登録してあるメールと LINE に知らせます。

注意: laravel・symfony・flask・django などは、いまは「新しい版が出たよ」と知らせるだけです。
GitHub では、毎晩と毎朝に、新しい版でテストが自動で動いています。

## 使い方
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 気をつけること
- pl2pl は慎重です。あぶなそうな所は、変えずに理由を知らせます。はやさは約束できません。
- テストは、小さなサンプルだけです。大きな本物のプログラムでは、まだ試していません。

## これから
- 型や中身(関数の本体)も共通の形にのせて、FastAPI への変換をそこを通すようにする。
- PHP: クラスで作る Blade の部品。
- 本物のメール通知と、AI による自動修正の確認。
- GPU で動かす確認(open-cuda / open-directx)。

## もっと知る
[docs/LANGUAGES.md](docs/LANGUAGES.md)(言葉の一覧) / [docs/COMBINATIONS.md](docs/COMBINATIONS.md)(組み合わせ) / [docs/DESIGN.md](docs/DESIGN.md)(設計。持ち主が書いています) / [components/README.md](components/README.md)(いっしょに入る部品)

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


## 2026-11-22 現状と完成予定と課題 / Status, plan and issues

### 日本語
**現状**: COBOL・C・C++・Fortran・Java を、選んだ組み合わせ(Axum/Poem/FastAPI/RPoem/RPoRea × HTML5/CSS3/TypeScript/Bootstrap × MySQL/MariaDB/PostgreSQL/SQLite)のウェブアプリにするお助けロボット。`converters/cobol_to` と `converters/java_to` は COBOL_to・Java_to と同じ内容を取り込み(COBOL: SORT/MERGE/SEARCH/EXEC SQL 一式、Java: メソッド参照・Comparator・Collectors)、試験55件成功。`pl2pl c2rust`・`f2rust` で単純な C/C++/Fortran を Rust へ。
**完成予定**: 元の COBOL_to / Java_to の削除(本人確認のうえ最後に)、Mac・iPhone 実機確認、COBOL→他 OS の COBOL の拡張、Kokkos の GPU 実行、メール到達の確認。
**課題**: 取り込み版と元リポジトリを手で同期している(自動同期なし)。C++/Fortran の Rust 化は単純な範囲のみ。Rust 側の EXEC SQL は疑似ハンドラーでの検証にとどまる。

### English
**Status**: A helper robot that turns COBOL, C, C++, Fortran and Java into web apps in the mix you choose (Axum/Poem/FastAPI/RPoem/RPoRea x HTML5/CSS3/TypeScript/Bootstrap x MySQL/MariaDB/PostgreSQL/SQLite). `converters/cobol_to` and `converters/java_to` mirror COBOL_to and Java_to (COBOL: SORT/MERGE/SEARCH/EXEC SQL; Java: method references, Comparator, Collectors); 55 tests pass. `pl2pl c2rust` and `f2rust` rewrite simple C/C++/Fortran into Rust.
**Planned**: deleting the original COBOL_to / Java_to (last, after the owner confirms), real Mac/iPhone checks, extending COBOL to other OSes, Kokkos on a GPU, a mail-arrival check.
**Issues**: The vendored copies are synced by hand (no automatic sync). C++/Fortran to Rust covers only simple code. Rust-side EXEC SQL is verified only with a fake handler.
