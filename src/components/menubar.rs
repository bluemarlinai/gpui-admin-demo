use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::{NavigationTab, ThemeMode, ToastKind};
use crate::theme::*;

pub struct MenuBar;

impl MenuBar {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let active_menu = app.state.active_menu;
        let is_auth = app.state.is_authenticated;

        // 未登录时仅提供“帮助”菜单；登录后提供完整桌面功能菜单
        let menus: &[(&str, &str)] = if is_auth {
            &[
                ("file", "文件与数据 (F)"),
                ("view", "视图导航 (V)"),
                ("ops", "运维工具 (T)"),
                ("help", "帮助 (H)"),
            ]
        } else {
            &[("help", "帮助 (H)")]
        };

        div()
            .h(px(30.0))
            .w_full()
            .bg(theme.bg_sidebar)
            .border_b_1()
            .border_color(theme.border)
            .px(px(10.0))
            .flex()
            .items_center()
            .justify_start()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .children(menus.iter().map(|(id, label)| {
                        let is_open = active_menu == Some(*id);
                        let id_str = *id;
                        div()
                            .id(SharedString::from(format!("menubar-item-{id}")))
                            .px_2p5()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(if is_open { PRIMARY_BLUE } else { Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 } })
                            .hover(move |s| s.bg(if is_open { PRIMARY_BLUE } else { theme.bg_hover }))
                            .text_xs()
                            .text_color(if is_open { c(0xffffff) } else { theme.text_secondary })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if this.state.active_menu == Some(id_str) {
                                    this.state.active_menu = None;
                                } else {
                                    this.state.active_menu = Some(id_str);
                                }
                                cx.notify();
                            }))
                            .child(*label)
                    }))
            )
    }

    /// 渲染当前展开的下拉菜单浮层
    pub fn render_dropdown_overlay(
        app: &AdminApp,
        theme: ThemePalette,
        cx: &mut Context<AdminApp>,
    ) -> Option<AnyElement> {
        let menu_id = app.state.active_menu?;
        let is_auth = app.state.is_authenticated;

        let left_offset = if !is_auth {
            px(10.0)
        } else {
            match menu_id {
                "file" => px(10.0),
                "view" => px(116.0),
                "ops" => px(212.0),
                "help" => px(308.0),
                _ => px(10.0),
            }
        };

        let dropdown_content = match menu_id {
            "file" => div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(Self::menu_row(
                    "menu-file-backup",
                    "icons/save.svg",
                    "立即执行数据热备份",
                    "Ctrl+B",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.trigger_data_backup(cx);
                    }),
                ))
                .child(Self::menu_row(
                    "menu-file-export-csv",
                    "icons/download.svg",
                    "导出合规审计报表 (CSV)",
                    "Ctrl+E",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.trigger_csv_export(cx);
                    }),
                ))
                .child(Self::menu_row(
                    "menu-file-new-user",
                    "icons/person_add.svg",
                    "新增组织成员档案...",
                    "Ctrl+N",
                    theme,
                    cx.listener(|this, _, window, cx| {
                        this.state.active_menu = None;
                        this.form_name_input.update(cx, |s, cx| s.set_value("", window, cx));
                        this.form_email_input.update(cx, |s, cx| s.set_value("", window, cx));
                        this.editing_user_id = None;
                        this.state.show_user_modal = true;
                        cx.notify();
                    }),
                ))
                .child(div().h(px(1.0)).my_1().bg(theme.border))
                .child(Self::menu_row(
                    "menu-file-logout",
                    "icons/lock.svg",
                    "注销当前登录会话",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.logout();
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-file-quit",
                    "icons/close.svg",
                    "安全退出系统",
                    "Alt+F4",
                    theme,
                    cx.listener(|_, _, _, cx| {
                        cx.quit();
                    }),
                )),
            "view" => div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(Self::menu_row(
                    "menu-view-dash",
                    "icons/dashboard.svg",
                    "仪表盘大屏概览",
                    "Alt+1",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.current_tab = NavigationTab::Dashboard;
                        this.state.active_menu = None;
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-view-users",
                    "icons/people.svg",
                    "用户账户与组织架构",
                    "Alt+2",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.current_tab = NavigationTab::Users;
                        this.state.active_menu = None;
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-view-mon",
                    "icons/dns.svg",
                    "集群服务与算力拓扑",
                    "Alt+3",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.current_tab = NavigationTab::Monitoring;
                        this.state.active_menu = None;
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-view-audit",
                    "icons/assignment.svg",
                    "操作与合规审计日志",
                    "Alt+4",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.current_tab = NavigationTab::AuditLogs;
                        this.state.active_menu = None;
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-view-settings",
                    "icons/settings.svg",
                    "系统运行环境与安全策略",
                    "Alt+5",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.current_tab = NavigationTab::Settings;
                        this.state.active_menu = None;
                        cx.notify();
                    }),
                ))
                .child(div().h(px(1.0)).my_1().bg(theme.border))
                .child(Self::menu_row(
                    "menu-view-fullscreen",
                    "icons/computer.svg",
                    "切换全屏显示模式",
                    "F11",
                    theme,
                    cx.listener(|this, _, window, cx| {
                        this.state.active_menu = None;
                        window.toggle_fullscreen();
                        cx.notify();
                    }),
                )),
            "ops" => div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(Self::menu_row(
                    "menu-ops-integrity",
                    "icons/shield.svg",
                    "存储引擎完整性自检",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.trigger_integrity_check(cx);
                    }),
                ))
                .child(Self::menu_row(
                    "menu-ops-refresh",
                    "icons/refresh.svg",
                    "同步刷新全网监控指标",
                    "F5",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.reload_from_db();
                        this.state.set_toast("所有集群监控与业务数据已同步刷新", ToastKind::Success);
                        cx.notify();
                    }),
                ))
                .child(div().h(px(1.0)).my_1().bg(theme.border))
                .child(Self::menu_row(
                    "menu-ops-theme-dark",
                    "icons/dark_mode.svg",
                    "切换深色主题模式",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.settings.theme_mode = ThemeMode::Dark;
                        this.db.save_settings(&this.settings);
                        cx.notify();
                    }),
                ))
                .child(Self::menu_row(
                    "menu-ops-theme-light",
                    "icons/light_mode.svg",
                    "切换浅色主题模式",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.settings.theme_mode = ThemeMode::Light;
                        this.db.save_settings(&this.settings);
                        cx.notify();
                    }),
                )),
            _ => div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(Self::menu_row(
                    "menu-help-diag",
                    "icons/terminal.svg",
                    "查看运行诊断日志状态",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.state.set_toast(
                            "运行诊断日志正实时写入 logs/novaops.log",
                            ToastKind::Info,
                        );
                        cx.notify();
                    }),
                ))
                .child(div().h(px(1.0)).my_1().bg(theme.border))
                .child(Self::menu_row(
                    "menu-help-about",
                    "icons/novaops_logo.svg",
                    "关于 NovaOps Pro",
                    "",
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.state.active_menu = None;
                        this.state.show_about_modal = true;
                        cx.notify();
                    }),
                )),
        };

        Some(
            div()
                .id("menubar-dropdown-backdrop")
                .occlude()
                .absolute()
                .inset_0()
                .top(px(30.0))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.state.active_menu = None;
                    cx.notify();
                }))
                .child(
                    div()
                        .absolute()
                        .top(px(2.0))
                        .left(left_offset)
                        .w(px(240.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .rounded_xl()
                        .shadow_xl()
                        .p(px(6.0))
                        .child(dropdown_content),
                )
                .into_any_element(),
        )
    }

    fn menu_row(
        id: &'static str,
        icon_path: &'static str,
        label: &'static str,
        shortcut: &'static str,
        theme: ThemePalette,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .id(id)
            .flex()
            .items_center()
            .justify_between()
            .px_2p5()
            .py_1p5()
            .rounded_lg()
            .cursor_pointer()
            .hover(move |s| s.bg(theme.bg_hover))
            .on_click(on_click)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(m_icon(icon_path, 14.0, theme.text_secondary))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(label),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(shortcut),
            )
    }
}
