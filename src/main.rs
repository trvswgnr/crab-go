use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

macro_rules! go {
    ($func:expr) => {{
        thread::spawn(move || {
            $func;
        });
    }};
    ($func:expr, $channel:expr) => {{
        let sender_clone = $channel.0.clone();
        thread::spawn(move || {
            let result = $func;
            sender_clone.send(result)
        });
    }};
}

macro_rules! recv {
    ($($channel:expr),+ $(,)?) => {
        ($( $channel.1.recv().unwrap(), )+)
    };
}

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
}
