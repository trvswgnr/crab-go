use std::{
    any::Any,
    error::Error,
    ops::{Deref, DerefMut},
};

pub struct UintPtr<T>(*mut T);

pub type AnyPtr = Box<dyn Any>;

pub fn to_uintptr<T: Any>(v: &T) -> UintPtr<T> {
    let trait_object: &dyn Any = v;
    UintPtr(trait_object as *const dyn Any as *mut T)
}

pub fn from_uintptr<'a, T: Any>(v: UintPtr<T>) -> &'a T {
    unsafe { &*(v.0 as *const T) }
}

trait ToUintPtr<T> {
    fn to_uintptr(&self) -> UintPtr<T>;
}

trait FromUintPtr<T> {
    fn as_ref<'a>(self) -> &'a T;
}

impl<T: 'static> FromUintPtr<T> for UintPtr<T> {
    fn as_ref<'a>(self) -> &'a T {
        from_uintptr(self)
    }
}

impl<T> Deref for UintPtr<Box<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const T) }
    }
}

impl<T: 'static> FromUintPtr<T> for UintPtr<Box<T>> {
    fn as_ref<'a>(self) -> &'a T {
        from_uintptr(UintPtr(&(*self) as *const T as *mut T))
    }
}

impl<T: 'static> ToUintPtr<T> for T {
    fn to_uintptr(&self) -> UintPtr<T> {
        to_uintptr(self)
    }
}

trait ToAnyPtr<T> {
    fn to_anyptr(&self) -> AnyPtr;
}

trait FromAnyPtr {
    fn to<T: 'static>(&self) -> Option<&T>;
}

impl FromAnyPtr for AnyPtr {
    fn to<T: 'static>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
    }
}

impl<T: 'static + Clone> ToAnyPtr<T> for T {
    fn to_anyptr(&self) -> AnyPtr {
        let boxed: Box<dyn Any> = Box::new(self.clone());
        boxed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uintptr() {
        let value = 10;
        let any_ptr = value.to_uintptr();
        let val = any_ptr.as_ref();
        assert_eq!(*val, 10);

        let value = vec![1, 2, 3];
        let any_ptr = value.to_uintptr();
        let val = any_ptr.as_ref();
        assert_eq!(*val, vec![1, 2, 3]);
    }

    #[test]
    fn test_to_uintptr_from_box() {
        let boxed_value = Box::new(42);
        let any_ptr = boxed_value.to_uintptr();
        let val = from_uintptr(any_ptr);
        assert_eq!(*val, Box::new(42));
    }

    #[test]
    fn test_to_uintptr_from_any() {
        fn f(arg: AnyPtr, seq: AnyPtr, delay: i64) -> String {
            let arg = arg.to::<i32>();
            let seq = seq.to::<i32>();
            assert_eq!(arg, Some(&10));
            assert_eq!(seq, Some(&20));
            format!("arg: {:?}, seq: {:?}", arg.unwrap(), seq.unwrap())
        }

        let arg = 10.to_anyptr();
        let seq = 20.to_anyptr();
        let delay = 30;
        let result = f(arg, seq, delay);
        assert_eq!(result, "arg: 10, seq: 20");
    }
}
