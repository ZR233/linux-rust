use crate::bindings::*;
use super::uart_opt::UartOps;

pub struct UartPort(uart_port);
unsafe impl Send for UartPort {}
unsafe impl Sync for UartPort {}

impl From<uart_port> for UartPort {
    fn from(value: uart_port) -> Self {
        Self(value)
    }
}

impl UartPort {

    pub (crate) unsafe fn as_ptr(&self) -> *mut uart_port {
        &self.0 as *const _ as *mut _
    }


    pub fn set_ops(&mut self, ops: &UartOps) {
        unsafe {
            self.0.ops = ops.as_ptr();
        }
    }


}