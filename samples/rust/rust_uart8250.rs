// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
#![allow(non_upper_case_globals)]

/// linux
pub mod linux;
/// rport
pub mod rport;
use linux::from_result;
mod sbi_print;
use crate::linux::ops::UartOps;
use crate::linux::port::KPort;
use crate::rport::RPort;
pub use crate::sbi_print::*;
use core::convert::AsRef;
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
use kernel::learn::uart_driver::*;
use kernel::learn::platform_driver::*;
use kernel::learn::uart_port::*;


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
    platform_driver: PlatformDriver,
    data: Arc<PlatformData>,
}

struct PlatformData {
    ops: UartOps,
    of: OfDeviceIdList,
    driver: UartDriver,
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



extern "C" fn console_write(co: *mut console, char: *const i8, count: u32) {
    unsafe {
        let bytes = &*slice_from_raw_parts(char, count as _);
        print_bytes(bytes);
    }
}

impl kernel::Module for RustUartModule {
    fn init(module: &'static ThisModule) -> Result<Self> {
        pr_println!("Rust UART (init)");
        let of = OfDeviceIdList(of_device_id_list());

        let mut driver =UartDriver::from( uart_driver{
            nr: NR,
            major: TTY_MAJOR,
            minor: TTY_MINOR,
            dev_name: DEV_NAME.as_char_ptr(),
            driver_name: DRIVER_NAME.as_char_ptr(),
            ..Default::default()
        });
        

        // reg.reg.cons = cons.as_ptr();
        pr_println!("uart register begin");
        driver.register(module)?;
        pr_println!("uart register ok");
        unsafe {
            let ops = UartOps::new();

            let devs =
                platform_device_alloc(PLATFORM_DRIVER_NAME.as_char_ptr(), PLAT8250_DEV_LEGACY);

            let mut platform_driver = PlatformDriver::from(platform_driver{
                driver: device_driver{
                    name: c_str!("of_serial").as_char_ptr(),
                    of_match_table: of.0.as_ptr(),
                   ..Default::default()
                },
                probe: Some(probe),
                ..Default::default()
            });
            // platform_driver.0.probe = Some(prob);
            // platform_driver.0.remove = Some(remove);
            // platform_driver.0.suspend = Some(suspend);
            // platform_driver.0.resume = Some(resume);
            // platform_driver.0.driver.name = PLATFORM_DRIVER_NAME.as_char_ptr();
            // platform_driver.0.driver.of_match_table = of.0.as_ptr();
            // platform_driver.0.driver.name = c_str!("of_serial").as_char_ptr();

            let data: Arc<PlatformData> = Arc::try_new(PlatformData { ops, of, driver })?;
            let data2: Arc<PlatformData> = data.clone();
            let data_ptr = data2.into_raw();
            (*devs).dev.platform_data = data_ptr as _;

            to_result(platform_device_add(devs))?;
            platform_driver.register(module);
            pr_println!("platform_driver register finish");

            platform_device_del(devs);
            pr_println!("init finish");
            Ok(RustUartModule {
                platform_driver,
                data,
            })
        }
    }
}

impl Drop for RustUartModule {
    fn drop(&mut self) {
        pr_info!("Rust UART (exit)\n");
    }
}
extern "C" fn probe(dev: *mut platform_device) -> i32 {
    pr_println!("probe");
    from_result(|| {
        unsafe {
            // let dev = &mut *dev;
            // let data_ptr = dev.dev.platform_data as *const PlatformData;
            // let platform = Arc::from_raw(data_ptr);
            // let port = DevPort::new()?;
            // let port: Arc<DevPort> = Arc::pin_init(port)?;

            // let port_ptr = port.into_raw();
        
            // let mut kport: KPort = KPort::new(0, &platform.as_ref().ops)?;
            // kport.0.private_data = port_ptr as _;
            
            

            // dev.dev.driver_data = ;
        }
        Ok(0)
    })
}
extern "C" fn remove(dev: *mut platform_device) -> i32 {
    // pr_println!("remove");
    0
}
extern "C" fn suspend(dev: *mut platform_device, msg: pm_message) -> i32 {
    // pr_println!("suspend");
    0
}
extern "C" fn resume(dev: *mut platform_device) -> i32 {
    // pr_println!("resume");
    0
}
struct OfDeviceIdList(Vec<of_device_id>);
unsafe impl Send for OfDeviceIdList {}
unsafe impl Sync for OfDeviceIdList {}

fn of_device_id_list() -> Vec<of_device_id> {
    let mut out = Vec::new();
    // out.try_push(new_of_device_id(c_str!("ns8250"), PORT_8250)).unwrap();
    out.try_push(new_of_device_id(c_str!("ns16550a"), PORT_16550A))
        .unwrap();

    out
}

fn new_of_device_id(comp: &CStr, data: u32) -> of_device_id {
    let mut compatible = [0; 128];
    let ptr = comp.as_bytes();
    for i in 0..ptr.len() {
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

#[pin_data]
struct DevPort {
    #[pin]
    rport: SpinLock<RPort>,
}

impl DevPort {
    fn new() -> Result<impl PinInit<Self>> {
        Ok(pin_init!(Self {
            rport<- new_spinlock!(RPort::new())
        }))
    }
}
