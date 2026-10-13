use gpui::*;
use gpui_kit::component::button::ButtonVariants;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::{ThemeMode, Toast, ToastKind};
use crate::theme::*;

pub struct SettingsView;

impl SettingsView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let settings = &app.settings;
        let current_mode = settings.theme_mode;

        let theme_cards = [
            (
                ThemeMode::Light,
                "icons/light_mode.svg",
                "浅色主题 (Light)",
                "高对比明亮办公配色，适合日间环境与大屏投影演示",
            ),
            (
                ThemeMode::Dark,
                "icons/dark_mode.svg",
                "深色主题 (Dark)",
                "护眼深空科技蓝灰配色，适合运维中心全天候大屏监控",
            ),
            (
                ThemeMode::System,
                "icons/brightness_auto.svg",
                "跟随系统 (System)",
                "自动实时同步操作系统深浅色个性化主题设定",
            ),
        ];

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            // 组 0: 界面外观与主题模式切换
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(theme.bg_card)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border)
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
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(m_icon("icons/brightness_auto.svg", 18.0, PRIMARY_BLUE))
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.text_primary)
                                                    .child("界面外观与主题配色模式")
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("支持浅色、深色及跟随操作系统外观动态切换")
                                    )
                            )
                            .child(
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded_full()
                                    .bg(theme.badge_info_bg)
                                    .text_xs()
                                    .text_color(PRIMARY_BLUE)
                                    .child(format!(
                                        "当前生效: {} ({})",
                                        current_mode.label(),
                                        if theme.is_dark { "深色渲染" } else { "浅色渲染" }
                                    ))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(14.0))
                            .w_full()
                            .children(theme_cards.into_iter().map(|(mode, icon_path, title, desc)| {
                                let is_selected = current_mode == mode;
                                let border_col = if is_selected { PRIMARY_BLUE } else { theme.border };
                                let icon_col = if is_selected { PRIMARY_BLUE } else { theme.text_secondary };

                                div()
                                    .id(SharedString::from(format!("settings-theme-card-{:?}", mode)))
                                    .flex_1()
                                    .p(px(14.0))
                                    .rounded_xl()
                                    .cursor_pointer()
                                    .bg(if is_selected { theme.badge_info_bg } else { theme.bg_input })
                                    .border_1()
                                    .border_color(border_col)
                                    .hover(move |s| s.bg(theme.bg_hover).border_color(PRIMARY_BLUE))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.settings.theme_mode = mode;
                                        this.db.save_settings(&this.settings);
                                        this.db.insert_audit_log(
                                            "系统管理员",
                                            &format!("切换主题模式为 {}", mode.label()),
                                            "外观偏好配置",
                                            "INFO",
                                        );
                                        this.reload_from_db();
                                        this.toast = Some(Toast {
                                            message: format!("已切换为【{}】外观模式", mode.label()),
                                            kind: ToastKind::Success,
                                        });
                                        cx.notify();
                                    }))
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
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
                                                    .child(m_icon(icon_path, 16.0, icon_col))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(title)
                                                    )
                                            )
                                            .children(if is_selected {
                                                Some(m_icon("icons/check_circle.svg", 15.0, PRIMARY_BLUE))
                                            } else {
                                                None
                                            })
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(desc)
                                    )
                            }))
                    )
            )
            // 组 1: 核心安全策略
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(theme.bg_card)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border)
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
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(m_icon("icons/lock.svg", 18.0, PRIMARY_BLUE))
                                            .child(div().text_base().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child("平台账户与访问安全"))
                                    )
                                    .child(div().text_xs().text_color(theme.text_muted).child("统一安全鉴权中心与动态访问策略控制"))
                            )
                            .child(
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded_full()
                                    .bg(theme.badge_success_bg)
                                    .text_xs()
                                    .text_color(ACCENT_EMERALD)
                                    .child("等保三级合规")
                            )
                    )
                    .child(Self::toggle_item(
                        "toggle-2fa",
                        "强制全员双因子认证 (2FA / TOTP)",
                        "所有管理员登入时必须通过 Google Authenticator 或硬件安全 Key 校验",
                        settings.two_factor_auth,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.two_factor_auth = !this.settings.two_factor_auth;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
                    .child(Self::toggle_item(
                        "toggle-ratelimit",
                        "全局 API 突发流量自适应限流保护",
                        "当单 IP 超出 5,000 req/s 时自动开启防雪崩验证码拦截模式",
                        settings.api_rate_limit,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.api_rate_limit = !this.settings.api_rate_limit;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
            )
            // 组 2: 容灾备份与运维通知
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(theme.bg_card)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.border)
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
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(m_icon("icons/save.svg", 18.0, ACCENT_EMERALD))
                                            .child(div().text_base().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child("数据灾备与警报推送联动"))
                                    )
                                    .child(div().text_xs().text_color(theme.text_muted).child("分布式异地冷备与全渠道自动化运维响应渠道"))
                            )
                            .child(
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded_full()
                                    .bg(theme.badge_info_bg)
                                    .text_xs()
                                    .text_color(PRIMARY_BLUE)
                                    .child("多可用区多活")
                            )
                    )
                    .child(Self::toggle_item(
                        "toggle-backup",
                        "跨可用区数据库自动快照与异地冷备",
                        "每 6 小时自动增量备份快照并同步推送至异地加密对象存储归档",
                        settings.auto_backup,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.auto_backup = !this.settings.auto_backup;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
                    .child(Self::toggle_item(
                        "toggle-email",
                        "P0/P1 核心灾难级故障邮件紧急广播",
                        "当核心网关不可用或节点宕机时，立即向运维应急响应组发送短信与邮件",
                        settings.email_alerts,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.email_alerts = !this.settings.email_alerts;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
                    .child(Self::toggle_item(
                        "toggle-dingtalk",
                        "企业微信 / 钉钉机器人 Webhook 实时通知",
                        "向集群告警协作群实时投递部署、扩缩容与安全事件卡片",
                        settings.dingtalk_webhook,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.dingtalk_webhook = !this.settings.dingtalk_webhook;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
                    .child(Self::toggle_item(
                        "toggle-debug",
                        "全链路系统诊断与详细运行日志",
                        "开启详细的调用链路追踪与性能分析指标记录 (Trace)",
                        settings.debug_logging,
                        theme,
                        cx.listener(|this, _, _, cx| {
                            this.settings.debug_logging = !this.settings.debug_logging;
                            this.db.save_settings(&this.settings);
                            cx.notify();
                        }),
                    ))
            )
            // 底部保存按钮条
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
                            .items_center()
                            .gap_2()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(m_icon("icons/shield.svg", 14.0, ACCENT_CYAN))
                            .child("安全策略与系统偏好修改将在保存后立即生效")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                gpui_kit::component::button::Button::new("reset-settings-btn")
                                    .label("重置默认")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.settings = Default::default();
                                        this.db.save_settings(&this.settings);
                                        this.db.insert_audit_log(
                                            "系统管理员",
                                            "重置系统设置为默认配置",
                                            "安全与环境策略",
                                            "INFO",
                                        );
                                        this.reload_from_db();
                                        this.state.set_toast(
                                            "已恢复系统默认安全与环境配置",
                                            ToastKind::Info,
                                        );
                                        cx.notify();
                                    }))
                            )
                            .child(
                                gpui_kit::component::button::Button::new("save-settings-btn")
                                    .primary()
                                    .label("保存配置")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.db.save_settings(&this.settings);
                                        this.db.insert_audit_log(
                                            "系统管理员",
                                            "保存并应用系统环境安全参数",
                                            "安全与环境策略",
                                            "INFO",
                                        );
                                        this.reload_from_db();
                                        this.state.set_toast(
                                            "系统环境与安全策略配置已保存生效",
                                            ToastKind::Success,
                                        );
                                        cx.notify();
                                    }))
                            )
                    )
            )
    }

    fn toggle_item(
        btn_id: &'static str,
        title: &'static str,
        desc: &'static str,
        is_on: bool,
        theme: ThemePalette,
        listener: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .flex()
            .justify_between()
            .items_center()
            .p_3()
            .rounded_lg()
            .hover(move |s| s.bg(theme.bg_hover))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(div().text_xs().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
                    .child(div().text_xs().text_color(theme.text_muted).child(desc))
            )
            .child(
                gpui_kit::component::switch::Switch::new(btn_id)
                    .checked(is_on)
                    .on_click(listener)
            )
    }
}
