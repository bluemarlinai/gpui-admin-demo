# GPUI 现代化跨平台桌面应用架构规范与系统提示词 (Prompt)

> **核心定位**：基于 Rust、GPUI Kit、多源数据引擎与本地 AI 能力的次时代高性能、企业级跨平台桌面应用架构设计指南。包含系统架构分析、桌面端外壳规范、状态受控机制、**标准化专业工程目录规范**，以及**可直接交付给 AI 结对编程的完整入门系统提示词 (Starter Prompt)**。

---

## 目录
- [一、 专业桌面工程分层与目录样板规范](#一-专业桌面工程分层与目录样板规范)
  - [1. 三层目录分离原则 (源码 / 运行时数据 / 最终产物)](#1-三层目录分离原则-源码--运行时数据--最终产物)
  - [2. 标准工程目录样板树](#2-标准工程目录样板树)
- [二、 关键技术选型与组件库评估](#二-关键技术选型与组件库评估)
  - [1. 为什么强烈建议引入 `longbridge/gpui-kit`？](#1-为什么强烈建议引入-longbridgegpui-kit)
  - [2. 图标系统：Lucide 矢量 SVG 与专属品牌图标嵌入](#2-图标系统lucide-矢量-svg-与专属品牌图标嵌入)
  - [3. 数据持久化：本地 SQLite 离线优先与数据热备份快照](#3-数据持久化本地-sqlite-离线优先与数据热备份快照)
  - [4. 登录认证与安全体系：本地 Argon2id + 远程 JWT + OS Keyring](#4-登录认证与安全体系本地-argon2id--远程-jwt--os-keyring)
  - [5. 桌面级外壳基础设施：菜单栏、状态栏、系统托盘与自适应窗口](#5-桌面级外壳基础设施菜单栏状态栏系统托盘与自适应窗口)
  - [6. Windows 纯静态单文件即拷即用与体积优化标准](#6-windows-纯静态单文件即拷即用与体积优化标准)
- [三、 完整 AI 编程助手系统提示词 (AI Starter Prompt)](#三-完整-ai-编程助手系统提示词-ai-starter-prompt)
- [四、 核心组件快速上手范例 (Quickstart)](#四-核心组件快速上手范例-quickstart)

---

## 一、 专业桌面工程分层与目录样板规范

工业级桌面应用程序在文件组织上必须遵循**严格的关注点分离与产物隔离原则**，严禁在项目根目录下随处散落编译二进制、数据库或临时日志。

### 1. 三层目录分离原则 (源码 / 运行时数据 / 最终产物)

1. **核心源码与资源层 (Source & Assets Layer)**：
   - 纳入 Git 版本控制。包含 Rust 源码（`src/`）、静态图标设计资产（`assets/`）、编译构建脚本（`build.rs`）与依赖配置。
2. **运行时数据与诊断层 (Runtime Data Layer)**：
   - **按需自动建立**。运行时生成的 SQLite 数据库（`*.db`）、自动快照热备份（`backups/`）以及结构化诊断日志（`logs/`）。
   - **严格 Git 忽略**：在 `.gitignore` 中全量屏蔽，杜绝本地用户数据与日志污染版本库。
3. **打包交付与分发产物层 (Distribution Artifacts Layer)**：
   - **统一收拢至 `dist/`（或 `output/`）**：
     - 严禁将最终的可执行文件或压缩包存放在工程根目录。
     - `target/` 仅作为 Cargo 内部的编译缓存目录（易被 `cargo clean` 清理）。
     - 所有面向用户的绿色便携版（如 `dist/NovaOps管理系统.exe`）及发布归档压缩包（如 `dist/NovaOps-Pro-Windows-v1.2.0.zip`）统一集中存放在 **`dist/`** 目录下。

---

### 2. 标准工程目录样板树

```text
gpui-admin-demo/
├── .cargo/
│   └── config.toml           # 编译器全局配置 (如配置 target-feature=+crt-static 纯静态链接)
├── .github/
│   └── workflows/
│       └── release.yml       # 多平台 CI/CD 自动化矩阵构建脚本 (Windows/macOS/Linux)
├── .gitignore                # 忽略 target/, dist/, backups/, logs/, *.db 等本地文件
├── Cargo.toml                # 项目元数据、依赖项声明与 release 编译优化配置
├── build.rs                  # 编译期生成 64x64 图标并自动编译入 Windows PE 资源段
├── ARCHITECTURE_SPEC.md      # 系统架构规范与 AI 编程提示词标准
├── README.md                 # 样板工程介绍与本地构建使用手册
├── dist/                     # 【核心分发目录】存放最终打包交付物 (绿色便携单文件与发布 zip)
│   ├── NovaOps管理系统.exe   # 纯静态、零外部依赖、双击即跑的 Windows 单文件客户端 (~28 MB)
│   └── NovaOps-Pro-Windows-v1.2.0.zip # 最终发布的便携归档压缩包 (~10 MB)
├── assets/                   # 应用设计资产
│   ├── app_icon.svg          # 品牌专属矢量 SVG 图标
│   └── app_icon.ico          # 编译期生成的 Windows 多尺寸图标
├── backups/                  # [运行期自动创建] 数据热备份快照 (*.db) 与导出的审计 CSV
├── logs/                     # [运行期自动创建] 按天滚动的系统追踪日志 (novaops.log)
└── src/                      # 纯 Rust 源码实现
    ├── main.rs               # 主应用生命周期、自适应窗口、系统托盘指令轮询
    ├── logger.rs             # tracing + tracing-appender 结构化滚动日志
    ├── tray.rs               # 原生 Win32 系统托盘守护 (System Tray) 与右键菜单
    ├── state.rs              # AppState 响应式全局状态模型 (含会话、菜单、备份状态)
    ├── db.rs                 # SQLite 存储、热备份快照、CSV 导出与完整性校验
    ├── theme.rs              # 设计令牌与主题调色板 (浅色 / 深色 / 随系统)
    ├── icons.rs              # 复合图标资产加载源 (AppAssets)
    ├── models.rs             # 领域模型定义
    ├── services/             # 后台独立业务服务
    │   ├── auth.rs           # Argon2id 加盐哈希离线密码认证服务
    │   └── async_worker.rs   # Tokio 多线程后台异步计算与 I/O 工作池
    ├── components/           # 桌面外壳与通用组件
    │   ├── menubar.rs        # 顶部原生风格菜单栏 (未登录仅显帮助，登录后全量开放)
    │   ├── header.rs         # 顶部操作工具栏 (搜索、全屏、主题切换)
    │   ├── sidebar.rs        # 左侧常驻导航栏与操作员名片
    │   ├── statusbar.rs      # 底部常驻状态栏 (连接指标、热备状态、版本关于)
    │   ├── user_modal.rs     # 新增/编辑成员复杂模态表单 (InputState, Chips, Switch, Checkbox)
    │   └── about_modal.rs    # 「关于 NovaOps Pro」品牌模态对话框
    └── views/                # 业务页面视图
        ├── login.rs          # 独立安全登录页
        ├── dashboard.rs      # 仪表盘大屏与微服务拓扑
        ├── users.rs          # 用户账户与组织权限治理
        ├── monitoring.rs     # 集群监控与实时日志流
        ├── audit_logs.rs     # 合规审计日志与 CSV 导出
        └── settings.rs       # 系统环境与安全策略配置
```

---

## 二、 关键技术选型与组件库评估

### 1. 为什么强烈建议引入 `longbridge/gpui-kit`？

官方原生 GPUI（来自 Zed 编辑器）本质上是一个**极简的底层 GPU 绘制与 Flex 布局引擎**，只提供了 `div()`、`text()`、`img()`、`svg()` 等最基础的原语。在原生 GPUI 上开发桌面企业级应用面临以下痛点：
- 缺乏完整的通用输入控件：如支持 IME 中文输入法、光标选区与剪贴板的 Input/Textarea。
- 缺乏常用的桌面交互层：如 Select 下拉选择、Modal 弹窗、Dropdown、Tooltip、Popover、Toast 消息通知。
- 复杂场景难以手工实现：如支持海量数据（10万+ 行）的虚拟滚动表格（Virtual Table）、树状视图（Tree）、可拖拽多标签 Dock 布局系统等。

**长桥证券开源的 `gpui-kit` (75+ 组件，近 17K Stars)** 完美填补了这一空白：
1. **源自真实商业实战**：作为商业级股票交易桌面端（Longbridge Pro）的核心支撑框架，经历了严苛的生产环境验证。
2. **现代 shadcn/ui 视觉设计**：UI 规范参考业界广受好评的 `shadcn/ui`，内置一致的设计令牌（Design Tokens）、语义化颜色、状态反馈与暗色/亮色切换。
3. **单一依赖封装**：`gpui-kit` 将匹配的 `gpui`、`gpui-base`（非样式化状态与无障碍交互）、`gpui-component`（带样式的完整组件库）统一打包导出，避免多 crate 之间的 GPUI 版本错位冲突。
4. **内置 AccessKit 与 Headless 测试**：自带无障碍支持和无头窗口 UI 自动化测试体系。

---

### 2. 图标系统：Lucide 矢量 SVG 与专属品牌图标嵌入

- **基础通用图标（Lucide 矢量体系）**：
  - `gpui-kit` 默认集成了完整 **Lucide Icons**（通过 `gpui-kit-assets`）。
  - 所有图标均为矢量 SVG，支持任意缩放、抗锯齿、无缝继承文字颜色（`currentColor`）以及状态过渡动画。
- **专属品牌应用图标（Composite AssetSource）**：
  - 在 `assets/app_icon.svg` 中设计专属品牌矢量图标（科技六边形立方体），并导出为 `assets/app_icon.ico`。
  - 实现复合 `AssetSource`（如 `AppAssets`），优先装载应用自定义品牌 SVG，未命中则回退至 `gpui_kit::assets::Assets`。
  - 通过 `build.rs` + `winresource` 将 `.ico` 在编译期嵌入 Windows PE 的 `Resource ID 1`，使文件管理器、任务栏、窗口左上角图标均原生匹配。

---

### 3. 数据持久化：本地 SQLite 离线优先与数据热备份快照

现代桌面客户端推崇 **Offline-First（离线优先）** 架构：

| 数据层级 | 技术选型 | 核心职责 |
| :--- | :--- | :--- |
| **本地存储 (Offline-First)** | `rusqlite` 或 `sqlx (sqlite)` | 本地配置、离线缓存、用户主凭证、会话草稿、操作审计日志。保障应用在无网或弱网下秒开并完整运行。 |
| **全量热备份 (Hot Backup)** | SQLite 文件级快照 / 增量归档 | 提供本地快速全量冷备（如 `backups/novaops_backup_*.db`），并提供审计日志 CSV 导出与完整性校验（`PRAGMA integrity_check`）。 |
| **远程同步 (Cloud/Enterprise)** | `sqlx` (PostgreSQL / MySQL) / `supabase-rs` | 团队协作数据、远程资产库、多端云同步。通过后台工作池（Worker Pool）执行增量同步。 |
| **抽象层** | `Repository Trait` 接口抽象 | 上层视图仅与统一的数据仓储 Trait 交互，底层自由切换本地 SQLite 或远程数据源。 |

---

### 4. 登录认证与安全体系：本地 Argon2id + 远程 JWT + OS Keyring

1. **本地离线认证（离线与单机首登）**：
   - 本地用户凭证安全存放在本地 SQLite 中。
   - 密码严禁明文或简单 Hash，强制采用抗 GPU/ASIC 破解的 **Argon2id** 算法进行哈希与加盐校验。
2. **远程认证（JWT 令牌）**：
   - 若系统接入远程微服务、MySQL/Postgres API 或 Supabase，本地登录成功或通过远程 SSO 换取 JWT（`access_token` 与 `refresh_token`）。
   - JWT 仅作为请求远程接口的身份凭据，解析其 Claims 获取远程租户和角色信息。
3. **敏感凭据安全存储（OS Keyring）**：
   - 严禁将远程 API Token、第三方 App Secret 以明文写入 SQLite 或本地 `.json`/`.env` 配置文件。
   - 推荐使用 Rust `keyring` crate，将凭据委托给操作系统底层安全托管（Windows Credential Manager / macOS Keychain / Linux Secret Service）。

---

### 5. 桌面级外壳基础设施：菜单栏、状态栏、系统托盘与自适应窗口

完善的桌面端体验需要以下外壳层支撑：

1. **顶部桌面菜单栏 (MenuBar)**：
   - 保持经典原生风格：从左侧直接排列顶层菜单分类（`文件与数据 (F)`、`视图导航 (V)`、`运维工具 (T)`、`帮助 (H)`），禁止放置花哨的外置大按钮。
   - **权限与状态隔离 (Auth-Aware Menus)**：
     - **未登录状态下**：仅显示 `帮助 (H)` 菜单（可查看关于与运行诊断日志），避免向未认证用户暴露受限业务功能。
     - **登录成功后**：动态展开全量业务与数据管理菜单。
2. **底部常驻状态栏 (StatusBar)**：
   - 高度固定为 `26px`，全宽横跨窗口底部。
   - 左侧展示：控制面就绪指示灯、在线节点统计、数据快照状态（支持点击一键触发热备）。
   - 右侧展示：当前操作员与角色、安全合规等级、系统版本号（点击可唤起关于弹窗）。
3. **关于模态弹窗 (AboutModal)**：
   - 展示专属设计的 SVG 应用图标、版本与 LTS 构建代号、合规认证、数据容灾状态、托盘守护状态与版权信息。
4. **系统托盘与常驻守护 (System Tray)**：
   - 注册原生 Win32 托盘图标（`Shell_NotifyIconW`），支持鼠标左键双击激活/还原前置主窗口，右键弹出上下文菜单（打开控制台、立即执行备份、关于、安全退出）。
5. **多分辨率自适应窗口尺寸与任务栏遮挡规避**：
   - **自适应尺寸算法**：严禁写死 `1320x860` 等超出小屏高度的尺寸。通过 `cx.primary_display()` 读取扣除任务栏后的可用工作区 `display.visible_bounds()`，以 `1240x720` 为基准并按可用工作区的 `88%`（宽）和 `85%`（高）动态钳制，并在工作区内居中。
   - **Windows 最大化遮挡修复**：使用系统原生标题栏（`appears_transparent: false`）时，Windows DirectComposition 会因顶部标题栏产生约 31px 的垂直偏移。在程序启动入口设置 `GPUI_DISABLE_DIRECT_COMPOSITION=1`，使 SwapChain 严格对齐客户区（`ClientRect`），最大化时底部与任务栏严丝合缝。

---

### 6. Windows 纯静态单文件即拷即用与体积优化标准

1. **纯静态单文件（零外部 DLL / 零运行库依赖）**：
   - 在 `.cargo/config.toml` 中配置 `rustflags = ["-C", "target-feature=+crt-static"]`，将 `VCRUNTIME140.dll` 静态编入 `.exe`。
   - 使用 `rusqlite` 的 `features = ["bundled"]` 将 SQLite C 源码完全内嵌。
   - 整个应用程序只需一个单文件 `NovaOps管理系统.exe`，拷到任意 Windows 电脑双击即跑。
2. **编译优化与体积控制 (Debug vs Release)**：
   - Debug 模式（`cargo build`）默认开启 `debuginfo = 2`，保留全部未优化的 AST 符号表、栈回溯信息，体积约为 60MB。
   - Release 优化模式（`cargo build --release`）：配置 `strip = true` 与 `lto = "thin"`，体积大幅缩减至 **28 MB 左右**（压缩打包为 zip 仅需 **10 MB**）。

---

## 三、 完整 AI 编程助手系统提示词 (AI Starter Prompt)

> **使用方式**：将以下框内文本直接作为 System Prompt 提供给 AI 编程助手（Cursor / Claude / Gemini CLI 等），AI 即可直接根据此规范进行高水准的桌面应用开发与维护。

```markdown
# Role & Objective
你是一位深谙现代化 Rust 客户端工程架构的资深桌面软件专家。
你的目标是指导开发者使用 **GPUI Kit (`gpui-kit`) + Rust + 多源数据库 (SQLite 离线优先 + 远程云端) + 本地/云端 AI 能力** 构建一个高性能 (120 FPS)、颜值现代 (shadcn 质感)、企业级规范外壳 (菜单栏、状态栏、系统托盘、安全鉴权) 与专业工程目录结构的跨平台桌面应用程序。

---

## 1. 工程目录样板与产物收拢规范 (Project Layout & Dist Directory)

在工程搭建中必须遵循三层分离标准，严禁在根目录散落编译二进制或临时数据：

1. **源码与配置层 (Tracked in Git)**：
   - `src/`：业务、UI、状态与服务实现。
   - `assets/`：设计资产（SVG 品牌图标、多尺寸 .ico）。
   - `build.rs`：编译期资源注入脚本。
   - `.cargo/config.toml`：配置 target-feature=+crt-static 保证纯静态无运行库依赖。
2. **运行时数据层 (Ignored in Git)**：
   - `*.db`、`backups/`（数据快照）、`logs/`（滚动日志）。必须在 `.gitignore` 中彻底屏蔽。
3. **打包交付与分发产物层 (`dist/`)**：
   - **所有面向用户的单文件可执行文件（如 `NovaOps管理系统.exe`）及发布压缩包（如 `.zip`、`.dmg`、`.tar.gz`）必须统一输出并收拢至根目录的 `dist/` 文件夹**。
   - 严禁在代码根目录直接堆放打包文件。

---

## 2. 核心技术栈与外壳架构规范 (Core Tech Stack & App Shell)

1. **GUI 表现层与组件系统**：
   - 基础框架：采用 `gpui-kit` (长桥开源桌面级套件，统一集成了 GPUI、gpui-base 与 gpui-component)。
   - 视觉风格：基于 `shadcn/ui` 现代审美理念，完全支持暗色/亮色无缝切换。
   - 矢量图标：统一采用 `gpui-kit-assets` 内置的 **Lucide SVG** 矢量系统，以及项目自定义的 `AppAssets` 复合矢量图标。
   - 高级控件支持：优先使用 `gpui-kit` 内置的虚拟滚动表格（Virtual Table）、Dock 多面板拖拽布局、Markdown 渲染与通知 Toast。

2. **桌面级应用外壳系统 (Desktop App Shell Standards)**：
   - **顶部菜单栏 (MenuBar)**：经典原生菜单（文件、视图、工具、帮助），右侧不放置多余外置大按钮。
     - **权限隔离感知**：未登录时仅显示「帮助」菜单；登录认证通过后显示全量业务菜单。
   - **底部状态栏 (StatusBar)**：常驻窗口最底部（高度 26px），左侧显示引擎就绪指示、节点与成员规模、数据快照备份状态；右侧显示操作员身份、安全等级与版本号（点击可唤起关于弹窗）。
   - **关于弹窗 (AboutModal)**：居中模态对话框，展示应用矢量品牌 Logo、版本构建代号、合规认证、数据容灾状态与版权声明。
   - **系统托盘守护 (System Tray)**：基于原生 Win32 (`Shell_NotifyIconW`)，支持左键双击激活/还原前置窗口，右键弹出菜单（打开控制台、立即热备、关于、退出），与主 UI 线程解耦。
   - **窗口自适应与任务栏防遮挡**：
     - 通过 `cx.primary_display().visible_bounds()` 获取扣除任务栏后的工作区，初始尺寸设为 1240x720 并限制在工作区的 88%（宽）和 85%（高）以内居中。
     - 在 Windows 平台设置 `GPUI_DISABLE_DIRECT_COMPOSITION=1`，确保 SwapChain 严格对齐客户区，最大化时底部与任务栏严丝合缝。

3. **数据持久化与热备份规范 (Data Persistence)**：
   - **本地存储 (Offline-First)**：采用 `SQLite` (`rusqlite` 或 `sqlx`) 存储应用基础配置、本地离线数据、会话缓存与审计流水。
   - **数据热备份快照**：提供本地快速冷备引擎（写入 `backups/novaops_backup_*.db`），支持审计报表 CSV 导出与 `PRAGMA integrity_check` 完整性校验。
   - **架构解耦**：采用 Repository 模式抽象数据接口，上层 UI 无感切换本地与远程数据流。

4. **认证体系与安全性 (Auth & Security)**：
   - **本地离线认证**：本地用户凭证存盘密码强制采用工业级 `argon2id` 加盐哈希，严禁明文或弱哈希。
   - **远程 JWT 鉴权**：对接远程数据源或云服务时，通过 JWT (Access/Refresh Token) 维持身份鉴权。
   - **系统级安全存储**：API 密钥、远程 Token 等高密凭据，优先通过 `keyring` crate 托管于系统级钥匙串（Windows Credential Manager / macOS Keychain）。

5. **工程基建与异步协同**：
   - 异步调度：GPUI 内置 Executor 与后台 `tokio` 多线程运行时协同，严禁在主线程执行阻塞 I/O 或哈希计算。
   - 结构化日志：使用 `tracing` + `tracing-appender` 滚动输出日志至本地 `logs/` 目录。
   - 生产化文案规范：禁止在面向用户的 UI 界面暴露底层数据库表名、技术栈名词或测试调试快捷按钮。

---

## 3. 关键代码模式与受控组件规范 (Code Patterns)

1. **真实输入框必须绑定 `Entity<InputState>`**：
   - 严禁使用静态 `div` 模拟输入框。
   - 初始化：`cx.new(|cx| InputState::new(window, cx).default_value("...").placeholder("..."))`
   - 渲染：`Input::new(&self.input_state).cleanable(true)`，密码框附加 `.mask_toggle()`。
2. **Checkbox 与 Switch 完全受控机制**：
   - `Checkbox` 与 `Switch` 均为完全受控组件，必须绑定 `bool` 状态字段并注册回调更新：
     ```rust
     Checkbox::new("perm-read")
         .label("只读权限")
         .checked(this.perm_read)
         .on_click(cx.listener(|this, checked: &bool, _, cx| {
             this.perm_read = *checked;
             cx.notify();
         }))
     ```
3. **主题切换与组件库 Token 实时同步**：
   - 当应用切换深浅色主题时，必须同步调用：
     ```rust
     gpui_kit::component::theme::Theme::change(target_mode, Some(window), cx);
     ```
4. **编译体积优化配置 (`Cargo.toml`)**：
   - Debug 模式包含调试符号体积较大（~60MB）；正式发布包使用 Release 优化可缩减至 ~28MB：
     ```toml
     [profile.release]
     strip = true
     lto = "thin"

     [profile.release.package.gpui-pre]
     debug-assertions = true

     [profile.release.package.gpui-pre-macros]
     debug-assertions = true
     ```

请在编写代码前确认上下文与需求，严格遵循上述规范输出优雅、高效且符合人体工程学的 Rust 代码。
```

---

## 四、 核心组件快速上手范例 (Quickstart)

### 1. `Cargo.toml` 依赖引入

```toml
[package]
name = "modern-gpui-app"
version = "0.1.0"
edition = "2024"

[dependencies]
# 核心 UI 引擎与组件套件
gpui-kit = "0.7.3"

# 本地数据存储与密码加密
rusqlite = { version = "0.40.2", features = ["bundled"] }
argon2 = "0.5"
rand_core = { version = "0.6", features = ["getrandom"] }

# 异步与网络通信
tokio = { version = "1.40", features = ["full"] }

# 序列化与系统钥匙串
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
keyring = "3.6"

# 日志体系
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
tracing-appender = "0.2"
anyhow = "1.0"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62", features = [
    "Win32_Foundation",
    "Win32_Graphics_Gdi",
    "Win32_System_LibraryLoader",
    "Win32_UI_Shell",
    "Win32_UI_WindowsAndMessaging",
] }

[target.'cfg(target_os = "windows")'.build-dependencies]
winresource = "0.1"

[profile.release]
strip = true
lto = "thin"

[profile.release.package.gpui-pre]
debug-assertions = true

[profile.release.package.gpui-pre-macros]
debug-assertions = true
```

### 2. 主程序最小入口示例 (`main.rs`)

```rust
use gpui::*;
use gpui_kit::component::button::*;
use gpui_kit::component::*;

pub struct AppRoot;

impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0x18181b))
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0xf4f4f5))
                    .child("基于 GPUI Kit 的高性能桌面系统")
            )
            .child(
                Button::new("launch_btn")
                    .primary()
                    .label("开始探索")
                    .on_click(|_, _, _| {
                        println!("Button clicked!");
                    }),
            )
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    if std::env::var_os("GPUI_DISABLE_DIRECT_COMPOSITION").is_none() {
        unsafe {
            std::env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "1");
        }
    }

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);

            let bounds = if let Some(display) = cx.primary_display() {
                let work_area = display.visible_bounds();
                let win_w = 1240.0_f32.min(f32::from(work_area.size.width) * 0.88);
                let win_h = 720.0_f32.min(f32::from(work_area.size.height) * 0.85);
                let win_size = size(px(win_w), px(win_h));
                let origin_x = work_area.origin.x + (work_area.size.width - win_size.width) / 2.0;
                let origin_y = work_area.origin.y + (work_area.size.height - win_size.height) / 2.0;
                Bounds::new(point(origin_x, origin_y), win_size)
            } else {
                Bounds::centered(None, size(px(1240.0), px(720.0)), cx)
            };

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(1020.0), px(580.0))),
                titlebar: Some(TitlebarOptions {
                    title: Some("NovaOps Pro - 企业级云原生管理平台".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            gpui_kit::open_window(options, cx, |_, cx| cx.new(|_| AppRoot))
                .expect("创建主窗口失败");
        });
}
```
