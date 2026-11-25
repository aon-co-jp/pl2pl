> Note: the Japanese PORTING.md in the repository root is the original. This English version uses plain words. It was translated by AI and then reviewed by AI for natural English. A human native speaker has not checked it yet.

# pl2pl hand-over
(Older, detailed notes are in `docs/HISTORY-ja.md`, in Japanese.)

## Where we are (2026-11-19)
- Version 0.1.0 is out, with installers for Windows, Linux and Mac, and tests in CI.
- PHP → FastAPI is tested end to end with real FastAPI: routes, Eloquent, query builder, Blade, middleware, relations and the `@can` family.
- `pl2pl web` also spots PHP now.
- `pl2pl watch` finds new versions, tests them, fixes problems, and sends an alert if nothing works. GitHub also tests with the newest versions every night and every morning.
- `pl2pl ir` (a test version) puts the routes of PHP, Python and Java (Spring, JAX-RS, Servlet, Spark) into one shared shape, and writes JSON, OpenAPI or a FastAPI starter. The IR also carries typed parameters, function bodies, models, middleware and templates, and `pl2pl php` now writes its output through the shared emitter in `src/ir_emit.rs` (output unchanged from before; 49 end-to-end checks pass).
- The guide and download page is live: <https://easy-web.tokyo/pl2pl/> (on the VPS, static). There is also a card on the easy-web.tokyo top page.
- COBOL_to and Java_to are merged into `converters/`.

## Still to do (most important first)
1. **Check**: a real e-mail arriving (`version-control --test-notify`; this PC has no SMTP settings), automatic fixing through version-control `--loop`, and a test on the newest fastapi (the nightly run on GitHub will check it).
2. Put types and function bodies into the IR. Send the current FastAPI output (`pl2pl php`) through the IR. If route finding misses things, think about tree-sitter.
3. `watch` was tried with a real new version: with fastapi 0.165.0 (starlette 1.10), `tests/run-php.sh` passed 49 checks, so the registry's tested version moved to 0.165. Still open: the new laravel 13 / django 6.1 / spring-boot 4.1 stay "unchecked" because the end-to-end tests do not use the real frameworks (report only until such tests exist).
4. Set up the nightly and morning runs on a PC (not done: the check command does not work on this PC and would give false alarms; see the top of `scripts/pl2pl-watch.cmd` for an example).
5. (Done) A pl2pl card is on the easy-web.tokyo top page and live in production (open-easy-web `95cb1e7`; the old binary is `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`).
6. PHP (done, with limits): Blade `@php` now handles assignments and `++` `--` `+=` `-=` `*=` `/=` `.=` as `{% set %}`; `$attributes` works as output, `->merge` (string values or convertible expressions), `->class([...])` (a list of strings), `->only` and `->except`; `morphToMany` (type column in the pivot table); and class-based Blade components (`app/View/Components/*.php`: constructor arguments become defaults and `_props`, and a `view('x.y')` in `render()` is followed) (55 end-to-end checks, 41 unit tests). Still open: public methods of a component class (a `{{ $method() }}` in its template stays a TODO, and the report counts them), keyed (conditional) `->class()`, and `@php` with if/foreach or function calls (stays a TODO).
7. aruaru-llm link-up (done): on 2026-11-19 we pulled in aruaru-llm v0.2.5 and ran a real `pl2pl llm setup`. The automatic pick, 3B, failed because its weights are split into shards, so setup now falls back to the next smaller model (not when you pin one). `GenerateQwen` returns text with qwen2.5-1.5b-instruct (about 2.9 GB, on CPU). **`pl2pl php --explain`** (optional; `--explain-max N`, default 3) adds an AI note under each TODO item of the report. Measured: about 2 minutes per item on CPU, and **the quality is poor and sometimes wrong** (1 of 2 was off the mark), so it is off by default. A restarted aruaru-llm has no model selected and answers with an error (400), so pl2pl selects an installed model first. **A bigger model (about 12 GB)**: the largest in aruaru-llm's catalog is 3B (about 6.2 GB), and 3B and 7B ship as several shard files that the current installer cannot fetch. Shard support (in aruaru-llm) and CPU speed (it is already this slow at 1.5B) come first. Kokkos on the GPU and kernels on open-cuda / open-directx are still unchecked. **GPU (checked 2026-11-19)**: on a GeForce GT 730 (2 GB, sm_35, driver 475 = CUDA 11.4 class) we installed the CUDA 11.8 nvcc without admin rights through micromamba (NVIDIA's conda channel), ran a plain CUDA kernel and the output of `pl2pl gpu --target open-cuda` (`vec_loop12.cu`) on the real GPU, and saw 0 mismatches against the CPU (`tests/gpu-check.ps1`). Pitfalls: MSVC 14.44 needs `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` and `-allow-unsupported-compiler`; PyPI has no Windows nvcc; CUDA 12 cannot target sm_35. **Still unchecked**: the Kokkos CUDA backend, and running on open-cuda (the Rust implementation) and open-directx.
8. Delete the original COBOL_to and Java_to (**this cannot be undone, so ask for a final OK first**).
9. The owner writes `docs/DESIGN.md`. We re-read `docs/LANGUAGES.md` on 2026-11-19 and found nothing out of line with the current state (Java and COBOL have converters). It is still a knowledge-based list, not checked against a fresh web search.

## Tests
`bash tests/run-all.sh` (WSL) / `bash tests/run-php.sh` (PHP end to end) / `cargo test --release`

## Message for starting again
“Resume F:\pl2pl. Work through ‘Still to do’ in PORTING.md from the top. Never hide what cannot be rewritten; leave a TODO. Check by really running before saying it is done. Reply in Japanese only. Commit and push at each step, and update the VPS too.”

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

## 2026-11-25 Addendum: c2rust, arrays and break/continue

`pl2pl c2rust` now handles fixed-size arrays (`int a[8] = {...};` and `double w[4];`; missing initial values are filled with 0), reading and writing by index (`a[j + 1]` becomes `a[(j + 1) as usize]`), `break;`, `continue;`, and `while (1)` as `loop`. A sample with bubble sort and `while (1)` was built with gcc and rustc in WSL, and the outputs match exactly (`1 2 3 4 5 7 8 9` / `6.00 12.00 9`). 56 unit tests pass. Not yet: pointers, structs, 2-D arrays, passing arrays to functions, and implicit conversion between int and float (`k * 1.5` must be written with `(double)k`).
