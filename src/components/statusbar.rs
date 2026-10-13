use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::theme::*;

pub struct StatusBar;

impl StatusBar {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let online_nodes = app.nodes.len();
        let total_users = app.users.len();
        let last_backup = app.state.last_backup_time.clone();
        let current_operator = app
            .state
            .current_user
            .as_ref()
            .map(|u| format!("{} ({})", u.display_name, u.role))
            .unwrap_or_else(|| "系统管理员".to_string());

        div()
            .h(px(26.0))
            .w_full()
            .bg(theme.bg_sidebar)
            .border_t_1()
            .border_color(theme.border)
            .px(px(14.0))
            .flex()
            .items_center()
            .justify_between()
            .text_xs()
            // 左侧状态指标组
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.0))
                    // 1. 核心连接指示
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .size(px(7.0))
                                    .rounded_full()
                                    .bg(ACCENT_EMERALD)
                            )
                            .child(
                                div()
                                    .text_color(theme.text_secondary)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("控制面就绪")
                            )
                    )
                    .child(div().text_color(theme.border).child("|"))
                    // 2. 集群节点与用户规模
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_color(theme.text_muted)
                            .child(m_icon("icons/dns.svg", 12.0, ACCENT_CYAN))
                            .child(format!("在线节点: {online_nodes} 台 · 组织成员: {total_users} 人"))
                    )
                    .child(div().text_color(theme.border).child("|"))
                    // 3. 数据备份状态 (点击可立即触发一次备份)
                    .child(
                        div()
                            .id("statusbar-backup-trigger")
                            .flex()
                            .items_center()
                            .gap_1()
                            .cursor_pointer()
                            .px_1p5()
                            .py_0p5()
                            .rounded(px(4.0))
                            .hover(move |s| s.bg(theme.bg_hover))
                            .text_color(theme.text_secondary)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.trigger_data_backup(cx);
                            }))
                            .child(m_icon("icons/save.svg", 12.0, ACCENT_EMERALD))
                            .child(format!("数据快照: {last_backup} (点击备份)"))
                    )
            )
            // 右侧环境与版本信息组
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.0))
                    // 当前操作员
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_color(theme.text_muted)
                            .child(m_icon("icons/people.svg", 12.0, theme.text_muted))
                            .child(format!("操作员: {current_operator}"))
                    )
                    .child(div().text_color(theme.border).child("|"))
                    // 安全合规指示
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_color(theme.text_muted)
                            .child(m_icon("icons/shield.svg", 12.0, PRIMARY_BLUE))
                            .child(if app.settings.two_factor_auth {
                                "2FA 已启用 · 等保三级"
                            } else {
                                "基础鉴权 · 等保三级"
                            })
                    )
                    .child(div().text_color(theme.border).child("|"))
                    // 版本号 (点击打开关于弹窗)
                    .child(
                        div()
                            .id("statusbar-version-about")
                            .cursor_pointer()
                            .px_1p5()
                            .py_0p5()
                            .rounded(px(4.0))
                            .hover(move |s| s.bg(theme.bg_hover))
                            .text_color(PRIMARY_BLUE)
                            .font_weight(FontWeight::MEDIUM)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.state.show_about_modal = true;
                                cx.notify();
                            }))
                            .child("v1.2.0-Ent")
                    )
            )
    }
}
