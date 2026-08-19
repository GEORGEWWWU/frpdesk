# FRP Desk

FRP Desk 是一个基于 Tauri + Vue 3 的桌面 GUI 管理工具，用于本地启动/停止 frpc（FRP 客户端），查看运行日志、管理 frpc 配置并快速控制内网映射。目标是为使用 FRP 做内网穿透的用户提供一个简洁、现代的桌面管理面板（Windows 优先，Tauri + Rust 提供后端进程控制与日志转发）。

仓库地址：https://github.com/GEORGEWWWU/frpdesk

---

目录
- 项目简介
- 功能亮点
- 代码结构
- 快速开始（开发）
- 运行与打包（发行版）
- 配置说明与使用指南
- 常见问题与故障排查
- 贡献指南
- 许可证

---

项目简介

FRP Desk 将 frpc 的进程管理和配置编辑以桌面应用形式呈现：
- 在 UI 中选择 frpc 可执行文件与 frpc.toml 配置文件
- 启动/停止 frpc，查看 stdout/stderr 日志并将日志实时推送到前端
- 读取/写入 frpc.toml（前端会把表单内容转成 toml 并写回磁盘）
- 显示运行时长、远程/本地端口等关键信息

技术栈
- 前端：Vue 3 + Vite
- 桌面容器：Tauri（Rust 后端）
- 后端：Rust（tauri 命令 + 子进程管理）

功能亮点
- 一键启动与停止 frpc
- 实时日志（从 frpc stdout/stderr 读行并通过事件发送到前端）
- 简易的 frpc.toml 解析/生成（前端使用正则匹配特定字段）
- 本地保存上次选择的可执行文件和配置路径（localStorage）
- 运行时长由后端根据真实启动时间计算，保障准确性

代码结构（重点文件）
- 前端：[src/App.vue](I:/A_Tauri/frpdesk/src/App.vue)（主 UI 与逻辑）
- 前端入口：[src/main.ts](I:/A_Tauri/frpdesk/src/main.ts)
- 后端（Tauri/Rust）：[src-tauri/src/main.rs](I:/A_Tauri/frpdesk/src-tauri/src/main.rs)
  - 包含：start_frp / stop_frp / read_config / save_config / get_frp_stats
- 示例配置：[src-tauri/frpc.toml](I:/A_Tauri/frpdesk/src-tauri/frpc.toml)
- Tauri 配置：[src-tauri/tauri.conf.json](I:/A_Tauri/frpdesk/src-tauri/tauri.conf.json)
- package.json（npm 脚本 & 依赖）：[package.json](I:/A_Tauri/frpdesk/package.json)

注意：仓库根目录有空的 README.md（本文件会覆盖），以及已包含的示例 frpc.toml。

快速开始（开发环境）

先决条件（主要针对 Windows）：
- Node.js（建议最新版 LTS），npm 可用
- Rust 和 Cargo（用于 Tauri 后端）：https://www.rust-lang.org/
- Tauri 需要的系统依赖：在 Windows 上通常需要 MSVC 编译工具（Build Tools for Visual Studio）。请参阅 Tauri 官方文档获取完整先决条件。
- frpc 可执行文件（frpc.exe），请从 FRP 官方站或维护渠道下载并放在本地。

安装依赖

在仓库根目录执行：

```bash
# 安装前端依赖
npm install
```

本地开发（两种常见方式）：

方式 A（并行运行前端与 tauri dev）
1. 在一个终端运行前端开发服务器：
   npm run dev
2. 在另一个终端运行 Tauri 开发模式（会以集成的方式打开桌面窗口）：
   npm run tauri dev

方式 B（使用 tauri 启动，它会自动运行前端 dev 命令）

```bash
# 让 tauri 启动 dev 环境（tauri CLI 需要在环境中可用）
npm run tauri dev
```

注：scripts 中定义了 "tauri": "tauri"，所以可以通过 npm run tauri -- dev 传递子命令，或直接安装并使用 npx/cargo-tauri。

运行后：打开 App，进入“软件设置”面板，选择 frpc.exe 与 frpc.toml（或使用仓库内示例），加载配置并在“概览”中启动服务，查看日志。

运行与打包（生产构建）

构建前端并打包 Tauri 应用（生成安装包）：

```bash
# 1) 先构建前端静态资源
npm run build

# 2) 使用 tauri 打包（在 Windows 上会生成 .msi/.exe、在 macOS/.dmg、Linux 包）
# 如果 package.json 未提供 tauri build 封装，直接：
npm run tauri build
# 或者：npx tauri build
```

最终生成的安装包位置取决于 Tauri 的 bundle 配置（参考 src-tauri/tauri.conf.json -> bundle）。

配置说明与使用指南

- 在首次打开应用时，前往“软件设置”选择 frpc.exe 路径与 frpc.toml 文件。应用会把路径保存在 localStorage（本地浏览器存储）以便下次加载。
- 选择配置文件后，会尝试读取并解析部分字段（serverAddr、serverPort、auth.token、localIP、localPort、remotePort）。
- 在“参数配置”页可以编辑这些字段并保存，会把前端生成的 TOML 覆盖目标文件（save_config 调用写回磁盘）。
- 启动服务时，应用会：
  - 静默保存（覆盖）配置文件
  - 以子进程方式启动 frpc（传入 -c <config> 参数）并捕获 stdout/stderr
  - 将每一行日志通过 Tauri 事件 emit 到前端并显示在“运行日志”页
- 停止服务会杀掉子进程并清除运行时间计时器

重要文件参考
- 后端进程管理与事件转发：[src-tauri/src/main.rs](I:/A_Tauri/frpdesk/src-tauri/src/main.rs)
  - 读日志：通过 BufReader 循环读取 stdout/stderr 并 emit("frpc-log", line)
  - 保存/读取配置：read_config / save_config 简单使用 fs::read_to_string 与 fs::write
  - 运行时长：后端用 Instant 记录启动时间，get_frp_stats 返回格式化的 HH:MM:SS
- 前端核心逻辑与 UI： [src/App.vue](I:/A_Tauri/frpdesk/src/App.vue)
  - 选择文件使用 tauri 的 dialog 插件（open）
  - 调用后端命令使用 invoke（start_frp, stop_frp, read_config, save_config, get_frp_stats）

常见问题与故障排查

Q: 启动 frpc 报错“启动失败: <io error>”/进程无法启动
- 确认 frpc.exe 路径是否正确（可在资源管理器双击测试）
- 检查 frpc.exe 是否有执行权限（Windows 上以管理员权限尝试）
- 如果 frpc 期望的依赖不存在或版本不对，尝试手动在终端运行：frpc.exe -c <path> 看看直观错误

Q: 不能读取/写入配置文件（权限错误）
- 确认配置文件路径的写权限，若在系统受保护目录（例如 Program Files），请复制到用户可写目录或以管理员权限运行应用

Q: 日志为空或没有输出
- 确认 frpc 是否确实在运行（可以在任务管理器查看进程）
- 手动使用命令行运行 frpc 查看是否有输出
- 确保应用在启动 frpc 时将 stdout/stderr 管道打开（项目代码已处理）

安全性注意事项
- 保存与写回配置会覆盖目标 toml，请确保备份重要配置
- 不要把敏感 token/密钥上传到公共仓库；frpdesk 不会将配置文件发送到远端，但本地文件仍可能包含敏感信息
- 为生产环境考虑权限与最小权限原则：仅给 frpc.exe 和配置文件最低限度所需的读写权限

贡献指南
- 欢迎提交 issue 与 PR。建议先提 issue 讨论大型变更。
- 代码风格：前端使用 TypeScript + Vue 3，后端使用 Rust/Tauri。
- 测试：当前仓库未包含自动化测试用例，提交 PR 前请手动验证变更不会破坏本地运行与打包流程。

许可证
- 本项目使用仓库根目录的 LICENSE（请参阅 LICENSE 文件）