use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex as StdMutex;

pub struct GoMutex {
    lock: StdMutex<()>,
    is_locked: AtomicBool,
}

impl GoMutex {
    pub fn new() -> Self {
        Self {
            lock: StdMutex::new(()),
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> GoMutexGuard {
        while let Err(_) = self.lock.try_lock() {
            // spin-wait, woof
            std::thread::yield_now();
        }
        self.is_locked.store(true, Ordering::SeqCst);
        GoMutexGuard { mutex: &self }
    }

    fn unlock(&self) {
        if self.is_locked.load(Ordering::SeqCst) {
            self.is_locked.store(false, Ordering::SeqCst);
            drop(self.lock.lock().unwrap()); // unlock the mutex
        }
    }
}

pub struct GoMutexGuard<'a> {
    mutex: &'a GoMutex,
}

impl<'a> Drop for GoMutexGuard<'a> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
