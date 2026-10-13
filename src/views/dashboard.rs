use gpui::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::*;
use crate::theme::*;

pub struct DashboardView;

impl DashboardView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let selected_period = app.selected_period;

        div()
            .flex()
            .flex_col()
            .gap(px(20.0))
            .w_full()
            // 顶部统计指标卡片组 (从 SQLite kpi_cards 读取)
            .child(Self::render_stat_cards(&app.kpi_cards, theme))
            // 中部核心图表与资源水位 (从 SQLite traffic_trends 与 resource_gauges 读取)
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_traffic_chart(selected_period, &app.traffic_bars, theme, cx))
                    .child(Self::render_resource_gauges(&app.resource_gauges, theme))
            )
            // 底部服务健康度与最新审计告警 (从 SQLite service_health 与 live_events 读取)
            .child(
                div()
                    .flex()
                    .gap(px(16.0))
                    .w_full()
                    .child(Self::render_service_health_table(&app.services, theme))
                    .child(Self::render_live_activity_stream(&app.live_events, theme))
            )
    }

    fn render_stat_cards(kpis: &[KpiCardItem], theme: ThemePalette) -> impl IntoElement {
        div()
            .flex()
            .gap(px(16.0))
            .w_full()
            .children(kpis.iter().map(|item| {
                let accent = c(item.color_hex);
                let is_positive = item.is_positive;
                div()
                    .flex_1()
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_xl()
                    .p(px(18.0))
                    .hover(move |s| s.bg(theme.bg_card_alt).border_color(accent))
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
                                    .text_sm()
                                    .text_color(theme.text_secondary)
                                    .child(item.title.clone())
                            )
                            .child(
                                div()
                                    .size(px(34.0))
                                    .rounded_lg()
                                    .bg(theme.bg_input)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(m_icon(item.icon.clone(), 18.0, accent))
                            )
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(item.value.clone())
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .text_xs()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .text_color(if is_positive { ACCENT_EMERALD } else { ACCENT_AMBER })
                                    .child(item.trend.clone())
                            )
                            .child(
                                div()
                                    .text_color(theme.text_muted)
                                    .child(item.subtext.clone())
                            )
                    )
            }))
    }

    fn render_traffic_chart(
        selected_period: &'static str,
        bars: &[TrafficBarItem],
        theme: ThemePalette,
        cx: &mut Context<AdminApp>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p(px(20.0))
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0p5()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(m_icon("icons/trending_up.svg", 18.0, PRIMARY_BLUE))
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child("核心网关流量调用趋势")
                                    )
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("实时统计核心网关吞吐量与峰值调用走势")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .id("period-7d")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(if selected_period == "7天" { PRIMARY_BLUE } else { theme.bg_input })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if selected_period == "7天" { c(0xffffff) } else { theme.text_secondary })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.selected_period = "7天";
                                        this.traffic_bars = this.db.load_traffic_trends("7天");
                                        cx.notify();
                                    }))
                                    .child("7 天")
                            )
                            .child(
                                div()
                                    .id("period-30d")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(if selected_period == "30天" { PRIMARY_BLUE } else { theme.bg_input })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if selected_period == "30天" { c(0xffffff) } else { theme.text_secondary })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.selected_period = "30天";
                                        this.traffic_bars = this.db.load_traffic_trends("30天");
                                        cx.notify();
                                    }))
                                    .child("30 天")
                            )
                            .child(
                                div()
                                    .id("period-quarter")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(if selected_period == "季度" { PRIMARY_BLUE } else { theme.bg_input })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if selected_period == "季度" { c(0xffffff) } else { theme.text_secondary })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.selected_period = "季度";
                                        this.traffic_bars = this.db.load_traffic_trends("季度");
                                        cx.notify();
                                    }))
                                    .child("季度")
                            )
                    )
            )
            // 柱状图展示区
            .child(
                div()
                    .flex()
                    .h(px(170.0))
                    .items_end()
                    .justify_between()
                    .pt(px(10.0))
                    .pb(px(4.0))
                    .px(px(12.0))
                    .bg(theme.bg_input)
                    .rounded_lg()
                    .children(bars.iter().map(|bar| {
                        let is_hi = bar.is_highlight;
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if is_hi { ACCENT_CYAN } else { theme.text_muted })
                                    .child(bar.val_label.clone())
                            )
                            .child(
                                div()
                                    .w(px(34.0))
                                    .h(px((bar.height_pct as f32) * 1.1))
                                    .rounded_t_md()
                                    .bg(if is_hi { ACCENT_CYAN } else { PRIMARY_BLUE })
                                    .hover(|s| s.bg(PRIMARY_BLUE_HOVER))
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if is_hi { theme.text_primary } else { theme.text_secondary })
                                    .child(bar.day_label.clone())
                            )
                    }))
            )
    }

    fn render_resource_gauges(gauges: &[ResourceGaugeItem], theme: ThemePalette) -> impl IntoElement {
        div()
            .w(px(380.0))
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(14.0))
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
                            .child(m_icon("icons/bolt.svg", 18.0, ACCENT_EMERALD))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("集群计算资源水位")
                            )
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_full()
                            .bg(theme.badge_success_bg)
                            .text_xs()
                            .text_color(ACCENT_EMERALD)
                            .child("实时采集")
                    )
            )
            .children(gauges.iter().map(|g| {
                let bar_color = c(g.color_hex);
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .child(div().text_color(theme.text_primary).font_weight(FontWeight::MEDIUM).child(g.title.clone()))
                            .child(div().text_color(theme.text_secondary).child(g.pct_text.clone()))
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(7.0))
                            .rounded_full()
                            .bg(theme.bg_input)
                            .child(
                                div()
                                    .h_full()
                                    .w(px((g.pct_val as f32) * 3.3))
                                    .rounded_full()
                                    .bg(bar_color)
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(g.desc.clone())
                    )
            }))
    }

    fn render_service_health_table(services: &[ServiceHealthItem], theme: ThemePalette) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p(px(20.0))
            .gap(px(14.0))
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
                            .child(m_icon("icons/hub.svg", 18.0, ACCENT_CYAN))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("核心微服务运行拓扑")
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .child("生产集群 · 全链路健康检测")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(theme.bg_input)
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_secondary)
                            .child(div().w(px(160.0)).child("服务标识"))
                            .child(div().w(px(90.0)).child("角色类型"))
                            .child(div().w(px(60.0)).child("端口"))
                            .child(div().w(px(70.0)).child("状态"))
                            .child(div().w(px(70.0)).child("延迟"))
                            .child(div().flex_1().child("Pod 副本"))
                    )
                    .children(services.iter().map(|s| {
                        let color = c(s.color_hex);
                        div()
                            .flex()
                            .items_center()
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .hover(move |st| st.bg(theme.bg_hover))
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(div().w(px(160.0)).font_weight(FontWeight::MEDIUM).child(s.name.clone()))
                            .child(div().w(px(90.0)).text_color(theme.text_secondary).child(s.role.clone()))
                            .child(div().w(px(60.0)).text_color(theme.text_muted).child(s.port.clone()))
                            .child(
                                div()
                                    .w(px(70.0))
                                    .text_color(color)
                                    .child(format!("● {}", s.status))
                            )
                            .child(div().w(px(70.0)).text_color(theme.text_secondary).child(s.latency.clone()))
                            .child(div().flex_1().text_color(ACCENT_INDIGO).child(s.pods.clone()))
                    }))
            )
    }

    fn render_live_activity_stream(events: &[LiveEventItem], theme: ThemePalette) -> impl IntoElement {
        div()
            .w(px(380.0))
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .rounded_xl()
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(14.0))
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
                            .child(m_icon("icons/notifications.svg", 18.0, ACCENT_AMBER))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child("实时系统审计动态")
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(PRIMARY_BLUE)
                            .child("安全事件流")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(events.iter().map(|ev| {
                        let color = c(ev.color_hex);
                        div()
                            .flex()
                            .gap_3()
                            .items_start()
                            .p_2()
                            .rounded_lg()
                            .hover(move |s| s.bg(theme.bg_hover))
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .bg(theme.bg_input)
                                    .text_xs()
                                    .text_color(color)
                                    .child(ev.badge.clone())
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .text_xs()
                                            .child(div().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child(format!("{} · {}", ev.source, ev.actor)))
                                            .child(div().text_color(theme.text_muted).child(ev.time.clone()))
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(ev.desc.clone())
                                    )
                            )
                    }))
            )
    }
}
