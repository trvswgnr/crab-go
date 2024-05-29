use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::lib::Runtime;

static mut CONFIG: Mutex<Option<Config>> = Mutex::new(None);

#[derive(Debug, PartialEq)]
pub enum RuntimeType {
    AsyncStdRuntime,
    StdRuntime,
    TokioRuntime,
    Other(*mut dyn Any),
}

impl RuntimeType {
    pub fn new<R: Runtime>(r: R) -> Self {
        Self::Other(Box::into_raw(Box::new(r)))
    }

    pub fn get<'a, R: Runtime>(&'a self) -> &'a R {
        match self {
            Self::Other(x) => from_raw(*x),
            _ => todo!(),
        }
    }
}

fn from_raw<'a, R: Runtime>(x: *mut dyn Any) -> &'a R {
    unsafe {
        let any = &*x;
        if let Some(runtime) = any.downcast_ref::<R>() {
            runtime
        } else {
            panic!("Failed to downcast from *mut dyn Any to the specific Runtime type")
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub runtime: RuntimeType,
}

impl Config {
    pub fn get_runtime<'a, R: Runtime>(&'a self) -> &'a R {
        self.runtime.get()
    }
}

pub fn get_config() -> Config {
    unsafe {
        let mut conf = CONFIG.lock().unwrap();
        if conf.is_none() {
            *conf = Some(Config {
                runtime: RuntimeType::StdRuntime,
            });
        }
        conf.take().unwrap()
    }
}

pub fn set_config(config: Config) {
    unsafe {
        *CONFIG.lock().unwrap() = Some(config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct DummyRuntime;

    impl Runtime for DummyRuntime {
        type Handle<U> = ();

        fn spawn<F, T>(f: F) -> Self::Handle<T> {
            ()
        }
    }

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert_eq!(config.runtime, RuntimeType::StdRuntime);
        set_config(Config {
            runtime: RuntimeType::AsyncStdRuntime,
        });
        let config = get_config();
        assert_eq!(config.runtime, RuntimeType::AsyncStdRuntime);
        set_config(Config {
            runtime: RuntimeType::new(DummyRuntime),
        });
        let config = get_config();
        assert_eq!(config.runtime, RuntimeType::new(DummyRuntime));
    }

    #[test]
    fn test_get_runtime() {
        set_config(Config {
            runtime: RuntimeType::new(DummyRuntime),
        });
        let config = get_config();
        let runtime = config.runtime.get::<DummyRuntime>();
        assert_eq!(runtime, &DummyRuntime);
    }

    #[test]
    fn test_get_runtime_from_config() {
        set_config(Config {
            runtime: RuntimeType::new(DummyRuntime),
        });
        let config = get_config();
        assert_eq!(runtime, &DummyRuntime);
    }
}
