// // const (
// // 	maxAlign  = 8
// // 	hchanSize = unsafe.Sizeof(hchan{}) + uintptr(-int(unsafe.Sizeof(hchan{}))&(maxAlign-1))
// // 	debugChan = false
// // )

// /*
// equivalent go code:
// ```go
// type hchan struct {
//     qcount   uint           // total data in the queue
//     dataqsiz uint           // size of the circular queue
//     buf      unsafe.Pointer // points to an array of dataqsiz elements
//     elemsize uint16
//     closed   uint32
//     timer    *timer // timer feeding this chan
//     elemtype *_type // element type
//     sendx    uint   // send index
//     recvx    uint   // receive index
//     recvq    waitq  // list of recv waiters
//     sendq    waitq  // list of send waiters

//     // lock protects all fields in hchan, as well as several
//     // fields in sudogs blocked on this channel.
//     //
//     // Do not change another G's status while holding this lock
//     // (in particular, do not ready a G), as this can deadlock
//     // with stack shrinking.
//     lock mutex
// }
// ```
// */

// use std::sync::Mutex;

// type uint = usize;

// struct Hchan<T> {
//     qcount: uint,
//     dataqsiz: uint,
//     buf: *mut u8,
//     elemsize: u16,
//     closed: u32,
//     timer: &Timer,
//     elemtype: &_Type,
//     sendx: uint,
//     recvx: uint,
//     recvq: Waitq,
//     sendq: Waitq,
//     lock: Mutex<T>,
// }



// const MAX_ALIGN: usize = 8;
// const HCHAN_SIZE: usize = unsafe { std::mem::size_of::<hchan>() };
// const CHAN_SIZE: usize = HCHAN_SIZE + (MAX_ALIGN - HCHAN_SIZE as usize % MAX_ALIGN) as usize;
