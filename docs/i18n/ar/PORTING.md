> ملاحظة: ملف PORTING.md الياباني في جذر المستودع هو الأصل. هذه النسخة ترجمها الذكاء الاصطناعي وراجعها الذكاء الاصطناعي لضمان سلاسة اللغة؛ ولم يراجعها بعدُ متحدث أصلي من البشر.

# مذكرة التسليم لـ pl2pl
(الملاحظات الأقدم والأكثر تفصيلًا في `docs/HISTORY-ja.md`، باليابانية.)

## أين وصلنا (2026-11-19)
- صدر الإصدار 0.1.0 مع مثبّتات لـ Windows وLinux وMac، واختبارات في CI.
- تحويل PHP → FastAPI مُختبَر من البداية إلى النهاية مع FastAPI حقيقي: المسارات (routes) وEloquent ومنشئ الاستعلامات (query builder) وBlade والوسائط البرمجية (middleware) والعلاقات وعائلة `@can`.
- صار `pl2pl web` يتعرّف على PHP أيضًا.
- يجد `pl2pl watch` الإصدارات الجديدة ويختبرها ويصلح المشكلات، ويرسل تنبيهًا إن لم ينجح شيء. كما يجري GitHub اختبارات بأحدث الإصدارات كل ليلة وكل صباح.
- يضع `pl2pl ir` (نسخة تجريبية) مسارات PHP وPython وJava (Spring وJAX-RS وServlet وSpark) في شكل مشترك واحد، ويكتب JSON أو OpenAPI أو هيكلًا أوليًّا لـ FastAPI. ويحمل IR أيضًا المعاملات ذات الأنواع وأجسام الدوال والنماذج والوسائط البرمجية والقوالب، وصار `pl2pl php` يكتب مخرجاته عبر أداة الكتابة المشتركة في `src/ir_emit.rs` (المخرجات لم تتغير عن قبل؛ وتنجح 49 فحصًا شاملًا).
- صفحة الشرح والتنزيل تعمل: <https://easy-web.tokyo/pl2pl/> (على VPS، ثابتة). وهناك أيضًا بطاقة في الصفحة الرئيسية لـ easy-web.tokyo.
- دُمج COBOL_to وJava_to في `converters/`.

## ما بقي (الأهم أولًا)
1. **التحقق**: وصول بريد إلكتروني حقيقي (`version-control --test-notify`؛ لا إعدادات SMTP على هذا الحاسوب)، والإصلاح التلقائي عبر `--loop` في version-control، واختبار على أحدث fastapi (سيتحقق منه التشغيل الليلي على GitHub).
2. وضع الأنواع وأجسام الدوال في IR. وتمرير مخرجات FastAPI الحالية (`pl2pl php`) عبر IR. وإذا فاتت عملية إيجاد المسارات أشياء، فيُنظر في tree-sitter.
3. جُرّب `watch` مع إصدار جديد حقيقي: مع fastapi 0.165.0 (starlette 1.10) نجح `tests/run-php.sh` في 49 فحصًا، فانتقل الإصدار المُختبَر في السجل (registry) إلى 0.165. ولا يزال مفتوحًا: بقيت laravel 13 وdjango 6.1 وspring-boot 4.1 الجديدة «غير مفحوصة» لأن الاختبارات الشاملة لا تستخدم الأطر الحقيقية (تقرير فقط إلى أن توجد اختبارات كهذه).
4. إعداد التشغيل الليلي والصباحي على حاسوب (لم يُنجز: أمر التحقق لا يعمل على هذا الحاسوب وكان سيعطي إنذارات كاذبة؛ انظر أعلى `scripts/pl2pl-watch.cmd` للاطلاع على مثال).
5. (تم) توجد بطاقة pl2pl في الصفحة الرئيسية لـ easy-web.tokyo وهي تعمل في الإنتاج (open-easy-web `95cb1e7`؛ والملف التنفيذي القديم هو `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (تم، بحدود): صار `@php` في Blade يعالج الإسناد و`++` و`--` و`+=` و`-=` و`*=` و`/=` و`.=` على هيئة `{% set %}`؛ ويعمل `$attributes` كمخرَج، وكذلك `->merge` (قيم نصية أو تعابير قابلة للتحويل) و`->class([...])` (قائمة نصوص) و`->only` و`->except`؛ و`morphToMany` (عمود النوع في الجدول الوسيط)؛ ومكوّنات Blade القائمة على الأصناف (`app/View/Components/*.php`: تصبح معاملات الباني قيمًا افتراضية و`_props`، ويُتتبَّع `view('x.y')` داخل `render()`) (55 فحصًا شاملًا و41 اختبار وحدة). ولا يزال مفتوحًا: الدوال العامة (public methods) في صنف المكوّن (يبقى `{{ $method() }}` في قالبه `TODO`، ويحصي التقرير عددها)، و`->class()` ذات المفاتيح (الشرطية)، و`@php` التي فيها if/foreach أو استدعاءات دوال (تبقى `TODO`).
7. الربط مع aruaru-llm (تم): في 2026-11-19 سحبنا aruaru-llm v0.2.5 وشغّلنا `pl2pl llm setup` فعليًّا. فشل الاختيار التلقائي، 3B، لأن أوزانه مقسّمة إلى أجزاء (shards)، فصار setup يتراجع إلى النموذج الأصغر التالي (لا عند تثبيت نموذج بعينه). ويُرجع `GenerateQwen` نصًّا مع qwen2.5-1.5b-instruct (نحو 2.9 GB، على CPU). و**`pl2pl php --explain`** (اختياري؛ و`--explain-max N`، الافتراضي 3) يضيف ملاحظة من الذكاء الاصطناعي تحت كل بند TODO في التقرير. القياس: نحو دقيقتين لكل بند على CPU، و**الجودة ضعيفة وأحيانًا خاطئة** (أخطأ المرمى 1 من أصل 2)، لذا هو معطّل افتراضيًّا. وaruaru-llm بعد إعادة تشغيله لا يكون فيه نموذج محدد (400)، فيختار أولًا نموذجًا مثبّتًا. **نموذج أكبر (نحو 12 GB)**: أكبر نموذج في كتالوج aruaru-llm هو 3B (نحو 6.2 GB)، ويأتي 3B و7B في عدة ملفات أجزاء لا يستطيع المثبّت الحالي جلبها. فدعم الأجزاء (في aruaru-llm) وسرعة CPU (وهي بطيئة هكذا حتى مع 1.5B) يأتيان أولًا. ولا يزال Kokkos على GPU والنوى (kernels) على open-cuda / open-directx غير مفحوصة. **GPU (فُحص في 2026-11-19)**: على GeForce GT 730 (2 GB، sm_35، السائق 475 = فئة CUDA 11.4) ثبّتنا nvcc من CUDA 11.8 دون صلاحيات المسؤول عبر micromamba (قناة conda الخاصة بـ NVIDIA)، وشغّلنا نواة CUDA بسيطة ومخرجات `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) على الـ GPU الحقيقي، ورأينا 0 فروقات مقارنةً بـ CPU (`tests/gpu-check.ps1`). المطبّات: يحتاج MSVC 14.44 إلى `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` و`-allow-unsupported-compiler`؛ وليس في PyPI nvcc لـ Windows؛ ولا يستطيع CUDA 12 استهداف sm_35. **لا يزال غير مفحوص**: الواجهة الخلفية CUDA لـ Kokkos، والتشغيل على open-cuda (التنفيذ بـ Rust) وopen-directx.
8. حذف COBOL_to وJava_to الأصليين (**لا رجعة في هذا، فاطلب موافقة نهائية أولًا**).
9. يكتب المالك `docs/DESIGN.md`. أعدنا قراءة `docs/LANGUAGES.md` في 2026-11-19 فلم نجد ما يخالف الوضع الحالي (لـ Java وCOBOL محوِّلات). وهي ما زالت قائمة مبنية على المعرفة، لم تُقارَن ببحث جديد على الويب.

## الاختبارات
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP من البداية إلى النهاية) / `cargo test --release`

## رسالة للبدء من جديد
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never hide what cannot be rewritten; leave a TODO. Check by really running before saying it is done. Reply in Japanese only. Commit and push at each step, and update the VPS too.”

## الجديد في v0.2.1 (2026-11-20)
- عند اختيار `rpoem` / `rporea` في `pl2pl web --backend` تظهر تنبيهات (باليابانية والإنجليزية): RPoem يتضمّن أصلًا ميزات متوافقة مع Tauri ومع WunderGraph Cosmo، إضافةً إلى tokio وWASM، فلا تختَرْها؛ ويُستخدم HTML5 وCSS3 لكن بأقل قدر ممكن؛ ولا تُستخدم JavaScript ولا TypeScript ولا Node.js ولا REST بحكم التصميم. وما يُضاف بـ `--also` وهو مضمَّن أصلًا يُرفض (رمز الخروج 2)؛ وما هو مضمَّن لكنه لم ينضج بعدُ يبقى قابلًا للاختيار مع تحذير.
- يحكم `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` على هذا النضج بتشغيل اختبارات فحص (السجل: `registry/maturity.toml`). ورأي الذكاء الاصطناعي من aruaru-llm لا يغيّر الحكم وحده أبدًا.
- مع التركيبات غير RPoem / RPoRea يُقبل `--also nodejs` (أو `next.js`) مع تنبيه: نحن لا نفرض سياستنا في عدم استخدام Node.js، لكننا لا ننصح به. وتُعرض بدائل للّغة المختارة (RPoem / RPoRea + Reaxt، وFastAPI + Jinja2، وRS-Node.js، وأخرى لم نتحقق منها).
- المستودعات الشقيقة تُطوَّر معًا: Reaxt وRPoRea وRPoem وRMojo وopen-cosmo وRS-TypeScript وRS-JavaScript وRS-Node.js وpl2pl. وعند تغيّر أحدها تُفحص البقية في اليوم نفسه.
- يشرح `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` و`webpage/info.html` مبادئ تصميم RPoRea / RPoem مع المزايا والعيوب بـ 11 لغة.
- بقي `COBOL_to` / `Java_to` الأصليان (لم يُحذفا)؛ وأعطت النسخ المدمجة في `converters/` مخرجات مطابقة.
