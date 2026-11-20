> 说明:仓库根目录的日文 PORTING.md 是原文。本文是 AI 翻译,用简单易懂的话写成,尚未经母语者审校。

# pl2pl 交接
(更早的详细记录在 `docs/HISTORY-ja.md`,日文。)

## 现在的状态(2026-11-19)
- 已发布 v0.1.0,有 Windows、Linux、Mac 的安装程序和 CI 测试。
- PHP → FastAPI 已用真实的 FastAPI 做过端到端测试:路由、Eloquent、查询构建器、Blade、中间件、关联关系和 `@can` 系列。
- `pl2pl web` 现在也能认出 PHP。
- `pl2pl watch` 发现新版本、测试、修复,实在不行就发通知。GitHub 每天晚上和早上也会用最新版本测试。
- `pl2pl ir`(试作)把 PHP、Python、Java(Spring、JAX-RS、Servlet、Spark)的路由变成共同的形式,输出 JSON、OpenAPI 或 FastAPI 雏形。已确认输出真的能被 Python 读取。
- 介绍和下载页面已上线:<https://easy-web.tokyo/pl2pl/>(在 VPS 上,静态页面)。
- COBOL_to 和 Java_to 已并入 `converters/`。

## 待办(重要的在前)
1. **确认**:真实邮件是否送达(`version-control --test-notify`;这台电脑没有 SMTP 设置)、通过 version-control `--loop` 自动修复、用最新的 fastapi 测试(GitHub 每晚的运行会确认)。
2. 把类型和函数内容放进 IR。让现在的 FastAPI 输出(`pl2pl php`)经过 IR。如果找路由时有遗漏,再考虑 tree-sitter。
3. 已用真正的新版本试过 `watch`:fastapi 0.165.0(starlette 1.10)下 `tests/run-php.sh` 通过 49 项,所以 registry 的已测版本升到 0.165。尚未完成:新的 laravel 13 / django 6.1 / spring-boot 4.1 仍为“未检查”,因为通关测试不使用真正的框架(在做出使用真框架的测试前只报告)。
4. 在电脑上登记每晚和每早的自动运行(未做:这台电脑上检查命令跑不起来,会误报;示例见 `scripts/pl2pl-watch.cmd` 开头)。
5. 在 easy-web.tokyo 的首页加上 pl2pl 的介绍卡片。
6. PHP:用类编写的 Blade 组件、`$attributes`、`@php`、`morphToMany`。
7. 与 aruaru-llm 联动(`llm setup` 的实际安装未验证)。Kokkos 的 GPU 运行;在 open-cuda / open-directx 上运行内核。
8. 删除原来的 COBOL_to 和 Java_to(**无法撤销,须先得到最终确认**)。
9. `docs/DESIGN.md` 由所有者撰写。确认 `docs/LANGUAGES.md` 没有过时。

## 测试
`bash tests/run-all.sh`(WSL)/ `bash tests/run-php.sh`(PHP 端到端)/ `cargo test --release`

## 重新开始用的消息
“继续 F:\pl2pl。从 PORTING.md 的‘待办’由上往下做。改写不了的地方不要偷偷删掉,留 TODO。真正运行确认后再说完成。回复只用日语。每个阶段都提交并 push,VPS 也要更新。”

## v0.2.1 的新内容(2026-11-20)
- 在 `pl2pl web --backend` 中选 `rpoem` / `rporea` 时会显示提示(日文和英文):RPoem 已包含 Tauri 兼容、WunderGraph Cosmo 兼容、tokio 和 WASM,因此不要另选;HTML5 和 CSS3 会用,但尽量少;按设计不使用 JavaScript、TypeScript、Node.js 和 REST。用 `--also` 添加的东西若已被包含,会被拒绝(退出码 2);已包含但兼容尚未成熟的,仍可选择,并附带提醒。
- `pl2pl maturity [--repo DIR] [--llm URL] [--apply]` 通过运行探测测试来判断成熟度(台账:`registry/maturity.toml`)。aruaru-llm 的 AI 意见不会单独改变判断结果。
- 在 RPoem / RPoRea 以外的组合中,`--also nodejs`(或 `next.js`)会被接受并附带提示:我们不强加“不用 Node.js”的方针,但不推荐。会列出适合所选语言的替代品(RPoem / RPoRea + Reaxt、FastAPI + Jinja2、RS-Node.js,以及我们尚未验证的其他框架)。
- 一起开发的姐妹仓库:Reaxt、RPoRea、RPoem、RMojo、open-cosmo、RS-TypeScript、RS-JavaScript、RS-Node.js 和 pl2pl。一个有变动,当天检查其他仓库。
- `pl2pl info [ui|nojs|json|connect|shared] [--lang ...]` 和 `webpage/info.html` 用 11 种语言说明 RPoRea / RPoem 的设计思想及其优缺点。
- 原来的 `COBOL_to` / `Java_to` 保留(不删除);`converters/` 中合并的副本输出完全一致。
