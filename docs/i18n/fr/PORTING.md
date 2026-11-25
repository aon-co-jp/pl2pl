> Remarque : le PORTING.md japonais à la racine du dépôt est l'original. Cette version a été traduite par une IA, puis relue par une IA pour sa fluidité en langue maternelle ; elle n'a pas encore été relue par une personne de langue maternelle.

# pl2pl – passation
(Les anciennes notes détaillées sont dans `docs/HISTORY-ja.md`, en japonais.)

## Où on en est (2026-11-19)
- La version 0.1.0 est publiée, avec des installateurs pour Windows, Linux et Mac et des tests en CI.
- PHP → FastAPI est testé de bout en bout avec un vrai FastAPI : routes, Eloquent, constructeur de requêtes, Blade, middleware, relations et la famille `@can`.
- `pl2pl web` repère aussi PHP maintenant.
- `pl2pl watch` trouve les nouvelles versions, les teste, corrige les problèmes et envoie une alerte si rien ne marche. GitHub teste aussi avec les versions les plus récentes chaque soir et chaque matin.
- `pl2pl ir` (version d'essai) met les routes de PHP, Python et Java (Spring, JAX-RS, Servlet, Spark) sous une forme commune et produit du JSON, de l'OpenAPI ou un squelette FastAPI. L'IR porte aussi les paramètres typés, le contenu des fonctions, les modèles, les middlewares et les gabarits, et `pl2pl php` écrit désormais sa sortie via l'émetteur commun de `src/ir_emit.rs` (sortie inchangée par rapport à avant ; 49 contrôles de bout en bout réussis).
- La page de présentation et de téléchargement est en ligne : <https://easy-web.tokyo/pl2pl/> (sur le VPS, statique). Il y a aussi une carte sur la page d'accueil de easy-web.tokyo.
- COBOL_to et Java_to sont intégrés dans `converters/`.

## Reste à faire (le plus important d'abord)
1. **Vérifier** : l'arrivée d'un vrai e-mail (`version-control --test-notify` ; ce PC n'a pas de réglages SMTP), la réparation automatique via version-control `--loop`, et un test avec le dernier fastapi (l'exécution de chaque nuit sur GitHub le vérifiera).
2. Mettre les types et le contenu des fonctions dans l'IR. Faire passer la sortie FastAPI actuelle (`pl2pl php`) par l'IR. Si la recherche des routes laisse des choses de côté, envisager tree-sitter.
3. `watch` a été essayé avec une vraie nouvelle version : avec fastapi 0.165.0 (starlette 1.10), `tests/run-php.sh` a réussi 49 contrôles, la version testée du registre passe donc à 0.165. Reste à voir : les nouveaux laravel 13 / django 6.1 / spring-boot 4.1 restent « non vérifiés », car les tests de bout en bout n'utilisent pas les vrais frameworks (signalement seulement tant que de tels tests n'existent pas).
4. Programmer les exécutions du soir et du matin sur un PC (pas encore fait : la commande de vérification ne marche pas sur ce PC et donnerait de fausses alertes ; voir un exemple en haut de `scripts/pl2pl-watch.cmd`).
5. (Fait) Une carte pl2pl est sur la page d'accueil de easy-web.tokyo et en ligne en production (open-easy-web `95cb1e7` ; l'ancien binaire est `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (fait, avec des limites) : le `@php` de Blade convertit maintenant les affectations et `++` `--` `+=` `-=` `*=` `/=` `.=` en `{% set %}` ; `$attributes` fonctionne comme sortie, ainsi que `->merge` (valeurs textuelles ou expressions convertibles), `->class([...])` (une liste de textes), `->only` et `->except` ; `morphToMany` (colonne de type dans la table pivot) ; et les composants Blade écrits en classes (`app/View/Components/*.php` : les arguments du constructeur deviennent des valeurs par défaut et `_props`, et un `view('x.y')` dans `render()` est suivi) (55 contrôles de bout en bout, 41 tests unitaires). Reste à faire : les méthodes publiques d'une classe de composant (un `{{ $method() }}` dans son gabarit reste un TODO, et le rapport les compte), `->class()` avec clés (conditionnel), et `@php` avec if/foreach ou appels de fonctions (reste un TODO).
7. Liaison avec aruaru-llm (faite) : le 2026-11-19, nous avons repris aruaru-llm v0.2.5 et lancé un vrai `pl2pl llm setup`. Le choix automatique, 3B, a échoué parce que ses poids sont découpés en fragments (shards) ; l'installation se rabat donc maintenant sur le modèle plus petit suivant (pas quand on en impose un). `GenerateQwen` renvoie du texte avec qwen2.5-1.5b-instruct (environ 2,9 Go, sur CPU). **`pl2pl php --explain`** (facultatif ; `--explain-max N`, 3 par défaut) ajoute une note d'IA sous chaque point TODO du rapport. Mesuré : environ 2 minutes par point sur CPU, et **la qualité est mauvaise et parfois fausse** (1 sur 2 était à côté), donc c'est désactivé par défaut. Un aruaru-llm redémarré n'a aucun modèle choisi (400), il en choisit donc d'abord un installé. **Un modèle plus grand (environ 12 Go)** : le plus gros du catalogue d'aruaru-llm est 3B (environ 6,2 Go), et 3B et 7B sont livrés en plusieurs fichiers fragments que l'installeur actuel ne sait pas récupérer. Le support des fragments (dans aruaru-llm) et la vitesse CPU (déjà aussi lente à 1.5B) passent d'abord. Kokkos sur GPU et les noyaux sur open-cuda / open-directx ne sont toujours pas vérifiés. **GPU (vérifié le 2026-11-19)** : sur une GeForce GT 730 (2 Go, sm_35, pilote 475 = classe CUDA 11.4), nous avons installé le nvcc de CUDA 11.8 sans droits d'administrateur via micromamba (canal conda de NVIDIA), exécuté un simple noyau CUDA et la sortie de `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) sur le vrai GPU, et constaté 0 écart par rapport au CPU (`tests/gpu-check.ps1`). Pièges : MSVC 14.44 demande `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` et `-allow-unsupported-compiler` ; PyPI n'a pas de nvcc pour Windows ; CUDA 12 ne peut pas cibler sm_35. **Toujours non vérifié** : le backend CUDA de Kokkos, et l'exécution sur open-cuda (l'implémentation en Rust) et open-directx.
8. Supprimer les COBOL_to et Java_to d'origine (**irréversible : demander d'abord l'accord final**).
9. Le propriétaire écrit `docs/DESIGN.md`. Nous avons relu `docs/LANGUAGES.md` le 2026-11-19 et n'y avons rien trouvé qui s'écarte de l'état actuel (Java et COBOL ont des convertisseurs). C'est toujours une liste fondée sur des connaissances, non vérifiée par une recherche web récente.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP de bout en bout) / `cargo test --release`

## Message pour reprendre
« Reprendre F:\pl2pl. Parcourir « Reste à faire » dans PORTING.md depuis le haut. Ne jamais cacher ce qui ne peut pas être récrit : laisser un TODO. Vérifier en exécutant pour de vrai avant de dire que c'est fini. Répondre uniquement en japonais. Faire un commit et un push à chaque étape, et mettre aussi le VPS à jour. »

## Nouveautés de la v0.2.1 (2026-11-20)
- Si l'on choisit `rpoem` / `rporea` dans `pl2pl web --backend`, des avis s'affichent (japonais et anglais) : RPoem inclut déjà les fonctions compatibles Tauri et WunderGraph Cosmo, tokio et WASM, donc ne pas les choisir en plus ; HTML5 et CSS3 sont utilisés mais en petite quantité ; JavaScript, TypeScript, Node.js et REST ne sont pas utilisés par conception. Ce qu'on ajoute avec `--also` et qui est déjà inclus est refusé (code de sortie 2) ; ce qui est inclus mais pas encore mûr reste sélectionnable, avec une mise en garde.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` juge cette maturité en lançant des tests-sondes (registre : `registry/maturity.toml`). L'avis de l'IA d'aruaru-llm ne change jamais seul le verdict.
- Avec les piles autres que RPoem / RPoRea, `--also nodejs` (ou `next.js`) est accepté avec un avis : nous n'imposons pas notre règle « pas de Node.js », mais nous ne le recommandons pas. Des alternatives adaptées au langage choisi sont listées (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js, et d'autres que nous n'avons pas vérifiées).
- Dépôts frères développés ensemble : Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js et pl2pl. Quand l'un change, les autres sont vérifiés le jour même.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` et `webpage/info.html` expliquent les principes de conception de RPoRea / RPoem, avec avantages et inconvénients, en 11 langues.
- Les originaux `COBOL_to` / `Java_to` sont conservés (non supprimés) ; les copies fusionnées dans `converters/` ont donné une sortie identique.

## Nouveautés (du 2026-11-22 au 2026-11-23)
- COBOL (`converters/cobol_to`) : ajout de `SORT` (`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), de `MERGE`, de `SEARCH` / `SEARCH ALL` et de `INSPECT ... LEADING` / `CHARACTERS`. Les usages `COMP-3` et `COMP` sont désormais lus.
- SQL intégré : `EXEC SQL` (curseurs, `FETCH`, `SELECT ... INTO`), `WHENEVER`, variables indicatrices (y compris la forme avec le mot-clé `INDICATOR`), `SQLSTATE`, `EXECUTE IMMEDIATE`, ainsi que `PREPARE` / `EXECUTE ... USING`. Le Python généré a vraiment été exécuté ; du côté Rust, la vérification n'a été faite qu'avec un faux gestionnaire.
- `EXEC CICS` (minimal) : `SEND TEXT` affiche du texte, et `RETURN` / `ABEND` terminent le programme. Les autres commandes restent en `TODO`. Les écrans BMS ne sont pas reproduits.
- Java (`converters/java_to`) : références de méthode, `Comparator`, `Collectors` et classes anonymes (toutes les méthodes sont conservées). Côté Rust, les opérations sur List (`Vec`, `push`, accès par indice, `for`) fonctionnent, et le Rust généré passe `cargo check` pour spring, jaxrs, servlet et spark.
- Essai sur Spring PetClinic REST (un projet qui part d'OpenAPI) : les routes trouvées sont passées de 1 à 37 ; le Python généré compte 0 erreur de syntaxe ; les requêtes dérivées (`Or`, `Like`, comparaisons, `OrderBy`, etc.) s'exécutent sur un dépôt en mémoire ; l'application peut être importée (41 routes) ; les noms de `this.field` sont maintenant alignés, si bien qu'un appel traverse contrôleur → service → dépôt.
- Reste à faire : génération du dépôt et du mappeur MapStruct (les appels s'arrêtent sur un `TODO`), une vraie base de données, une vraie exécution du FastAPI généré, les écrans et fichiers CICS, et les corps Rust qui utilisent Map ou Stream. Tests de pl2pl : 55 réussis.

## 2026-11-25 État et points restants

État : un outil qui récrit de vieux programmes (COBOL, C, C++, Fortran, Java, etc.) dans le langage et le framework de votre choix. Récemment, il a intégré la prise en charge de PetClinic de Java_to (ResponseEntity, Mapper, classes de données parentes), et la documentation multilingue a été synchronisée. Reste à faire : vérifier sur un vrai Mac et un vrai iPhone (nous n'avons pas ces appareils ici). Traductions : faites par IA (hors japonais et anglais), pas encore relues par des personnes de langue maternelle.

## 2026-11-25 Ajout : synchronisation des convertisseurs

Nous avons intégré dans `converters/cobol_to` la conversion de EXEC CICS LINK/XCTL en CALL de COBOL_to (`7d67bbf`), et dans `converters/java_to` l'opérateur ternaire de Java_to (corps Rust, `e9ac300`). Tests : Java_to 39, COBOL_to 25, pl2pl 55, tous réussis. Reste à faire : de vrais Mac et iPhone, et le démarrage des FastAPI générés avec une vraie base de données.

## 2026-11-25 Ajout : le choix entre plusieurs implémentations est fixé par un test

Quand une interface de dépôt a plusieurs implémentations, le générateur préférait déjà celle qui tourne en mémoire (dont le nom ne contient ni Jdbc ni Jpa) et, à défaut, prenait la première (l'ancienne mention « non pris en charge » était périmée). C'est désormais fixé par le test unitaire `multiple_impls_prefer_non_db_one` (Java_to : 40 réussis). Reste à faire : les mappeurs MapStruct complexes et le démarrage avec une vraie base de données.

## 2026-11-25 Ajout : prise en charge de @Mapping de MapStruct

Les `@Mapping(target=..., source="a.b")` (attribut source imbriqué), `ignore=true` et `constant="..."` placés sur les méthodes d'un mappeur sont désormais transmis à `MapperBase._to(cls, src, rename, ignore, const)` dans le Python généré. Cela est couvert par le test unitaire `mapstruct_mapping_annotations` (Java_to : 41 réussis) et par une vérification à l'exécution (`owner.name` → `fullName`, ignore, constante). Reste à faire : la forme imbriquée `@Mappings({...})`, `expression=`, `@MappingTarget` et le démarrage avec une vraie base de données.

<!-- directive-20261125-1130 -->
## 2026-11-25 Directive (pas de confirmation jusqu'à 11:30)
Jusqu'au 2026-11-25 11:30, mener à bien, automatiquement et sans demander confirmation, tout le travail inachevé. Même si la demande paraît faible ou peu nécessaire, ou s'il faut tout refaire de zéro à grande échelle, ne pas s'en tirer par une autre méthode : refaire le travail pour de bon. Développés ensemble : COBOL_to, Java_to, MFastAPI, 2MFastAPI (convertit d'autres langages et frameworks vers du code compatible MFastAPI ; Node.js et TypeScript sont une exception facultative, et ne choisir ni l'un ni l'autre est permis), RMojo, RPoRea (RPoem et Reaxt intégrés), 2RPoRea (convertit d'autres langages et frameworks vers RPoRea) et Reaxt (React + Next.js + Nuxt.js + Nest.js intégrés). Quand l'un change, vérifier les autres et mettre à jour README, CLAUDE, PORTING et POLICY en japonais et en anglais le même jour.

<!-- directive-20261125-mojo -->
## 2026-11-25 Ajout : MFastAPI doit être utilisable depuis Mojo
MFastAPI garde la **spécification d'être utilisable depuis Mojo** (syntaxe Python, cœur en Rust, prise en charge de Mojo ; sous Windows, le vrai Mojo dans WSL est piloté via `compat/wsl-bridge` de RMojo). Le résultat de 2MFastAPI doit lui aussi être utilisable depuis Mojo. État actuel : `mojo/` est une implémentation de référence, et ses serveurs des étapes B et C sont expérimentaux. Seul ce qui a réellement été exécuté est qualifié de « vérifié ». On continue automatiquement et sans demander confirmation jusqu'à 11:30.

## 2026-11-25 Ajout : c2rust, tableaux et break/continue

`pl2pl c2rust` gère désormais les tableaux de taille fixe (`int a[8] = {...};`, `double w[4];`, les valeurs initiales manquantes étant complétées par 0), la lecture et l'écriture par indice (`a[j + 1]` devient `a[(j + 1) as usize]`), `break;`, `continue;` et `while (1)` → `loop`. Un exemple contenant un tri à bulles et un `while (1)` a été compilé et exécuté avec gcc et rustc dans WSL, et les sorties sont strictement identiques (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 56 tests unitaires réussis. Pas encore pris en charge : les pointeurs, les structures, les tableaux à deux dimensions, le passage d'un tableau à une fonction et la conversion implicite entre entier et décimal (`k * 1.5` doit s'écrire `(double)k`).
