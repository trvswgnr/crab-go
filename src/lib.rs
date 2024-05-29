use std::any::Any;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;

impl Runtime for Box<dyn Any> {
    type Handle<U> = ();

    fn spawn<F, T>(f: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
    }
}

pub trait Runtime: Any {
    type Handle<U>;

    fn spawn<F, T>(f: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}

#[macro_export]
macro_rules! go {
    ($func:expr) => {{
        std::thread::spawn(move || {
            $func;
        });
    }};
    ($func:expr, $channel:expr) => {{
        let sender_clone = $channel.0.clone();
        std::thread::spawn(move || {
            let result = $func;
            sender_clone.send(result)
        });
    }};
}

#[macro_export]
macro_rules! recv {
    ($($channel:expr),+ $(,)?) => {
        ($( $channel.1.recv().unwrap(), )+)
    };
}
