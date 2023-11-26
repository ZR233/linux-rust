use core::ptr::null_mut;

#[allow(unused)]
use crate::bindings::*;
use crate::c_str;
use crate::error::*;
use crate::prelude::*;

#[derive(Default)]
pub struct Registration {
    pub reg: uart_driver,
    is_ok: bool,
}
unsafe impl Send for Registration {}
unsafe impl Sync for Registration {}

impl Registration {
    pub fn register(&mut self, module: &ThisModule) -> Result {
        let mut s = Self::default();

        unsafe {
            let reg = &mut self.reg;
            reg.owner = module.0;
            to_result(uart_register_driver(reg))?;
            self.is_ok = true;
            Ok(())
        }
    }
}
impl Drop for Registration {
    fn drop(&mut self) {
        if self.is_ok {
            unsafe { uart_unregister_driver(&mut self.reg) }
        }
    }
}

#[derive(Default)]
pub struct PlatformDriver(pub platform_driver, bool);

unsafe impl Send for PlatformDriver {}
unsafe impl Sync for PlatformDriver {}

impl PlatformDriver {
    pub fn register(&mut self, model: &ThisModule) -> Result {
        unsafe {
            to_result(__platform_driver_register(&mut self.0, model.0))?;
            self.1 = true;
        }
        Ok(())
    }
    pub fn as_ptr(&mut self)-> *mut platform_driver{
        &mut self.0
    }
}

impl Drop for PlatformDriver {
    fn drop(&mut self) {
        if self.1 {
            unsafe { platform_driver_unregister(&mut self.0) }
        }
    }
}
