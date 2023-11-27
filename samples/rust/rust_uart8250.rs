// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
#![allow(non_upper_case_globals)]

/// linux
pub mod linux;
/// rport
pub mod rport;
mod sbi_print;
use crate::linux::ops::UartOps;
use crate::linux::port::KPort;
pub use crate::sbi_print::*;
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
use kernel::uart::*;

const NR: i32 = 4;
const TTY_MAJOR: i32 = 4;
const TTY_MINOR: i32 = 64;
const DEV_NAME: &CStr = c_str!("ttySZ");
const DRIVER_NAME: &CStr = c_str!("serial");
const PLATFORM_DRIVER_NAME: &CStr = c_str!("serial8250");

module! {
    type: RustUartModule,
    name: "Rust_UART",
    author: "ZR233",
    description: "Rust simple uart",
    license: "GPL",
}

struct RustUartModule {
    reg: Registration,
    platform_driver: PlatformDriver,
    ops: UartOps,
    of: OfDeviceIdList,
}

// struct UART8250 {
//     ports: Vec<Pin<UniqueArc<Port>>>,
// }

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

extern "C" fn prob(dev: *mut platform_device) -> i32 {
    pr_println!("prob");
    unsafe {
        let dev = &mut *dev;

        // dev.dev.driver_data = ;
    }

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

// impl UART8250 {
//     fn new(reg: *mut uart_driver, model: &ThisModule) -> Result<Self> {
//         Ok(Self { ports })
//     }
// }

impl kernel::Module for RustUartModule {
    fn init(module: &'static ThisModule) -> Result<Self> {
        pr_println!("Rust UART (init)");
        let of = OfDeviceIdList(of_device_id_list());
        let mut reg = Registration::default();
        let reg_ptr = &mut reg.reg as *mut _;
        // let mut cons = Console::new(reg_ptr);
        reg.reg.nr = NR;
        reg.reg.major = TTY_MAJOR;
        reg.reg.minor = TTY_MINOR;
        reg.reg.dev_name = DEV_NAME.as_char_ptr();
        reg.reg.driver_name = DRIVER_NAME.as_char_ptr();

        // reg.reg.cons = cons.as_ptr();
        pr_println!("uart register begin");
        reg.register(module)?;
        pr_println!("uart register ok");
        unsafe {
            let ops = UartOps::new();

            let devs =
                platform_device_alloc(PLATFORM_DRIVER_NAME.as_char_ptr(), PLAT8250_DEV_LEGACY);

            to_result(platform_device_add(devs))?;

            let mut platform_driver = PlatformDriver::default();
            platform_driver.0.probe = Some(prob);
            platform_driver.0.remove = Some(remove);
            platform_driver.0.suspend = Some(suspend);
            platform_driver.0.resume = Some(resume);
            // platform_driver.0.driver.name = PLATFORM_DRIVER_NAME.as_char_ptr();
            platform_driver.0.driver.of_match_table = of.0.as_ptr();
            platform_driver.0.driver.name = c_str!("of_serial").as_char_ptr();

            platform_driver.register(module);
            pr_println!("platform_driver register finish");

            platform_device_del(devs);
            pr_println!("init finish");
            Ok(RustUartModule {
                reg,
                platform_driver,
                ops,
                of,
            })
        }
    }
}

impl Drop for RustUartModule {
    fn drop(&mut self) {
        pr_info!("Rust UART (exit)\n");
    }
}

struct OfDeviceIdList(Vec<of_device_id>);
unsafe impl Send for OfDeviceIdList {}
unsafe impl Sync for OfDeviceIdList {}

fn of_device_id_list() -> Vec<of_device_id> {
    let mut out = Vec::new();
    // out.try_push(new_of_device_id(c_str!("ns8250"), PORT_8250)).unwrap();
    out.try_push(new_of_device_id(c_str!("ns16550a"), PORT_16550A)).unwrap();

    out
}

fn new_of_device_id(comp: &CStr, data: u32) -> of_device_id {
    let mut compatible = [0; 128];
    let ptr = comp.as_bytes();
    for i in 0..ptr.len(){
        compatible[i] = ptr[i] as _;
    }

    // let src = unsafe { &*slice_from_raw_parts(ptr, comp.len()-1) };
    // compatible.copy_from_slice(src);

    of_device_id {
        name: [0; 32],
        compatible,
        type_: [0; 32],
        data: data as _,
    }
}
