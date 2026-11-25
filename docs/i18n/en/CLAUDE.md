> Note: the Japanese CLAUDE.md in the repository root is the original. This English version uses plain words. It was translated by AI and then reviewed by AI for natural English. A human native speaker has not checked it yet.

# pl2pl dev notes
- **Job**: a tool that rewrites code between languages, frameworks and operating systems (made in Rust, in `src/`). Source languages are listed in `docs/LANGUAGES.md`, combinations in `docs/COMBINATIONS.md`. The owner writes the design in `docs/DESIGN.md`, little by little. **Never make up the spec for them.**
- **Direction**: rewrite "many to many" between well-known languages. Do not build everything at once; start with the chosen ones. The website only has guides and downloads. The rewriting happens in the PC app.
- **Rules**: whatever cannot be rewritten stays as a `TODO` and is reported (never hide it). Be careful (if a dependency or side effect is possible, do not change it). Run it for real before saying "done". No new REST. Reply in Japanese only.
- **Shared shape (IR)**: `src/ir.rs` (types, function bodies, models, middleware, templates) and the writer side in `src/ir_emit.rs`. The input side (PHP, Python, Java) and the output side (JSON, OpenAPI, FastAPI) are kept apart so new combinations are easy to add. `pl2pl php` goes through the same writer too; for a change that must not alter the output, prove it with an empty diff of the output before and after. Report files where no route was found. Check the output for real, for example with `py_compile`.
- **PHP**: `src/php.rs`, `src/blade.rs`, `src/middleware.rs`. End-to-end test: `tests/run-php.sh` (real FastAPI, SQLite, Jinja2). Unit tests: `cargo test --release`. In WSL: `wsl -d Ubuntu -- bash <file>`.
- **Auto catch-up**: `pl2pl watch` (`src/watch.rs`, `registry/ecosystem.toml`). Items with an empty check command are only shown as "unchecked" and are never updated by themselves. Alerts go through version-control `--notify-file`.
- **Deploy**: the guide page is `webpage/index.html`. On the VPS: `/root/easy-web.tokyo/pl2pl` (update with `git pull`; it is static, so no restart), `pl2pl-web.service` on port 8142, already added to `domains.toml`.
- **Traps**: write files with Japanese using Python `-X utf8`. Keep shell scripts in LF. Git Bash rewrites `/mnt/...` paths. For text with many quotes or `\`, use the Write / Edit tools, not a heredoc.
- **COBOL_to / Java_to**: already merged into `converters/`. Deleting the original repositories cannot be undone, so ask for a final OK first.
- **Translation system** (human language) stays apart from the converters. Converters do not rely on an LLM.
- **Document languages**: the Japanese files are the originals. The English files were rewritten in plain words. The other eight languages (`docs/i18n/`) are AI translations of the same content (not yet checked by native speakers). Older history: `docs/HISTORY-ja.md`.

## What's new in v0.2.1 (2026-11-20)
- Pick `rpoem` / `rporea` in `pl2pl web --backend` and it shows notices (Japanese and English): RPoem already includes Tauri-compatible and WunderGraph Cosmo-compatible features, tokio and WASM, so do not pick those; HTML5 and CSS3 are used but kept small; JavaScript, TypeScript, Node.js and REST are not used by design. Things added with `--also` that are already included are declined (exit code 2); things included but not yet mature stay selectable, with a caution.
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` judges that maturity by running probe tests (ledger: `registry/maturity.toml`). The AI opinion from aruaru-llm never changes the verdict by itself.
- With stacks other than RPoem / RPoRea, `--also nodejs` (or `next.js`) is accepted with a notice: we do not impose our no-Node.js policy, but we do not recommend it. Alternatives for the chosen language are listed (RPoem / RPoRea + Reaxt, FastAPI + Jinja2, RS-Node.js, and others we have not verified).
- Sister repositories developed together: Reaxt, RPoRea, RPoem, RMojo, open-cosmo, RS-TypeScript, RS-JavaScript, RS-Node.js and pl2pl. When one changes, the others are checked the same day.
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` and `webpage/info.html` explain the design principles of RPoRea / RPoem with pros and cons in 11 languages.
- The original `COBOL_to` / `Java_to` are kept (not deleted); the merged copies in `converters/` gave identical output.

## What's new (2026-11-22 to 2026-11-23)
- COBOL (`converters/cobol_to`): added `SORT` (`USING` / `GIVING`, `INPUT` / `OUTPUT PROCEDURE`, `RELEASE`, `RETURN`), `MERGE`, `SEARCH` / `SEARCH ALL`, and `INSPECT ... LEADING` / `CHARACTERS`. `COMP-3` and `COMP` usages are now read.
- Embedded SQL: `EXEC SQL` (cursors, `FETCH`, `SELECT ... INTO`), `WHENEVER`, indicator variables (including the `INDICATOR` keyword form), `SQLSTATE`, `EXECUTE IMMEDIATE`, and `PREPARE` / `EXECUTE ... USING`. The generated Python was really run; the Rust side is checked only with a fake handler.
- `EXEC CICS` (minimal): `SEND TEXT` shows text, and `RETURN` / `ABEND` end the program. Other commands stay as a `TODO`. BMS maps are not reproduced.
- Java (`converters/java_to`): method references, `Comparator`, `Collectors`, and anonymous classes (every method is kept). On the Rust side, List operations (`Vec`, `push`, index access, `for`) work, and the generated Rust passes `cargo check` for spring, jaxrs, servlet and spark.
- Trial on Spring PetClinic REST (a project that starts from OpenAPI): routes found went from 1 to 37; the generated Python has 0 syntax errors; derived queries (`Or`, `Like`, comparisons, `OrderBy` and so on) run on an in-memory repository; the app can be imported (41 routes); and `this.field` names now match, so a call goes controller → service → repository.
- Still open: repository and MapStruct mapper generation (calls stop at a `TODO`), a real database, a real run of the generated FastAPI, CICS maps and files, and Rust bodies with Map or Stream. pl2pl tests: 55 pass.

## 2026-11-25 Status and open items

Status: a tool that rewrites old programs (COBOL, C, C++, Fortran, Java and more) into the language and framework you choose. Recently it took in Java_to's PetClinic support (ResponseEntity, Mapper, parent data classes), and the translated docs were synced. Open: checks on a real Mac and iPhone (we have no such devices here). Translations: AI-translated (other than ja/en); not yet checked by native speakers.

## 2026-11-25 Addendum: converter sync

Synced `converters/cobol_to` with COBOL_to's EXEC CICS LINK/XCTL to CALL (`7d67bbf`), and `converters/java_to` with Java_to's ternary operator in Rust bodies (`e9ac300`). Tests: Java_to 39, COBOL_to 25, pl2pl 55, all pass. Open: real Mac and iPhone devices, and starting the generated FastAPI apps against a real DB.

## 2026-11-25 Addendum: choosing among several implementations, now pinned by a test

When a repository interface has several implementations, the generator already prefers one that does not have Jdbc or Jpa in its name (one that runs in memory). If there is none, it takes the first. (The earlier "unsupported" note was out of date.) This is now pinned by the unit test `multiple_impls_prefer_non_db_one` (Java_to: 40 pass). Open: complex MapStruct mappers, and starting against a real DB.

## 2026-11-25 Addendum: MapStruct @Mapping support

For a mapper method, `@Mapping(target=..., source="a.b")` (a nested source), `ignore=true` and `constant="..."` are now passed to the generated Python `MapperBase._to(cls, src, rename, ignore, const)`. This is covered by the unit test `mapstruct_mapping_annotations` (Java_to: 41 pass) and by a check at run time (`owner.name` to `fullName`, ignore, constant). Open: the nested `@Mappings({...})` form, `expression=`, `@MappingTarget`, and starting against a real DB.

<!-- directive-20261125-1130 -->
## 2026-11-25 Directive (no need to ask until 11:30)

Until 2026-11-25 11:30, carry unfinished work through to completion automatically, without asking. Even if demand looks low, or a large rewrite from scratch is needed, do not use a different approach instead; do the real work. These are developed together: COBOL_to, Java_to, MFastAPI, 2MFastAPI (converts other languages and frameworks to MFastAPI-compatible code; Node.js and TypeScript are an optional exception, and choosing neither is allowed), RMojo, RPoRea (RPoem and Reaxt integrated), 2RPoRea (converts other languages and frameworks to RPoRea) and Reaxt (React + Next.js + Nuxt.js + Nest.js integrated). When one changes, check the others, and update README, CLAUDE, PORTING and POLICY in Japanese and English on the same day.

<!-- directive-20261125-mojo -->
## 2026-11-25 Addendum: MFastAPI must be usable from Mojo

MFastAPI keeps the **specification of being usable from Mojo** (Python syntax, Rust core, Mojo support). On Windows, the real Mojo in WSL is driven through RMojo's `compat/wsl-bridge`. The output of 2MFastAPI must be usable from Mojo too. Current state: `mojo/` is a reference implementation, and its stage B and C servers are experimental. Only what was actually run is called "verified". Continue automatically without asking until 11:30.

## Versionless API policy (2026-11-25)

The APIs of this repository are **Versionless**: no `v1` or similar in URLs or package names. They evolve by adding only, without breaking compatibility. Rules: only add fields; a new field is optional; ignore fields you do not know; to rename, add the new name and deprecate the old one (the default deadline is 6 months or more, and remove it only after you confirm it is used 0 times); accept enum values you do not know; an emergency fix for safety is the exception. Existing `.v1` names stay for now so callers do not break. A versionless alias is added, and `.v1` is retired step by step. Details and sources: the end of [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md). We will apply this step by step from now on.

## API policy: no more REST, use gRPC / Connect

This repository **does not add new REST APIs**. An API called from a browser or `curl` uses a **Connect unary call** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`). Calls between services use **gRPC / gRPC-web**. GraphQL is read-only. Existing REST is moved when we touch it, and the old route is removed once moved. Read JSON with RS-JSON (`from_str_secure` / `parse_secure`). The only exceptions are cases where someone else's standard decides the shape (ACME, OAuth/OIDC callbacks, third-party webhooks, health checks, static files, OTLP). When in doubt, use Connect. The source of truth is [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md).
