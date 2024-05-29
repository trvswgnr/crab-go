# `crab-go` Project

> [!CAUTION]
> This project is still in early development and is not even close to ready for production use.

## Overview
`crab-go` is a Rust project that aims to implement concurrency patterns and utilities inspired by Go's channels and goroutines. It provides a set of tools to facilitate concurrent programming in Rust, mimicking Go's behavior for educational and practical purposes.

## Features
- **Channels**: Safe communication between threads.
- **Mutexes**: Custom mutex implementation mimicking Go's mutex behavior.
- **Timers**: Timer utilities for managing timed operations.
- **WaitGroups**: Synchronization aid that allows one goroutine to wait for multiple goroutines to finish.
- **Utilities**: Helper functions and types for pointer and type conversions.

## Usage

### Channels
Channels allow passing messages between threads. The `Chan` type is a thread-safe communication channel.

```rust
fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    println!("Received: {}", rx.recv().unwrap());
}
```

### WaitGroups
WaitGroups are used to wait for a collection of goroutines to finish executing.

```rust
struct WaitGroup {
    counter: Arc<Mutex<i32>>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    fn add(&self, delta: i32) {
        let mut counter = self.counter.lock().unwrap();
        *counter += delta;
    }

    fn done(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1;
    }

    fn wait(&self) {
        loop {
            let counter = self.counter.lock().unwrap();
            if *counter <= 0 {
                break;
            }
        }
    }
}
```

### Mutexes
Custom mutex implementation that mimics Go's mutex behavior, including a spin-wait lock.

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex as StdMutex;

pub struct GoMutex {
    lock: StdMutex<()>,
    is_locked: AtomicBool,
}

impl GoMutex {
    pub fn new() -> Self {
        GoMutex {
            lock: StdMutex::new(()),
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> GoMutexGuard {
        while let Err(_) = self.lock.try_lock() {
            // Spin-wait here to mimic Go's blocking behavior
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
```

### Timers
Coming soon!

## Installation
Add `crab-go` to your Rust project by including it in your `Cargo.toml`:

```toml
[dependencies]
crab-go = { git = "https://github.com/trvswgnr/crab-go" }
```

## Contributing
Contributions are welcome! Please feel free to submit pull requests or open issues to discuss potential improvements or additions to the project.

## License
`crab-go` is distributed under the MIT license. See the LICENSE file for more details.