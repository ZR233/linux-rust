// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
use core::default::Default;
use core::ptr::null_mut;
use core::ptr::slice_from_raw_parts;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::str::CString;
use kernel::sync::*;
use kernel::uart::Registration;
use port::Port;
pub mod port;
mod sbi_print;

const NR: i32 = 4;
const TTY_MAJOR: i32 = 4;
const TTY_MINOR: i32 = 64;
const DEV_NAME: &CStr = c_str!("ttySZ");
const DRIVER_NAME: &CStr = c_str!("serial");

use core::arch::asm;
pub use sbi_print::*;

module! {
    type: RustUart,
    name: "Rust_UART",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustUart {
    reg: Registration,
    cons: Console,
    uart: Pin<UniqueArc<UART8250>>,
}

#[pin_data]
struct UART8250 {
    #[pin]
    inner: SpinLock<UART8250Inner>,
}
struct UART8250Inner {
    platform_driver: platform_driver,
    ports: Vec<Pin<UniqueArc<Port>>>,
    opts: uart_ops,
}

struct Console(console);
impl Console {
    fn new(reg: *mut uart_driver) -> Self {
        let mut cons = console::default();
        cons.write = Some(console_write);

        cons.data = reg as _;
        Self(cons)
    }

    fn as_ptr(&mut self) -> *mut console {
        &mut self.0 as _
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}

unsafe impl Send for UART8250Inner {}

extern "C" fn prob(dev: *mut platform_device) -> i32 {
    pr_println!("prob");
    0
}
extern "C" fn remove(dev: *mut platform_device) -> i32 {
    pr_println!("remove");
    0
}
extern "C" fn suspend(dev: *mut platform_device, msg: pm_message) -> i32 {
    pr_println!("suspend");
    0
}
extern "C" fn resume(dev: *mut platform_device) -> i32 {
    pr_println!("resume");
    0
}

extern "C" fn console_write(co: *mut console, char: *const i8, count: u32) {
    unsafe {
        let bytes = &*slice_from_raw_parts(char, count as _);
        print_bytes(bytes);
    }
}

impl UART8250 {
    fn new(reg: *mut uart_driver) -> Result<impl PinInit<Self>> {
        let mut platform_driver = platform_driver::default();
        let mut opts = uart_ops::default();
        platform_driver.probe = Some(prob);
        platform_driver.remove = Some(remove);
        platform_driver.suspend = Some(suspend);
        platform_driver.resume = Some(resume);

        let mut ports = Vec::try_with_capacity(NR as _).unwrap();

        unsafe {
            let devs =
                platform_device_alloc(c_str!("serial8250").as_char_ptr(), PLAT8250_DEV_LEGACY);

            let dev = &mut (*devs).dev as *mut _;
            to_result(platform_device_add(devs))?;

            for i in 0..NR {
                let port = Port::new(i as u32, &opts, reg, dev)?;
                let port = UniqueArc::pin_init(port).unwrap();
                ports.try_push(port)?;
                pr_println!("port [{}] ok", i);
            }

            platform_device_del(devs);
        }


        let inner = UART8250Inner {
            platform_driver,
            ports,
            opts,
        };
        Ok(pin_init!( Self{
            inner <- new_spinlock!(inner)
        }))
    }
}

impl kernel::Module for RustUart {
    fn init(module: &'static ThisModule) -> Result<Self> {
        pr_println!("Rust UART (init)");

        let mut reg = Registration::default();
        let reg_ptr = &mut reg.reg as *mut _;
        let mut cons = Console::new(reg_ptr);
        reg.reg.nr = NR;
        reg.reg.major = TTY_MAJOR;
        reg.reg.minor = TTY_MINOR;
        reg.reg.dev_name = DEV_NAME.as_char_ptr();
        reg.reg.driver_name = DRIVER_NAME.as_char_ptr();

        reg.reg.cons = cons.as_ptr();
        pr_println!("register begin");

        reg.register(module)?;

        pr_println!("register ok");
        let uart8250 = UART8250::new(reg_ptr).unwrap();

        pr_println!("init finish");

        let uart = UniqueArc::pin_init(uart8250).unwrap();
        Ok(RustUart { reg, cons, uart })
    }
}

impl Drop for RustUart {
    fn drop(&mut self) {
        pr_info!("Rust UART (exit)\n");
    }
}
