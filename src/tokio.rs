pub struct TokioRuntime;

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
