use crate::linux::port::*;
use crate::linux::*;
use crate::pr_println;
use crate::UART_DRIVER;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::learn::platform_driver::PlatformDriver;
use kernel::learn::uart_opt::*;
use kernel::learn::uart_port::UartPort;
use kernel::macros::pin_data;
use kernel::prelude::*;
use kernel::sync::lock::spinlock::SpinLockBackend;
use kernel::sync::lock::Guard;
use kernel::{
    init::InPlaceInit, init::PinInit, new_spinlock, pin_init, spin_lock_init, sync::SpinLock,
};

#[derive(Default)]
pub struct RPort {
    pub index: usize,
    lock: spinlock_t,
    inner: Inner,
}

#[derive(Default)]
struct Inner {
    fcr: u32,
    tx_loadsz: u32,
    flags: u32,
    rxtrig_bytes: [u32; 4],
}

struct SpinGuard {
    c: SpinCommon,
    irq: bool,
}
struct SpinCommon {
    lock: *mut spinlock_t,
    inner: *mut Inner,
}

impl SpinCommon {
    fn as_ref_mut<'a>(&'a self) -> &'a mut Inner {
        unsafe { &mut *self.inner }
    }
}
impl SpinGuard {
    fn as_ref_mut(&self) -> &mut Inner {
        self.c.as_ref_mut()
    }
}
impl Drop for SpinGuard {
    fn drop(&mut self) {
        unsafe {
            if self.irq {
                spin_unlock_irq(self.c.lock)
            } else {
                spin_unlock(self.c.lock)
            }
        }
    }
}
struct SpinIrqSaveGuard {
    c: SpinCommon,
    flag: core::ffi::c_ulong,
}
impl SpinIrqSaveGuard {
    fn as_ref_mut(&self) -> &mut Inner {
        self.c.as_ref_mut()
    }
}
impl Drop for SpinIrqSaveGuard {
    fn drop(&mut self) {
        unsafe { spin_unlock_irqrestore(self.c.lock, self.flag) }
    }
}
impl RPort {
    pub fn new(index: usize) -> Result<Box<Self>> {
        let mut lock = spinlock_t::default();
        unsafe {
            spin_lock_init!(&mut lock);
        }
        let s = Self {
            index,
            lock,
            ..Default::default()
        };

        Box::try_init(s)
    }
    fn __lock(&self, irq: bool) -> SpinGuard {
        unsafe {
            let lock = &self.lock as *const _ as *mut _;
            spin_lock(lock);
            SpinGuard {
                c: SpinCommon {
                    lock,
                    inner: &self.inner as *const _ as *mut _,
                },
                irq,
            }
        }
    }
    fn lock(&self) -> SpinGuard {
        self.__lock(false)
    }
    fn lock_irq(&self) -> SpinGuard {
        self.__lock(true)
    }
    fn lock_irqsave(&self) -> SpinIrqSaveGuard {
        unsafe {
            let lock = &self.lock as *const _ as *mut _;
            let mut flag = 0;
            spin_lock_irqsave(lock, &mut flag);
            SpinIrqSaveGuard {
                c: SpinCommon {
                    lock,
                    inner: &self.inner as *const _ as *mut _,
                },
                flag,
            }
        }
    }

    pub(crate) unsafe fn register(
        index: usize,
        uport: &UartPort,
        pdev: *mut platform_device,
    ) -> Result {
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

            spin_lock_init!(&mut port.lock);

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

    pub(crate) fn config_port(p: *mut uart_port) {
        unsafe {
            let port = &mut *p;
            port.iotype = 2;
            let port_config = Serial8250Config::ns16550a();
            port.fifosize = port_config.fifo_size;
            port.name = port_config.name.as_char_ptr();

            let port = RPort::ref_from_port(port);

            let guard = port.lock();
            let g = guard.as_ref_mut();

            // let mut g = port.lock();
            g.tx_loadsz = port_config.tx_loadsz;
            g.flags = port_config.flags;
            g.fcr = port_config.fcr;
            g.rxtrig_bytes = port_config.rxtrig_bytes;
        }
        pr_println!("config_port ok");
    }

    pub(crate) fn set_termios(port: *mut uart_port, k1: *mut ktermios, k2: *const ktermios) {}
    pub(crate) fn startup(port: *mut uart_port) -> Result {
        Ok(())
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
