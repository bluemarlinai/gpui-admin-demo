use std::path::PathBuf;
use rusqlite::{params, Connection, Result};
use crate::models::*;

pub struct AppDatabase {
    db_path: PathBuf,
}

impl AppDatabase {
    pub fn new() -> Self {
        let db_path = Self::resolve_db_path();
        let db = Self { db_path };
        let _ = db.init_and_seed();
        db
    }

    pub fn resolve_db_path() -> PathBuf {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(dir) = exe_path.parent() {
                for ancestor in dir.ancestors() {
                    if ancestor.join("Cargo.toml").exists() {
                        return ancestor.join("novaops.db");
                    }
                }
                return dir.join("novaops.db");
            }
        }
        PathBuf::from("novaops.db")
    }

    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    pub fn init_and_seed(&self) -> Result<()> {
        let conn = self.connect()?;

        // 1. KPI 指标卡片表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kpi_cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                value TEXT NOT NULL,
                trend TEXT NOT NULL,
                is_positive INTEGER NOT NULL,
                subtext TEXT NOT NULL,
                icon TEXT NOT NULL,
                color_hex INTEGER NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM kpi_cards", [], |r| r.get(0))?;
        if count == 0 {
            let kpis = [
                ("今日平台总流水", "¥ 1,892,400", "+14.8%", 1, "月度达成率 108.5%", "icons/payments.svg", 0x3b82f6u32),
                ("全网在线活跃实例", "1,428 台", "+32 今日新增", 1, "健康度 99.98%", "icons/computer.svg", 0x10b981u32),
                ("实时并发请求 QPS", "48,920 req/s", "峰值 62,400", 1, "平均响应 18.2 ms", "icons/bolt.svg", 0x06b6d4u32),
                ("待处理风险预警", "0 严重 / 2 提示", "已自动隔离", 0, "SLA 99.999% 达成", "icons/shield.svg", 0xf59e0bu32),
            ];
            for (title, val, trend, pos, sub, icon, color) in kpis {
                conn.execute(
                    "INSERT INTO kpi_cards (title, value, trend, is_positive, subtext, icon, color_hex)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![title, val, trend, pos, sub, icon, color],
                )?;
            }
        } else {
            let _ = conn.execute("UPDATE kpi_cards SET icon = 'icons/payments.svg' WHERE title = '今日平台总流水'", []);
            let _ = conn.execute("UPDATE kpi_cards SET icon = 'icons/computer.svg' WHERE title = '全网在线活跃实例'", []);
            let _ = conn.execute("UPDATE kpi_cards SET icon = 'icons/bolt.svg' WHERE title = '实时并发请求 QPS'", []);
            let _ = conn.execute("UPDATE kpi_cards SET icon = 'icons/shield.svg' WHERE title = '待处理风险预警'", []);
        }

        // 2. 流量趋势柱状图表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS traffic_trends (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                period TEXT NOT NULL,
                day_label TEXT NOT NULL,
                height_pct INTEGER NOT NULL,
                val_label TEXT NOT NULL,
                is_highlight INTEGER NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM traffic_trends", [], |r| r.get(0))?;
        if count == 0 {
            let trends = [
                // 7天
                ("7天", "周一", 62u8, "28.4k", 0),
                ("7天", "周二", 78u8, "36.2k", 0),
                ("7天", "周三", 54u8, "24.1k", 0),
                ("7天", "周四", 92u8, "42.8k", 0),
                ("7天", "周五", 85u8, "39.5k", 0),
                ("7天", "周六", 46u8, "21.0k", 0),
                ("7天", "今日", 72u8, "33.6k", 1),
                // 30天
                ("30天", "W1", 52u8, "112k", 0),
                ("30天", "W2", 74u8, "158k", 0),
                ("30天", "W3", 68u8, "142k", 0),
                ("30天", "W4", 95u8, "210k", 0),
                ("30天", "W5", 82u8, "176k", 0),
                ("30天", "W6", 60u8, "130k", 0),
                ("30天", "当前", 88u8, "194k", 1),
                // 季度
                ("季度", "Q1", 45u8, "420k", 0),
                ("季度", "Q2", 65u8, "610k", 0),
                ("季度", "Q3", 88u8, "850k", 0),
                ("季度", "Q4", 92u8, "920k", 0),
                ("季度", "预1", 70u8, "700k", 0),
                ("季度", "预2", 80u8, "790k", 0),
                ("季度", "当季", 94u8, "950k", 1),
            ];
            for (period, day, h, val, hi) in trends {
                conn.execute(
                    "INSERT INTO traffic_trends (period, day_label, height_pct, val_label, is_highlight)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![period, day, h, val, hi],
                )?;
            }
        }

        // 3. 集群资源水位表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS resource_gauges (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                pct_text TEXT NOT NULL,
                pct_val INTEGER NOT NULL,
                color_hex INTEGER NOT NULL,
                desc TEXT NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM resource_gauges", [], |r| r.get(0))?;
        if count == 0 {
            let gauges = [
                ("CPU 综合占用率", "38.2%", 38u8, 0x10b981u32, "64 Cores / AMD EPYC"),
                ("内存物理占用", "64.5%", 65u8, 0xf59e0bu32, "165.2 GB / 256 GB"),
                ("NVMe 高速存储池", "41.0%", 41u8, 0x3b82f6u32, "3.28 TB / 8.0 TB"),
                ("对外网络出口带宽", "26.8%", 27u8, 0x06b6d4u32, "2.68 Gbps / 10 Gbps"),
            ];
            for (title, pt, pv, col, desc) in gauges {
                conn.execute(
                    "INSERT INTO resource_gauges (title, pct_text, pct_val, color_hex, desc)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![title, pt, pv, col, desc],
                )?;
            }
        }

        // 4. 微服务运行拓扑表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS service_health (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                role TEXT NOT NULL,
                port TEXT NOT NULL,
                status TEXT NOT NULL,
                latency TEXT NOT NULL,
                pods TEXT NOT NULL,
                color_hex INTEGER NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM service_health", [], |r| r.get(0))?;
        if count == 0 {
            let services = [
                ("api-gateway-v2", "网关路由", "8443", "正常", "12 ms", "8 / 8", 0x10b981u32),
                ("oauth2-identity", "鉴权中台", "9001", "正常", "8 ms", "4 / 4", 0x10b981u32),
                ("order-settle-service", "交易结算", "9080", "正常", "26 ms", "6 / 6", 0x10b981u32),
                ("clickhouse-olap", "数仓分析", "8123", "轻载", "54 ms", "3 / 3", 0x06b6d4u32),
                ("kafka-broker-node", "事件总线", "9092", "正常", "3 ms", "12 / 12", 0x10b981u32),
            ];
            for (name, role, port, status, latency, pods, color) in services {
                conn.execute(
                    "INSERT INTO service_health (name, role, port, status, latency, pods, color_hex)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![name, role, port, status, latency, pods, color],
                )?;
            }
        }

        // 5. 实时系统审计动态表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS live_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                time TEXT NOT NULL,
                source TEXT NOT NULL,
                actor TEXT NOT NULL,
                desc TEXT NOT NULL,
                badge TEXT NOT NULL,
                color_hex INTEGER NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM live_events", [], |r| r.get(0))?;
        if count == 0 {
            let events = [
                ("10:42:15", "超级管理员", "张明", "控制台密码登录认证成功", "安全", 0x10b981u32),
                ("10:35:00", "K8s 调度器", "System", "触发自动横向扩容 (新增 2 副本)", "运维", 0x3b82f6u32),
                ("10:12:30", "冷备任务", "Cron", "完成每日增量数据库快照 (6.8GB)", "存储", 0x06b6d4u32),
                ("09:48:12", "WAF 防火墙", "Gateway", "拦截海外 IP 异常暴力探测扫描", "警告", 0xf59e0bu32),
                ("09:15:00", "配置中心", "李思涵", "更新 payment-service 降级限流阈值", "配置", 0x6366f1u32),
            ];
            for (time, source, actor, desc, badge, color) in events {
                conn.execute(
                    "INSERT INTO live_events (time, source, actor, desc, badge, color_hex)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![time, source, actor, desc, badge, color],
                )?;
            }
        }

        // 6. 用户管理表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                avatar_abbr TEXT NOT NULL,
                email TEXT NOT NULL,
                role TEXT NOT NULL,
                dept TEXT NOT NULL,
                status TEXT NOT NULL,
                last_login TEXT NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0))?;
        if count == 0 {
            let users = [
                (1001u32, "张明", "ZM", "zhang.ming@novaops.io", "超级管理员", "基础设施架构部", "Active", "2026-10-12 10:42"),
                (1002u32, "李思涵", "SH", "li.sihan@novaops.io", "架构与运维", "DevOps效能部", "Active", "2026-10-12 09:15"),
                (1003u32, "王建国", "JG", "wang.jg@novaops.io", "安全审计", "信息合规安全部", "Active", "2026-10-12 08:30"),
                (1004u32, "赵婷婷", "TT", "zhao.tt@novaops.io", "研发工程师", "支付与结算业务", "Active", "2026-10-11 18:22"),
                (1005u32, "陈志远", "ZY", "chen.zy@novaops.io", "研发工程师", "大数据分析中心", "Inactive", "2026-10-09 14:10"),
                (1006u32, "刘雨欣", "YX", "liu.yx@novaops.io", "安全审计", "风险合规组", "Suspended", "2026-10-05 11:00"),
            ];
            for (id, name, abbr, email, role, dept, status, login) in users {
                conn.execute(
                    "INSERT INTO users (id, name, avatar_abbr, email, role, dept, status, last_login)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![id, name, abbr, email, role, dept, status, login],
                )?;
            }
        }

        // 7. 集群监控节点表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS nodes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                role TEXT NOT NULL,
                ip TEXT NOT NULL,
                cpu_pct INTEGER NOT NULL,
                mem_pct INTEGER NOT NULL,
                disk_pct INTEGER NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM nodes", [], |r| r.get(0))?;
        if count == 0 {
            let nodes = [
                ("master-01 (K8s Control)", "Controller", "10.200.0.11", 28u8, 44u8, 35u8, "Healthy"),
                ("worker-01 (Compute)", "Worker", "10.200.1.21", 68u8, 82u8, 54u8, "Busy"),
                ("worker-02 (Compute)", "Worker", "10.200.1.22", 48u8, 61u8, 42u8, "Healthy"),
                ("db-cluster-01 (Primary)", "Database", "10.200.2.31", 42u8, 76u8, 68u8, "Healthy"),
                ("storage-ceph-01", "Storage", "10.200.3.41", 31u8, 48u8, 79u8, "Warning"),
            ];
            for (name, role, ip, cpu, mem, disk, status) in nodes {
                conn.execute(
                    "INSERT INTO nodes (name, role, ip, cpu_pct, mem_pct, disk_pct, status)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![name, role, ip, cpu, mem, disk, status],
                )?;
            }
        }

        // 8. 运维终端实时日志表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS terminal_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                time TEXT NOT NULL,
                level TEXT NOT NULL,
                src TEXT NOT NULL,
                msg TEXT NOT NULL,
                color_hex INTEGER NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM terminal_logs", [], |r| r.get(0))?;
        if count == 0 {
            let tlogs = [
                ("[10:48:02]", "INFO", "k8s-controller", "Reconciling deployment 'payment-service-v2' (desired: 6, current: 6)", 0x9ca3afu32),
                ("[10:48:05]", "METRIC", "prometheus-agent", "Node 'worker-02' cpu usage dropped from 72% to 48%", 0x10b981u32),
                ("[10:48:12]", "HEALTH", "consul-health", "Service check passed: redis-cluster-shard-01 (latency 1.8ms)", 0x06b6d4u32),
                ("[10:48:20]", "WARN", "ceph-storage", "OSD #14 disk latency spike detected (92ms), throttle applied", 0xf59e0bu32),
                ("[10:48:31]", "INFO", "envoy-ingress", "TLS cert auto-renewal check complete: *.novaops.internal (valid 84d)", 0x3b82f6u32),
                ("[10:48:45]", "METRIC", "otel-collector", "Exported 14,290 spans to clickhouse tracer in 12ms", 0x6366f1u32),
            ];
            for (time, level, src, msg, color) in tlogs {
                conn.execute(
                    "INSERT INTO terminal_logs (time, level, src, msg, color_hex)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![time, level, src, msg, color],
                )?;
            }
        }

        // 9. 合规审计日志表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY,
                operator TEXT NOT NULL,
                action TEXT NOT NULL,
                target TEXT NOT NULL,
                ip TEXT NOT NULL,
                time TEXT NOT NULL,
                level TEXT NOT NULL
            )",
            [],
        )?;

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM audit_logs", [], |r| r.get(0))?;
        if count == 0 {
            let alogs = [
                (88401u32, "张明 (Tech Lead)", "更新全局限流阈值策略", "api-gateway-v2 / 突发限制", "192.168.1.108", "2026-10-12 10:45:10", "INFO"),
                (88402u32, "自动化探针 Cron", "检测到异地备份冷存储同步完成", "backup-s3-bucket-east", "10.200.0.5", "2026-10-12 10:30:00", "INFO"),
                (88403u32, "WAF 防火墙引擎", "拦截非法 SQL 盲注与目录扫描", "/api/v1/auth/internal", "185.220.101.5", "2026-10-12 09:52:18", "WARN"),
                (88404u32, "李思涵 (DevOps)", "重启并滚动发布支付工作节点", "payment-worker-pod-04", "192.168.1.115", "2026-10-12 09:12:44", "INFO"),
                (88405u32, "王建国 (SecOps)", "挂起异常锁定用户账户", "user_id: 1006 (刘雨欣)", "192.168.1.120", "2026-10-11 16:40:22", "CRITICAL"),
            ];
            for (id, op, act, target, ip, time, level) in alogs {
                conn.execute(
                    "INSERT INTO audit_logs (id, operator, action, target, ip, time, level)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![id, op, act, target, ip, time, level],
                )?;
            }
        }

        // 10. 系统环境配置表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                theme_mode TEXT NOT NULL DEFAULT 'System',
                auto_backup INTEGER NOT NULL,
                two_factor_auth INTEGER NOT NULL,
                api_rate_limit INTEGER NOT NULL,
                email_alerts INTEGER NOT NULL,
                dingtalk_webhook INTEGER NOT NULL,
                debug_logging INTEGER NOT NULL
            )",
            [],
        )?;

        // 兼容升级已有表结构
        let _ = conn.execute(
            "ALTER TABLE system_settings ADD COLUMN theme_mode TEXT NOT NULL DEFAULT 'System'",
            [],
        );

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM system_settings", [], |r| r.get(0))?;
        if count == 0 {
            conn.execute(
                "INSERT INTO system_settings (id, theme_mode, auto_backup, two_factor_auth, api_rate_limit, email_alerts, dingtalk_webhook, debug_logging)
                 VALUES (1, 'System', 1, 1, 1, 1, 0, 0)",
                [],
            )?;
        }

        // 11. 初始化本地管理员凭证表 (Argon2id 密码校验)
        crate::services::auth::AuthService::init_auth_tables(&conn)?;

        Ok(())
    }

    pub fn load_kpi_cards(&self) -> Vec<KpiCardItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT title, value, trend, is_positive, subtext, icon, color_hex FROM kpi_cards ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(KpiCardItem {
                title: row.get(0)?,
                value: row.get(1)?,
                trend: row.get(2)?,
                is_positive: row.get::<_, i64>(3)? != 0,
                subtext: row.get(4)?,
                icon: row.get(5)?,
                color_hex: row.get(6)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_traffic_trends(&self, period: &str) -> Vec<TrafficBarItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT period, day_label, height_pct, val_label, is_highlight FROM traffic_trends WHERE period = ?1 ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map(params![period], |row| {
            Ok(TrafficBarItem {
                period: row.get(0)?,
                day_label: row.get(1)?,
                height_pct: row.get(2)?,
                val_label: row.get(3)?,
                is_highlight: row.get::<_, i64>(4)? != 0,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_resource_gauges(&self) -> Vec<ResourceGaugeItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT title, pct_text, pct_val, color_hex, desc FROM resource_gauges ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(ResourceGaugeItem {
                title: row.get(0)?,
                pct_text: row.get(1)?,
                pct_val: row.get(2)?,
                color_hex: row.get(3)?,
                desc: row.get(4)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_service_health(&self) -> Vec<ServiceHealthItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT name, role, port, status, latency, pods, color_hex FROM service_health ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(ServiceHealthItem {
                name: row.get(0)?,
                role: row.get(1)?,
                port: row.get(2)?,
                status: row.get(3)?,
                latency: row.get(4)?,
                pods: row.get(5)?,
                color_hex: row.get(6)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_live_events(&self) -> Vec<LiveEventItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT time, source, actor, desc, badge, color_hex FROM live_events ORDER BY id DESC LIMIT 6"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(LiveEventItem {
                time: row.get(0)?,
                source: row.get(1)?,
                actor: row.get(2)?,
                desc: row.get(3)?,
                badge: row.get(4)?,
                color_hex: row.get(5)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_users(&self) -> Vec<UserItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT id, name, avatar_abbr, email, role, dept, status, last_login FROM users ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            let status_str: String = row.get(6)?;
            Ok(UserItem {
                id: row.get(0)?,
                name: row.get(1)?,
                avatar_abbr: row.get(2)?,
                email: row.get(3)?,
                role: row.get(4)?,
                dept: row.get(5)?,
                status: UserStatus::from_str(&status_str),
                last_login: row.get(7)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_nodes(&self) -> Vec<NodeMetric> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT name, role, ip, cpu_pct, mem_pct, disk_pct, status FROM nodes ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            let status_str: String = row.get(6)?;
            Ok(NodeMetric {
                name: row.get(0)?,
                role: row.get(1)?,
                ip: row.get(2)?,
                cpu_pct: row.get(3)?,
                mem_pct: row.get(4)?,
                disk_pct: row.get(5)?,
                status: NodeStatus::from_str(&status_str),
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_terminal_logs(&self) -> Vec<TerminalLogItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT time, level, src, msg, color_hex FROM terminal_logs ORDER BY id ASC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(TerminalLogItem {
                time: row.get(0)?,
                level: row.get(1)?,
                src: row.get(2)?,
                msg: row.get(3)?,
                color_hex: row.get(4)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_audit_logs(&self) -> Vec<AuditLogItem> {
        let Ok(conn) = self.connect() else { return Vec::new() };
        let Ok(mut stmt) = conn.prepare(
            "SELECT id, operator, action, target, ip, time, level FROM audit_logs ORDER BY id DESC"
        ) else { return Vec::new() };

        stmt.query_map([], |row| {
            Ok(AuditLogItem {
                id: row.get(0)?,
                operator: row.get(1)?,
                action: row.get(2)?,
                target: row.get(3)?,
                ip: row.get(4)?,
                time: row.get(5)?,
                level: row.get(6)?,
            })
        })
        .map(|rows| rows.filter_map(Result::ok).collect())
        .unwrap_or_default()
    }

    pub fn load_settings(&self) -> SystemSettings {
        let Ok(conn) = self.connect() else { return SystemSettings::default() };
        conn.query_row(
            "SELECT theme_mode, auto_backup, two_factor_auth, api_rate_limit, email_alerts, dingtalk_webhook, debug_logging
             FROM system_settings WHERE id = 1",
            [],
            |row| {
                let theme_str: String = row.get(0).unwrap_or_else(|_| "System".to_string());
                Ok(SystemSettings {
                    theme_mode: ThemeMode::from_str(&theme_str),
                    auto_backup: row.get::<_, i64>(1)? != 0,
                    two_factor_auth: row.get::<_, i64>(2)? != 0,
                    api_rate_limit: row.get::<_, i64>(3)? != 0,
                    email_alerts: row.get::<_, i64>(4)? != 0,
                    dingtalk_webhook: row.get::<_, i64>(5)? != 0,
                    debug_logging: row.get::<_, i64>(6)? != 0,
                })
            },
        )
        .unwrap_or_default()
    }

    pub fn update_user_status(&self, user_id: u32, new_status: UserStatus) {
        if let Ok(conn) = self.connect() {
            let _ = conn.execute(
                "UPDATE users SET status = ?1 WHERE id = ?2",
                params![new_status.as_str(), user_id],
            );
        }
    }

    pub fn add_new_user(&self) -> Option<String> {
        let conn = self.connect().ok()?;
        let max_id: u32 = conn
            .query_row("SELECT COALESCE(MAX(id), 1000) FROM users", [], |r| r.get(0))
            .unwrap_or(1006);
        let next_id = max_id + 1;
        let name = format!("周浩然_{}", next_id - 1006);
        let email = format!("zhou.hr{}@novaops.io", next_id - 1006);

        conn.execute(
            "INSERT INTO users (id, name, avatar_abbr, email, role, dept, status, last_login)
             VALUES (?1, ?2, 'HR', ?3, '研发工程师', '云原生平台组', 'Active', '2026-10-12 11:00')",
            params![next_id, name, email],
        )
        .ok()?;

        Some(name)
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<crate::services::auth::AuthUser, String> {
        let conn = self.connect().map_err(|e| format!("数据库连接失败: {e}"))?;
        crate::services::auth::AuthService::authenticate(&conn, username, password)
    }

    pub fn create_custom_user(
        &self,
        name: &str,
        email: &str,
        role: &str,
        dept: &str,
        status: UserStatus,
    ) -> Result<u32, String> {
        let conn = self.connect().map_err(|e| format!("数据库连接失败: {e}"))?;
        let max_id: u32 = conn
            .query_row("SELECT COALESCE(MAX(id), 1000) FROM users", [], |r| r.get(0))
            .unwrap_or(1006);
        let next_id = max_id + 1;
        
        let abbr = if name.chars().count() >= 2 {
            name.chars().take(2).collect::<String>()
        } else {
            "UN".to_string()
        };

        conn.execute(
            "INSERT INTO users (id, name, avatar_abbr, email, role, dept, status, last_login)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now', 'localtime'))",
            params![next_id, name, abbr, email, role, dept, status.as_str()],
        )
        .map_err(|e| format!("插入用户失败: {e}"))?;

        Ok(next_id)
    }

    pub fn delete_user(&self, user_id: u32) -> Result<(), String> {
        let conn = self.connect().map_err(|e| format!("数据库连接失败: {e}"))?;
        conn.execute("DELETE FROM users WHERE id = ?1", params![user_id])
            .map_err(|e| format!("删除用户失败: {e}"))?;
        Ok(())
    }

    /// 执行全量数据快照热备份，返回 (备份文件路径, 时间戳字符串, 文件字节大小)
    pub fn create_backup(&self) -> Result<(String, String, u64), String> {
        let backup_dir = PathBuf::from("backups");
        std::fs::create_dir_all(&backup_dir).map_err(|e| format!("创建备份目录失败: {e}"))?;

        let conn = self.connect().map_err(|e| format!("数据库连接失败: {e}"))?;
        let now_tag: String = conn
            .query_row(
                "SELECT strftime('%Y%m%d_%H%M%S', 'now', 'localtime')",
                [],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "snapshot".to_string());
        let readable_time: String = conn
            .query_row(
                "SELECT strftime('%H:%M:%S', 'now', 'localtime')",
                [],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "刚刚".to_string());

        let backup_file = backup_dir.join(format!("novaops_backup_{now_tag}.db"));
        if backup_file.exists() {
            let _ = std::fs::remove_file(&backup_file);
        }
        std::fs::copy(&self.db_path, &backup_file).map_err(|e| format!("写入备份快照失败: {e}"))?;
        let size_bytes = std::fs::metadata(&backup_file).map(|m| m.len()).unwrap_or(0);

        Ok((
            backup_file.to_string_lossy().to_string(),
            readable_time,
            size_bytes,
        ))
    }

    /// 导出合规审计日志至本地 CSV 文件
    pub fn export_audit_logs_csv(&self) -> Result<String, String> {
        let backup_dir = PathBuf::from("backups");
        std::fs::create_dir_all(&backup_dir).map_err(|e| format!("创建导出目录失败: {e}"))?;
        let csv_path = backup_dir.join("audit_logs_export.csv");

        let logs = self.load_audit_logs();
        let mut content = String::from("事件ID,操作人,执行动作,关联目标,客户端IP,安全等级,发生时间\n");
        for item in logs {
            content.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                item.id, item.operator, item.action, item.target, item.ip, item.level, item.time
            ));
        }
        std::fs::write(&csv_path, content).map_err(|e| format!("导出 CSV 失败: {e}"))?;
        Ok(csv_path.to_string_lossy().to_string())
    }

    /// 校验本地存储完整性
    pub fn check_integrity(&self) -> Result<String, String> {
        let conn = self.connect().map_err(|e| format!("连接存储引擎失败: {e}"))?;
        let status: String = conn
            .query_row("PRAGMA integrity_check;", [], |r| r.get(0))
            .map_err(|e| format!("完整性校验异常: {e}"))?;
        Ok(status)
    }

    pub fn save_settings(&self, settings: &SystemSettings) {
        if let Ok(conn) = self.connect() {
            let _ = conn.execute(
                "UPDATE system_settings SET
                    theme_mode = ?1,
                    auto_backup = ?2,
                    two_factor_auth = ?3,
                    api_rate_limit = ?4,
                    email_alerts = ?5,
                    dingtalk_webhook = ?6,
                    debug_logging = ?7
                 WHERE id = 1",
                params![
                    settings.theme_mode.as_str(),
                    settings.auto_backup as i64,
                    settings.two_factor_auth as i64,
                    settings.api_rate_limit as i64,
                    settings.email_alerts as i64,
                    settings.dingtalk_webhook as i64,
                    settings.debug_logging as i64,
                ],
            );
        }
    }

    pub fn insert_audit_log(&self, operator: &str, action: &str, target: &str, level: &str) {
        if let Ok(conn) = self.connect() {
            let next_id: u32 = conn
                .query_row("SELECT COALESCE(MAX(id), 88400) + 1 FROM audit_logs", [], |r| r.get(0))
                .unwrap_or(88406);
            let _ = conn.execute(
                "INSERT INTO audit_logs (id, operator, action, target, ip, time, level)
                 VALUES (?1, ?2, ?3, ?4, '127.0.0.1', '2026-10-12 11:02:00', ?5)",
                params![next_id, operator, action, target, level],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_seed_and_load_all_tables() {
        let db = AppDatabase::new();
        assert_eq!(db.load_kpi_cards().len(), 4);
        assert_eq!(db.load_traffic_trends("7天").len(), 7);
        assert_eq!(db.load_traffic_trends("30天").len(), 7);
        assert_eq!(db.load_traffic_trends("季度").len(), 7);
        assert_eq!(db.load_resource_gauges().len(), 4);
        assert_eq!(db.load_service_health().len(), 5);
        assert_eq!(db.load_live_events().len(), 5);
        assert!(db.load_users().len() >= 6);
        assert_eq!(db.load_nodes().len(), 5);
        assert_eq!(db.load_terminal_logs().len(), 6);
        assert!(db.load_audit_logs().len() >= 5);
        let settings = db.load_settings();
        assert!(settings.two_factor_auth);
    }

    #[test]
    fn test_offline_argon2id_authentication() {
        let db = AppDatabase::new();
        // 正确密码验证
        let admin = db.authenticate("admin", "admin123").expect("admin 登录应当成功");
        assert_eq!(admin.username, "admin");
        assert_eq!(admin.role, "超级管理员");

        let ops = db.authenticate("ops", "ops123").expect("ops 登录应当成功");
        assert_eq!(ops.username, "ops");

        // 错误密码拦截
        assert!(db.authenticate("admin", "wrong_pass").is_err());
        assert!(db.authenticate("non_existent", "123456").is_err());
    }

    #[test]
    fn test_create_and_delete_custom_user() {
        let db = AppDatabase::new();
        let new_id = db
            .create_custom_user(
                "测试工程师",
                "qa.test@novaops.io",
                "研发工程师",
                "DevOps效能部",
                UserStatus::Active,
            )
            .expect("创建自定义用户失败");
        assert!(new_id > 1000);
        db.delete_user(new_id).expect("删除测试用户失败");
    }

    #[test]
    fn test_backup_export_and_integrity() {
        let db = AppDatabase::new();
        let integrity = db.check_integrity().expect("存储完整性检查应当成功");
        assert_eq!(integrity.to_lowercase(), "ok");

        let (backup_path, _time, size_bytes) = db.create_backup().expect("执行数据热备份应当成功");
        assert!(size_bytes > 0);
        let _ = std::fs::remove_file(backup_path);

        let csv_path = db.export_audit_logs_csv().expect("导出 CSV 应当成功");
        assert!(std::path::Path::new(&csv_path).exists());
    }
}

