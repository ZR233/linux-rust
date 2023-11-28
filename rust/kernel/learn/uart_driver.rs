use crate::bindings::*;
use crate::error::*;
use crate::prelude::*;
use super::uart_port::UartPort;
/// UartDriver
pub struct UartDriver(uart_driver, bool);
unsafe impl Send for UartDriver {}
unsafe impl Sync for UartDriver {}
/// UartDriverRef
pub struct UartDriverRef(*mut uart_driver);

impl From<uart_driver> for UartDriver {
    fn from(value: uart_driver) -> Self {
        Self(value, false)
    }
}
impl From<*mut uart_driver> for UartDriverRef {
    fn from(value: *mut uart_driver) -> Self {
        Self(value)
    }
}

impl UartDriver {
    /// uart_register_driver
    pub fn register(&mut self, module: &ThisModule) -> Result {
        unsafe {
            self.0.owner = module.0;
            to_result(uart_register_driver(&mut self.0))?;
            self.1 = true;
            Ok(())
        }
    }
}

impl Drop for UartDriver {
    fn drop(&mut self) {
        if self.1 {
            unsafe { uart_unregister_driver(&mut self.0) }
        }
    }
}

impl UartDriverRef {
    fn as_ptr(&self) -> *mut uart_driver {
        self.0
    }
    /// uart_add_one_port
    pub fn add_one_port(&self, port: &UartPort) -> Result {
        unsafe {
            to_result(uart_add_one_port(self.0, port.as_ptr()))
        }
    }
} 
