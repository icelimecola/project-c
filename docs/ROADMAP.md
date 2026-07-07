# Project C Roadmap

Project C 是一个 Windows-first、键盘优先、本地优先的剪贴板管理器。它的核心方向是：快速捕获、快速搜索、文件夹组织、隐私友好的本地存储。

这份文档是轻量开发路线图，不是正式项目管理规范。它用来记录当前重点、下一步计划、已完成工作，以及影响后续开发的关键判断。

## Current Focus

把自动剪贴板监听变成一个可控、可搜索、可长期演进的桌面能力。

当前已经完成自动捕获文本剪贴板的第一版。接下来重点不是继续堆捕获能力，而是先让它可关闭、可解释、可配置。

## Now

### Add Clipboard Monitor Setting

目标：给用户控制自动捕获剪贴板的能力。

计划工作：

- 新增 SQLite `settings` 表。
- 存储 `clipboard_monitor_enabled`。
- 新增 Tauri 命令，用来读取和更新 settings。
- Rust 剪贴板监听循环每轮读取设置。
- 当监听关闭时，跳过剪贴板捕获。
- 前端加一个轻量开关，例如 `Monitor On / Off`。

建议提交信息：

```bash
Add clipboard monitor setting
```

## Next

### Add Backend Clip Search

目标：把搜索逻辑从前端数组过滤，逐步移动到 Rust/SQLite。

计划工作：

- 新增 `search_clips` 命令。
- 支持搜索关键词。
- 支持 folder 过滤。
- 支持 type/kind 过滤。
- 保持 pinned 和 recent 的排序规则稳定。
- 前端保留当前搜索输入框，但改为调用后端搜索。

建议提交信息：

```bash
Add backend clip search
```

### Add SQLite FTS Search

目标：让搜索在剪贴板历史变多后依然足够快、足够准确。

计划工作：

- 新增 FTS5 虚拟表。
- 将 clip 的 title/content/source 写入 FTS 索引。
- 插入、更新、软删除 clip 时同步更新 FTS。
- 用 FTS 结果做搜索排序。

建议提交信息：

```bash
Add SQLite FTS search
```

### Add Global Shortcut For Quick Access

目标：让 Project C 更像真正的桌面工具，而不是普通窗口应用。

计划工作：

- 优先使用 Tauri 插件实现全局快捷键。
- 快捷键触发时打开或聚焦主窗口。
- 先设置一个临时默认快捷键。
- 后续再把快捷键做成可配置项。

建议提交信息：

```bash
Add global shortcut for quick access
```

## Later

### Windows-First Clipboard Integration

目标：从跨平台轮询，逐步过渡到 Windows 原生剪贴板事件。

计划工作：

- 使用 `AddClipboardFormatListener` / `WM_CLIPBOARDUPDATE`。
- 捕获来源应用 `source_app`。
- 捕获来源窗口 `source_window`。
- 新增 Windows-specific platform 模块。
- 保留跨平台 fallback。

### Tray And Background Mode

目标：让 Project C 能像工具软件一样安静运行。

计划工作：

- 添加系统托盘。
- 关闭窗口时隐藏而不是退出。
- 后续支持开机启动。
- 后续支持暂停监听、恢复监听。

### Rich Clipboard Types

目标：从纯文本剪贴板扩展到 HTML、图片、文件等类型。

建议顺序：

- `text/plain`
- `text/html`
- `image/png`
- file lists

可能的数据模型：

```text
clips
clip_payloads
attachments/
```

文本可以继续存在 SQLite 里。图片、大文件、复杂 payload 更适合放在 app data 目录，SQLite 只存 metadata 和路径。

### Quick Panel UI

目标：提供真正高频使用的快速交互界面。

计划工作：

- 键盘优先的搜索面板。
- 上下键选择。
- Enter 复制或粘贴。
- folder/type/source 过滤。
- pin/favorite 操作。

## Done

- Tauri + SvelteKit + TypeScript + Rust 项目初始化。 ---- Completed before roadmap tracking
- Mock clipboard workspace UI。 ---- Completed before roadmap tracking
- 前端组件拆分。 ---- Completed before roadmap tracking
- Rust command layer：folders / clips。 ---- Completed before roadmap tracking
- SQLite-backed mock data layer。 ---- Completed before roadmap tracking
- 基础 clip 操作：create、delete、pin/unpin。 ---- Completed before roadmap tracking
- 手动剪贴板文本捕获。 ---- Completed before roadmap tracking
- 真实剪贴板历史所需的 schema 字段。 ---- Completed before roadmap tracking
- 基于 `content_hash` 的去重。 ---- Completed before roadmap tracking
- 自动剪贴板监听循环。 ---- Completed before roadmap tracking

## Completed Details

这里记录从 `Now` 移入 `Done` 的完整功能说明。`Done` 只保留轻量条目和完成时间，详细目标、计划工作、验证方式和备注放在这里。

当前已有完成项发生在路线图规则建立之前，因此暂不补写历史细节。从下一次完成 `Now` 任务开始，移动完整说明到这里。

## Working Agreements

- 每次尽量推进一个清晰的小里程碑。
- 一个里程碑对应一个语义清楚的 commit。
- 默认由用户自己 commit 和 push。
- Codex 可以建议 commit message，但不主动提交。
- 后续 `Now` 中完成的功能不要删掉。
- 完成时，在 `Done` 中新增一条轻量记录。
- `Done` 条目格式使用：

```text
- 功能名称。 ---- Completed at: YYYY-MM-DD HH:MM:SS +08:00
```

- 完成时，将 `Now` 中的完整说明移动到 `Completed Details`。
- 移入 `Completed Details` 时，将标题降低一级，例如 `###` 改成 `####`。
- 移入 `Completed Details` 时，在说明末尾添加完成时间，精确到秒。
- 每个开发里程碑至少运行：

```bash
cargo check
pnpm check
```

- 涉及 Tauri runtime、窗口、剪贴板、事件监听时，还要运行：

```bash
pnpm tauri dev
```

## Notes

- `docs/ROADMAP.md` 用于开发计划和路线。
- `README.md` 用于项目介绍、安装和快速开始。
- 未来如果架构决策变多，可以新增 `docs/DECISIONS.md` 或 `docs/adr/`。
- 未来如果开始正式发布版本，可以新增 `CHANGELOG.md`。
- 当前阶段建议维护一份中文主文档，关键标题双语；等项目对外协作需求变强后，再考虑英文版。
