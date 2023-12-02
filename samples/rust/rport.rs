use crate::linux::port::*;
use crate::pr_println;
use crate::UART_DRIVER;
use kernel::bindings::*;
use kernel::error::*;
use kernel::learn::platform_driver::PlatformDriver;
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
        let s = Self { index, inner: b };

        Box::try_init(s)
    }

    pub(crate) unsafe fn register(index: usize, uport: &UartPort, pdev: *mut platform_device) -> Result {
        let mut resource = resource::default();
        unsafe {
            let port = &mut *uport.as_ptr();

            let np = (*pdev).dev.of_node;
            let dev = &mut (*pdev).dev as *mut _;

            to_result(of_address_to_resource(np, 0, &mut resource))?;

            port.irq = of_irq_get(np, 0) as _;
            port.uartclk = 0x00384000;
            port.regshift = 0;
            port.dev = dev;

            port.mapbase = resource.start;
            port.mapsize = resource.end - resource.start + 1;
            port.iotype = UPIO_MEM as _;
            port.flags = UPF_SHARE_IRQ | UPF_BOOT_AUTOCONF | UPF_FIXED_PORT | UPF_FIXED_TYPE;

            spin_lock_init(&mut port.lock);

            let p = RPort::new(index)?;
            let ptr = Box::into_raw(p);

            (&mut *port.dev).driver_data = ptr as _;


            port.flags |= UPF_IOREMAP;
            pr_println!("add_one_port begin");
            UART_DRIVER.add_one_port(uport)?;

            Ok(())
        }
    }

    pub(crate) fn ref_from_port(p: *mut uart_port) -> &'static RPort {
        unsafe {
            let port = &*p;
            let ptr = (&*port.dev).driver_data as *const RPort;
            let rport = &*ptr;
            return rport;
        }
    }
    pub(crate) fn ref_from_kport(p: &UartPort) -> &'static RPort {
        unsafe { Self::ref_from_port(p.as_ptr()) }
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
