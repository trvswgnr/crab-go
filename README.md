# Crab-Go

> [!CAUTION]
> This project is still in early development and is not even close to ready for production use.

## Overview
Crab-Go is a Rust project that aims to simplify common concurrency patterns and utilities inspired by Go's channels and goroutines. It provides a set of tools to facilitate concurrent programming in Rust, mimicking Go's behavior for educational and practical purposes.

## Usage

Currently, the main feature is the `go!` macro. It allows easy execution across
threads.

First, add the dependency to your `Cargo.toml` and enable the flag for the runtime you want to use:

```toml
[dependencies]
crab-go = { git = "https://github.com/trvswgnr/crab-go", features = ["rt-tokio"] }
```

The available features are:
- `rt-tokio`: Uses the Tokio runtime.
- `rt-async-std`: Uses the async-std runtime.
- `rt-native`: Uses native threads.
- `rt-custom`: Use a custom runtime that you implement `RuntimeTrait` for.

Then, import the necessary modules and functions:

```rust
use crag_go::prelude::*;
use crab_go::{go, recv, channel};

fn say(s: &str) {
    for _ in 0..5 {
      println!("{s}");
      std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn example_1() {
    go!(say("world"));
    say("hello");
}

fn example_2() {
    let c = channel();
    let first_half = vec![7, 2, 8];
    let second_half = vec![-9, 4, 0];
    go!(sum(&first_half), c);
    go!(sum(&second_half), c);
    let (x, y) = recv!(c);
    println!("{} {} {}", x, y, x + y);
}

#[tokio::main]
async fn main() {
    example_1();
    example_2();
}

pub fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

pub fn say(s: &str, delay: u64) {
    for _ in 0..5 {
        std::thread::sleep(std::time::Duration::from_millis(delay));
        println!("{s}");
    }
}
```

## Contributing
Contributions are welcome! Please feel free to submit pull requests or open issues to discuss potential improvements or additions to the project.

## License
Crab-Go is distributed under the MIT license. See the LICENSE file for more details.