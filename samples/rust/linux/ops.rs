use core::default::Default;
use core::ptr::null;
use kernel::bindings::*;
use kernel::c_str;

use crate::pr_println;

pub struct UartOps(uart_ops);

unsafe impl Send for UartOps {}
unsafe impl Sync for UartOps {}

#[no_mangle]
extern "C" fn tx_empty(port: *mut uart_port) -> u32 {
    0
}

#[no_mangle]
extern "C" fn set_mctrl(port: *mut uart_port, mctrl: u32) {}

#[no_mangle]
extern "C" fn get_mctrl(port: *mut uart_port) -> u32 {
    0
}

#[no_mangle]
extern "C" fn stop_tx(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn start_tx(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn stop_rx(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn startup(port: *mut uart_port) -> i32 {
    0
}
#[no_mangle]
extern "C" fn shutdown(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn set_termios(port: *mut uart_port, k1: *mut ktermios, k2: *const ktermios) {}
#[no_mangle]
extern "C" fn type_(port: *mut uart_port) -> *const i8 {
    pr_println!("port type: {}", (*port).type_);    

    c_str!("unknown").as_char_ptr()
}
#[no_mangle]
extern "C" fn config_port(port: *mut uart_port, flags: i32) {
    pr_println!("config_port {}", flags);
}
impl UartOps {
    pub fn new() -> Self {
        let mut ops = uart_ops::default();

        ops.tx_empty = Some(tx_empty);
        ops.set_mctrl = Some(set_mctrl);
        ops.get_mctrl = Some(get_mctrl);
        ops.stop_tx = Some(stop_tx);
        ops.start_tx = Some(start_tx);
        ops.stop_rx = Some(stop_rx);
        ops.startup = Some(startup);
        ops.shutdown = Some(shutdown);
        ops.set_termios = Some(set_termios);
        ops.type_ = Some(type_);
        ops.config_port = Some(config_port);

        UartOps(ops)
    }

    pub fn as_ptr(&self) -> *const uart_ops {
        &self.0 as *const _
    }

    
}
