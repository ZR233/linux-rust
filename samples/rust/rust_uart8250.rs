// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample
#![allow(unused)]
#![allow(non_upper_case_globals)]

/// linux
pub mod linux;
/// rport
pub mod rport;
mod sbi_print;
use crate::linux::port::UART_OPS;
use crate::rport::*;
pub use crate::sbi_print::*;
use core::convert::AsRef;
use core::default::Default;
use core::f32::consts::E;
use core::ptr::null_mut;
use core::ptr::slice_from_raw_parts;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::learn::platform_driver::*;
use kernel::learn::uart_driver::*;
use kernel::learn::uart_opt::*;
use kernel::learn::uart_port::*;
use kernel::new_spinlock;
use kernel::prelude::*;
use kernel::str::CString;
use kernel::sync::*;

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
static CONSOLE: Console = unsafe { Console::new(UART_DRIVER.as_ptr()) };
static mut PORTS: [UartPort; NR as usize] = unsafe { [UartPort::zero(); NR as usize] };
static UART_DRIVER: UartDriver = unsafe {
    UartDriver::from_struct(uart_driver {
        nr: NR,
        major: TTY_MAJOR,
        minor: TTY_MINOR,
        dev_name: DEV_NAME.as_char_ptr(),
        driver_name: DRIVER_NAME.as_char_ptr(),
        owner: &THIS_MODULE as *const _ as _,
        cons: CONSOLE.as_ptr(),
        state: null_mut(),
        tty_driver: null_mut(),
    })
};

struct RustUartModule {
    platform_driver: PlatformDriver,
    data: Arc<PlatformData>,
}

struct PlatformData {
    of: OfDeviceIdList,
    // driver: UartDriver,
    test: u32,
}

struct Console(console);
impl Console {
    const unsafe fn new(reg: *mut uart_driver) -> Self {
        let mut name = [
            't' as _, 't' as _, 'y' as _, 'S' as _, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        Self(console {
            name,
            write: Some(console_write),
            read: Some(console_read),
            device: Some(uart_console_device),
            unblank: None,
            setup: Some(console_setup),
            exit: None,
            match_: Some(console_match),
            flags: (cons_flags_CON_PRINTBUFFER | cons_flags_CON_ANYTIME) as _,
            index: -1,
            cflag: 0,
            ispeed: 0,
            ospeed: 0,
            seq: 0,
            dropped: 0,
            data: reg as _,
            node: hlist_node {
                next: null_mut(),
                pprev: null_mut(),
            },
            write_atomic: None,
            nbcon_state: atomic_t { counter: 0 },
            nbcon_seq: atomic64_t { counter: 0 },
            pbufs: null_mut(),
        })
    }

    const fn as_ptr(&self) -> *mut console {
        unsafe { &self.0 as *const _ as _ }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}

extern "C" fn console_write(co: *mut console, char: *const i8, count: u32) {
    // pr_println!("console write");
    unsafe {
        // let bytes = &*slice_from_raw_parts(char, count as _);
        // print_bytes(bytes);
    }
}
extern "C" fn console_read(co: *mut console, char: *mut i8, count: u32) -> i32 {
    pr_println!("console read");
    unsafe {
        let bytes = &*slice_from_raw_parts(char, count as _);
        print_bytes(bytes);
    }
    0
}

extern "C" fn console_setup(co: *mut console, options: *mut i8) -> i32 {
    unsafe {
        let co = &mut *co;
        let index = co.index as usize;
        let op = CStr::from_char_ptr(options);
        pr_println!("console {index} setup: {}", op.to_str().unwrap());
        let port = up_from_kport( &PORTS[index]);
        
    }
    0
}

fn u8250_console_setup(port: &mut uart_port, options: *mut i8, early: bool) -> Result {
    let mut baud = 9600;
    let mut bits = 8;
    let mut parity = 'n' as i32;
    let mut flow = 'n' as i32;
    // let mut resource = resource::default();

    if (port.iobase == 0 && port.membase.is_null()) {
        return Err(code::ENODEV);
    }
    unsafe {
        if !port.dev.is_null() {
            to_result(__pm_runtime_resume(port.dev, RPM_GET_PUT as _))?;
        }
    }

    Ok(())
}

extern "C" fn console_match(co: *mut console, name: *mut i8, index: i32, options: *mut i8) -> i32 {
    from_result(|| unsafe {
        let name = CStr::from_char_ptr(name).to_str().unwrap();
        pr_println!("console match name:{}", name);
        if !options.is_null() {
            let options_str = CStr::from_char_ptr(options).to_str().unwrap();

            pr_println!("options:{}", options_str);
        }

        let mut iotype = 0;
        let mut addr = 0;

        /* try to match the port specified on the command line */
        for i in 0..NR as usize {
            let port_w = &PORTS[i];
            let port = &mut *port_w.as_ptr();

            let index = i as i16;
            (&mut *co).index = index;
            port.cons = co;
            pr_println!("use port {index} as console");

            return Ok(0);
        }
        Ok(0)
    })
}

impl kernel::Module for RustUartModule {
    fn init(module: &'static ThisModule) -> Result<Self> {
        pr_println!("Rust UART (init)");
        let of = OfDeviceIdList(of_device_id_list());

        pr_println!("uart register begin");
        UART_DRIVER.register(module)?;
        pr_println!("uart register ok");
        unsafe {
            let devs =
                platform_device_alloc(PLATFORM_DRIVER_NAME.as_char_ptr(), PLAT8250_DEV_LEGACY);

            let mut platform_driver = PlatformDriver::from(platform_driver {
                driver: device_driver {
                    name: c_str!("of_serial").as_char_ptr(),
                    of_match_table: of.0.as_ptr(),
                    ..Default::default()
                },
                probe: Some(probe),
                ..Default::default()
            });

            let data: Arc<PlatformData> = Arc::try_new(PlatformData {
                of,
                // driver,
                test: 5,
            })?;

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
extern "C" fn probe(pl_dev: *mut platform_device) -> i32 {
    from_result(|| {
        unsafe {
            let pdev = &mut *pl_dev;
            let np = pdev.dev.of_node;
            let dev = &mut pdev.dev as *mut _;
            let name = CStr::from_char_ptr(pdev.name);
            let co = &*CONSOLE.as_ptr();
            let index = co.index as i32;
            pr_println!("probe: {}, console=[{}]", name, index);

            let mut resource = resource::default();

            let index = if index < 0 { 0 } else { index as usize };

            let kport = &PORTS[index];
            let port = &mut *kport.as_ptr();

            to_result(of_address_to_resource(np, 0, &mut resource))?;

            pm_runtime_enable(dev);
            __pm_runtime_resume(dev, RPM_GET_PUT as _);

            port.irq = of_irq_get(np, 0) as _;
            port.uartclk = 0x00384000;
            port.regshift = 0;
            port.dev = dev;

            port.mapbase = resource.start;
            port.mapsize = resource.end - resource.start + 1;
            port.iotype = UPIO_MEM as _;
            port.flags = UPF_SHARE_IRQ | UPF_BOOT_AUTOCONF | UPF_FIXED_PORT | UPF_FIXED_TYPE;

            spin_lock_init(&mut port.lock);
            // platform_get_resource(pdev, arg2, arg3);

            // RPort::new(kport)?;
            let p = RPort::new(index as usize)?;
            let ptr = Box::into_raw(p);
   
            (&mut *port.dev).driver_data = ptr as _;

            // let rport_ptr = Box::into_raw(rport);
            // let mut kport = UartPort::from(port);
            // kport.set_ops(&UART_OPS);
            // let port_ptr = port_warp.into_raw();
            // pdev.dev.driver_data = port_ptr as _;
            // pdev.dev.driver_data = rport_ptr as _;

            port.flags |= UPF_IOREMAP;
            pr_println!("add_one_port begin");
            UART_DRIVER.add_one_port(kport)?;
        }
        pr_println!("probe finish");
        Ok(0)
    })
}
extern "C" fn remove(dev: *mut platform_device) -> i32 {
    pr_println!("remove");
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
///static initcall_t __initcall__kmod_8250__2_721_univ8250_console_initcon
///    __attribute__((__used__))
///    __attribute__((__section__(".con_initcall"
///                   ".init"))) = univ8250_console_init;
#[doc(hidden)]
#[link_section = ".con_initcall.init"]
#[used]
pub static __Rust_UART_con_initcall: extern "C" fn() -> core::ffi::c_int = __Rust_UART_console_init;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __Rust_UART_console_init() -> core::ffi::c_int {
    console_init();
    0
}

fn console_init() {
    pr_println!("console init");

    unsafe {
        for i in 0..NR as usize {
            let one = &PORTS[i];
            uart_port_init(i as _, one);
        }

        register_console(CONSOLE.as_ptr());
        pr_println!("console register ok");
    }
}
