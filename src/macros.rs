#[macro_export]
macro_rules! go {
    ($e:expr) => {
        <$crate::Runtime>::spawn_task(async move { $e })
    };
    ($b:block) => {
        <$crate::Runtime>::spawn_task(async move {
            {
                $b
            }
        })
    };
    ($func:expr, $channel:expr) => {{
        let sender_clone = $channel.0.clone();
        <$crate::Runtime>::spawn_task(async move {
            let result = $func;
            sender_clone.send(result).unwrap()
        })
    }};
}

#[macro_export]
macro_rules! recv {
    ($($channel:expr),+ $(,)?) => {
        ($( $channel.1.recv().unwrap(), )+)
    };
}
