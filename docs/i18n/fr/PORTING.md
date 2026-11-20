> Remarque : le PORTING.md japonais à la racine du dépôt est l'original. Cette version est une traduction par IA, en mots simples, et n'a pas encore été relue par des locuteurs natifs.

# pl2pl – passation
(Les anciennes notes détaillées sont dans `docs/HISTORY-ja.md`, en japonais.)

## Où on en est (2026-11-19)
- La version 0.1.0 est publiée, avec des installateurs pour Windows, Linux et Mac et des tests en CI.
- PHP → FastAPI est testé de bout en bout avec un vrai FastAPI : routes, Eloquent, constructeur de requêtes, Blade, middleware, relations et la famille `@can`.
- `pl2pl web` repère aussi PHP maintenant.
- `pl2pl watch` trouve les nouvelles versions, les teste, corrige les problèmes et envoie une alerte si rien ne marche. GitHub teste aussi avec les versions les plus récentes chaque soir et chaque matin.
- `pl2pl ir` (version d'essai) met les routes de PHP, Python et Java (Spring, JAX-RS, Servlet, Spark) sous une forme commune et produit du JSON, de l'OpenAPI ou un squelette FastAPI. Nous avons vérifié que Python sait vraiment lire la sortie.
- La page de présentation et de téléchargement est en ligne : <https://easy-web.tokyo/pl2pl/> (sur le VPS, statique).
- COBOL_to et Java_to sont intégrés dans `converters/`.

## Reste à faire (le plus important d'abord)
1. **Vérifier** : l'arrivée d'un vrai e-mail (`version-control --test-notify` ; ce PC n'a pas de réglages SMTP), la réparation automatique via version-control `--loop`, et un test avec le dernier fastapi (l'exécution de chaque nuit sur GitHub le vérifiera).
2. Mettre les types et le contenu des fonctions dans l'IR. Faire passer la sortie FastAPI actuelle (`pl2pl php`) par l'IR. Si la recherche des routes laisse des choses de côté, envisager tree-sitter.
3. `watch` a été essayé avec une vraie nouvelle version : avec fastapi 0.165.0 (starlette 1.10), `tests/run-php.sh` a réussi 49 contrôles, la version testée du registre passe donc à 0.165. Reste : les nouveaux laravel 13 / django 6.1 / spring-boot 4.1 restent « non vérifiés », car les tests de bout en bout n'utilisent pas les vrais frameworks (signalement seulement tant que de tels tests n'existent pas).
4. Programmer les exécutions du soir et du matin sur un PC (pas encore fait : la commande de vérification ne marche pas sur ce PC et donnerait de fausses alertes ; voir un exemple en haut de `scripts/pl2pl-watch.cmd`).
5. Ajouter une carte pl2pl sur la page d'accueil de easy-web.tokyo.
6. PHP : composants Blade écrits en classes, `$attributes`, `@php`, `morphToMany`.
7. Liaison avec aruaru-llm (l'installation réelle par `llm setup` n'est pas vérifiée). Kokkos sur GPU ; noyaux sur open-cuda / open-directx.
8. Supprimer les COBOL_to et Java_to d'origine (**irréversible : demander d'abord l'accord final**).
9. Le propriétaire écrit `docs/DESIGN.md`. Vérifier que `docs/LANGUAGES.md` n'est pas périmé.

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
