/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header that contains the code (mostly headers) for which Rust bindings
 * will be automatically generated by `bindgen`.
 *
 * Sorted alphabetically.
 */

#include <kunit/test.h>
#include <linux/errname.h>
#include <linux/slab.h>
#include <linux/refcount.h>
#include <linux/wait.h>
#include <linux/sched.h>
#include <linux/workqueue.h>
#include <linux/module.h>
#include <linux/acpi.h>
#include <linux/moduleparam.h>
#include <linux/ioport.h>
#include <linux/init.h>
#include <linux/console.h>
#include <linux/sysrq.h>
#include <linux/delay.h>
#include <linux/platform_device.h>
#include <linux/pm_runtime.h>
#include <linux/tty.h>
#include <linux/ratelimit.h>
#include <linux/tty_flip.h>
#include <linux/serial.h>
#include <linux/serial_8250.h>
#include <linux/nmi.h>
#include <linux/mutex.h>
#include <linux/slab.h>
#include <linux/string_helpers.h>
#include <linux/uaccess.h>
#include <linux/io.h>
#include <linux/ioport.h>
#include <linux/init.h>
#include <linux/console.h>
#include <linux/sysrq.h>
#include <linux/platform_device.h>
#include <linux/tty.h>
#include <linux/tty_flip.h>
#include <linux/serial_core.h>
#include <linux/serial.h>
#include <linux/clk.h>
#include <linux/delay.h>
#include <linux/ktime.h>
#include <linux/pinctrl/consumer.h>
#include <linux/rational.h>
#include <linux/slab.h>
#include <linux/of.h>
#include <linux/of_address.h>
#include <linux/of_irq.h>
#include <linux/io.h>
#include <linux/dma-mapping.h>
#include <asm/irq.h>
#include <asm/sbi.h>
#include <linux/dma/imx-dma.h>
#include <linux/nmi.h>
#include <linux/serial.h>
#include <linux/irqreturn.h>

/* `bindgen` gets confused at certain things. */
const size_t RUST_CONST_HELPER_ARCH_SLAB_MINALIGN = ARCH_SLAB_MINALIGN;
const gfp_t RUST_CONST_HELPER_GFP_KERNEL = GFP_KERNEL;
const gfp_t RUST_CONST_HELPER___GFP_ZERO = __GFP_ZERO;


const upf_t RUST_CONST_HELPER_UPF_IOREMAP = UPF_IOREMAP;
const upf_t RUST_CONST_HELPER_UPF_SHARE_IRQ = UPF_SHARE_IRQ;
const upf_t RUST_CONST_HELPER_UPF_BOOT_AUTOCONF = UPF_BOOT_AUTOCONF;
const upf_t RUST_CONST_HELPER_UPF_FIXED_PORT = UPF_FIXED_PORT;
const upf_t RUST_CONST_HELPER_UPF_FIXED_TYPE = UPF_FIXED_TYPE;
const unsigned int RUST_CONST_HELPER_UART_CAP_FIFO = BIT(8);

const upstat_t RUST_CONST_HELPER_UPSTAT_AUTOCTS = UPSTAT_AUTOCTS;
const upstat_t RUST_CONST_HELPER_UPSTAT_AUTORTS = UPSTAT_AUTORTS;
const irqreturn_t RUST_CONST_HELPER_IRQ_NONE = IRQ_NONE;
const irqreturn_t RUST_CONST_HELPER_IRQ_HANDLED = IRQ_HANDLED;