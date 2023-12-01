use crate::linux::port::*;
use kernel::bindings::*;
use kernel::learn::uart_port::UartPort;
use kernel::learn::uart_opt::*;

pub struct RPort {}

impl RPort {
    pub fn new() -> Self {
        RPort {}
    }
}

pub(crate) fn uart_port_init(index: u32, port: &UartPort) {
    unsafe {
        let portw = port;
        let port = &mut *portw.as_ptr();
        port.port_id = index;
        port.line = index;
        port.ctrl_id = 0;
        port.pm = None;
        port.ops = UART_OPS.as_ptr();
        port.has_sysrq = b'1';

        
    }
}
