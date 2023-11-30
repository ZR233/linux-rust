
use super::uart_opt::UartOps;
use crate::bindings::*;
use crate::prelude::*;
use core::mem::size_of;
use core::ptr::null_mut;

#[derive(Default, Clone, Copy)]
pub struct UartPort(uart_port);
unsafe impl Send for UartPort {}
unsafe impl Sync for UartPort {}

// impl From<uart_port> for UartPort {
//     fn from(value: uart_port) -> Self {
//         Self(value)
//     }
// }

impl UartPort {
    pub const unsafe fn zero() -> Self {
        unsafe {
            let v = core::mem::MaybeUninit::<uart_port>::zeroed();
            let v = v.assume_init();
            Self(v)
        }
    }


    // pub const fn zero() -> Self {
    //     Self(uart_port {
    //         lock: spinlock {
    //             _bindgen_opaque_blob: [0; 3],
    //         },
    //         iobase: 0,
    //         membase: null_mut(),
    //         serial_in: None,
    //         serial_out: None,
    //         set_termios: None,
    //         set_ldisc: None,
    //         get_mctrl: None,
    //         set_mctrl: None,
    //         get_divisor: None,
    //         set_divisor: None,
    //         startup: None,
    //         shutdown: None,
    //         throttle: None,
    //         unthrottle: None,
    //         handle_irq: None,
    //         pm: None,
    //         handle_break: None,
    //         rs485_config: None,
    //         iso7816_config: None,
    //         ctrl_id: 0,
    //         port_id: 0,
    //         irq: 0,
    //         irqflags: 0,
    //         uartclk: 0,
    //         fifosize: 0,
    //         x_char: 0,
    //         regshift: 0,
    //         iotype: 0,
    //         quirks: 0,
    //         read_status_mask: 0,
    //         ignore_status_mask: 0,
    //         state: null_mut(),
    //         icount: uart_icount {
    //             cts: 0,
    //             dsr: 0,
    //             rng: 0,
    //             dcd: 0,
    //             rx: 0,
    //             tx: 0,
    //             frame: 0,
    //             overrun: 0,
    //             parity: 0,
    //             brk: 0,
    //             buf_overrun: 0,
    //         },
    //         cons: null_mut(),
    //         flags: 0,
    //         status: 0,
    //         hw_stopped: false,
    //         mctrl: 0,
    //         frame_time: 0,
    //         type_: 0,
    //         ops: null_mut(),
    //         custom_divisor: 0,
    //         line: 0,
    //         minor: 0,
    //         mapbase: 0,
    //         mapsize: 0,
    //         dev: null_mut(),
    //         port_dev: null_mut(),
    //         sysrq: 0,
    //         sysrq_ch: 0,
    //         has_sysrq: 0,
    //         sysrq_seq: 0,
    //         hub6: 0,
    //         suspended: 0,
    //         console_reinit: 0,
    //         name: null_mut(),
    //         attr_group: null_mut(),
    //         tty_groups: null_mut(),
    //         rs485: serial_rs485_zero(),
    //         rs485_supported: serial_rs485_zero(),
    //         rs485_term_gpio: null_mut(),
    //         rs485_rx_during_tx_gpio: null_mut(),
    //         iso7816: serial_iso7816 {
    //             tg: 0,
    //             sc_fi: 0,
    //             sc_di: 0,
    //             clk: 0,
    //             reserved: [0; 5],
    //             flags: 0,
    //         },
    //         private_data: null_mut(),
    //     })
    // }

    pub unsafe fn as_ptr(&self) -> *mut uart_port {
        &self.0 as *const _ as _
    }

    pub fn set_ops(&mut self, ops: &UartOps) {
        unsafe {
            self.0.ops = ops.as_ptr();
        }
    }
}

// const fn serial_rs485_zero() -> serial_rs485 {
//     serial_rs485 {
//         flags: 0,
//         delay_rts_before_send: 0,
//         delay_rts_after_send: 0,
//         __bindgen_anon_1: serial_rs485__bindgen_ty_1_zero(),
//     }
// }

// const fn serial_rs485__bindgen_ty_1_zero()-> serial_rs485__bindgen_ty_1 {
//     serial_rs485__bindgen_ty_1 {
//         padding: [0; 5],
//         __bindgen_anon_1: serial_rs485__bindgen_ty_1__bindgen_ty_1_zero(),
//     }
// }

// const fn serial_rs485__bindgen_ty_1__bindgen_ty_1_zero() -> serial_rs485__bindgen_ty_1__bindgen_ty_1
// {
//     serial_rs485__bindgen_ty_1__bindgen_ty_1 {
//         addr_recv: 0,
//         addr_dest: 0,
//         padding0: [0; 2],
//         padding1: [0; 4],
//     }
// }
