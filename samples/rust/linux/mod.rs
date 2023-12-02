pub mod port;
use kernel::bindings::*;
use kernel::c_str;
use kernel::error::*;
use kernel::prelude::*;

/// Calls a closure returning a [`crate::error::Result<T>`] and converts the result to
/// a C integer result.
///
/// This is useful when calling Rust functions that return [`crate::error::Result<T>`]
/// from inside `extern "C"` functions that need to return an integer error result.
///
/// `T` should be convertible from an `i16` via `From<i16>`.
///
/// # Examples
///
/// ```ignore
/// # use kernel::from_result;
/// # use kernel::bindings;
/// unsafe extern "C" fn probe_callback(
///     pdev: *mut bindings::platform_device,
/// ) -> core::ffi::c_int {
///     from_result(|| {
///         let ptr = devm_alloc(pdev)?;
///         bindings::platform_set_drvdata(pdev, ptr);
///         Ok(0)
///     })
/// }
/// ```
// TODO: Remove `dead_code` marker once an in-kernel client is available.
#[allow(dead_code)]
pub(crate) fn from_result<T, F>(f: F) -> T
where
    T: From<i16>,
    F: FnOnce() -> Result<T>,
{
    match f() {
        Ok(v) => v,
        // NO-OVERFLOW: negative `errno`s are no smaller than `-bindings::MAX_ERRNO`,
        // `-bindings::MAX_ERRNO` fits in an `i16` as per invariant above,
        // therefore a negative `errno` always fits in an `i16` and will not overflow.
        Err(e) => T::from(e.to_errno() as i16),
    }
}

pub(crate) struct Serial8250Config {
    pub(crate) name: &'static CStr,
    pub(crate) fifo_size: u32,
    pub(crate) tx_loadsz: u32,
    pub(crate) fcr: u32,
    pub(crate) rxtrig_bytes: [u32; 4],
    pub(crate) flags: u32,
}

impl Serial8250Config {
    /// .name		= "16550A",
    /// .fifo_size	= 16,
    /// .tx_loadsz	= 16,
    /// .fcr		= UART_FCR_ENABLE_FIFO | UART_FCR_R_TRIG_10,
    /// .rxtrig_bytes	= {1, 4, 8, 14},
    /// .flags		= UART_CAP_FIFO,
    pub(crate) fn ns16550a() -> Self {
        Serial8250Config {
            name: c_str!("16550A"),
            fifo_size: 16,
            tx_loadsz: 16,
            fcr: UART_FCR_ENABLE_FIFO | UART_FCR_R_TRIG_10,
            rxtrig_bytes: [1, 4, 8, 14],
            flags: UART_CAP_FIFO,
        }
    }
}
