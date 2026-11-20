> Note: the Japanese CLAUDE.md in the repository root is the original. This English version uses plain words.

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
