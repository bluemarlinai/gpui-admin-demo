#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationTab {
    Dashboard,
    Users,
    Monitoring,
    AuditLogs,
    Settings,
}

#[derive(Debug, Clone)]
pub struct KpiCardItem {
    pub title: String,
    pub value: String,
    pub trend: String,
    pub is_positive: bool,
    pub subtext: String,
    pub icon: String,
    pub color_hex: u32,
}

#[derive(Debug, Clone)]
pub struct TrafficBarItem {
    pub period: String,
    pub day_label: String,
    pub height_pct: u8,
    pub val_label: String,
    pub is_highlight: bool,
}

#[derive(Debug, Clone)]
pub struct ResourceGaugeItem {
    pub title: String,
    pub pct_text: String,
    pub pct_val: u8,
    pub color_hex: u32,
    pub desc: String,
}

#[derive(Debug, Clone)]
pub struct ServiceHealthItem {
    pub name: String,
    pub role: String,
    pub port: String,
    pub status: String,
    pub latency: String,
    pub pods: String,
    pub color_hex: u32,
}

#[derive(Debug, Clone)]
pub struct LiveEventItem {
    pub time: String,
    pub source: String,
    pub actor: String,
    pub desc: String,
    pub badge: String,
    pub color_hex: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Active => "Active",
            UserStatus::Inactive => "Inactive",
            UserStatus::Suspended => "Suspended",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Inactive" => UserStatus::Inactive,
            "Suspended" => UserStatus::Suspended,
            _ => UserStatus::Active,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserItem {
    pub id: u32,
    pub name: String,
    pub avatar_abbr: String,
    pub email: String,
    pub role: String,
    pub dept: String,
    pub status: UserStatus,
    pub last_login: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Healthy,
    Warning,
    Busy,
}

impl NodeStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Busy" => NodeStatus::Busy,
            "Warning" => NodeStatus::Warning,
            _ => NodeStatus::Healthy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeMetric {
    pub name: String,
    pub role: String,
    pub ip: String,
    pub cpu_pct: u8,
    pub mem_pct: u8,
    pub disk_pct: u8,
    pub status: NodeStatus,
}

#[derive(Debug, Clone)]
pub struct TerminalLogItem {
    pub time: String,
    pub level: String,
    pub src: String,
    pub msg: String,
    pub color_hex: u32,
}

#[derive(Debug, Clone)]
pub struct AuditLogItem {
    pub id: u32,
    pub operator: String,
    pub action: String,
    pub target: String,
    pub ip: String,
    pub time: String,
    pub level: String,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
            ThemeMode::System => "System",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Light" => ThemeMode::Light,
            "Dark" => ThemeMode::Dark,
            _ => ThemeMode::System,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ThemeMode::Light => "浅色",
            ThemeMode::Dark => "深色",
            ThemeMode::System => "随系统",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemSettings {
    pub theme_mode: ThemeMode,
    pub auto_backup: bool,
    pub two_factor_auth: bool,
    pub api_rate_limit: bool,
    pub email_alerts: bool,
    pub dingtalk_webhook: bool,
    pub debug_logging: bool,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            auto_backup: true,
            two_factor_auth: true,
            api_rate_limit: true,
            email_alerts: true,
            dingtalk_webhook: false,
            debug_logging: false,
        }
    }
}
