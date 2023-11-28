use crate::pr_println;
use core::default::Default;
use kernel::c_str;
use kernel::error::*;
use kernel::learn::uart_opt::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::sync::*;
use kernel::{bindings::*, init::PinInit};
use crate::rport::RPort;

#[pin_data]
pub(crate) struct PortWarp {
    pub(crate) ops: UartOps,
    #[pin]
    rport: SpinLock<RPort>,
}

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
impl PortWarp {
    pub(crate) fn new() -> Result<impl PinInit<Self>> {
        Ok(pin_init!(Self {
            ops: UartOps::from(
                uart_ops {
                    tx_empty: Some(tx_empty),
                    set_mctrl: Some(set_mctrl),
                    get_mctrl: Some(get_mctrl),
                    stop_tx: Some(stop_tx),
                    start_tx: Some(start_tx),
                    stop_rx: Some(stop_rx),
                    startup: Some(startup),
                    shutdown: Some(shutdown),
                    set_termios: Some(set_termios),
                    type_: Some(type_),
                    config_port: Some(config_port),
                    ..Default:: default()
                }
            ),
            rport<- new_spinlock!(RPort::new())
        }))
    }
}
