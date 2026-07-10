# Project C Roadmap

Project C 是一个 Windows-first、键盘优先、本地优先的剪贴板管理器。它的核心方向是：快速捕获、快速搜索、文件夹组织、隐私友好的本地存储。

这份文档是轻量开发路线图，不是正式项目管理规范。它用来记录当前重点、下一步计划、已完成工作，以及影响后续开发的关键判断。

## Current Focus

把自动剪贴板监听变成一个可控、可搜索、可长期演进的桌面能力。

当前已经完成自动捕获文本剪贴板、监听开关、后端搜索、SQLite FTS、全局快捷键和基础键盘操作流。接下来重点是让 Project C 能安静地留在后台运行，而不是依赖一个常驻前台窗口。

## Now

### Add Tray And Background Mode

目标：让 Project C 能像工具软件一样安静运行，而不是依赖一个常驻前台窗口。

计划工作：

- 添加系统托盘。
- 关闭窗口时隐藏而不是退出。
- 托盘菜单提供 show、pause monitor、quit。
- 为后续开机启动和后台运行做准备。

建议提交信息：

```bash
Add tray background mode
```

## Next

### Add Configurable Shortcut

目标：把当前临时默认快捷键变成用户可修改的设置。

计划工作：

- 在 settings 里保存 quick access shortcut。
- 前端提供快捷键录制或输入控件。
- 更新快捷键时重新注册 global shortcut。
- 快捷键冲突或注册失败时给出可见状态。

建议提交信息：

```bash
Add configurable shortcut
```

### Add Copy Feedback And Monitor Suppression

目标：让复制动作更可感知，并避免写回剪贴板后监听循环产生多余事件。

计划工作：

- 复制成功后提供轻量状态反馈。
- 写回系统剪贴板时记录最近写回的 hash。
- 监听循环识别最近写回内容，跳过多余 capture。
- 评估是否增加 direct paste 模式。

建议提交信息：

```bash
Add copy feedback and monitor suppression
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
- 添加剪贴板监听开关。 ---- Completed at: 2026-07-07 16:32:44 +08:00
- 添加后端剪贴板搜索。 ---- Completed at: 2026-07-07 17:00:39 +08:00
- 添加 SQLite FTS 搜索。 ---- Completed at: 2026-07-07 17:19:54 +08:00
- 添加全局快捷键快速入口。 ---- Completed at: 2026-07-07 17:19:54 +08:00
- 添加 quick panel 键盘操作流。 ---- Completed at: 2026-07-08 11:39:24 +08:00

## Completed Details

这里记录从 `Now` 移入 `Done` 的完整功能说明。`Done` 只保留轻量条目和完成时间，详细目标、计划工作、验证方式和备注放在这里。

当前部分历史完成项发生在路线图规则建立之前，因此暂不补写历史细节。从路线图规则建立后完成的 `Now` 任务开始，移动完整说明到这里。

#### Add Clipboard Monitor Setting

目标：给用户控制自动捕获剪贴板的能力。

完成内容：

- 新增 SQLite `settings` 表。
- 存储 `clipboard_monitor_enabled`。
- 新增 Tauri 命令，用来读取和更新 settings。
- Rust 剪贴板监听循环每轮读取设置。
- 当监听关闭时，跳过剪贴板捕获。
- 前端加一个轻量开关：`Monitor On / Off`。

验证：

```bash
cargo check
pnpm check
pnpm tauri dev
```

备注：自动化验证确认 app 可启动、settings 表默认值存在。由于沙箱限制，直接从命令行写入 app data SQLite 进行开关行为验证被拒绝；开关点击行为留待手动审核。

Completed at: 2026-07-07 16:32:44 +08:00

#### Add Backend Clip Search

目标：把搜索逻辑从前端数组过滤，逐步移动到 Rust/SQLite。

完成内容：

- 新增 `search_clips` Tauri 命令。
- Rust/SQLite 层支持搜索关键词。
- Rust/SQLite 层支持 folder 过滤。
- Rust/SQLite 层预留 type/kind 过滤参数。
- 保持 pinned 和 recent 的排序规则稳定。
- 前端保留当前搜索输入框，但改为调用后端搜索。
- 搜索输入和 folder 切换使用短 debounce，避免每次按键立即打满后端调用。

验证：

```bash
cargo check
pnpm check
pnpm tauri dev
```

备注：这一版仍然使用 SQLite `LIKE` 查询，适合完成前后端调用路径和行为收口；真正的大量历史搜索性能会在下一步 `Add SQLite FTS Search` 中处理。

Completed at: 2026-07-07 17:00:39 +08:00

#### Add SQLite FTS Search

目标：让搜索在剪贴板历史变多后依然足够快、足够准确。

完成内容：

- 新增 `clips_fts` FTS5 虚拟表。
- 使用 SQLite `trigram` tokenizer，更接近原来的 substring 搜索体验。
- 将 clip 的 title/content/source/source_app 写入 FTS 索引。
- 新增 insert/update/delete triggers，让 FTS 索引跟随 clips 表变化。
- 为已有 clips 初次创建 FTS 时执行一次 rebuild。
- 搜索关键词适合 FTS 时使用 `MATCH` 和 `bm25` 排序。
- 对过短关键词保留 `LIKE` fallback，避免短词、符号和两个汉字这类查询失效。
- 保持 folder、kind、pinned 和 recent 规则稳定。

验证：

```bash
cargo check
pnpm check
```

备注：这一版先用 FTS 替换底层搜索路径，不做高亮、snippet 或复杂查询语法。后续如果需要更像搜索引擎的排序和预览，再在这个基础上扩展。

Completed at: 2026-07-07 17:19:54 +08:00

#### Add Global Shortcut For Quick Access

目标：让 Project C 更像真正的桌面工具，而不是普通窗口应用。

完成内容：

- 新增 Tauri 官方 `global-shortcut` 插件依赖。
- 新增 Rust `shortcuts` 模块。
- 注册临时默认快捷键 `Ctrl+Shift+Space`。
- 快捷键触发时显示、取消最小化并聚焦主窗口。
- 快捷键注册失败时只打印错误，不阻止 app 启动。
- 后续保留把快捷键做成 settings 配置项的空间。

验证：

```bash
cargo check
pnpm check
```

备注：当前只是快速入口骨架，还没有做快捷键设置 UI，也没有做唤起后自动聚焦搜索框或 Enter 写回剪贴板；这些已经拆到后续路线里。

Completed at: 2026-07-07 17:19:54 +08:00

#### Add Quick Panel Keyboard Flow

目标：让全局快捷键打开 Project C 后，可以不用鼠标完成高频剪贴板操作。

完成内容：

- 全局快捷键聚焦窗口后发出 `quick-access-opened` 事件。
- 前端监听 `quick-access-opened`，自动聚焦并选中搜索框内容。
- 保留当前搜索输入框，继续触发现有 backend/FTS search。
- 支持 `ArrowDown` / `ArrowUp` 在当前结果中移动 selected clip。
- `ArrowDown` / `ArrowUp` 移动 selected clip 时，列表滚动跟随当前选中项。
- 支持 `Enter` 将当前 selected clip 写回系统剪贴板。
- 写回时更新 clip 的 `last_used_at`。
- 写回成功后隐藏窗口，让 quick panel 更接近 Ditto/PasteNow 的选择手感。
- 支持 `Escape`：有搜索内容时清空搜索；无搜索内容时隐藏窗口。
- 键盘事件只接管 quick panel 场景，不干扰新增 clip 表单里的正常输入。

验证：

```bash
cargo check
pnpm check
```

备注：这一版 Enter 写回后只隐藏窗口，不做 direct paste；Dock、Mission Control、系统托盘和 skip taskbar 行为放到后台模式里处理。

Completed at: 2026-07-08 11:39:24 +08:00

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
- 当前阶段建议维护一份中文主文档，标题保持简单英文；等项目对外协作需求变强后，再考虑英文版。
