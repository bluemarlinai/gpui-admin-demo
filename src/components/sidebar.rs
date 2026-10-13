use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::NavigationTab;
use crate::theme::*;

pub struct Sidebar;

impl Sidebar {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let current_tab = app.current_tab;
        let nav_items = [
            (NavigationTab::Dashboard, "icons/dashboard.svg", "仪表盘大屏", None),
            (NavigationTab::Users, "icons/people.svg", "用户与权限", Some("5在线")),
            (NavigationTab::Monitoring, "icons/dns.svg", "集群服务监控", Some("优良")),
            (NavigationTab::AuditLogs, "icons/assignment.svg", "操作审计日志", None),
            (NavigationTab::Settings, "icons/settings.svg", "系统环境配置", None),
        ];

        div()
            .w(px(224.0))
            .flex_shrink_0()
            .h_full()
            .bg(theme.bg_sidebar)
            .border_r_1()
            .border_color(theme.border)
            .flex()
            .flex_col()
            .justify_between()
            .p(px(16.0))
            // 顶部 Logo 与系统标识
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .size(px(38.0))
                                    .rounded_xl()
                                    .bg(PRIMARY_BLUE)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(m_icon("icons/novaops_logo.svg", 22.0, c(0xffffff)))
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_sm()
                                                    .text_color(theme.text_primary)
                                                    .child("NovaOps Pro")
                                            )
                                            .child(
                                                div()
                                                    .px_1p5()
                                                    .py_0p5()
                                                    .rounded(px(4.0))
                                                    .bg(theme.bg_input)
                                                    .text_xs()
                                                    .text_color(ACCENT_INDIGO)
                                                    .child("v2.4")
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("企业级云管理中台")
                                    )
                            )
                    )
                    // 导航菜单列表
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child("功能模块")
                            )
                            .children(nav_items.into_iter().map(|(tab, icon_path, label, badge)| {
                                let is_active = current_tab == tab;
                                let tab_id = format!("nav-tab-{:?}", tab);
                                let icon_color = if is_active { c(0xffffff) } else { theme.text_secondary };

                                div()
                                    .id(SharedString::from(tab_id))
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .px_3()
                                    .py_2p5()
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .bg(if is_active { PRIMARY_BLUE } else { Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 } })
                                    .hover(move |s| s.bg(if is_active { PRIMARY_BLUE_HOVER } else { theme.bg_hover }))
                                    .on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                                        this.current_tab = tab;
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2p5()
                                            .child(m_icon(icon_path, 16.0, icon_color))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(icon_color)
                                                    .child(label)
                                            )
                                    )
                                    .children(badge.map(|b| {
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_full()
                                            .bg(if is_active { c(0x1d4ed8) } else { theme.bg_input })
                                            .text_xs()
                                            .text_color(if is_active { c(0xffffff) } else { ACCENT_CYAN })
                                            .child(b)
                                    }))
                            }))
                    )
            )
            // 底部集群状态与当前操作人卡片
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    // 集群小卡片
                    .child(
                        div()
                            .bg(theme.bg_card_alt)
                            .border_1()
                            .border_color(theme.border)
                            .rounded_lg()
                            .p(px(10.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_xs()
                                    .child(div().text_color(theme.text_muted).child("主集群"))
                                    .child(div().text_color(ACCENT_EMERALD).child("● cn-east-01"))
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_xs()
                                    .child(div().text_color(theme.text_muted).child("当前主题"))
                                    .child(
                                        div()
                                            .text_color(theme.text_primary)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(format!(
                                                "{} ({})",
                                                app.settings.theme_mode.label(),
                                                if theme.is_dark { "暗色" } else { "亮色" }
                                            ))
                                    )
                            )
                    )
                    // 当前登录用户卡片与登出操作
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(10.0))
                            .rounded_xl()
                            .bg(theme.bg_card_alt)
                            .border_1()
                            .border_color(theme.border)
                            .child(
                                div()
                                    .flex()
                                    .gap(px(10.0))
                                    .items_center()
                                    .child(
                                        div()
                                            .size(px(34.0))
                                            .rounded_full()
                                            .bg(ACCENT_INDIGO)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(c(0xffffff))
                                            .child(
                                                app.state
                                                    .current_user
                                                    .as_ref()
                                                    .map(|u| u.avatar_abbr.clone())
                                                    .unwrap_or_else(|| "SA".to_string())
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(
                                                        app.state
                                                            .current_user
                                                            .as_ref()
                                                            .map(|u| u.display_name.clone())
                                                            .unwrap_or_else(|| "系统管理员".to_string())
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(ACCENT_EMERALD)
                                                    .child(format!(
                                                        "● 在线 · {}",
                                                        app.state
                                                            .current_user
                                                            .as_ref()
                                                            .map(|u| u.role.clone())
                                                            .unwrap_or_else(|| "超级管理员".to_string())
                                                    ))
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .id("sidebar-logout-btn")
                                    .cursor_pointer()
                                    .px_2()
                                    .py_1()
                                    .rounded_lg()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .hover(|s| s.bg(theme.bg_input).text_color(rgb(0xef4444)))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.logout();
                                        cx.notify();
                                    }))
                                    .child("退出")
                            )
                    )
            )
    }
}
