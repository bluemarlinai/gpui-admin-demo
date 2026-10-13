use crate::models::{NavigationTab, Toast, ToastKind, UserItem};
use crate::services::auth::AuthUser;

#[derive(Debug, Clone)]
pub struct AppState {
    // 认证与会话状态
    pub is_authenticated: bool,
    pub current_user: Option<AuthUser>,
    pub is_offline_mode: bool,
    pub login_error: Option<String>,

    // 路由与活动面板
    pub current_tab: NavigationTab,
    pub selected_period: &'static str,
    pub selected_role: &'static str,
    pub notifications_count: usize,

    // 顶部菜单栏、表单与模态窗口交互状态
    pub active_menu: Option<&'static str>,
    pub show_user_modal: bool,
    pub show_about_modal: bool,
    pub editing_user: Option<UserItem>,

    // 状态栏与备份记录
    pub last_backup_time: String,
    pub last_backup_file: Option<String>,

    // 反馈机制
    pub toast: Option<Toast>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            current_user: None,
            is_offline_mode: true,
            login_error: None,
            current_tab: NavigationTab::Dashboard,
            selected_period: "7天",
            selected_role: "全部角色",
            notifications_count: 3,
            active_menu: None,
            show_user_modal: false,
            show_about_modal: false,
            editing_user: None,
            last_backup_time: "自动快照已就绪".into(),
            last_backup_file: None,
            toast: None,
        }
    }
}

impl AppState {
    pub fn login_success(&mut self, user: AuthUser) {
        self.is_authenticated = true;
        self.current_user = Some(user);
        self.login_error = None;
        self.active_menu = None;
        self.toast = Some(Toast {
            message: "欢迎回来，已成功进入 NovaOps Pro 控制台".into(),
            kind: ToastKind::Success,
        });
    }

    pub fn logout(&mut self) {
        self.is_authenticated = false;
        self.current_user = None;
        self.show_user_modal = false;
        self.show_about_modal = false;
        self.active_menu = None;
        self.login_error = None;
        self.toast = Some(Toast {
            message: "已安全退出当前会话".into(),
            kind: ToastKind::Info,
        });
    }

    pub fn set_toast(&mut self, message: impl Into<String>, kind: ToastKind) {
        self.toast = Some(Toast {
            message: message.into(),
            kind,
        });
    }
}
