use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::theme::*;

pub struct AuditLogsView;

impl AuditLogsView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let logs = &app.audit_logs;

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            // 过滤与导出
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .bg(theme.bg_card)
                    .p(px(16.0))
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .id("filter-log-all")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .bg(PRIMARY_BLUE)
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(c(0xffffff))
                                    .child(format!("全部审计记录 ({} 条)", logs.len()))
                            )
                            .child(
                                div()
                                    .id("filter-log-high")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .bg(theme.bg_input)
                                    .hover(move |s| s.bg(theme.bg_card_alt))
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("仅高危变更")
                            )
                            .child(
                                div()
                                    .id("filter-log-compliance")
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .bg(theme.bg_input)
                                    .hover(move |s| s.bg(theme.bg_card_alt))
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child("安全合规归档")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(theme.bg_input)
                                    .border_1()
                                    .border_color(theme.border)
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child(m_icon("icons/shield.svg", 14.0, ACCENT_CYAN))
                                    .child("合规等级: ISO 27001")
                            )
                            .child(
                                div()
                                    .id("export-csv-btn")
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(theme.bg_input)
                                    .hover(move |s| s.bg(theme.bg_card_alt))
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.trigger_csv_export(cx);
                                    }))
                                    .child(m_icon("icons/download.svg", 14.0, PRIMARY_BLUE))
                                    .child("导出 CSV 存证")
                            )
                    )
            )
            // 审计表格
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(theme.bg_card)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border)
                    .p(px(16.0))
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py_3()
                            .rounded_lg()
                            .bg(theme.bg_input)
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_secondary)
                            .child(div().w(px(70.0)).child("事件 ID"))
                            .child(div().w(px(130.0)).child("操作员"))
                            .child(div().w(px(160.0)).child("执行动作"))
                            .child(div().w(px(200.0)).child("目标对象 / 资源"))
                            .child(div().w(px(130.0)).child("来源 IP"))
                            .child(div().w(px(100.0)).child("风险级别"))
                            .child(div().flex_1().child("发生时间"))
                    )
                    .children(logs.iter().map(|log| {
                        let (badge_color, badge_bg) = match log.level.as_str() {
                            "INFO" => (ACCENT_EMERALD, theme.badge_success_bg),
                            "WARN" => (ACCENT_AMBER, theme.badge_warn_bg),
                            "CRITICAL" => (ACCENT_ROSE, theme.badge_danger_bg),
                            _ => (PRIMARY_BLUE, theme.badge_info_bg),
                        };

                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py_3()
                            .rounded_lg()
                            .hover(move |s| s.bg(theme.bg_hover))
                            .text_xs()
                            .child(div().w(px(70.0)).text_color(theme.text_muted).child(format!("#{:05}", log.id)))
                            .child(div().w(px(130.0)).font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child(log.operator.clone()))
                            .child(div().w(px(160.0)).text_color(ACCENT_CYAN).child(log.action.clone()))
                            .child(div().w(px(200.0)).text_color(theme.text_secondary).child(log.target.clone()))
                            .child(div().w(px(130.0)).text_color(theme.text_muted).child(log.ip.clone()))
                            .child(
                                div()
                                    .w(px(100.0))
                                    .px_2()
                                    .py_0p5()
                                    .rounded_full()
                                    .bg(badge_bg)
                                    .text_color(badge_color)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(log.level.clone())
                            )
                            .child(div().flex_1().text_color(theme.text_secondary).child(log.time.clone()))
                    }))
            )
    }
}
