> 说明：仓库根目录的日文 README.md 是原文。本文用简单易懂的话写成，由 AI 翻译，并经 AI 按母语水平审校语言是否自然；尚未经真人母语者（母语为简体中文的人）审校，可能仍有不自然之处。

# pl2pl

## pl2pl 是什么?
程序是用各种“语言”(编程语言)写成的。
**pl2pl 是把用一种语言写的程序，改写成另一种语言的工具。**
比如，可以把用 PHP(Laravel)做的网站，变成 Python(FastAPI)的网站。
支持 Windows、Linux 和 Mac(v0.1.0，还在开发中)。

- 使用时要安装到自己的电脑上。网站上只有说明和下载 → <https://easy-web.tokyo/pl2pl/>
- 改写不了的地方，pl2pl 不会偷偷删掉。它会留下 `TODO`(以后要做的事)，并在报告里写明原因。
- 我们的目标是：让知名的语言之间，尽可能多地互相改写。

## 现在能做什么
| 命令 | 作用 |
|---|---|
| `pl2pl php` | PHP(Laravel / Symfony)→ FastAPI。详情见 [PHP.md](../../PHP.md)(日文) |
| `pl2pl web` | 查看文件夹，认出 Java、COBOL、PHP、Flask/Django，交给合适的工具 |
| `pl2pl ir` | (试作)把 PHP、Python、Java 的“路由”变成同一种形式，再生成 JSON、OpenAPI 或 FastAPI 的雏形。PHP 的转换也会先把类型、函数主体、模型、中间件和模板放进这种形式，再写出结果 |
| `pl2pl watch` | 发现语言和工具的新版本，并自动跟上 |
| `pl2pl cobol` | COBOL + JCL → GnuCOBOL(Linux) |
| `pl2pl omp` / `kokkos` / `gpu` | 让 C/C++ 的循环跑得更快(gpu 为试作) |
| `pl2pl cport` | 让 UNIX 用的 C 代码在 Windows 和 Linux 上都能运行 |
| `pl2pl pyhpc` | 用 Numba 让 Python 的计算更快 |
| `pl2pl llm setup` | 安装 AI 助手(aruaru-llm)。实际安装还没有验证 |

## 自动跟上新版本
每天晚上和早上运行 `pl2pl watch`。
1. 检查语言和工具有没有新版本(清单在 `registry/ecosystem.toml`)。
2. 用新版本测试。通过的话，就自动更新“已支持的版本”。
3. 没通过，就由 version-control(借助 AI)尝试修复。
4. 还是修不好，就给已登记的邮箱和 LINE 发通知。

请注意：laravel、symfony、flask、django 等目前只会提醒“有新版本了”。
在 GitHub 上，每天晚上和早上也会自动用最新版本做测试。

## 使用方法
```
pl2pl php ./myapp --out ./out
pl2pl web ./project --target fastapi --out ./out
pl2pl ir ./project --out ./out --format openapi
```

## 请知道
- pl2pl 很谨慎。看起来有风险的地方，会原样保留并说明原因。我们不保证速度。
- 目前只在小型示例上测试过，还没有在大型真实程序上试过。

## 接下来
- 把类型和函数的内容也放进共同的形式，让 FastAPI 的转换也经过它。
- PHP：用类编写的 Blade 组件。
- 确认真实的邮件通知和 AI 自动修复。
- 确认在 GPU 上运行(open-cuda / open-directx)。

## v0.2.0 新功能
- 自由组合：`pl2pl web <dir> --backend <rust-axum|rust-poem|fastapi|mfastapi|rpoem|rporea> [--frontend bootstrap-ts|rust-wasm|none] [--db none|mysql|mariadb|postgres|sqlite] --out <dir>`。COBOL 会被改写；C、C++、Fortran 保持原样运行，由网页调用。
- `pl2pl c2rust`(简单的 C 和 C++)与 `pl2pl f2rust`(简单的 Fortran)可生成 Rust。`pl2pl cobol --os <linux|macos|unix|windows>` 生成 GnuCOBOL 的构建脚本。

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

## 2026-11-25 现状与未完成事项

现状：这是一个把旧程序（COBOL、C、C++、Fortran、Java 等）改写成你所选语言和框架的工具。最近并入了 Java_to 对 PetClinic 的支持（ResponseEntity、Mapper、父数据类），并同步了多语言文档。待办：在真实的 Mac 和 iPhone 上验证（这里没有这些设备）。翻译：由 AI 翻译并经 AI 审校，尚未经母语者确认。

<!-- directive-20261125-1130 -->
## 2026-11-25 指示（11:30 前无需确认）
2026-11-25 11:30 之前，未完成的事项不必确认，自动做到完成。即使判断需求少、必要性低，或者需要从头大规模重做，也不要用别的办法糊弄，而要正面重做。同时开发的对象：COBOL_to、Java_to、MFastAPI、2MFastAPI（把其他语言和框架转换为兼容 MFastAPI 的代码；Node.js 和 TypeScript 可作为例外选用，也可以不选）、RMojo、RPoRea（RPoem 与 Reaxt 的整合）、2RPoRea（把其他语言和框架转换为 RPoRea）、Reaxt（React + Next.js + Nuxt.js + Nest.js 的整合）。改动其中一个，就要检查其他的，并在同一天用日文和英文更新 README、CLAUDE、PORTING 和 POLICY。

<!-- directive-20261125-mojo -->
## 2026-11-25 补充：MFastAPI 必须能在 Mojo 中使用
MFastAPI 要一直保持**可在 Mojo 中使用的规格**（Python 语法、Rust 实现、支持 Mojo；在 Windows 上，通过 RMojo 的 `compat/wsl-bridge` 使用 WSL 里的 Mojo）。2MFastAPI 的转换结果也必须能在 Mojo 中使用。现状：`mojo/` 是参考实现，阶段 B、C 的服务器还是实验性的。从 Mojo 调用的真机确认，只把实际确认过的范围写成“已确认”。11:30 之前不必确认，自动推进。

## 2026-11-25 补充：c2rust 支持数组与 break/continue

`pl2pl c2rust` 现在支持定长数组（`int a[8] = {...};`、`double w[4];`，初始值不足的部分用 0 补齐）、下标的读写（`a[j + 1]` → `a[(j + 1) as usize]`）、`break;`、`continue;`，以及 `while (1)` → `loop`。我们把含冒泡排序和 `while (1)` 的示例，用 WSL 里的 gcc 和 rustc 分别运行，输出完全一致（`1 2 3 4 5 7 8 9` / `6.00 12.00 9`）。单元测试 56 项通过。尚不支持：指针、结构体、二维数组、把数组传给函数，以及整数与小数之间的隐式类型转换（`k * 1.5` 必须写成 `(double)k`）。

## 了解更多
[LANGUAGES.md](../../LANGUAGES.md)(语言清单) / [COMBINATIONS.md](../../COMBINATIONS.md)(组合) / [DESIGN.md](../../DESIGN.md)(设计，由所有者撰写) / [components/README.md](../../../components/README.md)(附带的组件)。这些文件为日文。
