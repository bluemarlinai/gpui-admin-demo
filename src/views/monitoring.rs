use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::{NodeStatus, Toast, ToastKind};
use crate::theme::*;

pub struct MonitoringView;

impl MonitoringView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let nodes = &app.nodes;
        let terminal_logs = &app.terminal_logs;

        div()
            .flex()
            .flex_col()
            .gap(px(18.0))
            .w_full()
            // 节点网格卡片 (数据源: SQLite nodes 表)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(m_icon("icons/computer.svg", 18.0, PRIMARY_BLUE))
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child(format!("集群物理机与计算节点 ({} 个在线节点)", nodes.len()))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(PRIMARY_BLUE)
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(c(0xffffff))
                                            .child("全部节点")
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.bg_input)
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child("控制平面")
                                    )
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.bg_input)
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child("工作节点")
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(14.0))
                            .w_full()
                            .children(nodes.iter().map(|node| {
                                let (status_str, status_color, status_bg) = match node.status {
                                    NodeStatus::Healthy => ("正常", ACCENT_EMERALD, theme.badge_success_bg),
                                    NodeStatus::Busy => ("高负载", ACCENT_AMBER, theme.badge_warn_bg),
                                    NodeStatus::Warning => ("告警", ACCENT_ROSE, theme.badge_danger_bg),
                                };

                                div()
                                    .flex_1()
                                    .bg(theme.bg_card)
                                    .border_1()
                                    .border_color(theme.border)
                                    .rounded_xl()
                                    .p(px(16.0))
                                    .hover(move |s| s.bg(theme.bg_card_alt).border_color(status_color))
                                    .flex()
                                    .flex_col()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .items_center()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .child(div().font_weight(FontWeight::SEMIBOLD).text_xs().text_color(theme.text_primary).child(node.name.clone()))
                                                    .child(div().text_xs().text_color(theme.text_muted).child(node.ip.clone()))
                                            )
                                            .child(
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded_full()
                                                    .bg(status_bg)
                                                    .text_color(status_color)
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(status_str)
                                            )
                                    )
                                    .child(Self::node_metric_bar("CPU", node.cpu_pct, ACCENT_EMERALD, theme))
                                    .child(Self::node_metric_bar("内存", node.mem_pct, ACCENT_AMBER, theme))
                                    .child(Self::node_metric_bar("磁盘", node.disk_pct, PRIMARY_BLUE, theme))
                            }))
                    )
            )
            // 实时运维终端控制台 (数据源: SQLite terminal_logs 表)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(if theme.is_dark { c(0x0a0d14) } else { theme.bg_card })
                    .border_1()
                    .border_color(theme.border)
                    .rounded_xl()
                    .p(px(16.0))
                    .gap(px(12.0))
                    // 终端顶栏控制
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .items_center()
                                    .child(div().size(px(10.0)).rounded_full().bg(ACCENT_ROSE))
                                    .child(div().size(px(10.0)).rounded_full().bg(ACCENT_AMBER))
                                    .child(div().size(px(10.0)).rounded_full().bg(ACCENT_EMERALD))
                                    .child(m_icon("icons/terminal.svg", 16.0, ACCENT_CYAN))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child("集群实时调度与指标日志流")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .id("pause-stream-btn")
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2p5()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.bg_input)
                                            .cursor_pointer()
                                            .hover(move |s| s.bg(theme.bg_card_alt))
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.toast = Some(Toast {
                                                    message: "已暂停实时日志流滚动显示。".into(),
                                                    kind: ToastKind::Info,
                                                });
                                                cx.notify();
                                            }))
                                            .child(m_icon("icons/pause.svg", 13.0, theme.text_secondary))
                                            .child("暂停流")
                                    )
                                    .child(
                                        div()
                                            .id("clear-term-btn")
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2p5()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.bg_input)
                                            .cursor_pointer()
                                            .hover(move |s| s.bg(theme.bg_card_alt))
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.reload_from_db();
                                                this.toast = Some(Toast {
                                                    message: "已重新拉取最新集群终端日志输出".into(),
                                                    kind: ToastKind::Info,
                                                });
                                                cx.notify();
                                            }))
                                            .child(m_icon("icons/refresh.svg", 13.0, theme.text_secondary))
                                            .child("重载日志")
                                    )
                                    .child(
                                        div()
                                            .id("export-diag-btn")
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2p5()
                                            .py_1()
                                            .rounded_md()
                                            .bg(PRIMARY_BLUE)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(PRIMARY_BLUE_HOVER))
                                            .text_xs()
                                            .text_color(c(0xffffff))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.toast = Some(Toast {
                                                    message: "已生成并开始下载集群诊断数据压缩包 (tar.gz)...".into(),
                                                    kind: ToastKind::Success,
                                                });
                                                cx.notify();
                                            }))
                                            .child(m_icon("icons/download.svg", 13.0, c(0xffffff)))
                                            .child("导出诊断包")
                                    )
                            )
                    )
                    // 终端日志行 (保持深色黑客风输出区以保证高亮日志清晰可读)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(12.0))
                            .bg(c(0x0b0f19))
                            .rounded_lg()
                            .border_1()
                            .border_color(c(0x1e293b))
                            .gap(px(6.0))
                            .children(terminal_logs.iter().map(|item| {
                                let color = c(item.color_hex);
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_xs()
                                    .child(div().text_color(c(0x64748b)).child(item.time.clone()))
                                    .child(
                                        div()
                                            .px_1p5()
                                            .py_0p5()
                                            .rounded(px(4.0))
                                            .bg(c(0x161e2e))
                                            .text_color(color)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(item.level.clone())
                                    )
                                    .child(div().text_color(ACCENT_CYAN).child(format!("[{}]", item.src)))
                                    .child(div().text_color(c(0xf1f5f9)).child(item.msg.clone()))
                            }))
                    )
            )
    }

    fn node_metric_bar(label: &'static str, pct: u8, color: Rgba, theme: ThemePalette) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_0p5()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .child(div().text_color(theme.text_secondary).child(label))
                    .child(div().text_color(theme.text_primary).font_weight(FontWeight::MEDIUM).child(format!("{}%", pct)))
            )
            .child(
                div()
                    .w_full()
                    .h(px(5.0))
                    .rounded_full()
                    .bg(theme.bg_input)
                    .child(
                        div()
                            .h_full()
                            .w(px((pct as f32) * 1.5))
                            .rounded_full()
                            .bg(color)
                    )
            )
    }
}
