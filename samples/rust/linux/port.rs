use crate::pr_println;
use crate::rport::RPort;
use core::default::Default;
use kernel::c_str;
use kernel::error::*;
use kernel::learn::uart_opt::*;
use kernel::learn::uart_port::UartPort;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::sync::*;
use kernel::{bindings::*, init::PinInit};

#[no_mangle]
extern "C" fn tx_empty(port: *mut uart_port) -> u32 {
    0
}

#[no_mangle]
extern "C" fn set_mctrl(port: *mut uart_port, mctrl: u32) {
    pr_println!("set_mctrl {}", mctrl);
}

#[no_mangle]
extern "C" fn get_mctrl(port: *mut uart_port) -> u32 {
    0
}

#[no_mangle]
extern "C" fn stop_tx(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn start_tx(port: *mut uart_port) {
    pr_println!("start_tx");
}
#[no_mangle]
extern "C" fn stop_rx(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn startup(port: *mut uart_port) -> i32 {
    pr_println!("startup");
    from_result(|| {
        RPort::startup(port)?;
        Ok(0)
    })
}
#[no_mangle]
extern "C" fn shutdown(port: *mut uart_port) {}
#[no_mangle]
extern "C" fn set_termios(port: *mut uart_port, k1: *mut ktermios, k2: *const ktermios) {
    pr_println!("set_termios");
    unsafe {
        RPort::set_termios(port, k1, k2);
    }
}
#[no_mangle]
extern "C" fn type_(port: *mut uart_port) -> *const i8 {
    pr_println!("port type: {}", (*port).type_);

    c_str!("unknown").as_char_ptr()
}
#[no_mangle]
extern "C" fn config_port(port: *mut uart_port, flags: i32) {
    unsafe {
        pr_println!("config_port [{}]", (*port).port_id);
        RPort::config_port(port);
    }
}
extern "C" fn verify_port(port: *mut uart_port, ser: *mut serial_struct) ->i32{
    unsafe {
        pr_println!("verify_port");
        0
    }
}


pub(crate) static UART_OPS: UartOps = unsafe {
    UartOps::from_struct(uart_ops {
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
        throttle: None,
        unthrottle: None,
        send_xchar: None,
        start_rx: None,
        enable_ms: None,
        break_ctl: None,
        flush_buffer: None,
        set_ldisc: None,
        pm: None,
        release_port: None,
        request_port: None,
        verify_port: Some(verify_port),
        ioctl: None,
    })
};
