// type Type struct {
// 	Size_       uintptr
// 	PtrBytes    uintptr // number of (prefix) bytes in the type that can contain pointers
// 	Hash        uint32  // hash of type; avoids computation in hash tables
// 	TFlag       TFlag   // extra type information flags
// 	Align_      uint8   // alignment of variable with this type
// 	FieldAlign_ uint8   // alignment of struct field with this type
// 	Kind_       Kind    // enumeration for C
// 	// function for comparing objects of this type
// 	// (ptr to object A, ptr to object B) -> ==?
// 	Equal func(unsafe.Pointer, unsafe.Pointer) bool
// 	// GCData stores the GC type data for the garbage collector.
// 	// If the KindGCProg bit is set in kind, GCData is a GC program.
// 	// Otherwise it is a ptrmask bitmap. See mbitmap.go for details.
// 	GCData    *byte
// 	Str       NameOff // string form
// 	PtrToThis TypeOff // type for pointer to this type, may be zero
// }

use crate::util::{uintptr, AnyPtr, Byte, GoPtr, UnsafePointer};

pub struct Type {
    pub size: uintptr,
    pub ptr_bytes: uintptr,
    pub hash: u32,
    pub tflag: TFlag,
    pub align: u8,
    pub field_align: u8,
    pub kind: Kind,
    pub equal: fn(UnsafePointer, UnsafePointer) -> bool,
    pub gc_data: GoPtr<Byte>,
    pub str: NameOff,
    pub ptr_to_this: TypeOff,
}

type Kind = u8;

type TFlag = u8;

type NameOff = i32;

type TypeOff = i32;
