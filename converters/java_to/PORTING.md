# Java_to 引き継ぎ / Hand-off
## 到達点 (2026-11-18)
初版: 判定58件、ルート抽出(Spring/JAX-RS/Micronaut/Servlet/Spark)、Python 本体翻訳、Rust 骨組み、4OSインストーラーのCI定義。FastAPI e2e 7件・poem/rpoem cargo check 成功。
## 未了 / Open
1. 実プロジェクト(Spring PetClinic 等)での変換・実行確認。
2. CI(release.yml)は未実行。タグ `v0.1.0` を打って各OSの試験を確認する。
3. Rust 側の本体翻訳、Struts/JSF 等の画面系、Kotlin/Scala。
4. pl2pl(統合構想)での共通IR化。
## 再開用メッセージ
「F:\Java_to を再開。PORTING.md の未了1から。まず Spring PetClinic を clone して `java_to convert` → FastAPI 実行確認。」


## 2026-11-21 更新: ブロック本体のラムダと Stream API に対応 / Update: block-bodied lambdas and the Stream API

**日本語**: 複数の文を持つブロック本体のラムダ(`x -> { int d = x * 2; ...; return d; }`)を、その文の直前に `def _lam1(x):` として出して使う形で変換できます。Stream API は `.stream()`・`.map`・`.filter`・`.mapToInt`・`.forEach`・`.anyMatch/allMatch/noneMatch`・`.distinct`・`.limit/.skip`・`.sorted()`・`.sum`・`.count`・`.findFirst`・`.reduce(初期値, 関数)`・`.collect(Collectors.toList()/toSet()/joining(区切り))` を Python に変換します(途中結果は list で即時評価)。変換した Python を実際に動かし、`[1..6]` に `filter(x > 1)` とブロックラムダ(`if` で上限 10)をかけて `[4, 6, 8, 10, 10]`、`mapToInt(...).sum()` が 21 になることを確認しました。単体試験は 18 件成功です。**まだ**: 比較関数つきの `.sorted(...)`、`Collectors.groupingBy` など他の collector、匿名クラス、メソッド参照、Rust 側の本体翻訳(現在は単純な `return` のみ)、Spring PetClinic など実プロジェクトでの通し。

**English**: Block-bodied lambdas with several statements (`x -> { int d = x * 2; ...; return d; }`) are now converted: the body is emitted as `def _lam1(x):` just before the enclosing statement and used by name. The Stream API (`.stream()`, `.map`, `.filter`, `.mapToInt`, `.forEach`, `.anyMatch/allMatch/noneMatch`, `.distinct`, `.limit/.skip`, `.sorted()`, `.sum`, `.count`, `.findFirst`, `.reduce(identity, fn)`, `.collect(Collectors.toList()/toSet()/joining(sep))`) is translated to Python, with intermediate results evaluated eagerly as lists. We ran the generated Python: `filter(x > 1)` and a block lambda (cap 10 via `if`) over `[1..6]` gives `[4, 6, 8, 10, 10]`, and `mapToInt(...).sum()` gives 21. 18 unit tests pass. **Not yet**: `.sorted(...)` with a comparator, other collectors such as `Collectors.groupingBy`, anonymous classes, method references, body translation on the Rust side (simple `return` only), and real projects such as Spring PetClinic.
## 世界の言語での 2026-11-21 更新(ブロック本体のラムダ・Stream API) / 2026-11-21 update in 10 more languages (AI翻訳+AI確認、人間のネイティブ確認はまだ)

### 简体中文 — 更新 (2026-11-21)

*以下为 AI 翻译,并已由 AI 按母语水平审校;尚未经人类母语者校对。*

现已支持带代码块的 lambda 和常用的 Stream API。多条语句的 lambda(如 `x -> { int d = x * 2; ...; return d; }`)会在所在语句之前生成 `def _lam1(x):`,并在原处使用该名称;`.stream()`、`.map(...)`、`.filter(...)`、`.mapToInt(...)`、`.forEach(...)`、`.anyMatch(...)`、`.allMatch(...)`、`.noneMatch(...)`、`.distinct()`、`.limit(n)`、`.skip(n)`、`.sorted()`、`.sum()`、`.count()`、`.findFirst()`、`.reduce(初始值, 函数)`,以及 `.collect(Collectors.toList())`、`Collectors.toSet()`、`Collectors.joining(分隔符)` 会转换为 Python(中间结果以 list 即时求值)。我们实际运行了生成的 Python:对 `[1..6]` 执行 `filter(x > 1)`,再对每个元素执行含 `if` 的块 lambda(上限 10),得到 `[4, 6, 8, 10, 10]`,`mapToInt(...).sum()` 得到 21;单元测试 18 项通过。**尚未支持**:带比较器的 `.sorted(...)`、`Collectors.groupingBy` 等其他收集器、匿名类、方法引用、Rust 一侧的方法体翻译(目前仅简单 `return`),以及在 Spring PetClinic 等真实项目上的完整验证。上面各语言章节中的“计划”清单已经过时,将由每日翻译更新来修正。

### 繁體中文(台灣) — 更新 (2026-11-21)

*以下為 AI 翻譯,並已由 AI 以母語水準審校;尚未經人類母語者校對。*

現已支援含程式區塊的 lambda 與常用的 Stream API。含多條陳述式的 lambda(如 `x -> { int d = x * 2; ...; return d; }`)會在所在陳述式之前產生 `def _lam1(x):`,並在原處使用該名稱;`.stream()`、`.map(...)`、`.filter(...)`、`.mapToInt(...)`、`.forEach(...)`、`.anyMatch(...)`、`.allMatch(...)`、`.noneMatch(...)`、`.distinct()`、`.limit(n)`、`.skip(n)`、`.sorted()`、`.sum()`、`.count()`、`.findFirst()`、`.reduce(初始值, 函式)`,以及 `.collect(Collectors.toList())`、`Collectors.toSet()`、`Collectors.joining(分隔符號)` 會轉換為 Python(中間結果以 list 立即求值)。我們實際執行了產生的 Python:對 `[1..6]` 執行 `filter(x > 1)`,再對每個元素執行含 `if` 的區塊 lambda(上限 10),得到 `[4, 6, 8, 10, 10]`,`mapToInt(...).sum()` 得到 21;單元測試 18 項通過。**尚未支援**:帶比較器的 `.sorted(...)`、`Collectors.groupingBy` 等其他收集器、匿名類別、方法參照、Rust 端的方法本體翻譯(目前僅簡單 `return`),以及在 Spring PetClinic 等真實專案上的完整驗證。上方各語言章節中的「計畫」清單已經過時,將由每日翻譯更新來修正。

### 한국어 — 업데이트 (2026-11-21)

*아래는 AI가 번역하고 AI가 원어민 수준으로 검수한 것이며, 사람 원어민의 검수는 아직 받지 않았습니다.*

이제 블록 본문을 가진 람다와 자주 쓰는 Stream API를 변환합니다. 문장이 여러 개인 람다(예: `x -> { int d = x * 2; ...; return d; }`)는 해당 문장 바로 앞에 `def _lam1(x):`로 내보내고 원래 자리에서는 그 이름을 씁니다. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(초깃값, 함수)`, 그리고 `.collect(Collectors.toList())`, `Collectors.toSet()`, `Collectors.joining(구분자)`는 Python으로 바뀌며, 중간 결과는 list로 즉시 계산합니다. 생성된 Python을 실제로 실행해 확인했습니다. `[1..6]`에 `filter(x > 1)`을 적용하고 `if`가 있는 블록 람다(상한 10)로 바꾸면 `[4, 6, 8, 10, 10]`이 되고, `mapToInt(...).sum()`은 21이었습니다. 단위 테스트 18건이 통과했습니다. **아직 지원하지 않는 것**: 비교자를 받는 `.sorted(...)`, `Collectors.groupingBy` 같은 다른 수집기, 익명 클래스, 메서드 참조, Rust 쪽 메서드 본문 변환(현재는 단순한 `return`만), 그리고 Spring PetClinic 같은 실제 프로젝트 전체 검증. 위의 다른 언어 절에 있는 "계획" 목록은 지금은 오래된 내용이며, 매일의 번역 갱신에서 고칩니다.

### Deutsch — Aktualisierung (2026-11-21)

*Die folgenden Texte wurden von einer KI übersetzt und von einer KI auf muttersprachliche Natürlichkeit geprüft; eine Prüfung durch menschliche Muttersprachler steht noch aus.*

Jetzt werden Lambdas mit Blockrumpf und die gängigen Stream-API-Aufrufe umgewandelt. Ein Lambda mit mehreren Anweisungen (zum Beispiel `x -> { int d = x * 2; ...; return d; }`) wird als `def _lam1(x):` unmittelbar vor der umgebenden Anweisung ausgegeben und an der ursprünglichen Stelle mit diesem Namen verwendet. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(Startwert, Funktion)` sowie `.collect(Collectors.toList())`, `Collectors.toSet()` und `Collectors.joining(Trenner)` werden nach Python übersetzt; Zwischenergebnisse werden sofort als list ausgewertet. Wir haben das erzeugte Python tatsächlich ausgeführt: `filter(x > 1)` auf `[1..6]`, danach ein Block-Lambda mit `if` (Obergrenze 10), ergibt `[4, 6, 8, 10, 10]`, und `mapToInt(...).sum()` ergibt 21. 18 Unit-Tests bestehen. **Noch nicht unterstützt**: `.sorted(...)` mit Komparator, weitere Collector wie `Collectors.groupingBy`, anonyme Klassen, Methodenreferenzen, die Übersetzung der Methodenrümpfe auf der Rust-Seite (derzeit nur einfaches `return`) und ein vollständiger Durchlauf mit echten Projekten wie Spring PetClinic. Die Listen „Geplant“ in den übrigen Sprachabschnitten oben sind damit veraltet und werden durch die tägliche Übersetzungsaktualisierung korrigiert.

### Français — Mise à jour (2026-11-21)

*Les textes suivants ont été traduits par une IA et relus par une IA pour leur naturel ; ils n'ont pas encore été relus par des locuteurs natifs humains.*

Les lambdas à corps de bloc et les appels courants de l'API Stream sont désormais convertis. Un lambda à plusieurs instructions (par exemple `x -> { int d = x * 2; ...; return d; }`) est émis sous la forme `def _lam1(x):` juste avant l'instruction qui le contient, puis utilisé sous ce nom à l'endroit d'origine. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(valeur initiale, fonction)`, ainsi que `.collect(Collectors.toList())`, `Collectors.toSet()` et `Collectors.joining(séparateur)` sont traduits en Python ; les résultats intermédiaires sont évalués tout de suite sous forme de list. Nous avons réellement exécuté le Python généré : `filter(x > 1)` sur `[1..6]`, puis un lambda à bloc contenant un `if` (plafond 10), donne `[4, 6, 8, 10, 10]`, et `mapToInt(...).sum()` donne 21. 18 tests unitaires réussis. **Pas encore pris en charge** : `.sorted(...)` avec comparateur, d'autres collecteurs comme `Collectors.groupingBy`, les classes anonymes, les références de méthode, la traduction des corps de méthodes côté Rust (seulement un simple `return` pour l'instant) et un essai complet sur de vrais projets comme Spring PetClinic. Les listes « Prévu » des autres sections de langue ci-dessus sont donc périmées ; la mise à jour quotidienne des traductions les corrigera.

### Español — Actualización (2026-11-21)

*Los textos siguientes fueron traducidos por una IA y revisados por una IA en cuanto a naturalidad; aún no los ha revisado un hablante nativo humano.*

Ahora se convierten las lambdas con cuerpo de bloque y las llamadas habituales de la API Stream. Una lambda con varias sentencias (por ejemplo `x -> { int d = x * 2; ...; return d; }`) se emite como `def _lam1(x):` justo antes de la sentencia que la contiene y se usa con ese nombre en el lugar original. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(valor inicial, función)`, además de `.collect(Collectors.toList())`, `Collectors.toSet()` y `Collectors.joining(separador)`, se traducen a Python; los resultados intermedios se evalúan al instante como list. Ejecutamos realmente el Python generado: `filter(x > 1)` sobre `[1..6]` y luego una lambda de bloque con un `if` (tope 10) da `[4, 6, 8, 10, 10]`, y `mapToInt(...).sum()` da 21. Pasan 18 pruebas unitarias. **Aún no admitido**: `.sorted(...)` con comparador, otros colectores como `Collectors.groupingBy`, las clases anónimas, las referencias a métodos, la traducción de los cuerpos de método del lado de Rust (por ahora solo un `return` sencillo) y una prueba completa con proyectos reales como Spring PetClinic. Las listas «Previsto» de las demás secciones de idioma de arriba han quedado desactualizadas; la actualización diaria de traducciones las corregirá.

### Português (BR) — Atualização (2026-11-21)

*Os textos a seguir foram traduzidos por uma IA e revisados por uma IA quanto à naturalidade; ainda não foram revisados por falantes nativos humanos.*

Agora são convertidas as lambdas com corpo em bloco e as chamadas comuns da API Stream. Uma lambda com várias instruções (por exemplo `x -> { int d = x * 2; ...; return d; }`) é emitida como `def _lam1(x):` logo antes da instrução que a contém e usada com esse nome no lugar original. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(valor inicial, função)`, além de `.collect(Collectors.toList())`, `Collectors.toSet()` e `Collectors.joining(separador)`, são traduzidos para Python; os resultados intermediários são avaliados na hora como list. Executamos de fato o Python gerado: `filter(x > 1)` sobre `[1..6]` e depois uma lambda de bloco com um `if` (teto 10) dá `[4, 6, 8, 10, 10]`, e `mapToInt(...).sum()` dá 21. 18 testes unitários passam. **Ainda não suportado**: `.sorted(...)` com comparador, outros coletores como `Collectors.groupingBy`, classes anônimas, referências a métodos, a tradução dos corpos de método do lado do Rust (por enquanto apenas um `return` simples) e um teste completo com projetos reais como o Spring PetClinic. As listas «Planejado» das demais seções de idioma acima estão desatualizadas; a atualização diária das traduções vai corrigi-las.

### Русский — Обновление (2026-11-21)

*Следующие тексты переведены ИИ и проверены ИИ на естественность звучания; носители языка — люди — их пока не проверяли.*

Теперь преобразуются лямбды с блочным телом и самые распространённые вызовы Stream API. Лямбда из нескольких операторов (например, `x -> { int d = x * 2; ...; return d; }`) выводится как `def _lam1(x):` непосредственно перед содержащим её оператором, а на прежнем месте используется это имя. `.stream()`, `.map(...)`, `.filter(...)`, `.mapToInt(...)`, `.forEach(...)`, `.anyMatch(...)`, `.allMatch(...)`, `.noneMatch(...)`, `.distinct()`, `.limit(n)`, `.skip(n)`, `.sorted()`, `.sum()`, `.count()`, `.findFirst()`, `.reduce(начальное значение, функция)`, а также `.collect(Collectors.toList())`, `Collectors.toSet()` и `Collectors.joining(разделитель)` переводятся в Python; промежуточные результаты сразу вычисляются как list. Мы действительно запустили сгенерированный Python: `filter(x > 1)` для `[1..6]`, затем блочная лямбда с `if` (предел 10) даёт `[4, 6, 8, 10, 10]`, а `mapToInt(...).sum()` даёт 21. Проходят 18 модульных тестов. **Пока не поддерживается**: `.sorted(...)` с компаратором, другие коллекторы вроде `Collectors.groupingBy`, анонимные классы, ссылки на методы, перевод тел методов на стороне Rust (пока только простой `return`) и полная проверка на реальных проектах, например Spring PetClinic. Списки «Планируется» в остальных языковых разделах выше устарели; их исправит ежедневное обновление переводов.

### العربية — تحديث (2026-11-21)

<div dir="rtl" lang="ar">

*النص التالي ترجمه الذكاء الاصطناعي وراجعه الذكاء الاصطناعي من حيث سلامة اللغة، ولم يراجعه بعدُ متحدث أصلي من البشر.*

أصبحت تعابير lambda ذات الجسم الكتلي واستدعاءات واجهة Stream الشائعة مدعومة الآن. فتعبير lambda الذي يحوي عدة عبارات (مثل `x -> { int d = x * 2; ...; return d; }`) يُولَّد على هيئة `def _lam1(x):` قبل العبارة التي تحويه مباشرةً، ويُستخدَم هذا الاسم في موضعه الأصلي. وتُترجَم إلى Python كلٌّ من `.stream()` و`.map(...)` و`.filter(...)` و`.mapToInt(...)` و`.forEach(...)` و`.anyMatch(...)` و`.allMatch(...)` و`.noneMatch(...)` و`.distinct()` و`.limit(n)` و`.skip(n)` و`.sorted()` و`.sum()` و`.count()` و`.findFirst()` و`.reduce(القيمة الابتدائية, الدالة)`، إضافةً إلى `.collect(Collectors.toList())` و`Collectors.toSet()` و`Collectors.joining(الفاصل)`؛ وتُقيَّم النتائج الوسيطة فورًا في صورة list. شغّلنا فعليًا شيفرة Python المولَّدة: تطبيق `filter(x > 1)` على `[1..6]` ثم lambda كتلي فيه `if` (الحد الأقصى 10) يعطي `[4, 6, 8, 10, 10]`، و`mapToInt(...).sum()` يعطي 21. نجح 18 اختبارًا من اختبارات الوحدة. **غير مدعوم بعدُ**: `.sorted(...)` مع مقارِن، والمجمِّعات الأخرى مثل `Collectors.groupingBy`، والأصناف المجهولة، والإشارات إلى الدوال، وترجمة أجسام الدوال في جانب Rust (حاليًا `return` البسيط فقط)، والتجربة الكاملة على مشاريع حقيقية مثل Spring PetClinic. وقوائم «المخطَّط له» في أقسام اللغات الأخرى أعلاه صارت قديمة، وسيصحّحها التحديث اليومي للترجمات.

</div>

### فارسی (ایران) — به‌روزرسانی (2026-11-21)

<div dir="rtl" lang="fa">

*متن زیر را هوش مصنوعی ترجمه و از نظر روانی زبان بازبینی کرده است؛ هنوز هیچ فارسی‌زبانِ بومیِ انسانی آن را بررسی نکرده است.*

اکنون لامبداهای دارای بدنهٔ بلوکی و فراخوانی‌های رایج Stream API تبدیل می‌شوند. لامبدایی که چند دستور دارد (مثلاً `x -> { int d = x * 2; ...; return d; }`) به‌صورت `def _lam1(x):` درست پیش از دستورِ دربرگیرنده تولید می‌شود و در جای اصلی با همین نام به کار می‌رود. `.stream()`، `.map(...)`، `.filter(...)`، `.mapToInt(...)`، `.forEach(...)`، `.anyMatch(...)`، `.allMatch(...)`، `.noneMatch(...)`، `.distinct()`، `.limit(n)`، `.skip(n)`، `.sorted()`، `.sum()`، `.count()`، `.findFirst()`، `.reduce(مقدار آغازین, تابع)` و نیز `.collect(Collectors.toList())`، `Collectors.toSet()` و `Collectors.joining(جداکننده)` به Python ترجمه می‌شوند؛ نتیجه‌های میانی بلافاصله به‌صورت list محاسبه می‌شوند. کد Python تولیدشده را واقعاً اجرا کردیم: اعمال `filter(x > 1)` روی `[1..6]` و سپس لامبدای بلوکی دارای `if` (سقف 10) نتیجهٔ `[4, 6, 8, 10, 10]` را می‌دهد و `mapToInt(...).sum()` برابر 21 است. ۱۸ آزمون واحد موفق است. **هنوز پشتیبانی نمی‌شود**: `.sorted(...)` همراه با مقایسه‌گر، گردآورنده‌های دیگر مانند `Collectors.groupingBy`، کلاس‌های ناشناس، ارجاع به متد، ترجمهٔ بدنهٔ متدها در سمت Rust (فعلاً فقط `return` ساده) و آزمایش کامل روی پروژه‌های واقعی مانند Spring PetClinic. فهرست‌های «برنامه‌ریزی‌شده» در بخش‌های زبانی دیگرِ بالا اکنون قدیمی است و به‌روزرسانی روزانهٔ ترجمه‌ها آن را اصلاح خواهد کرد.

</div>
