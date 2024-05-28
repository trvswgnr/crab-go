// /*
// equivalent go code:

// ```go
// type timer struct {
//     // mu protects reads and writes to all fields, with exceptions noted below.
//     mu mutex

//     astate  atomic.Uint8 // atomic copy of state bits at last unlock
//     state   uint8        // state bits
//     isChan  bool         // timer has a channel; immutable; can be read without lock
//     blocked uint32       // number of goroutines blocked on timer's channel

//     // Timer wakes up at when, and then at when+period, ... (period > 0 only)
//     // each time calling f(arg, seq, delay) in the timer goroutine, so f must be
//     // a well-behaved function and not block.
//     //
//     // The arg and seq are client-specified opaque arguments passed back to f.
//     // When used from netpoll, arg and seq have meanings defined by netpoll
//     // and are completely opaque to this code; in that context, seq is a sequence
//     // number to recognize and squech stale function invocations.
//     // When used from package time, arg is a channel (for After, NewTicker)
//     // or the function to call (for AfterFunc) and seq is unused (0).
//     //
//     // Package time does not know about seq, but if this is a channel timer (t.isChan == true),
//     // this file uses t.seq as a sequence number to recognize and squelch
//     // sends that correspond to an earlier (stale) timer configuration,
//     // similar to its use in netpoll. In this usage (that is, when t.isChan == true),
//     // writes to seq are protected by both t.mu and t.sendLock,
//     // so reads are allowed when holding either of the two mutexes.
//     //
//     // The delay argument is nanotime() - t.when, meaning the delay in ns between
//     // when the timer should have gone off and now. Normally that amount is
//     // small enough not to matter, but for channel timers that are fed lazily,
//     // the delay can be arbitrarily long; package time subtracts it out to make
//     // it look like the send happened earlier than it actually did.
//     // (No one looked at the channel since then, or the send would have
//     // not happened so late, so no one can tell the difference.)
//     when   int64
//     period int64
//     f      func(arg any, seq uintptr, delay int64)
//     arg    any
//     seq    uintptr

//     // If non-nil, the timers containing t.
//     ts *timers

//     // sendLock protects sends on the timer's channel.
//     // Not used for async (pre-Go 1.23) behavior when debug.asynctimerchan.Load() != 0.
//     sendLock mutex
// }
// ```
// */

// use crate::{
//     mutex::GoMutex,
//     util::{AnyPtr, UintPtr},
// };
// use std::{
//     any::Any,
//     ops::{Deref, DerefMut},
//     sync::atomic::{AtomicI64, AtomicU32, AtomicU8, Ordering},
// };

// pub struct Timer {
//     mu: GoMutex,

//     astate: AtomicU8,
//     state: u8,
//     is_chan: bool,
//     blocked: u32,

//     when: i64,
//     period: i64,
//     f: fn(arg: AnyPtr, seq: AnyPtr, delay: i64) -> (),
//     arg: AnyPtr,
//     seq: AnyPtr,

//     ts: *mut Timers,

//     send_lock: GoMutex,
// }

// struct Timers {
//     mu: GoMutex,
//     heap: Vec<TimerWhen>,
//     len: AtomicU32,
//     zombies: AnyPtr,
//     race_ctx: AnyPtr,
//     min_when_heap: AtomicI64,
//     min_when_modified: AtomicI64,
// }

// struct TimerWhen {
//     timer: *mut Timer,
//     when: i64,
// }
