#[macro_export]
macro_rules! go {
    ($func:expr) => {{
        <CrabGoInternalRuntime>::spawn_task(async move {
            $func;
        });
    }};
    ($func:expr, $channel:expr) => {{
        let sender_clone = $channel.0.clone();
        <CrabGoInternalRuntime>::spawn_task(async move {
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

#[cfg(feature = "rt-tokio")]
#[macro_export]
macro_rules! set_runtime {
    () => {
        pub use $crate::RuntimeTrait as CrabGoInternalRuntimeTrait;
        pub type CrabGoInternalRuntime = $crate::TokioRuntime;
    };
}

#[cfg(feature = "rt-async-std")]
#[macro_export]
macro_rules! set_runtime {
    () => {
        pub use $crate::RuntimeTrait as CrabGoInternalRuntimeTrait;
        pub type CrabGoInternalRuntime = $crate::AsyncStdRuntime;
    };
}

#[cfg(feature = "rt-native")]
#[macro_export]
macro_rules! set_runtime {
    () => {
        use $crate::RuntimeTrait as CrabGoInternalRuntimeTrait;
        type CrabGoInternalRuntime = $crate::NativeRuntime;
    };
}

#[cfg(feature = "rt-custom")]
#[macro_export]
macro_rules! set_runtime {
    ($t:ty) => {
        pub use $crate::RuntimeTrait as CrabGoInternalRuntimeTrait;
        pub type CrabGoInternalRuntime = $t;
    };
}
