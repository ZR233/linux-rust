use crate::bindings::*;


pub struct UartPort(uart_port);
unsafe impl Send for UartPort {}
unsafe impl Sync for UartPort {}

impl UartPort {
    pub fn new(port: uart_port) -> Self {
        UartPort(port)
    }
    pub (crate) unsafe fn as_ptr(&self) -> *mut uart_port {
        &self.0 as *const _ as *mut _
    }
}