use crate::pr_println;
use core::default::Default;
use kernel::error::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::sync::*;
use kernel::{bindings::*, init::PinInit};

use super::ops::UartOps;

#[repr(transparent)]
pub struct KPort(uart_port);

unsafe impl Send for KPort {}
unsafe impl Sync for KPort {}


impl KPort {
    pub fn new(
        index: u32,
        ops: &UartOps,
    ) -> Result<Self> {
        pr_println!("port [{:?}] init", index);
        let mut port = uart_port::default();
        port.line = index;
        port.port_id = index;
        port.ops = ops.as_ptr();
        //     // to_result(uart_add_one_port(reg, &mut port))?;
        // }

        Ok(Self(port))
    }

    pub fn as_ptr(&self)->*mut uart_port{
        &self.0 as *const _ as *mut _
    }
}
