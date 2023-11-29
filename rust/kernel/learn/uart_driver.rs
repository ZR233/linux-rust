use super::uart_port::UartPort;
use crate::bindings::*;
use crate::error::*;
use crate::prelude::*;
/// UartDriver
pub struct UartDriver(uart_driver);
unsafe impl Send for UartDriver {}
unsafe impl Sync for UartDriver {}
/// UartDriverRef
pub struct UartDriverRef(*mut uart_driver);

impl From<uart_driver> for UartDriver {
    fn from(value: uart_driver) -> Self {
        Self(value)
    }
}
impl From<*mut uart_driver> for UartDriverRef {
    fn from(value: *mut uart_driver) -> Self {
        Self(value)
    }
}

impl UartDriver {
    pub const unsafe fn from_struct(div: uart_driver) -> UartDriver {
        UartDriver(div)
    }
    pub const unsafe fn as_ptr(&self) -> *mut uart_driver {
        &self.0 as *const _ as *mut _
    }

    /// uart_register_driver
    pub fn register(&self, module: &ThisModule) -> Result {
        unsafe {
            to_result(uart_register_driver(self.as_ptr()))?;
            Ok(())
        }
    }

    /// uart_add_one_port
    pub fn add_one_port(&self, port: &UartPort) -> Result {
        unsafe { to_result(uart_add_one_port(self.as_ptr(), port.as_ptr())) }
    }
}

impl Drop for UartDriver {
    fn drop(&mut self) {
        unsafe { uart_unregister_driver(&mut self.0) }
    }
}

impl UartDriverRef {
    fn as_ptr(&self) -> *mut uart_driver {
        self.0
    }

}
