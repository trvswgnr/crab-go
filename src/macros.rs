#[macro_export]
macro_rules! go {
    ($func:expr) => {{
        <$crate::Rt>::spawn(async move {
            $func;
        });
    }};
    ($func:expr, $channel:expr) => {{
        let sender_clone = $channel.0.clone();
        <$crate::Rt>::spawn(async move {
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

#[macro_export]
macro_rules! set_runtime {
    ($t:ty) => {
        type Rt = $t;
    };
}
