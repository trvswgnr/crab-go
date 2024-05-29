use std::{sync::mpsc::channel, thread, time::Duration};

mod config;
mod lib;
mod runtimes;

fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn say(s: &str) {
    for _ in 0..5 {
        thread::sleep(Duration::from_millis(10));
        println!("{s}");
    }
}

fn example_1() {
    go!(say("world"));
    say("hello");
}

fn example_2() {
    let numbers = vec![7, 2, 8, -9, 4, 0];
    let c = channel();

    let first_half = numbers[..numbers.len() / 2].to_vec();
    let second_half = numbers[numbers.len() / 2..].to_vec();

    go!(sum(&first_half), c);
    go!(sum(&second_half), c);

    let (x, y) = recv!(c, c);

    println!("{} {} {}", x, y, x + y);
}

fn main() {
    example_1();
    example_2();
    std::thread::spawn(move || {});
}
