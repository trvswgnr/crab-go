#[cfg(feature = "rt-async-std")]
mod async_std;
#[cfg(feature = "rt-async-std")]
pub use async_std::AsyncStdRuntime;

#[cfg(feature = "rt-tokio")]
mod tokio;
#[cfg(feature = "rt-tokio")]
pub use tokio::TokioRuntime;

#[cfg(feature = "rt-native")]
mod native;
#[cfg(feature = "rt-native")]
pub use native::NativeRuntime;

mod macros;

use std::future::Future;

pub trait RuntimeTrait {
    type Handle<U>;

    fn spawn_task<F, T>(f: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}
