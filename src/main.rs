mod _type;
mod runtime2;
mod chan;
mod mutex;
mod timer;
mod util;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

macro_rules! go {
    ($expr:expr) => {
        thread::spawn(move || $expr);
    };
}

// macro_rules! send {
//     ($ch:expr, $v:expr) => {
//         $ch.0.send($v).unwrap();
//     };
// }

fn send<T>(ch: &Chan<T>, v: T) {
    ch.lock().unwrap().0.send(v).unwrap();
}

fn recv<T>(ch: &Chan<T>) -> T {
    ch.lock().unwrap().1.try_recv().unwrap()
}

macro_rules! recv {
    ($ch:expr) => {
        $ch.1.recv().unwrap();
    };
}

macro_rules! defer {
    ($e:expr) => {
        struct Guard<F: FnOnce()>(Option<F>);
        impl<F: FnOnce()> Drop for Guard<F> {
            fn drop(&mut self) {
                (self.0.take().unwrap())();
            }
        }
        let _guard = Guard(Some($e));
    };
}

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

fn print_numbers(prefix: &str, wg: Arc<WaitGroup>) {
    defer! { || wg.done() };
    for i in 0..5 {
        println!("{}{}", prefix, i);
        thread::sleep(Duration::from_secs(1));
    }
}

type Chan<T> = Mutex<(Sender<T>, Receiver<T>)>;

fn sum(s: &[i32], c: &Chan<i32>) {
    let mut sum = 0;
    for v in s.iter() {
        sum += v;
    }
    send(c, sum);
}

fn main() {
    // let s = Arc::new(vec![7, 2, 8, -9, 4, 0]);

    // let c = Arc::new(Mutex::new(channel::<i32>()));
    // let c_clone = c.clone();
    // let s_clone = s.clone();
    // go! {
    //     sum(&s_clone[0..s_clone.len() / 2], &c_clone)
    // };

    // let c_clone = c.clone();
    // let s_clone = s.clone();
    // go! {
    //     sum(&s_clone[s_clone.len()/2..], &c_clone)
    // };

    // let x = recv(&c);
    // let y = recv(&c);

    // println!("{x} {y} {}", x + y);

    // let wg = Arc::new(WaitGroup::new());

    // // Start two routines
    // wg.add(2);
    // let wg_clone = wg.clone();
    // go! {
    //     print_numbers("routine 1:", wg_clone);
    // };
    // let wg_clone = wg.clone();
    // go! {
    //     print_numbers("routine 2:", wg_clone);
    // };

    // // wait for all to finish
    // wg.wait();
    // println!("main routine finished");
}
