use crate::bindings::*;



pub struct UartOps(uart_ops);

unsafe impl Send for UartOps {}
unsafe impl Sync for UartOps {}

impl UartOps {
    pub(crate) unsafe fn as_ptr(&self)->*const uart_ops{
        &self.0
    }
}

impl From<uart_ops> for UartOps {
    fn from(value: uart_ops) -> Self {
        Self(value)
    }
}