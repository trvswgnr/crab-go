pub struct StdJoinHandle<T> {
    receiver: std::sync::mpsc::Receiver<T>,
}

impl<T> std::future::Future for StdJoinHandle<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.receiver.try_recv() {
            Ok(result) => std::task::Poll::Ready(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => std::task::Poll::Pending,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Thread has panicked"),
        }
    }
}

pub fn block_on<F: std::future::Future>(mut future: F) -> F::Output {
    let waker = noop_waker();
    let mut context = std::task::Context::from_waker(&waker);
    pin_mut(&mut future);
    loop {
        match unsafe { std::pin::Pin::new_unchecked(&mut future) }.poll(&mut context) {
            std::task::Poll::Ready(val) => return val,
            std::task::Poll::Pending => {
                // spin-wait, woof
                std::thread::yield_now();
            }
        }
    }
}

// pin a future manually
fn pin_mut<T>(value: &mut T) -> std::pin::Pin<&mut T> {
    unsafe { std::pin::Pin::new_unchecked(value) }
}

fn noop_waker() -> std::task::Waker {
    unsafe { std::task::Waker::from_raw(noop_raw_waker()) }
}

fn noop_raw_waker() -> std::task::RawWaker {
    fn noop_clone(_: *const ()) -> std::task::RawWaker {
        noop_raw_waker()
    }
    fn noop(_: *const ()) {}
    fn noop_wake(_: *const ()) {}
    fn noop_wake_by_ref(_: *const ()) {}

    std::task::RawWaker::new(
        std::ptr::null(),
        &std::task::RawWakerVTable::new(noop_clone, noop_wake, noop_wake_by_ref, noop),
    )
}

pub struct NativeRuntime;

impl crate::RuntimeTrait for NativeRuntime {
    type Handle<T> = StdJoinHandle<T>;

    fn spawn_task<F, T>(future: F) -> Self::Handle<T>
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = block_on(future);
            tx.send(result)
        });
        StdJoinHandle { receiver: rx }
    }
}
