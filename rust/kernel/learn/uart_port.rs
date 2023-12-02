use super::uart_opt::UartOps;
use crate::bindings::*;
use crate::error::to_result;
use crate::prelude::*;
use core::mem::size_of;
use core::ptr::null_mut;

#[derive(Default, Clone, Copy)]
pub struct UartPort(uart_port);
unsafe impl Send for UartPort {}
unsafe impl Sync for UartPort {}

// impl From<uart_port> for UartPort {
//     fn from(value: uart_port) -> Self {
//         Self(value)
//     }
// }

impl UartPort {
    pub const unsafe fn zero() -> Self {
        unsafe {
            let v = core::mem::MaybeUninit::<uart_port>::zeroed();
            let v = v.assume_init();
            Self(v)
        }
    }

    pub fn pm_runtime_get_sync(&self) -> Result {
        unsafe {
            let dev = (&*self.as_ptr()).dev;
            if dev.is_null() {
                return Ok(());
            }
            to_result(pm_runtime_get_sync(dev))?;
        }
        Ok(())
    }

    pub unsafe fn as_ptr(&self) -> *mut uart_port {
        &self.0 as *const _ as _
    }

    pub fn set_ops(&mut self, ops: &UartOps) {
        unsafe {
            self.0.ops = ops.as_ptr();
        }
    }
    /// uart_set_options
    pub fn uart_set_options(&self, co: *mut console, options: UartOptions) -> Result {
        unsafe {
            to_result(uart_set_options(
                self.as_ptr(),
                co,
                options.baud,
                options.parity,
                options.bits,
                options.flow,
            ))
        }
    }
    /// uart_parse_options
    pub fn uart_parse_options(&self, options: *const core::ffi::c_char) -> Option<UartOptions> {
        let mut out = UartOptions::default();

        unsafe {
            if options.is_null() {
                return None;
            }

            uart_parse_options(
                options,
                &mut out.baud,
                &mut out.parity,
                &mut out.bits,
                &mut out.flow,
            )
        }
        Some(out)
    }
    /// serial_in
    pub fn serial_in(&self, offset: i32) -> u32 {
        unsafe {
            let port = &mut *self.as_ptr();
            let f = (&*port).serial_in.unwrap();
            f(port, offset)
        }
    }

    /// serial_out
    pub fn serial_out(&self, offset: i32, value: i32) {
        unsafe {
            let port = &mut *self.as_ptr();
            let f = (&*port).serial_out.unwrap();
            f(port, offset, value)
        }
    }

}

/// uart_serial_in
pub unsafe fn uart_serial_in(port: *mut uart_port, offset: i32) -> u32 {
    unsafe {
        let port = &mut *port;
        let f = (&*port).serial_in.unwrap();
        f(port, offset)
    }
}

/// uart_serial_out
pub unsafe fn uart_serial_out(port: *mut uart_port,  offset: i32, value: i32) {
    unsafe {
        let port = &mut *port;
        let f = (&*port).serial_out.unwrap();
        f(port, offset, value)
    }
}

#[derive(Default)]
pub struct UartOptions {
    pub baud: core::ffi::c_int,
    pub parity: core::ffi::c_int,
    pub bits: core::ffi::c_int,
    pub flow: core::ffi::c_int,
}
// const fn serial_rs485_zero() -> serial_rs485 {
//     serial_rs485 {
//         flags: 0,
//         delay_rts_before_send: 0,
//         delay_rts_after_send: 0,
//         __bindgen_anon_1: serial_rs485__bindgen_ty_1_zero(),
//     }
// }

// const fn serial_rs485__bindgen_ty_1_zero()-> serial_rs485__bindgen_ty_1 {
//     serial_rs485__bindgen_ty_1 {
//         padding: [0; 5],
//         __bindgen_anon_1: serial_rs485__bindgen_ty_1__bindgen_ty_1_zero(),
//     }
// }

// const fn serial_rs485__bindgen_ty_1__bindgen_ty_1_zero() -> serial_rs485__bindgen_ty_1__bindgen_ty_1
// {
//     serial_rs485__bindgen_ty_1__bindgen_ty_1 {
//         addr_recv: 0,
//         addr_dest: 0,
//         padding0: [0; 2],
//         padding1: [0; 4],
//     }
// }
