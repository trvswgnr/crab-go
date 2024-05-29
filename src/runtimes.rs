use crate::lib::Runtime;
use std::future::Future;
use tokio::task::JoinHandle as TokioJoinHandle;

pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    type Handle<T> = TokioJoinHandle<T>;

    fn spawn<F, T>(future: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        tokio::spawn(future)
    }
}

use async_std::task::JoinHandle as AsyncStdJoinHandle;

pub struct AsyncStdRuntime;

impl Runtime for AsyncStdRuntime {
    type Handle<T> = AsyncStdJoinHandle<T>;

    fn spawn<F, T>(future: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::spawn(future)
    }
}

use std::pin::Pin;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;

pub struct StdJoinHandle<T> {
    receiver: Receiver<T>,
}

impl<T> Future for StdJoinHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.receiver.try_recv() {
            Ok(result) => Poll::Ready(result),
            Err(mpsc::TryRecvError::Empty) => Poll::Pending,
            Err(mpsc::TryRecvError::Disconnected) => panic!("Thread has panicked"),
        }
    }
}

pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);
    pin_mut(&mut future);
    loop {
        match unsafe { Pin::new_unchecked(&mut future) }.poll(&mut context) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // Spin-wait, as we're not using an async runtime
                std::thread::yield_now();
            }
        }
    }
}

// Helper function to pin a future manually
fn pin_mut<T>(value: &mut T) -> Pin<&mut T> {
    unsafe { Pin::new_unchecked(value) }
}

// Noop waker implementation
fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
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

pub struct StdRuntime;

impl Runtime for StdRuntime {
    type Handle<T> = StdJoinHandle<T>;

    fn spawn<F, T>(future: F) -> Self::Handle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let result = block_on(future);
            tx.send(result).unwrap();
        });
        StdJoinHandle { receiver: rx }
    }
}
