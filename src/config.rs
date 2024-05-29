use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::lib::Runtime;

#[macro_export]
macro_rules! set_runtime {
    ($t:ty) => {
        type Rt = $t;
    };
}

struct DummyRuntime;
impl DummyRuntime {
    fn new() -> Self {
        Self
    }
}
