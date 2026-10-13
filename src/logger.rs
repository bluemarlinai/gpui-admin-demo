use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 初始化桌面端结构化日志记录器
/// - 输出至终端控制台 (开发实时诊断)
/// - 滚动持久化至本地文件 (logs/novaops.log)
pub fn init_logger() -> Option<WorkerGuard> {
    let log_dir = PathBuf::from("logs");
    let file_appender = tracing_appender::rolling::daily(log_dir, "novaops.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,gpui_admin_demo=debug"));

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_target(false)
        .compact();

    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_target(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init()
        .ok()?;

    tracing::info!("桌面应用结构化日志系统初始化就绪");
    Some(guard)
}
