pub struct TokioRuntime;
use std::sync::OnceLock;

fn get_or_create_runtime(
    runtime: Option<tokio::runtime::Runtime>,
) -> &'static tokio::runtime::Runtime {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| runtime.unwrap())
}

impl crate::RuntimeTrait for TokioRuntime {
    type Handle<T> = tokio::task::JoinHandle<T>;

    fn spawn_task<F, T>(future: F) -> Self::Handle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        tokio::spawn(future)
    }
}
