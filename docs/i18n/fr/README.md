> Remarque : le README.md japonais à la racine du dépôt est l'original. Cette version a été traduite par une IA, puis relue par une IA pour sa fluidité en langue maternelle ; elle n'a pas encore été relue par une personne de langue maternelle.

# pl2pl

## Qu'est-ce que pl2pl ?
Les programmes sont écrits dans beaucoup de « langues » (des langages de programmation).
**pl2pl est un outil qui récrit un programme d'une langue dans une autre.**
Par exemple, il peut transformer un site fait avec PHP (Laravel) en un site fait avec Python (FastAPI).
Il fonctionne sous Windows, Linux et Mac (version 0.1.0, encore en construction).

- On l'installe sur son ordinateur. Le site web ne propose que des guides et des téléchargements → <https://easy-web.tokyo/pl2pl/>
- Quand pl2pl ne peut pas récrire quelque chose, il ne le cache pas. Il laisse un `TODO` (une note pour plus tard) et explique pourquoi dans un rapport.
- Notre but : pouvoir récrire entre le plus grand nombre possible de langages connus.

## Ce qu'il sait faire aujourd'hui
| Commande | Ce qu'elle fait |
|---|---|
| `pl2pl php` | PHP (Laravel / Symfony) → FastAPI. Plus de détails dans [PHP.md](../../PHP.md) (en japonais) |
| `pl2pl web` | Regarde dans un dossier, repère Java, COBOL, PHP et Flask/Django, et confie chacun au bon outil |
| `pl2pl ir` | (version d'essai) Met les « routes » de PHP, Python et Java sous une même forme, puis produit du JSON, de l'OpenAPI ou un squelette FastAPI. La conversion PHP place aussi les types, le contenu des fonctions, les modèles, les middlewares et les gabarits sous cette forme avant d'écrire |
| `pl2pl watch` | Trouve les nouvelles versions des langages et des outils, et se met à jour tout seul |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL (Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | Rend les boucles C/C++ plus rapides (gpu est une version d'essai) |
| `pl2pl cport` | Fait fonctionner du code C pour UNIX sous Windows comme sous Linux |
| `pl2pl pyhpc` | Accélère les calculs Python avec Numba |
| `pl2pl llm setup` | Installe l'assistant IA (aruaru-llm). Nous n'avons pas encore vérifié l'installation réelle |

## Suivre les nouvelles versions tout seul
Nous lançons `pl2pl watch` chaque soir et chaque matin.
1. Il regarde si de nouvelles versions des langages et des outils sont sorties (la liste est dans `registry/ecosystem.toml`).
2. Il teste avec la nouvelle version. Si les tests passent, il met lui-même à jour la « version prise en charge ».
3. S'ils échouent, version-control (avec l'aide de l'IA) essaie de réparer.
4. Si ça ne suffit pas, il envoie un message aux adresses e-mail enregistrées et à LINE.

À noter : pour laravel, symfony, flask, django et quelques autres, il dit seulement « une nouvelle version est sortie » pour le moment.
Sur GitHub, des tests avec les versions les plus récentes tournent aussi tout seuls chaque soir et chaque matin.

## Comment l'utiliser
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## Bon à savoir
- pl2pl est prudent. Si quelque chose paraît risqué, il le laisse tel quel et dit pourquoi. Nous ne pouvons pas promettre la vitesse.
- Nous n'avons testé que de petits exemples. Nous n'avons pas encore essayé de gros programmes réels.

## La suite
- Mettre aussi les types et le contenu des fonctions dans la forme commune, et faire passer la sortie FastAPI par là.
- PHP : les composants Blade écrits sous forme de classes.
- Vérifier les vraies alertes par e-mail et la réparation automatique par l'IA.
- Vérifier le fonctionnement sur GPU (open-cuda / open-directx).

## Nouveautés de la v0.2.0
- Choisissez votre combinaison : `pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`. Le COBOL est réécrit ; C, C++ et Fortran tournent tels quels et sont appelés par le web.
- `pl2pl c2rust` (C et C++ simples) et `pl2pl f2rust` (Fortran simple) produisent du Rust. `pl2pl cobol --os <linux|macos|unix|windows>` crée un script de construction pour GnuCOBOL.

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

<!-- directive-20261125-1130 -->
## 2026-11-25 Directive (pas de confirmation jusqu'à 11:30)
Jusqu'au 2026-11-25 11:30, mener à bien, automatiquement et sans demander confirmation, tout le travail inachevé. Même si la demande paraît faible ou peu nécessaire, ou s'il faut tout refaire de zéro à grande échelle, ne pas s'en tirer par une autre méthode : refaire le travail pour de bon. Développés ensemble : COBOL_to, Java_to, MFastAPI, 2MFastAPI (convertit d'autres langages et frameworks vers du code compatible MFastAPI ; Node.js et TypeScript sont une exception facultative, et ne choisir ni l'un ni l'autre est permis), RMojo, RPoRea (RPoem et Reaxt intégrés), 2RPoRea (convertit d'autres langages et frameworks vers RPoRea) et Reaxt (React + Next.js + Nuxt.js + Nest.js intégrés). Quand l'un change, vérifier les autres et mettre à jour README, CLAUDE, PORTING et POLICY en japonais et en anglais le même jour.

<!-- directive-20261125-mojo -->
## 2026-11-25 Ajout : MFastAPI doit être utilisable depuis Mojo
MFastAPI garde la **spécification d'être utilisable depuis Mojo** (syntaxe Python, cœur en Rust, prise en charge de Mojo ; sous Windows, le vrai Mojo dans WSL est piloté via `compat/wsl-bridge` de RMojo). Le résultat de 2MFastAPI doit lui aussi être utilisable depuis Mojo. État actuel : `mojo/` est une implémentation de référence, et ses serveurs des étapes B et C sont expérimentaux. Seul ce qui a réellement été exécuté est qualifié de « vérifié ». On continue automatiquement et sans demander confirmation jusqu'à 11:30.

## 2026-11-25 Ajout : c2rust, tableaux et break/continue

`pl2pl c2rust` gère désormais les tableaux de taille fixe (`int a[8] = {...};`, `double w[4];`, les valeurs initiales manquantes étant complétées par 0), la lecture et l'écriture par indice (`a[j + 1]` devient `a[(j + 1) as usize]`), `break;`, `continue;` et `while (1)` → `loop`. Un exemple contenant un tri à bulles et un `while (1)` a été compilé et exécuté avec gcc et rustc dans WSL, et les sorties sont strictement identiques (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 56 tests unitaires réussis. Pas encore pris en charge : les pointeurs, les structures, les tableaux à deux dimensions, le passage d'un tableau à une fonction et la conversion implicite entre entier et décimal (`k * 1.5` doit s'écrire `(double)k`).

## En savoir plus
[LANGUAGES.md](../../LANGUAGES.md) (liste des langages) / [COMBINATIONS.md](../../COMBINATIONS.md) (combinaisons) / [DESIGN.md](../../DESIGN.md) (la conception, écrite par le propriétaire) / [components/README.md](../../../components/README.md) (éléments fournis avec l'outil). Ces fichiers sont en japonais.
