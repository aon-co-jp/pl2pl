> 说明：仓库根目录的日文 PORTING.md 是原文。本文用简单易懂的话写成，由 AI 翻译，并经 AI 按母语水平审校语言是否自然；尚未经真人母语者审校。

# pl2pl 交接
(更早的详细记录在 `docs/HISTORY-ja.md`，日文。)

## 现在的状态(2026-11-19)
- 已发布 v0.1.0，有 Windows、Linux、Mac 的安装程序和 CI 测试。
- PHP → FastAPI 已用真实的 FastAPI 做过端到端测试：路由、Eloquent、查询构建器、Blade、中间件、关联关系和 `@can` 系列。
- `pl2pl web` 现在也能认出 PHP。
- `pl2pl watch` 发现新版本、测试、修复，实在不行就发通知。GitHub 每天晚上和早上也会用最新版本测试。
- `pl2pl ir`(试作)把 PHP、Python、Java(Spring、JAX-RS、Servlet、Spark)的路由变成共同的形式，输出 JSON、OpenAPI 或 FastAPI 雏形。IR 还带有有类型的参数、函数主体、模型、中间件和模板，`pl2pl php` 现在通过 `src/ir_emit.rs` 里的共同写出器输出(输出与以前相同；49 项端到端检查通过)。
- 介绍和下载页面已上线：<https://easy-web.tokyo/pl2pl/>(在 VPS 上，静态页面)。easy-web.tokyo 的首页上也有一张卡片。
- COBOL_to 和 Java_to 已并入 `converters/`。

## 待办(重要的在前)
1. **确认**：真实邮件是否送达(`version-control --test-notify`；这台电脑没有 SMTP 设置)、通过 version-control `--loop` 自动修复、用最新的 fastapi 测试(GitHub 每晚的运行会确认)。
2. 把类型和函数内容放进 IR。让现在的 FastAPI 输出(`pl2pl php`)经过 IR。如果找路由时有遗漏，再考虑 tree-sitter。
3. 已用真正的新版本试过 `watch`:fastapi 0.165.0(starlette 1.10)下 `tests/run-php.sh` 通过 49 项，所以 registry 的已测版本升到 0.165。尚未完成：新的 laravel 13 / django 6.1 / spring-boot 4.1 仍为“未检查”，因为通关测试不使用真正的框架(在做出使用真框架的测试前只报告)。
4. 在电脑上登记每晚和每早的自动运行(未做：这台电脑上检查命令跑不起来，会误报；示例见 `scripts/pl2pl-watch.cmd` 开头)。
5. (已完成)easy-web.tokyo 首页上的 pl2pl 卡片已加好，并已在正式环境上线(open-easy-web `95cb1e7`；旧的二进制文件是 `server/target/release/open-easy-web-server.before-pl2pl-card-20261119`)。
6. PHP(已完成，有限制):Blade 的 `@php` 现在会把赋值以及 `++` `--` `+=` `-=` `*=` `/=` `.=` 转成 `{% set %}`;`$attributes` 可作为输出，`->merge`(字符串值或可转换的表达式)、`->class([...])`(字符串列表)、`->only` 和 `->except` 都能用；`morphToMany`(中间表里有类型列)；还有用类编写的 Blade 组件(`app/View/Components/*.php`：构造函数参数变成默认值和 `_props`,`render()` 里的 `view('x.y')` 也会跟进)(55 项端到端检查，41 项单元测试)。尚未完成：组件类的公开方法(模板里的 `{{ $method() }}` 仍留 TODO，报告里会统计数量)、带键(有条件)的 `->class()`，以及含 if/foreach 或函数调用的 `@php`(仍留 TODO)。
7. 与 aruaru-llm 联动(已完成):2026-11-19 我们引入了 aruaru-llm v0.2.5，并实际运行了 `pl2pl llm setup`。自动选中的 3B 失败了，因为它的权重被拆成多个分片(shard)，所以现在安装会退回到小一档的模型(手动指定模型时除外)。`GenerateQwen` 用 qwen2.5-1.5b-instruct(约 2.9 GB,CPU 上)可以生成文字。**`pl2pl php --explain`**(可选；`--explain-max N`，默认 3)会在报告的每个 TODO 项下加一条 AI 说明。实测：在 CPU 上每项约 2 分钟，而且**质量差，有时是错的**(2 项里有 1 项偏题)，所以默认关闭。重启后的 aruaru-llm 没有选定模型(400)，所以会先选一个已安装的。**更大的模型(约 12 GB)**：aruaru-llm 目录里最大的是 3B(约 6.2 GB)，而 3B 和 7B 是以多个分片文件发布的，现在的安装程序下载不了。首先要有分片支持(在 aruaru-llm 里)和 CPU 速度(1.5B 就已经这么慢了)。Kokkos 的 GPU 运行，以及在 open-cuda / open-directx 上运行内核，仍未验证。**GPU(2026-11-19 已确认)**：在 GeForce GT 730(2 GB,sm_35，驱动 475 = CUDA 11.4 级别)上，我们不需要管理员权限，通过 micromamba(NVIDIA 的 conda 频道)装上了 CUDA 11.8 的 nvcc，在真实 GPU 上运行了一个简单的 CUDA 内核和 `pl2pl gpu --target open-cuda` 的输出(`vec_loop12.cu`)，与 CPU 相比有 0 处不一致(`tests/gpu-check.ps1`)。注意点：MSVC 14.44 需要 `-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH` 和 `-allow-unsupported-compiler`;PyPI 上没有 Windows 版 nvcc;CUDA 12 不能以 sm_35 为目标。**仍未验证**：Kokkos 的 CUDA 后端，以及在 open-cuda(Rust 实现)和 open-directx 上运行。
8. 删除原来的 COBOL_to 和 Java_to(**无法撤销，须先得到最终确认**)。
9. `docs/DESIGN.md` 由所有者撰写。2026-11-19 我们重新读了 `docs/LANGUAGES.md`，没有发现与现状不符的地方(Java 和 COBOL 已有转换器)。它仍是凭已有知识写成的列表，没有用最新的网络搜索核对过。

## 测试
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端到端)/ `cargo test --release`

## 重新开始用的消息
“继续 F:\pl2pl。从 PORTING.md 的‘待办’由上往下做。改写不了的地方不要偷偷删掉，留 TODO。真正运行确认后再说完成。回复只用日语。每个阶段都提交并 push,VPS 也要更新。”

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
