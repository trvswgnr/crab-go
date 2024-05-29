// use crate::{
//     _type::Type,
//     mutex::GoMutex,
//     runtime2::Sudog,
//     timer::Timer,
//     util::{int, int_to_uintptr, uint, uintptr, uintptr_to_int, FromGoPtr, GoPtr, UnsafePointer},
// };

// pub struct Hchan {
//     qcount: uint,
//     dataqsiz: uint,
//     buf: UnsafePointer,
//     elemsize: u16,
//     closed: u32,
//     timer: GoPtr<Timer>,
//     elemtype: GoPtr<Type>,
//     sendx: uint,
//     recvx: uint,
//     recvq: Waitq,
//     sendq: Waitq,
//     lock: GoMutex,
// }

// impl Hchan {
//     fn new() -> Self {
//         Self {
//             qcount: 0,
//             dataqsiz: 0,
//             buf: UnsafePointer::null(),
//             elemsize: 0,
//             closed: 0,
//             timer: GoPtr::null(),
//             elemtype: GoPtr::null(),
//             sendx: 0,
//             recvx: 0,
//             recvq: Waitq::new(),
//             sendq: Waitq::new(),
//             lock: GoMutex::new(),
//         }
//     }
// }

// struct Waitq {
//     first: GoPtr<Sudog>,
//     last: GoPtr<Sudog>,
// }

// impl Waitq {
//     fn new() -> Self {
//         Self {
//             first: GoPtr::null(),
//             last: GoPtr::null(),
//         }
//     }
// }

// const MAX_ALIGN: usize = 8;
// const HCHAN_SIZE: usize = std::mem::size_of::<Hchan>()
//     + int_to_uintptr(-uintptr_to_int(std::mem::size_of::<Hchan>()) & ((MAX_ALIGN - 1) as int));
// const DEBUG_CHAN: bool = false;

// pub struct ChanType {
//     pub t: Type,
//     pub elem: GoPtr<Type>,
//     pub dir: ChanDir,
// }

// type ChanDir = isize;

// fn reflect_makechan(t: GoPtr<ChanType>, size: int) -> GoPtr<Hchan> {
//     makechan(t, size)
// }

// fn makechan64(t: GoPtr<ChanType>, size: i64) -> GoPtr<Hchan> {
//     if ((size as int) as i64) != size {
//         panic!("makechan: size out of range")
//     }
//     makechan(t, size as int)
// }

// fn makechan(t: GoPtr<ChanType>, size: int) -> GoPtr<Hchan> {
//     let elem = t.as_ref().elem.as_ref();

//     if elem.size >= 1 << 16 {
//         panic!("makechan: invalid channel element type");
//     }

//     if HCHAN_SIZE % MAX_ALIGN != 0 || elem.align > MAX_ALIGN {
//         panic!("makechan: bad alignment");
//     }

//     let c = Hchan::new();
// }
