use gpui::*;
use gpui_kit::component::button::*;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::theme::*;

pub struct AboutModal;

impl AboutModal {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let last_backup = app.state.last_backup_time.clone();

        div()
            .id("about-modal-backdrop")
            .occlude()
            .absolute()
            .inset_0()
            .bg(rgba(0x00000088))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(500.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_2xl()
                    .shadow_xl()
                    .p(px(28.0))
                    .flex()
                    .flex_col()
                    .gap_5()
                    // 顶部品牌标志与版本号
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3p5()
                                    .child(
                                        div()
                                            .size(px(54.0))
                                            .rounded_2xl()
                                            .bg(PRIMARY_BLUE)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .shadow_md()
                                            .child(m_icon("icons/novaops_logo.svg", 32.0, c(0xffffff)))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child("NovaOps Pro 企业级云原生管理平台")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(PRIMARY_BLUE)
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child("Enterprise Edition v1.2.0 (Build 2026.10 · LTS)")
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .id("close-about-modal")
                                    .cursor_pointer()
                                    .p_1p5()
                                    .rounded_lg()
                                    .hover(move |s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.show_about_modal = false;
                                        cx.notify();
                                    }))
                                    .child(m_icon("icons/close.svg", 16.0, theme.text_muted))
                            )
                    )
                    .child(div().h(px(1.0)).bg(theme.border))
                    // 平台简介
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_secondary)
                            .child("NovaOps Pro 是一套面向现代分布式基础设施的全栈可观测、组织权限管控与安全合规运维桌面终端，支持多可用区集群拓扑实时监控、细粒度 RBAC 权限治理与本地热备份归档。")
                    )
                    // 关键系统参数网格
                    .child(
                        div()
                            .bg(theme.bg_input)
                            .rounded_xl()
                            .p(px(14.0))
                            .flex()
                            .flex_col()
                            .gap_2p5()
                            .child(Self::info_row("安全合规认证", "国家信息安全等级保护三级 / ISO 27001", theme))
                            .child(Self::info_row("数据容灾状态", &format!("本地加密存储引擎 · 最近备份: {last_backup}"), theme))
                            .child(Self::info_row("桌面守护服务", "系统托盘常驻守护已启用 (支持托盘快捷备份)", theme))
                            .child(Self::info_row("授权许可类型", "企业永久商业授权 (NovaOps Enterprise License)", theme))
                    )
                    // 底部版权与确定按钮
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pt_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("© 2026 NovaOps Cloud Technologies Inc. All rights reserved.")
                            )
                            .child(
                                Button::new("ok-about-modal")
                                    .primary()
                                    .label("确定")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.show_about_modal = false;
                                        cx.notify();
                                    }))
                            )
                    )
            )
    }

    fn info_row(label: &'static str, val: &str, theme: ThemePalette) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .text_xs()
            .child(div().text_color(theme.text_muted).child(label))
            .child(
                div()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text_primary)
                    .child(val.to_string()),
            )
    }
}
