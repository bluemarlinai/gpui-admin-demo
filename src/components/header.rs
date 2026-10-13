use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::{NavigationTab, ThemeMode, Toast, ToastKind};
use crate::theme::*;

pub struct Header;

impl Header {
    pub fn render(app: &AdminApp, window: &Window, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let current_tab = app.current_tab;
        let current_theme_mode = app.settings.theme_mode;
        let notif_count = app.notifications_count;

        let tab_name = match current_tab {
            NavigationTab::Dashboard => "仪表盘大屏概览",
            NavigationTab::Users => "用户账户与组织架构",
            NavigationTab::Monitoring => "集群服务与算力拓扑",
            NavigationTab::AuditLogs => "操作与合规审计日志",
            NavigationTab::Settings => "系统运行环境与安全策略",
        };

        let theme_options = [
            (ThemeMode::Light, "icons/light_mode.svg", "浅色"),
            (ThemeMode::Dark, "icons/dark_mode.svg", "深色"),
            (ThemeMode::System, "icons/brightness_auto.svg", "随系统"),
        ];

        div()
            .h(px(56.0))
            .w_full()
            .bg(theme.bg_header)
            .border_b_1()
            .border_color(theme.border)
            .px(px(18.0))
            .flex()
            .items_center()
            .justify_between()
            // 面包屑导航
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    .text_xs()
                    .child(
                        div()
                            .text_color(theme.text_muted)
                            .child("控制台")
                    )
                    .child(div().text_color(theme.border).child("/"))
                    .child(
                        div()
                            .text_color(theme.text_primary)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(tab_name)
                    )
            )
            // 右侧快捷工具栏 (搜索、主题切换、刷新、全屏、通知)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // 快速全局搜索输入框
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .w(px(165.0))
                            .px_2p5()
                            .py_1p5()
                            .rounded_lg()
                            .bg(theme.bg_input)
                            .border_1()
                            .border_color(theme.border)
                            .child(m_icon("icons/search.svg", 13.0, theme.text_muted))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("搜索服务或节点...")
                            )
                    )
                    // 浅色 / 深色 / 随系统 三段式主题切换器
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .p(px(3.0))
                            .gap_1()
                            .rounded_lg()
                            .bg(theme.bg_input)
                            .border_1()
                            .border_color(theme.border)
                            .children(theme_options.into_iter().map(|(mode, icon_path, label)| {
                                let is_selected = current_theme_mode == mode;
                                let fg = if is_selected { c(0xffffff) } else { theme.text_secondary };

                                div()
                                    .id(SharedString::from(format!("header-theme-{:?}", mode)))
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2p5()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(if is_selected { PRIMARY_BLUE } else { Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 } })
                                    .hover(move |s| s.bg(if is_selected { PRIMARY_BLUE_HOVER } else { theme.bg_hover }))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(fg)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.settings.theme_mode = mode;
                                        this.db.save_settings(&this.settings);
                                        this.toast = Some(Toast {
                                            message: format!("已切换界面主题为【{}】模式", mode.label()),
                                            kind: ToastKind::Success,
                                        });
                                        cx.notify();
                                    }))
                                    .child(m_icon(icon_path, 13.0, fg))
                                    .child(label)
                            }))
                    )
                    // 集群健康度绿色提示
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_2p5()
                            .py_1()
                            .rounded_full()
                            .bg(theme.badge_success_bg)
                            .text_xs()
                            .text_color(ACCENT_EMERALD)
                            .child(m_icon("icons/check_circle.svg", 13.0, ACCENT_EMERALD))
                            .child("SLA 99.99%")
                    )
                    // 刷新数据按钮
                    .child(
                        div()
                            .id("refresh-data-btn")
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_3()
                            .py_1p5()
                            .rounded_lg()
                            .cursor_pointer()
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border)
                            .hover(move |s| s.bg(theme.bg_hover).border_color(PRIMARY_BLUE))
                            .text_xs()
                            .text_color(theme.text_primary)
                            .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                this.reload_from_db();
                                this.toast = Some(Toast {
                                    message: "所有集群监控与业务数据已同步刷新".into(),
                                    kind: ToastKind::Success,
                                });
                                cx.notify();
                            }))
                            .child(m_icon("icons/refresh.svg", 14.0, PRIMARY_BLUE))
                            .child("刷新")
                    )
                    // 全屏切换按钮
                    .child(
                        div()
                            .id("fullscreen-toggle-btn")
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_3()
                            .py_1p5()
                            .rounded_lg()
                            .cursor_pointer()
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border)
                            .hover(move |s| s.bg(theme.bg_hover).border_color(PRIMARY_BLUE))
                            .text_xs()
                            .text_color(theme.text_primary)
                            .on_click(cx.listener(|_this, _event: &ClickEvent, window, cx| {
                                window.toggle_fullscreen();
                                cx.notify();
                            }))
                            .child(if window.is_fullscreen() { "退出全屏" } else { "全屏" })
                    )
                    // 通知铃铛
                    .child(
                        div()
                            .id("notif-btn")
                            .size(px(32.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .bg(theme.bg_card)
                            .border_1()
                            .border_color(theme.border)
                            .hover(move |s| s.bg(theme.bg_hover))
                            .flex()
                            .items_center()
                            .justify_center()
                            .relative()
                            .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                this.notifications_count = 0;
                                this.toast = Some(Toast {
                                    message: "已将所有系统待办与指标预警标记为已读。".into(),
                                    kind: ToastKind::Info,
                                });
                                cx.notify();
                            }))
                            .child(m_icon("icons/notifications.svg", 16.0, theme.text_secondary))
                            .children(if notif_count > 0 {
                                Some(
                                    div()
                                        .absolute()
                                        .top(px(4.0))
                                        .right(px(4.0))
                                        .size(px(7.0))
                                        .rounded_full()
                                        .bg(ACCENT_ROSE)
                                )
                            } else {
                                None
                            })
                    )
            )
    }
}
