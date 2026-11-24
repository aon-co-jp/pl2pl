> 说明：仓库根目录的日文 CLAUDE.md 是原文。本文用简单易懂的话写成，由 AI 翻译，并经 AI 按母语水平审校语言是否自然；尚未经真人母语者审校。

# pl2pl 开发笔记
- **任务**：在语言、框架、操作系统之间改写代码的工具(用 Rust 编写，位于 `src/`)。源语言清单见 `docs/LANGUAGES.md`，组合见 `docs/COMBINATIONS.md`。设计由所有者在 `docs/DESIGN.md` 中一点一点地写。**不要擅自替他们定规格。**
- **方向**：在知名语言之间做“多对多”的改写。不一次做完，先做选定的。网站只有说明和下载，改写在 PC 应用里进行。
- **规则**：改写不了的地方留 `TODO` 并报告(不要偷偷删掉)。要谨慎(可能有依赖或副作用就不改)。真正运行过再说“完成”。不新增 REST。回复只用日语。
- **共同的形式(IR)**：`src/ir.rs`(类型、函数主体、模型、中间件、模板)和写出端 `src/ir_emit.rs`。输入端(PHP、Python、Java)和输出端(JSON、OpenAPI、FastAPI)分开，方便增加新组合。`pl2pl php` 也走同一个写出端；不应改变输出的修改，要用修改前后输出的 diff 为空来证明。没找到路由的文件要报告。输出要真正检查，例如用 `py_compile`。
- **PHP**：`src/php.rs`、`src/blade.rs`、`src/middleware.rs`。端到端测试：`tests/run-php.sh`(真实的 FastAPI、SQLite、Jinja2)。单元测试：`cargo test --release`。WSL 中：`wsl -d Ubuntu -- bash <file>`。
- **自动跟上**：`pl2pl watch`(`src/watch.rs`、`registry/ecosystem.toml`)。检查命令为空的项目只显示“未检查”，绝不自动更新。通知通过 version-control 的 `--notify-file`。
- **部署**：介绍页是 `webpage/index.html`。VPS 上是 `/root/easy-web.tokyo/pl2pl`(用 `git pull` 更新，是静态页面，无需重启)、`pl2pl-web.service` 端口 8142，已写入 `domains.toml`。
- **陷阱**：含日文的文件用 Python `-X utf8` 写入。shell 脚本保持 LF。Git Bash 会改写 `/mnt/...` 路径。引号或 `\` 很多的文本，用 Write / Edit 工具，不要用 heredoc。
- **COBOL_to / Java_to**：已并入 `converters/`。删除原仓库无法撤销，须先得到最终确认。
- **翻译系统**(人类语言)与转换器分开。转换器不依赖 LLM。
- **文档语言**：日文文件是原文。英文文件用简单的话重写过。`docs/i18n/` 里的其他八种语言，是把同样内容由 AI 翻译、再由 AI 审校语言是否自然的版本(尚未经真人母语者审校)。更早的经过见 `docs/HISTORY-ja.md`。

## v0.2.1 的新内容(2026-11-20)
- 在 `pl2pl web --backend` 中选 `rpoem` / `rporea` 时会显示提示(日文和英文):RPoem 已包含 Tauri 兼容、WunderGraph Cosmo 兼容、tokio 和 WASM，因此不要另选；HTML5 和 CSS3 会用，但尽量少；按设计不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 添加的东西若已被包含，会被拒绝(退出码 2)；已包含但兼容尚未成熟的，仍可选择，并附带提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 通过运行探测测试来判断成熟度(台账：`registry/maturity.toml`)。aruaru-llm 的 AI 意见不会单独改变判断结果。
- 在 RPoem / RPoRea 以外的组合中，`--also nodejs`(或 `next.js`)会被接受并附带提示：我们不强加“不用 Node.js”的方针，但不推荐。会列出适合所选语言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js，以及我们尚未验证的其他框架)。
- 一起开发的姐妹仓库：Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一个有变动，当天检查其他仓库。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 用 11 种语言说明 RPoRea / RPoem 的设计思想及其优缺点。
- 原来的 `COBOL_to` / `Java_to` 保留(不删除);`converters/` 中合并的副本输出完全一致。

## 新增内容(2026-11-22 至 2026-11-23)
- COBOL(`converters/cobol_to`):新增 `SORT`(`USING` / `GIVING`、`INPUT` / `OUTPUT PROCEDURE`、`RELEASE`、`RETURN`)、`MERGE`、`SEARCH` / `SEARCH ALL`,以及 `INSPECT ... LEADING` / `CHARACTERS`。现在可以读取 `COMP-3` 和 `COMP` 用法。
- 嵌入式 SQL:`EXEC SQL`(游标、`FETCH`、`SELECT ... INTO`)、`WHENEVER`、指示变量(包括 `INDICATOR` 关键字形式)、`SQLSTATE`、`EXECUTE IMMEDIATE`,以及 `PREPARE` / `EXECUTE ... USING`。生成的 Python 已实际运行过;Rust 一侧只用模拟处理器验证过。
- `EXEC CICS`(最小支持):`SEND TEXT` 用于显示文字,`RETURN` / `ABEND` 用于结束程序。其他命令保留为 `TODO`。不重现 BMS 画面映射。
- Java(`converters/java_to`):支持方法引用、`Comparator`、`Collectors` 和匿名类(保留每一个方法)。Rust 一侧支持 List 操作(`Vec`、`push`、下标访问、`for`),生成的 Rust 在 spring、jaxrs、servlet、spark 四种框架下都通过了 `cargo check`。
- 在 Spring PetClinic REST(以 OpenAPI 为起点的项目)上的试验:找到的路由从 1 条增加到 37 条;生成的 Python 语法错误为 0;派生查询(`Or`、`Like`、比较、`OrderBy` 等)可在内存仓库上运行;应用可以导入(41 条路由);`this.field` 的名称已统一,调用能依次经过控制器 → 服务 → 仓库。
- 尚未完成:仓库与 MapStruct 映射器的生成(调用会停在 `TODO`)、真实数据库、真正运行生成的 FastAPI、CICS 的画面映射与文件、含 Map 或 Stream 的 Rust 函数体。pl2pl 测试:55 项通过。
