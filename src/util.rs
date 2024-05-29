use std::{any::Any, ops::Deref};

// uintptr is an integer type that is large enough to hold the bit pattern of
// any pointer.
#[allow(non_camel_case_types)]
pub type uintptr = usize;

pub const fn int_to_uintptr(ptr: int) -> uintptr {
    ptr as uintptr
}

pub type ArbitraryType = int;

pub const fn uintptr_to_int(ptr: uintptr) -> int {
    ptr as int
}

#[allow(non_camel_case_types)]
pub type uint = usize;

#[allow(non_camel_case_types)]
pub type int = isize;

pub struct GoPtr<T>(*mut T);

impl<T> GoPtr<T> {
    pub const fn null() -> Self {
        GoPtr(std::ptr::null::<T>() as *mut T)
    }
}

#[allow(non_camel_case_types)]
pub type any = *mut dyn Any;

pub type UnsafePointer = GoPtr<any>;

pub type AnyPtr = GoPtr<any>;

pub type Byte = u8;

pub fn to_uintptr<T: Any>(v: &T) -> GoPtr<T> {
    let trait_object: &dyn Any = v;
    GoPtr(trait_object as *const dyn Any as *mut T)
}

pub fn from_uintptr<'a, T: Any>(v: GoPtr<T>) -> &'a T {
    unsafe { &*(v.0 as *const T) }
}

pub trait ToGoPtr<T> {
    fn to_goptr(&self) -> GoPtr<T>;
}

pub trait FromGoPtr<T> {
    fn as_ref<'a>(self) -> &'a T;
}

impl<T: 'static> FromGoPtr<T> for GoPtr<T> {
    fn as_ref<'a>(self) -> &'a T {
        from_uintptr(self)
    }
}

impl<T> Deref for GoPtr<Box<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const T) }
    }
}

impl<T: 'static> FromGoPtr<T> for GoPtr<Box<T>> {
    fn as_ref<'a>(self) -> &'a T {
        from_uintptr(GoPtr(&(*self) as *const T as *mut T))
    }
}

impl<T: 'static> ToGoPtr<T> for T {
    fn to_goptr(&self) -> GoPtr<T> {
        to_uintptr(self)
    }
}

trait ToAnyPtr<T> {
    fn as_any(&self) -> any;
}

trait FromAnyPtr {
    fn to<T: 'static>(&self) -> &T;
}

impl FromAnyPtr for any {
    fn to<T: 'static>(&self) -> &T {
        unsafe {
            // Assume that the pointer points to a Box<dyn Any>
            let boxed_any = &**self as &dyn Any;
            // Try to downcast the Box<dyn Any> to the specific type T
            if let Some(downcasted) = boxed_any.downcast_ref::<T>() {
                downcasted
            } else {
                // Handle the case where the downcast is not possible
                panic!("Failed to downcast from any to the specified type")
            }
        }
    }
}

pub trait NamedAny: Any {
    fn type_name() -> &'static str;
}

impl<T: Any> NamedAny for T {
    fn type_name() -> &'static str {
        std::any::type_name::<T>()
    }
}

impl<T: 'static + Clone> ToAnyPtr<T> for T {
    fn as_any(&self) -> any {
        let boxed = Box::new(self.clone());
        Box::into_raw(boxed) as any
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uintptr() {
        let value = 10;
        let any_ptr = value.to_goptr();
        let val = any_ptr.as_ref();
        assert_eq!(*val, 10);

        let value = vec![1, 2, 3];
        let any_ptr = value.to_goptr();
        let val = any_ptr.as_ref();
        assert_eq!(*val, vec![1, 2, 3]);
    }

    #[test]
    fn test_to_uintptr_from_box() {
        let boxed_value = Box::new(42);
        let any_ptr = boxed_value.to_goptr();
        let val = from_uintptr(any_ptr);
        assert_eq!(*val, Box::new(42));
    }

    #[test]
    fn test_to_uintptr_from_any() {
        fn f(arg: any, seq: any, delay: i64) -> String {
            let arg = arg.to::<i32>();
            let seq = seq.to::<i32>();
            format!("arg: {:?}, seq: {:?}, delay: {:?}", arg, seq, delay)
        }

        let arg = 10.as_any();
        let seq = 20.as_any();
        let delay = 30;
        let result = f(arg, seq, delay);
        assert_eq!(result, "arg: 10, seq: 20, delay: 30");
    }
}
