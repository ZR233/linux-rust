use crate::pr_println;
use core::default::Default;
use kernel::error::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::sync::*;
use kernel::{bindings::*, init::PinInit};

#[pin_data]
pub struct Port {
    id: u32,
    line: u32,
    #[pin]
    inner: PortInner,
}

struct PortInner {
    port: uart_port,
}

unsafe impl Send for PortInner {}

impl Port {
    pub fn new(
        index: u32,
        ops: *const uart_ops,
        reg: *mut uart_driver,
        dev: *mut device,
    ) -> Result<impl PinInit<Self>> {
        pr_println!("port [{:?}]register ", index);
        let mut port = uart_port::default();
        port.ctrl_id = 0;
        port.pm = None;
        port.ops = ops;
        unsafe {
            port.dev = dev;
            to_result(uart_add_one_port(reg, &mut port))?;
        }

        Ok(pin_init!( Self {
            id: index,
            line: index,
            inner:PortInner { port }
        }))
    }


}
