use core::cell::UnsafeCell;
use core::mem::size_of;
use core::ptr::null_mut;

use crate::linux::port::*;
use crate::linux::*;
use crate::pr_println;
use crate::NR;
use crate::PORTS;
use crate::UART_DRIVER;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::learn::platform_driver::PlatformDriver;
use kernel::learn::uart_opt::*;
use kernel::learn::uart_port::uart_serial_in;
use kernel::learn::uart_port::UartPort;
use kernel::macros::pin_data;
use kernel::prelude::*;
use kernel::sync::lock::spinlock::SpinLockBackend;
use kernel::sync::lock::Guard;
use kernel::{
    init::InPlaceInit, init::PinInit, new_spinlock, pin_init, spin_lock_init, sync::SpinLock,
};

static mut RPORTS: [PortWarpper; NR as usize] =
    unsafe { [PortWarpper { port: null_mut() }; NR as usize] };
static mut DEV_ATTRS: [*mut attribute; 2] = [null_mut(), null_mut()];
static mut DEV_ATTR_GROUP: *mut AG = null_mut();

struct AG(attribute_group);
impl AG {
    fn new(attrs: *mut *mut attribute) -> Result<Box<Self>, kernel::error::Error> {
        let s = Self(attribute_group {
            attrs,
            ..Default::default()
        });

        Box::try_init(s)
    }


}

pub(crate) fn init_dev_attr() {
    unsafe {
        let attr = (&*dev_attr_rx_trig_bytes(0)).attr;
        DEV_ATTRS[0] = &attr as *const _ as *mut attribute;
        let b:  Box<AG> = AG::new(DEV_ATTRS.as_mut_ptr()).unwrap();
        let ptr = Box::into_raw(b);

        DEV_ATTR_GROUP = ptr;
    }
}

#[derive(Clone, Copy)]
struct PortWarpper {
    port: *mut RPort,
}
impl PortWarpper {
    fn init(&mut self, index: usize) -> Result {
        let rport: Box<RPort> = RPort::new(index)?;
        let ptr = Box::into_raw(rport);
        self.port = ptr;
        Ok(())
    }
}

/// Rust Port
#[derive(Default)]
pub struct RPort {
    /// Index of the port
    pub index: usize,
    lock: spinlock_t,
    inner: UnsafeCell<Inner>,
}

unsafe impl Sync for RPort {}
unsafe impl Send for RPort {}

#[derive(Default)]
struct Inner {
    fcr: u32,
    lcr: u8,
    ier: u8,
    tx_loadsz: u32,
    flags: u32,
    rxtrig_bytes: [u32; 4],
    lsr_saved_flags: u16,
    lsr_save_mask: u16,
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
    /// new a RPort
    pub fn new(index: usize) -> Result<Box<Self>> {
        let mut lock = spinlock_t::default();
        unsafe {
            spin_lock_init!(&mut lock);
        }
        let s = Self {
            index,
            lock,
            inner: UnsafeCell::new(Inner {
                lsr_save_mask: LSR_SAVE_FLAGS as _,
                ..Default::default()
            }),
        };

        Box::try_init(s)
    }
    fn port(&self) -> &UartPort {
        unsafe { &PORTS[self.index] }
    }

    fn __lock(&self, irq: bool) -> SpinGuard {
        unsafe {
            let lock = &self.lock as *const _ as *mut _;
            spin_lock(lock);
            SpinGuard {
                c: SpinCommon {
                    lock,
                    inner: self.inner.get(),
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
            port.type_ = PORT_16550A;

            port.mapbase = resource.start;
            port.mapsize = resource.end - resource.start + 1;
            port.iotype = UPIO_MEM as _;
            port.flags = UPF_SHARE_IRQ | UPF_BOOT_AUTOCONF | UPF_FIXED_PORT | UPF_FIXED_TYPE;

            spin_lock_init!(&mut port.lock);

            to_result(uart_get_rs485_mode(port))?;

            port.attr_group = &mut (&mut*DEV_ATTR_GROUP).0;
            // has_acpi_companion
            if !is_acpi_device_node((&*port.dev).fwnode) {}

            port.flags |= UPF_IOREMAP;
            pr_println!("add_one_port begin");
            UART_DRIVER.add_one_port(uport)?;

            Ok(())
        }
    }

    pub(crate) fn ref_from_port(p: *mut uart_port) -> &'static RPort {
        unsafe {
            let index = (&*p).port_id as usize;
            get_port(index)
        }
    }
    pub(crate) fn ref_from_kport(p: &UartPort) -> &'static RPort {
        unsafe { Self::ref_from_port(p.as_ptr()) }
    }

    pub(crate) fn config_port(p: *mut uart_port) {
        pr_println!("config_port begin");
        unsafe {
            let port = &mut *p;
            port.iotype = 2;
            let port_config = Serial8250Config::ns16550a();
            port.fifosize = port_config.fifo_size;
            port.name = port_config.name.as_char_ptr();

            let size = port.mapsize;
            request_mem_region(port.mapbase, size, c_str!("serial").as_char_ptr());

            if port.flags & UPF_IOREMAP != 0 {
                port.membase = port.mapbase as *mut _;
            } else {
                port.membase = ioremap(port.mapbase, port.mapsize as _) as _;
            }

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

    pub(crate) fn set_termios(port: *mut uart_port, k1: *mut ktermios, old: *const ktermios) {
        unsafe {
            let port = &mut *port;
            let s = Self::ref_from_port(port);
            let termios = &mut *k1;
            let cval = compute_lcr(termios.c_cflag);
            let baud = s.uart_get_baud_rate(termios, old);
            let quot = uart_get_divisor(port, baud);
            let mut flags = 0;
            spin_lock_irqsave(&mut port.lock, &mut flags);
            let inner = &mut *s.inner.get();
            inner.lcr = cval;
            uart_update_timeout(port, termios.c_cflag, baud);

            port.read_status_mask = UART_LSR_OE | UART_LSR_THRE | UART_LSR_DR;
            if (termios.c_iflag & INPCK > 0) {
                port.read_status_mask |= UART_LSR_FE | UART_LSR_PE;
            }
            if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK) > 0) {
                port.read_status_mask |= UART_LSR_BI;
            }

            /*
             * Characters to ignore
             */
            port.ignore_status_mask = 0;
            if (termios.c_iflag & IGNPAR > 0) {
                port.ignore_status_mask |= UART_LSR_PE | UART_LSR_FE;
            }
            if (termios.c_iflag & IGNBRK > 0) {
                port.ignore_status_mask |= UART_LSR_BI;
                /*
                 * If we're ignoring parity and break indicators,
                 * ignore overruns too (for real raw support).
                 */
                if (termios.c_iflag & IGNPAR > 0) {
                    port.ignore_status_mask |= UART_LSR_OE;
                }
            }

            /*
             * ignore all characters if CREAD is not set
             */
            if ((termios.c_cflag & CREAD) == 0) {
                port.ignore_status_mask |= UART_LSR_DR;
            }

            /*
             * CTS flow control flag and modem status interrupts
             */
            inner.ier &= (!UART_IER_MSI) as u8;
            serial_out(port, UART_IER as _, inner.ier as u32 as _);

            serial_out(port, UART_LCR as _, (inner.lcr as u32 | UART_LCR_DLAB) as _);

            s.dl_write(quot);

            spin_unlock_irqrestore(&mut port.lock, flags);
            pr_println!("set_termios ok");
        }
    }
    pub(crate) fn startup(port: *mut uart_port) -> Result {
        Ok(())
    }

    pub(crate) fn dl_write(&self, value: u32) {
        unsafe {
            let port = self.port().as_ptr();

            serial_out(port, UART_DLL as _, (value & 0xff) as _);
            serial_out(port, UART_DLM as _, (value >> 8 & 0xff) as _);
        }
    }

    fn capabilities(&self) -> u32 {
        Serial8250Config::ns16550a().flags
    }

    fn serial_lsr_in(&self, port: *mut uart_port) -> u16 {
        unsafe {
            let inner = &mut *self.inner.get();
            let mut lsr = inner.lsr_saved_flags;
            lsr |= uart_serial_in(port, UART_LSR as _) as u16;
            inner.lsr_saved_flags = lsr & inner.lsr_save_mask;
            lsr
        }
    }

    pub(crate) fn wait_for_xmitr(&self, port: *mut uart_port, bits: i32) {
        let mut tmout = 10000;
        unsafe {
            /* Wait up to 10ms for the character(s) to be sent. */
            loop {
                let status = self.serial_lsr_in(port);

                if (status as i32 & bits) == bits {
                    break;
                }
                tmout -= 1;
                if tmout == 0 {
                    break;
                }
                udelay(1);
                touch_nmi_watchdog(0);
            }
        }
    }

    fn uart_get_baud_rate(&self, termios: *mut ktermios, old: *const ktermios) -> u32 {
        unsafe {
            let port = &mut *self.port().as_ptr();
            let tolerance = port.uartclk / 100;

            let min = port.uartclk / 16 / UART_DIV_MAX;
            let max = (port.uartclk + tolerance) / 16;
            uart_get_baud_rate(port, termios, old, min, max)
        }
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
        port.has_sysrq = 1 as _;
        port.serial_in = Some(serial_in);
        port.serial_out = Some(serial_out);
        let index = index as usize;

        RPORTS[index].init(index);

        pr_println!("port {} init", get_port(index).index);
    }
}

fn get_port(index: usize) -> &'static mut RPort {
    unsafe { &mut *RPORTS[index].port }
}

extern "C" fn serial_in(port: *mut uart_port, offset: i32) -> u32 {
    unsafe {
        let port = (&*port);
        let regshift = port.regshift as i32;
        let offset = offset << regshift;
        let addr = port.membase.offset(offset as _);

        readb(addr)
    }
}
extern "C" fn serial_out(port: *mut uart_port, offset: i32, value: i32) {
    unsafe {
        let port = (&*port);
        let regshift = port.regshift as i32;
        let offset = offset << regshift;
        let addr = port.membase.offset(offset as _);

        writeb(value, addr);
    }
}
fn compute_lcr(c_cflag: tcflag_t) -> core::ffi::c_uchar {
    let mut cval = unsafe { tty_get_char_size(c_cflag) } - 5;

    if c_cflag & CSTOPB > 0 {
        cval |= UART_LCR_STOP as u8;
    }
    if c_cflag & PARENB > 0 {
        cval |= UART_LCR_PARITY as u8;
    }
    if c_cflag & PARODD == 0 {
        cval |= UART_LCR_EPAR as u8;
    }
    if c_cflag & CMSPAR > 0 {
        cval |= UART_LCR_SPAR as u8;
    }

    cval
}
