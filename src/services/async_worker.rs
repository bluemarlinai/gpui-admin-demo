use std::sync::OnceLock;
use tokio::runtime::Runtime;

static TOKIO_RT: OnceLock<Runtime> = OnceLock::new();

/// 获取全局共享的 Tokio 异步运行时，供后台 I/O 和计算密集型任务使用
pub fn get_tokio_runtime() -> &'static Runtime {
    TOKIO_RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("novaops-async-worker")
            .enable_all()
            .build()
            .expect("初始化后台 Tokio 运行时失败")
    })
}

/// 在后台 Tokio 线程池异步执行任务
pub fn spawn_background<F, R>(task: F)
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let rt = get_tokio_runtime();
    rt.spawn_blocking(task);
}
