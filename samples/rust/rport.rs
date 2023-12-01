use crate::linux::port::*;
use crate::pr_println;
use kernel::bindings::*;
use kernel::error::*;
use kernel::learn::uart_opt::*;
use kernel::learn::uart_port::UartPort;
use kernel::macros::pin_data;
use kernel::prelude::*;
use kernel::{init::InPlaceInit, init::PinInit, new_spinlock, pin_init, sync::SpinLock};

pub struct RPort {
    pub index: usize,
    inner: Pin<Box<InnerWapper>>,
}
#[pin_data]
struct InnerWapper {
    #[pin]
    inner: SpinLock<PortInner>,
}
#[derive(Default)]
struct PortInner {
    cfr: u32,
}

impl InnerWapper {
    fn new() -> impl PinInit<Self> {
        pin_init!(Self {
            inner <- new_spinlock!(PortInner {
                ..Default::default()
             }),
        })
    }
}

impl RPort {
    pub fn new(index: usize) -> Result<Box<Self>> {
        let inner = InnerWapper::new();
        let b = Box::pin_init(inner)?;
        let s = Self{
            index,
            inner: b,
        };

        Box::try_init(s)
    }
}

pub(crate) fn up_from_port(p:*mut uart_port)->&'static RPort{
    unsafe {
        let port = &*p;
        let ptr = (&* port.dev).driver_data as *const RPort;
        let rport = &*ptr;
        return rport;
    }
}
pub(crate) fn up_from_kport(p: &UartPort)->&'static RPort{
    unsafe {
        up_from_port(p.as_ptr())
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






        pr_println!("{index}");

    }
}
