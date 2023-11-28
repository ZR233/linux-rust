use core::ptr::null_mut;

#[allow(unused)]
use crate::bindings::*;
use crate::c_str;
use crate::error::*;
use crate::prelude::*;



#[derive(Default)]
pub struct PlatformDriver(pub platform_driver, bool);

unsafe impl Send for PlatformDriver {}
unsafe impl Sync for PlatformDriver {}

impl From<platform_driver> for PlatformDriver {
    fn from(value: platform_driver) -> Self {
        Self(value, false)
    }
}

impl PlatformDriver {
    pub fn register(&mut self, module: &ThisModule) -> Result {
        unsafe {
            to_result(__platform_driver_register(&mut self.0, module.0))?;
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
