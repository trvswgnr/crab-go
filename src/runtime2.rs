// use crate::{
//     chan::Hchan,
//     util::{GoPtr, UnsafePointer},
// };

// pub struct Sudog {
//     next: GoPtr<Sudog>,
//     prev: GoPtr<Sudog>,
//     elem: UnsafePointer,
//     acquiretime: i64,
//     releasetime: i64,
//     ticket: u32,
//     is_select: bool,
//     success: bool,
//     waiters: u16,
//     parent: GoPtr<Sudog>,
//     waitlink: GoPtr<Sudog>,
//     waittail: GoPtr<Sudog>,
//     c: GoPtr<Hchan>,
// }
