use crate::{mutex::GoMutex, util::any};
use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU8};

pub struct Timer {
    mu: GoMutex,

    astate: AtomicU8,
    state: u8,
    is_chan: bool,
    blocked: u32,

    when: i64,
    period: i64,
    f: fn(arg: any, seq: any, delay: i64) -> (),
    arg: any,
    seq: any,

    ts: *mut Timers,

    send_lock: GoMutex,
}

struct Timers {
    mu: GoMutex,
    heap: Vec<TimerWhen>,
    len: AtomicU32,
    zombies: any,
    race_ctx: any,
    min_when_heap: AtomicI64,
    min_when_modified: AtomicI64,
}

struct TimerWhen {
    timer: *mut Timer,
    when: i64,
}
