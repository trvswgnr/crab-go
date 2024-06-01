pub struct AsyncStdRuntime;

impl crate::RuntimeTrait for AsyncStdRuntime {
    type Handle<T> = async_std::task::JoinHandle<T>;

    fn spawn_task<F, T>(future: F) -> Self::Handle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::spawn(future)
    }
}
