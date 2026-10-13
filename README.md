# NovaOps Pro - GPUI Kit + Rust 桌面应用通用样板工程 (Boilerplate)

基于 **GPUI Kit (`v0.7.3`)** 与 **Rust** 构建的现代化、高性能、次时代跨平台桌面应用企业级样板工程（含完整桌面外壳、状态栏、菜单栏、系统托盘、本地数据持久化与离线安全鉴权）。

> 📖 **架构指南与 AI 提示词**：请参阅 [ARCHITECTURE_SPEC.md](./ARCHITECTURE_SPEC.md)，包含完整技术架构选型与**可直接交给 AI 结对编程的系统提示词（AI Starter Prompt）**。

---

## 🌟 核心样板能力一览

1. **工业级 UI 与组件体系**：
   - 基于 `longbridge/gpui-kit` (shadcn/ui 设计质感) + Lucide 矢量 SVG 系统。
   - 包含真实受控输入框（`Entity<InputState>`）、受控复选框（`Checkbox`）、开关（`Switch`）、按钮（`Button`）。
   - 包含新增/编辑成员的复杂模态表单（`UserModal`，多字段响应式联动）。
2. **完整的桌面端外壳系统 (App Shell)**：
   - **顶部菜单栏 (MenuBar)**：经典原生菜单（文件与数据、视图导航、运维工具、帮助），未登录时权限隔离仅显示帮助菜单。
   - **顶部工具栏 (Header)**：紧凑面包屑、全局快速检索框、SLA 运行状态、深浅主题切换、全屏视图切换。
   - **底部状态栏 (StatusBar)**：常驻 26px，显示控制面连接指示灯、集群节点/成员统计、最近数据快照状态（支持点击一键热备）、操作员与版本号。
   - **关于模态弹窗 (AboutModal)**：展示专属设计的 SVG 应用图标、版本构建代号、合规认证与版权声明。
   - **系统托盘守护 (System Tray)**：Win32 原生托盘图标（带悬停提示），支持左键双击激活/前置窗口，右键上下文菜单（打开控制台、立即热备、关于、退出）。
   - **多分辨率自适应窗口**：基于主显示器可用工作区（`visible_bounds`，扣除任务栏）居中并按比例钳制（1240×720 基准），同时通过配置消除 DirectComposition 任务栏遮挡。
3. **本地离线优先数据与安全认证 (Offline-First)**：
   - 内置 SQLite 本地存储引擎（`rusqlite bundled` 纯静态编译，无需外部 DLL）。
   - 用户密码强制采用工业级 **`Argon2id`** 单向加盐哈希存储与离线鉴权。
   - 预设账户：管理员 `admin / admin123`，运维员 `ops / ops123`。
   - 支持全量数据快照热备份（写入 `backups/novaops_backup_*.db`）与合规审计报表 CSV 导出。
4. **纯静态零依赖单文件打包**：
   - 开启 `crt-static` 与 `strip = true`、`lto = "thin"`，生成的单文件 `.exe` 无需任何外部运行库（无需 VC++ 运行库、无需 sqlite3.dll、内嵌 PE 图标），拷走即用。

---

## 🚀 本地编译与运行指南

### 1. Windows 本地编译与打包

在项目根目录下直接运行：

```powershell
# 开发调试运行
cargo run

# 构建高度优化的零依赖单文件发布版
cargo build --release
```

- Cargo 内部编译产物位于 `target/release/gpui-admin-demo.exe`。
- 本地打包交付物统一收拢在 **`dist/`** 目录：
  - `dist/NovaOps管理系统.exe`：纯静态无依赖单文件版本（~28.9 MB）。
  - `dist/NovaOps-Pro-Windows-v1.2.0.zip`：便携发布压缩包（~10.4 MB）。
  - 可直接拷走在任意 Windows 10/11 电脑上双击运行。

### 2. macOS 本地编译（若在 Mac 设备上作为样板运行）

在 macOS 终端中执行（无需安装额外 C 依赖，macOS 原生自带 Metal/AppKit）：

```bash
cargo run
# 或打正式发布包
cargo build --release
```

编译产物位于 `target/release/gpui-admin-demo`。

### 3. Linux 本地编译（若在 Linux 设备上作为样板运行）

在 Ubuntu / Debian 终端中安装基础图形与 Wayland 开发头文件后编译：

```bash
sudo apt-get update
sudo apt-get install -y libwayland-dev libxkbcommon-dev libfontconfig1-dev libfreetype6-dev
cargo run
# 或打正式发布包
cargo build --release
```

---

## 📂 样板工程分层目录树

```text
gpui-admin-demo/
├── Cargo.toml                # 项目依赖与编译优化 (gpui-kit, rusqlite, argon2, windows)
├── build.rs                  # 编译期生成 64x64 图标并嵌入 PE 资源段
├── ARCHITECTURE_SPEC.md      # 架构选型与 AI 编程助手提示词规范
├── README.md                 # 样板工程说明文档
├── NovaOps管理系统.exe        # 编译就绪的 Windows 纯静态单文件可执行程序
├── NovaOps-Pro-Windows-v1.2.0.zip # 绿色便携发布压缩包 (~10 MB)
├── assets/
│   ├── app_icon.svg          # 专属品牌六边形星核立方体矢量图标
│   └── app_icon.ico          # 编译期生成的 Windows 多尺寸图标
├── backups/                  # 运行时自动热备份生成的数据库快照与 CSV 报表
├── logs/                     # 按天滚动的结构化运行诊断日志
└── src/
    ├── main.rs               # 主应用入口、自适应窗口配置、托盘指令轮询
    ├── logger.rs             # tracing 结构化滚动日志系统
    ├── tray.rs               # 原生系统托盘守护与右键菜单调度
    ├── state.rs              # AppState 响应式全局状态模型
    ├── db.rs                 # SQLite 存储、热备份快照、CSV 导出与完整性校验
    ├── theme.rs              # 设计令牌与主题配色 (深色 / 浅色 / 随系统)
    ├── icons.rs              # 复合图标资产加载源 (AppAssets)
    ├── models.rs             # 核心数据模型
    ├── services/
    │   ├── auth.rs           # Argon2id 密码哈希与离线认证服务
    │   └── async_worker.rs   # Tokio 多线程后台异步工作池
    ├── components/
    │   ├── menubar.rs        # 顶部桌面应用菜单栏 (未登录/登录权限隔离)
    │   ├── header.rs         # 顶部操作工具栏 (搜索、全屏、主题切换)
    │   ├── sidebar.rs        # 左侧常驻导航栏
    │   ├── statusbar.rs      # 底部常驻状态栏
    │   ├── user_modal.rs     # 新增/编辑成员复杂模态表单
    │   └── about_modal.rs    # 关于软件模态对话框
    └── views/
        ├── login.rs          # 独立安全登录页
        ├── dashboard.rs      # 数据大屏与拓扑视图
        ├── users.rs          # 用户账户与组织权限
        ├── monitoring.rs     # 集群监控与实时日志流
        ├── audit_logs.rs     # 合规审计日志
        └── settings.rs       # 系统环境与安全配置
```
