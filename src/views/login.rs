use gpui::*;
use gpui_kit::component::button::*;
use gpui_kit::component::checkbox::*;
use gpui_kit::component::input::Input;
use gpui_kit::component::switch::*;
use crate::theme::*;
use crate::icons::m_icon;
use crate::AdminApp;

pub struct LoginView;

impl LoginView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let login_error = app.state.login_error.clone();
        let is_offline = app.state.is_offline_mode;
        let remember_login = app.remember_login;

        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(theme.bg_app)
            .relative()
            // 背景光晕装饰
            .child(
                div()
                    .absolute()
                    .w(px(600.0))
                    .h(px(600.0))
                    .rounded_full()
                    .bg(rgba(0x3b82f612))
            )
            // 登录主卡片
            .child(
                div()
                    .relative()
                    .w(px(460.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_3xl()
                    .shadow_xl()
                    .p(px(36.0))
                    .flex()
                    .flex_col()
                    .gap_5()
                    // 品牌 Header
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(52.0))
                                    .h(px(52.0))
                                    .rounded_2xl()
                                    .bg(PRIMARY_BLUE)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .shadow_md()
                                    .child(m_icon("icons/novaops_logo.svg", 30.0, c(0xffffff)))
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child("NovaOps Pro")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("企业级云原生智能运维与安全管控平台")
                            )
                    )
                    // 错误提示条
                    .children(login_error.map(|err_msg| {
                        div()
                            .bg(rgba(0xef44441a))
                            .border_1()
                            .border_color(rgb(0xef4444))
                            .rounded_xl()
                            .px_3()
                            .py_2()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(m_icon("icons/warning.svg", 15.0, rgb(0xef4444)))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(0xef4444))
                                    .child(err_msg)
                            )
                    }))
                    // 表单主体
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3p5()
                            // 账号输入
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_secondary)
                                            .child("管理员账号")
                                    )
                                    .child(
                                        Input::new(&app.login_username_input)
                                            .prefix(m_icon("icons/people.svg", 15.0, theme.text_muted))
                                            .cleanable(true)
                                    )
                            )
                            // 密码输入
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_secondary)
                                            .child("登录密码")
                                    )
                                    .child(
                                        Input::new(&app.login_password_input)
                                            .prefix(m_icon("icons/lock.svg", 15.0, theme.text_muted))
                                            .mask_toggle()
                                            .cleanable(true)
                                    )
                            )
                            // 记住登录与免密保护
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .pt_1()
                                    .child(
                                        Checkbox::new("remember-login")
                                            .label("保持登录状态")
                                            .checked(remember_login)
                                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                                this.remember_login = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_xs().text_color(theme.text_muted).child("本机安全域"))
                                            .child(
                                                Switch::new("offline-mode-switch")
                                                    .checked(is_offline)
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.state.is_offline_mode = !this.state.is_offline_mode;
                                                        cx.notify();
                                                    }))
                                            )
                                    )
                            )
                    )
                    // 登录主操作按钮
                    .child(
                        Button::new("submit-login-btn")
                            .primary()
                            .label("登录控制台")
                            .on_click(cx.listener(|this, _, _, cx| {
                                let username = this.login_username_input.read(cx).value().to_string();
                                let password = this.login_password_input.read(cx).value().to_string();

                                tracing::info!(target: "auth", "尝试登录验证: {}", username);
                                match this.db.authenticate(&username, &password) {
                                    Ok(auth_user) => {
                                        tracing::info!(target: "auth", "用户 {} 认证成功, 角色: {}", auth_user.username, auth_user.role);
                                        this.state.login_success(auth_user);
                                    }
                                    Err(err) => {
                                        tracing::warn!(target: "auth", "用户 {} 认证失败: {}", username, err);
                                        this.state.login_error = Some(err);
                                    }
                                }
                                cx.notify();
                            }))
                    )
                    // 底部企业级安全合规说明
                    .child(
                        div()
                            .bg(theme.bg_input)
                            .rounded_xl()
                            .p_3()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1p5()
                                    .child(m_icon("icons/shield.svg", 13.0, ACCENT_EMERALD))
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child("企业级安全合规保障"))
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("系统启用端到端加密传输与多重身份鉴权策略，所有登录行为均纳入安全合规审计。")
                            )
                    )
            )
    }
}
