use gpui::*;
use gpui_kit::component::button::*;
use gpui_kit::component::checkbox::*;
use gpui_kit::component::input::Input;
use gpui_kit::component::switch::*;
use crate::models::{ToastKind, UserStatus};
use crate::theme::*;
use crate::icons::m_icon;
use crate::AdminApp;

pub struct UserModal;

impl UserModal {
    pub fn render(app: &AdminApp, theme: ThemePalette, cx: &mut Context<AdminApp>) -> impl IntoElement {
        let is_editing = app.editing_user_id.is_some();
        let modal_title = if is_editing { "编辑系统用户档案" } else { "新增组织成员" };
        let modal_subtitle = if is_editing {
            "修改用户角色、部门归属及操作安全权限策略"
        } else {
            "为组织成员分配账号，并配置部门归属与操作访问权限"
        };

        div()
            .id("user-modal-backdrop")
            .occlude()
            .absolute()
            .inset_0()
            .bg(rgba(0x00000088))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(580.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_2xl()
                    .shadow_lg()
                    .p(px(28.0))
                    .flex()
                    .flex_col()
                    .gap_5()
                    // 头部标题与关闭按钮
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .justify_between()
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
                                            .child(modal_title)
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(modal_subtitle)
                                    )
                            )
                            .child(
                                div()
                                    .id("close-user-modal")
                                    .cursor_pointer()
                                    .p_1p5()
                                    .rounded_lg()
                                    .hover(move |s| s.bg(theme.bg_hover))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.show_user_modal = false;
                                        cx.notify();
                                    }))
                                    .child(m_icon("icons/close.svg", 16.0, theme.text_muted))
                            )
                    )
                    // 分隔线
                    .child(div().h(px(1.0)).bg(theme.border))
                    // 表单正文区域
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            // 表单字段 1：姓名与邮箱
                            .child(
                                div()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .gap_1p5()
                                            .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child("成员姓名 *"))
                                            .child(
                                                Input::new(&app.form_name_input)
                                                    .cleanable(true)
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .gap_1p5()
                                            .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child("企业邮箱 *"))
                                            .child(
                                                Input::new(&app.form_email_input)
                                                    .cleanable(true)
                                            )
                                    )
                            )
                            // 表单字段 2：角色单选组
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child("系统权限角色 *"))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .children(["研发工程师", "架构与运维", "安全审计", "超级管理员"].map(|role| {
                                                let is_sel = app.form_role == role;
                                                let r_clone = role;
                                                div()
                                                    .id(SharedString::from(format!("role-chip-{}", role)))
                                                    .cursor_pointer()
                                                    .px_3()
                                                    .py_1p5()
                                                    .rounded_lg()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .bg(if is_sel { PRIMARY_BLUE } else { theme.bg_input })
                                                    .text_color(if is_sel { c(0xffffff) } else { theme.text_secondary })
                                                    .border_1()
                                                    .border_color(if is_sel { PRIMARY_BLUE } else { theme.border })
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.form_role = r_clone.to_string();
                                                        this.form_perm_read = true;
                                                        this.form_perm_audit = r_clone != "研发工程师";
                                                        this.form_perm_deploy = r_clone == "超级管理员" || r_clone == "架构与运维";
                                                        this.form_perm_secret = r_clone == "超级管理员";
                                                        cx.notify();
                                                    }))
                                                    .child(role)
                                            }))
                                    )
                            )
                            // 表单字段 3：部门划分
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child("所属组织部门 *"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .gap_2()
                                            .children(["基础设施架构部", "DevOps效能部", "信息合规安全部", "支付与结算业务", "大数据分析中心"].map(|dept| {
                                                let is_sel = app.form_dept == dept;
                                                let d_clone = dept;
                                                div()
                                                    .id(SharedString::from(format!("dept-chip-{}", dept)))
                                                    .cursor_pointer()
                                                    .px_2p5()
                                                    .py_1()
                                                    .rounded_lg()
                                                    .text_xs()
                                                    .bg(if is_sel { theme.bg_hover } else { theme.bg_input })
                                                    .text_color(if is_sel { theme.text_primary } else { theme.text_muted })
                                                    .border_1()
                                                    .border_color(if is_sel { PRIMARY_BLUE } else { theme.border })
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.form_dept = d_clone.to_string();
                                                        cx.notify();
                                                    }))
                                                    .child(dept)
                                            }))
                                    )
                            )
                            // 表单字段 4：状态开关
                            .child(
                                div()
                                    .bg(theme.bg_input)
                                    .rounded_xl()
                                    .p_3()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child("启用此用户账号"))
                                            .child(div().text_xs().text_color(theme.text_muted).child("禁用后该用户无法登录控制台或调用 OpenAPI"))
                                    )
                                    .child(
                                        Switch::new("user-status-switch")
                                            .checked(app.form_status == UserStatus::Active)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.form_status = if this.form_status == UserStatus::Active {
                                                    UserStatus::Inactive
                                                } else {
                                                    UserStatus::Active
                                                };
                                                cx.notify();
                                            }))
                                    )
                            )
                            // 表单字段 5：安全权限策略复选框组
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child("细粒度操作权限策略"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_wrap()
                                            .gap_4()
                                            .child(
                                                Checkbox::new("perm-read")
                                                    .label("仪表盘大屏只读")
                                                    .checked(app.form_perm_read)
                                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                                        this.form_perm_read = *checked;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Checkbox::new("perm-audit")
                                                    .label("查看合规审计")
                                                    .checked(app.form_perm_audit)
                                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                                        this.form_perm_audit = *checked;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Checkbox::new("perm-deploy")
                                                    .label("生产发布调度")
                                                    .checked(app.form_perm_deploy)
                                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                                        this.form_perm_deploy = *checked;
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                Checkbox::new("perm-secret")
                                                    .label("敏感密钥读取")
                                                    .checked(app.form_perm_secret)
                                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                                        this.form_perm_secret = *checked;
                                                        cx.notify();
                                                    }))
                                            )
                                    )
                            )
                    )
                    // 底部操作栏
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_end()
                            .gap_3()
                            .pt_2()
                            .child(
                                Button::new("cancel-user-modal")
                                    .label("取消")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.show_user_modal = false;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("submit-user-modal")
                                    .primary()
                                    .label(if is_editing { "保存变更" } else { "确认创建成员" })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        let raw_name = this.form_name_input.read(cx).value().trim().to_string();
                                        let raw_email = this.form_email_input.read(cx).value().trim().to_string();

                                        if raw_name.is_empty() {
                                            this.state.set_toast("请填写成员姓名", ToastKind::Warning);
                                            cx.notify();
                                            return;
                                        }
                                        let email = if raw_email.is_empty() {
                                            format!("user.{}@novaops.io", this.users.len() + 1)
                                        } else {
                                            raw_email
                                        };
                                        let role = this.form_role.clone();
                                        let dept = this.form_dept.clone();
                                        let status = this.form_status;

                                        match this.db.create_custom_user(&raw_name, &email, &role, &dept, status) {
                                            Ok(new_id) => {
                                                this.db.insert_audit_log(
                                                    "系统管理员",
                                                    "创建组织成员档案",
                                                    &format!("ID: {} ({})", new_id, raw_name),
                                                    "INFO",
                                                );
                                                this.reload_from_db();
                                                this.state.set_toast(
                                                    format!("已成功添加成员「{}」({} · {})", raw_name, role, dept),
                                                    ToastKind::Success,
                                                );
                                                this.state.show_user_modal = false;
                                            }
                                            Err(e) => {
                                                this.state.set_toast(format!("创建成员失败: {e}"), ToastKind::Warning);
                                            }
                                        }
                                        cx.notify();
                                    }))
                            )
                    )
            )
    }
}
