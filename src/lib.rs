#[cfg(feature = "rt-tokio")]
pub mod rt_tokio;
#[cfg(feature = "rt-tokio")]
pub use rt_tokio::TokioRuntime;
#[cfg(feature = "rt-tokio")]
pub type Runtime = TokioRuntime;

#[cfg(feature = "rt-async-std")]
mod rt_async_std;
#[cfg(feature = "rt-async-std")]
pub use rt_async_std::AsyncStdRuntime;
#[cfg(feature = "rt-async-std")]
pub type Runtime = AsyncStdRuntime;

#[cfg(feature = "rt-native")]
mod rt_native;
#[cfg(feature = "rt-native")]
pub use rt_native::NativeRuntime;
#[cfg(feature = "rt-native")]
pub type Runtime = NativeRuntime;

pub trait RuntimeTrait {
    type Handle<U>;

    fn spawn_task<F, T>(f: F) -> Self::Handle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static;
}

pub mod macros;

pub mod prelude {
    pub use crate::RuntimeTrait as InternalRuntimeTrait;
}

pub fn channel<T>() -> (std::sync::mpsc::Sender<T>, std::sync::mpsc::Receiver<T>) {
    std::sync::mpsc::channel()
}
