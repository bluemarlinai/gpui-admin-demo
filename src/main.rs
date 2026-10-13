#![cfg_attr(not(test), windows_subsystem = "windows")]

pub mod logger;
pub mod services;
pub mod state;
pub mod models;
pub mod theme;
pub mod icons;
pub mod db;
pub mod tray;
pub mod components;
pub mod views;

use std::time::Duration;
use gpui::*;
use gpui_kit::component::input::InputState;
use models::*;
use theme::*;
use icons::{m_icon, AppAssets};
use db::AppDatabase;
use state::AppState;
use components::sidebar::Sidebar;
use components::header::Header;
use components::user_modal::UserModal;
use components::menubar::MenuBar;
use components::about_modal::AboutModal;
use components::statusbar::StatusBar;
use views::dashboard::DashboardView;
use views::users::UsersView;
use views::monitoring::MonitoringView;
use views::audit_logs::AuditLogsView;
use views::settings::SettingsView;
use views::login::LoginView;

pub struct AdminApp {
    pub db: AppDatabase,
    pub state: AppState,
    pub current_tab: NavigationTab,
    pub selected_period: &'static str,
    pub selected_role: &'static str,
    pub notifications_count: usize,
    pub kpi_cards: Vec<KpiCardItem>,
    pub traffic_bars: Vec<TrafficBarItem>,
    pub resource_gauges: Vec<ResourceGaugeItem>,
    pub services: Vec<ServiceHealthItem>,
    pub live_events: Vec<LiveEventItem>,
    pub users: Vec<UserItem>,
    pub nodes: Vec<NodeMetric>,
    pub terminal_logs: Vec<TerminalLogItem>,
    pub audit_logs: Vec<AuditLogItem>,
    pub settings: SystemSettings,
    pub toast: Option<Toast>,

    // 登录真实 InputState 实体与受控 Checkbox 状态
    pub login_username_input: Entity<InputState>,
    pub login_password_input: Entity<InputState>,
    pub remember_login: bool,

    // 用户新增/编辑弹窗真实 InputState 实体与受控状态
    pub form_name_input: Entity<InputState>,
    pub form_email_input: Entity<InputState>,
    pub form_role: String,
    pub form_dept: String,
    pub form_status: UserStatus,
    pub form_perm_read: bool,
    pub form_perm_audit: bool,
    pub form_perm_deploy: bool,
    pub form_perm_secret: bool,
    pub editing_user_id: Option<u32>,
}

impl AdminApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let db = AppDatabase::new();
        let selected_period = "7天";
        let kpi_cards = db.load_kpi_cards();
        let traffic_bars = db.load_traffic_trends(selected_period);
        let resource_gauges = db.load_resource_gauges();
        let services = db.load_service_health();
        let live_events = db.load_live_events();
        let users = db.load_users();
        let nodes = db.load_nodes();
        let terminal_logs = db.load_terminal_logs();
        let audit_logs = db.load_audit_logs();
        let settings = db.load_settings();

        let login_username_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("请输入管理员账号")
        });

        let login_password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("请输入登录密码")
                .masked(true)
        });

        let form_name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("请输入成员姓名")
        });

        let form_email_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("请输入企业邮箱，例如：name@novaops.io")
        });

        // 启动后台托盘事件轮询协程，响应系统托盘右键菜单指令（备份、关于、退出）
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            loop {
                cx.background_executor().timer(Duration::from_millis(250)).await;
                let cmd = tray::take_tray_command();
                if cmd == tray::TRAY_CMD_NONE {
                    continue;
                }
                let should_continue = this
                    .update(cx, |app, cx| {
                        match cmd {
                            tray::TRAY_CMD_BACKUP => {
                                app.trigger_data_backup(cx);
                            }
                            tray::TRAY_CMD_ABOUT => {
                                app.state.show_about_modal = true;
                                cx.notify();
                            }
                            tray::TRAY_CMD_QUIT => {
                                cx.quit();
                            }
                            _ => {}
                        }
                    })
                    .is_ok();
                if !should_continue {
                    break;
                }
            }
        })
        .detach();

        Self {
            db,
            state: AppState::default(),
            current_tab: NavigationTab::Dashboard,
            selected_period,
            selected_role: "全部角色",
            notifications_count: 3,
            kpi_cards,
            traffic_bars,
            resource_gauges,
            services,
            live_events,
            users,
            nodes,
            terminal_logs,
            audit_logs,
            settings,
            toast: None,
            login_username_input,
            login_password_input,
            remember_login: true,
            form_name_input,
            form_email_input,
            form_role: "研发工程师".to_string(),
            form_dept: "基础设施架构部".to_string(),
            form_status: UserStatus::Active,
            form_perm_read: true,
            form_perm_audit: false,
            form_perm_deploy: false,
            form_perm_secret: false,
            editing_user_id: None,
        }
    }

    pub fn reload_from_db(&mut self) {
        self.kpi_cards = self.db.load_kpi_cards();
        self.traffic_bars = self.db.load_traffic_trends(self.selected_period);
        self.resource_gauges = self.db.load_resource_gauges();
        self.services = self.db.load_service_health();
        self.live_events = self.db.load_live_events();
        self.users = self.db.load_users();
        self.nodes = self.db.load_nodes();
        self.terminal_logs = self.db.load_terminal_logs();
        self.audit_logs = self.db.load_audit_logs();
        self.settings = self.db.load_settings();
    }

    /// 执行全量数据热备份并更新状态栏与审计记录
    pub fn trigger_data_backup(&mut self, cx: &mut Context<Self>) {
        match self.db.create_backup() {
            Ok((backup_path, time_str, size_bytes)) => {
                let kb = (size_bytes as f64 / 1024.0).max(1.0);
                self.state.last_backup_time = format!("{time_str} ({kb:.1} KB)");
                self.state.last_backup_file = Some(backup_path.clone());
                self.db.insert_audit_log(
                    "系统管理员",
                    "执行全量数据热备份快照",
                    &backup_path,
                    "INFO",
                );
                self.reload_from_db();
                self.state.set_toast(
                    format!("数据热备份已完成：{backup_path} ({kb:.1} KB)"),
                    ToastKind::Success,
                );
                tracing::info!("数据热备份成功: {} ({} bytes)", backup_path, size_bytes);
            }
            Err(err) => {
                self.state.set_toast(format!("数据备份失败: {err}"), ToastKind::Warning);
                tracing::error!("数据热备份失败: {}", err);
            }
        }
        cx.notify();
    }

    /// 导出合规审计日志至 CSV
    pub fn trigger_csv_export(&mut self, cx: &mut Context<Self>) {
        match self.db.export_audit_logs_csv() {
            Ok(csv_path) => {
                self.state.set_toast(
                    format!("合规审计报表已导出至：{csv_path}"),
                    ToastKind::Success,
                );
                tracing::info!("导出审计 CSV 成功: {}", csv_path);
            }
            Err(err) => {
                self.state.set_toast(format!("导出报表失败: {err}"), ToastKind::Warning);
            }
        }
        cx.notify();
    }

    /// 执行存储完整性校验
    pub fn trigger_integrity_check(&mut self, cx: &mut Context<Self>) {
        match self.db.check_integrity() {
            Ok(status) => {
                self.state.set_toast(
                    format!("存储引擎完整性自检通过 (状态: {status})"),
                    ToastKind::Success,
                );
            }
            Err(err) => {
                self.state.set_toast(format!("自检发现异常: {err}"), ToastKind::Warning);
            }
        }
        cx.notify();
    }
}

impl Render for AdminApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = ThemePalette::resolve(self.settings.theme_mode, window.appearance());

        // 同步 gpui-kit 组件库的深色/浅色主题状态
        let target_kit_mode = if theme.is_dark {
            gpui_kit::component::theme::ThemeMode::Dark
        } else {
            gpui_kit::component::theme::ThemeMode::Light
        };
        if gpui_kit::component::theme::Theme::global(cx).mode != target_kit_mode {
            gpui_kit::component::theme::Theme::change(target_kit_mode, Some(window), cx);
        }

        let current_tab = self.current_tab;
        let toast = self.state.toast.clone().or_else(|| self.toast.clone());
        let show_user_modal = self.state.show_user_modal;
        let show_about_modal = self.state.show_about_modal;
        let is_authenticated = self.state.is_authenticated;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_app)
            .text_color(theme.text_primary)
            .relative()
            // 1. 顶部桌面级应用菜单栏 (无论登录前或登录后均常驻显示)
            .child(MenuBar::render(self, theme, cx))
            // 2. 中间核心区域：未登录时显示安全登录视图，已登录时显示完整控制台工作台
            .child(if !is_authenticated {
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(LoginView::render(self, theme, cx))
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    // 左侧常驻导航栏
                    .child(Sidebar::render(self, theme, cx))
                    // 右侧主工作区
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            // 顶部 Header
                            .child(Header::render(self, window, theme, cx))
                            // 核心内容展示区
                            .child(
                                div()
                                    .id("main-content-scroll")
                                    .flex_1()
                                    .p(px(20.0))
                                    .overflow_y_scroll()
                                    .child(match current_tab {
                                        NavigationTab::Dashboard => DashboardView::render(self, theme, cx).into_any_element(),
                                        NavigationTab::Users => UsersView::render(self, theme, cx).into_any_element(),
                                        NavigationTab::Monitoring => MonitoringView::render(self, theme, cx).into_any_element(),
                                        NavigationTab::AuditLogs => AuditLogsView::render(self, theme, cx).into_any_element(),
                                        NavigationTab::Settings => SettingsView::render(self, theme, cx).into_any_element(),
                                    })
                            )
                    )
                    .into_any_element()
            })
            // 3. 底部常驻状态栏 (StatusBar)
            .child(StatusBar::render(self, theme, cx))
            // 4. 顶层菜单下拉浮层
            .children(MenuBar::render_dropdown_overlay(self, theme, cx))
            // 5. 用户新增/编辑复杂表单模态对话框
            .children(if show_user_modal {
                Some(UserModal::render(self, theme, cx))
            } else {
                None
            })
            // 6. 「关于 NovaOps Pro」模态对话框
            .children(if show_about_modal {
                Some(AboutModal::render(self, theme, cx))
            } else {
                None
            })
            // 7. 全局底部浮动 Toast 提示消息
            .children(toast.map(|t| {
                let (border_color, icon_color, icon_path) = match t.kind {
                    ToastKind::Success => (ACCENT_EMERALD, ACCENT_EMERALD, "icons/check_circle.svg"),
                    ToastKind::Info => (PRIMARY_BLUE, PRIMARY_BLUE, "icons/info.svg"),
                    ToastKind::Warning => (ACCENT_AMBER, ACCENT_AMBER, "icons/warning.svg"),
                };

                div()
                    .absolute()
                    .bottom(px(38.0))
                    .right(px(24.0))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .rounded_xl()
                    .bg(theme.bg_card_alt)
                    .border_1()
                    .border_color(border_color)
                    .shadow_lg()
                    .child(m_icon(icon_path, 16.0, icon_color))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .child(t.message)
                    )
                    .child(
                        div()
                            .id("dismiss-toast-btn")
                            .ml_2()
                            .px_1p5()
                            .py_0p5()
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .hover(move |s| s.bg(theme.bg_hover))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toast = None;
                                this.state.toast = None;
                                cx.notify();
                            }))
                            .child(m_icon("icons/close.svg", 14.0, theme.text_muted))
                    )
            }))
            .into_any_element()
    }
}

fn main() {
    // 1. 初始化滚动日志文件与终端 Tracing 追踪
    let _log_guard = logger::init_logger();

    // 2. 预热后台 Tokio 多线程异步运行时
    let _rt = services::async_worker::get_tokio_runtime();

    // 3. 在 Windows 平台使用标准原生标题栏时，禁用 DirectComposition 全窗口目标偏移，
    //    确保 DXGI SwapChain 严格绑定在客户区 (ClientRect) 内，最大化时底部绝不被任务栏遮挡
    #[cfg(target_os = "windows")]
    if std::env::var_os("GPUI_DISABLE_DIRECT_COMPOSITION").is_none() {
        unsafe {
            std::env::set_var("GPUI_DISABLE_DIRECT_COMPOSITION", "1");
        }
    }

    // 4. 启动原生系统托盘图标与窗口品牌图标注入线程
    tray::init_system_tray();

    // 5. 启动 GPUI Kit 桌面应用并挂载复合 SVG 资产库
    gpui_kit::application()
        .with_assets(AppAssets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);

            // 基于主显示器的可用工作区 (visible_bounds，已扣除任务栏区域) 动态计算合适的初始窗口大小
            let bounds = if let Some(display) = cx.primary_display() {
                let work_area = display.visible_bounds();
                let max_w = f32::from(work_area.size.width) * 0.88;
                let max_h = f32::from(work_area.size.height) * 0.85;
                let target_w = 1240.0_f32.min(max_w).max(1020.0);
                let target_h = 720.0_f32.min(max_h).max(580.0);
                let win_size = size(px(target_w), px(target_h));
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

            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| AdminApp::new(window, cx))
            })
            .unwrap();
        });
}
