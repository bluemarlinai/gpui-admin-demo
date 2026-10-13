use gpui::*;
use gpui_kit::component::button::ButtonVariants;
use crate::AdminApp;
use crate::icons::m_icon;
use crate::models::{Toast, ToastKind, UserStatus};
use crate::theme::*;

pub struct UsersView;

impl UsersView {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let selected_role = app.selected_role;
        let roles = ["全部角色", "超级管理员", "架构与运维", "研发工程师", "安全审计"];

        let filtered_users: Vec<_> = if selected_role == "全部角色" {
            app.users.clone()
        } else {
            app.users.iter().filter(|u| u.role == selected_role).cloned().collect()
        };

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w_full()
            // 顶部操作栏与过滤筛选器
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
                            .items_center()
                            .children(roles.into_iter().map(|role| {
                                let is_active = role == selected_role;
                                div()
                                    .id(SharedString::from(format!("role-filter-{}", role)))
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .bg(if is_active { PRIMARY_BLUE } else { theme.bg_input })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if is_active { c(0xffffff) } else { theme.text_secondary })
                                    .hover(move |s| s.bg(if is_active { PRIMARY_BLUE_HOVER } else { theme.bg_hover }))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.selected_role = role;
                                        cx.notify();
                                    }))
                                    .child(role)
                            }))
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
                                    .child(m_icon("icons/people.svg", 14.0, ACCENT_CYAN))
                                    .child(format!("共 {} 名成员", app.users.len()))
                            )
                            .child(
                                gpui_kit::component::button::Button::new("add-user-modal-btn")
                                    .primary()
                                    .label("新增成员")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.form_name_input.update(cx, |s, cx| {
                                            s.set_value("", window, cx);
                                        });
                                        this.form_email_input.update(cx, |s, cx| {
                                            s.set_value("", window, cx);
                                        });
                                        this.form_role = "研发工程师".to_string();
                                        this.form_dept = "基础设施架构部".to_string();
                                        this.form_status = UserStatus::Active;
                                        this.form_perm_read = true;
                                        this.form_perm_audit = false;
                                        this.form_perm_deploy = false;
                                        this.form_perm_secret = false;
                                        this.editing_user_id = None;
                                        this.state.show_user_modal = true;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 用户数据主表格
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
                    // 表头
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
                            .child(div().w(px(60.0)).child("ID"))
                            .child(div().w(px(180.0)).child("用户基本信息"))
                            .child(div().w(px(140.0)).child("岗位与权限角色"))
                            .child(div().w(px(130.0)).child("所属事业群 / 部门"))
                            .child(div().w(px(100.0)).child("账户状态"))
                            .child(div().w(px(140.0)).child("最后登入时间"))
                            .child(div().flex_1().child("管理操作"))
                    )
                    // 用户行列表
                    .children(filtered_users.into_iter().map(|user| {
                        let user_id = user.id;
                        let current_status = user.status;
                        let user_name = user.name.clone();
                        let is_suspended = user.status == UserStatus::Suspended;
                        let (status_text, status_color, status_bg) = match user.status {
                            UserStatus::Active => ("正常运行", ACCENT_EMERALD, theme.badge_success_bg),
                            UserStatus::Inactive => ("休眠离线", theme.text_muted, theme.bg_input),
                            UserStatus::Suspended => ("已锁定", ACCENT_ROSE, theme.badge_danger_bg),
                        };

                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py_3()
                            .rounded_lg()
                            .hover(move |s| s.bg(theme.bg_hover))
                            .text_xs()
                            .child(div().w(px(60.0)).text_color(theme.text_muted).child(format!("#{:04}", user.id)))
                            .child(
                                div()
                                    .w(px(180.0))
                                    .flex()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        div()
                                            .size(px(32.0))
                                            .rounded_full()
                                            .bg(PRIMARY_BLUE)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(c(0xffffff))
                                            .child(user.avatar_abbr.clone())
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(div().text_color(theme.text_primary).font_weight(FontWeight::MEDIUM).child(user.name.clone()))
                                            .child(div().text_color(theme.text_muted).child(user.email.clone()))
                                    )
                            )
                            .child(
                                div()
                                    .w(px(140.0))
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.bg_input)
                                    .text_color(ACCENT_INDIGO)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(user.role.clone())
                            )
                            .child(div().w(px(130.0)).text_color(theme.text_secondary).child(user.dept.clone()))
                            .child(
                                div()
                                    .w(px(100.0))
                                    .px_2()
                                    .py_1()
                                    .rounded_full()
                                    .bg(status_bg)
                                    .text_color(status_color)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(status_text)
                            )
                            .child(div().w(px(140.0)).text_color(theme.text_secondary).child(user.last_login.clone()))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .id(SharedString::from(format!("toggle-user-{}", user_id)))
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(theme.bg_input)
                                            .hover(|s| s.bg(PRIMARY_BLUE).text_color(c(0xffffff)))
                                            .text_color(theme.text_secondary)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                let next_status = match current_status {
                                                    UserStatus::Active => UserStatus::Suspended,
                                                    UserStatus::Inactive => UserStatus::Active,
                                                    UserStatus::Suspended => UserStatus::Active,
                                                };
                                                this.db.update_user_status(user_id, next_status);
                                                this.db.insert_audit_log(
                                                    "系统管理员",
                                                    &format!("修改用户状态为 {}", next_status.as_str()),
                                                    &format!("ID: {} ({})", user_id, user_name),
                                                    "WARN",
                                                );
                                                this.reload_from_db();
                                                this.toast = Some(Toast {
                                                    message: format!("已更新成员「{}」的账户状态", user_name),
                                                    kind: ToastKind::Success,
                                                });
                                                cx.notify();
                                            }))
                                            .child(m_icon(
                                                if is_suspended { "icons/lock_open.svg" } else { "icons/lock.svg" },
                                                13.0,
                                                if is_suspended { ACCENT_EMERALD } else { ACCENT_AMBER },
                                            ))
                                            .child(if is_suspended { "解除锁定" } else { "切换状态" })
                                    )
                                    .child(
                                        div()
                                            .id(SharedString::from(format!("perm-user-{}", user_id)))
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(theme.bg_input)
                                            .hover(move |s| s.bg(theme.bg_card_alt))
                                            .text_color(theme.text_muted)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.toast = Some(Toast {
                                                    message: format!("已加载成员 #{:04} 的 RBAC 访问控制策略", user_id),
                                                    kind: ToastKind::Info,
                                                });
                                                cx.notify();
                                            }))
                                            .child(m_icon("icons/shield.svg", 13.0, theme.text_muted))
                                            .child("查看权限")
                                    )
                            )
                    }))
            )
    }
}
