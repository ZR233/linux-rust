// SPDX-License-Identifier: GPL-2.0
/*
 * Non-trivial C macros cannot be used in Rust. Similarly, inlined C functions
 * cannot be called either. This file explicitly creates functions ("helpers")
 * that wrap those so that they can be called from Rust.
 *
 * Even though Rust kernel modules should never use directly the bindings, some
 * of these helpers need to be exported because Rust generics and inlined
 * functions may not get their code generated in the crate where they are
 * defined. Other helpers, called from non-inline functions, may not be
 * exported, in principle. However, in general, the Rust compiler does not
 * guarantee codegen will be performed for a non-inline function either.
 * Therefore, this file exports all the helpers. In the future, this may be
 * revisited to reduce the number of exports after the compiler is informed
 * about the places codegen is required.
 *
 * All symbols are exported as GPL-only to guarantee no GPL-only feature is
 * accidentally exposed.
 *
 * Sorted alphabetically.
 */

// #include "asm-generic/io.h"
#include "asm/sbi.h"
#include "linux/ioport.h"
#include "linux/nmi.h"
#include <asm/io.h>
#include "linux/serial_core.h"
#include "linux/spinlock_types.h"
#include <kunit/test-bug.h>
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/err.h>
#include <linux/errname.h>
#include <linux/mutex.h>
#include <linux/refcount.h>
#include <linux/sched/signal.h>
#include <linux/spinlock.h>
#include <linux/wait.h>
#include <linux/workqueue.h>
#include <linux/device.h>

__noreturn void rust_helper_BUG(void)
{
	BUG();
}
EXPORT_SYMBOL_GPL(rust_helper_BUG);

void rust_helper_mutex_lock(struct mutex *lock)
{
	mutex_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_mutex_lock);

void rust_helper___spin_lock_init(spinlock_t *lock, const char *name,
				  struct lock_class_key *key)
{
#ifdef CONFIG_DEBUG_SPINLOCK
	__raw_spin_lock_init(spinlock_check(lock), name, key, LD_WAIT_CONFIG);
#else
	spin_lock_init(lock);
#endif
}
EXPORT_SYMBOL_GPL(rust_helper___spin_lock_init);

void rust_helper_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock);

void rust_helper_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock);

void rust_helper_init_wait(struct wait_queue_entry *wq_entry)
{
	init_wait(wq_entry);
}
EXPORT_SYMBOL_GPL(rust_helper_init_wait);

int rust_helper_signal_pending(struct task_struct *t)
{
	return signal_pending(t);
}
EXPORT_SYMBOL_GPL(rust_helper_signal_pending);

refcount_t rust_helper_REFCOUNT_INIT(int n)
{
	return (refcount_t)REFCOUNT_INIT(n);
}
EXPORT_SYMBOL_GPL(rust_helper_REFCOUNT_INIT);

void rust_helper_refcount_inc(refcount_t *r)
{
	refcount_inc(r);
}
EXPORT_SYMBOL_GPL(rust_helper_refcount_inc);

bool rust_helper_refcount_dec_and_test(refcount_t *r)
{
	return refcount_dec_and_test(r);
}
EXPORT_SYMBOL_GPL(rust_helper_refcount_dec_and_test);

__force void *rust_helper_ERR_PTR(long err)
{
	return ERR_PTR(err);
}
EXPORT_SYMBOL_GPL(rust_helper_ERR_PTR);

bool rust_helper_IS_ERR(__force const void *ptr)
{
	return IS_ERR(ptr);
}
EXPORT_SYMBOL_GPL(rust_helper_IS_ERR);

long rust_helper_PTR_ERR(__force const void *ptr)
{
	return PTR_ERR(ptr);
}
EXPORT_SYMBOL_GPL(rust_helper_PTR_ERR);

const char *rust_helper_errname(int err)
{
	return errname(err);
}
EXPORT_SYMBOL_GPL(rust_helper_errname);

struct task_struct *rust_helper_get_current(void)
{
	return current;
}
EXPORT_SYMBOL_GPL(rust_helper_get_current);

void rust_helper_get_task_struct(struct task_struct *t)
{
	get_task_struct(t);
}
EXPORT_SYMBOL_GPL(rust_helper_get_task_struct);

void rust_helper_put_task_struct(struct task_struct *t)
{
	put_task_struct(t);
}
EXPORT_SYMBOL_GPL(rust_helper_put_task_struct);

struct kunit *rust_helper_kunit_get_current_test(void)
{
	return kunit_get_current_test();
}
EXPORT_SYMBOL_GPL(rust_helper_kunit_get_current_test);

void rust_helper_init_work_with_key(struct work_struct *work, work_func_t func,
				    bool onstack, const char *name,
				    struct lock_class_key *key)
{
	__init_work(work, onstack);
	work->data = (atomic_long_t)WORK_DATA_INIT();
	lockdep_init_map(&work->lockdep_map, name, key, 0);
	INIT_LIST_HEAD(&work->entry);
	work->func = func;
}
EXPORT_SYMBOL_GPL(rust_helper_init_work_with_key);

void rust_helper_sbi_console_put(int ch)
{
	sbi_ecall(0x01, 0, ch, 0, 0, 0, 0, 0);
}
EXPORT_SYMBOL_GPL(rust_helper_sbi_console_put);

void rust_helper_spin_lock_irq(spinlock_t *lock)
{
	spin_lock_irq(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock_irq);

void rust_helper_spin_unlock_irq(spinlock_t *lock)
{
	spin_unlock_irq(lock);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock_irq);

void rust_helper_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_unlock_irqrestore);

void rust_helper_spin_lock_irqsave(spinlock_t *lock, unsigned long *flags)
{
	spin_lock_irqsave(lock, *flags);
}
EXPORT_SYMBOL_GPL(rust_helper_spin_lock_irqsave);

struct uart_port rust_helper_uart_port_zero(int v)
{
	struct uart_port a = {};
	return a;
}
EXPORT_SYMBOL_GPL(rust_helper_uart_port_zero);

struct resource *rust_helper_request_mem_region(resource_size_t start,
						resource_size_t n,
						const char *name)
{
	return request_mem_region(start, n, name);
}
EXPORT_SYMBOL_GPL(rust_helper_request_mem_region);

void *rust_helper_ioremap(resource_size_t addr, unsigned int size)
{
	return ioremap(addr, size);
}
EXPORT_SYMBOL_GPL(rust_helper_ioremap);
void rust_helper_touch_nmi_watchdog(int a)
{
	touch_nmi_watchdog();
}
EXPORT_SYMBOL_GPL(rust_helper_touch_nmi_watchdog);

unsigned int rust_helper_readb(unsigned char *addr)
{
	return readb(addr);
}
EXPORT_SYMBOL_GPL(rust_helper_readb);

void rust_helper_writeb(int value, unsigned char *addr)
{
	writeb(value, addr);
}
EXPORT_SYMBOL_GPL(rust_helper_writeb);


static ssize_t rx_trig_bytes_show(struct device *dev,
	struct device_attribute *attr, char *buf)
{
	return 0;
}



static ssize_t rx_trig_bytes_store(struct device *dev,
	struct device_attribute *attr, const char *buf, size_t count)
{
	return 0;
}
static DEVICE_ATTR_RW(rx_trig_bytes);
struct device_attribute * rust_helper_dev_attr_rx_trig_bytes(int a)
{
	return &dev_attr_rx_trig_bytes ;
}
EXPORT_SYMBOL_GPL(rust_helper_dev_attr_rx_trig_bytes);


bool rust_helper_irqd_is_wakeup_set(struct irq_data *d)
{
	return irqd_is_wakeup_set(d) ;
}
EXPORT_SYMBOL_GPL(rust_helper_irqd_is_wakeup_set);

bool rust_helper_uart_tx_stopped(struct uart_port *port)
{
	return uart_tx_stopped(port) ;
}
EXPORT_SYMBOL_GPL(rust_helper_uart_tx_stopped);

void rust_helper_uart_xmit_advance(struct uart_port *up, unsigned int chars)
{
	uart_xmit_advance(up, chars);
}
EXPORT_SYMBOL_GPL(rust_helper_uart_xmit_advance);

/*
 * `bindgen` binds the C `size_t` type as the Rust `usize` type, so we can
 * use it in contexts where Rust expects a `usize` like slice (array) indices.
 * `usize` is defined to be the same as C's `uintptr_t` type (can hold any
 * pointer) but not necessarily the same as `size_t` (can hold the size of any
 * single object). Most modern platforms use the same concrete integer type for
 * both of them, but in case we find ourselves on a platform where
 * that's not true, fail early instead of risking ABI or
 * integer-overflow issues.
 *
 * If your platform fails this assertion, it means that you are in
 * danger of integer-overflow bugs (even if you attempt to add
 * `--no-size_t-is-usize`). It may be easiest to change the kernel ABI on
 * your platform such that `size_t` matches `uintptr_t` (i.e., to increase
 * `size_t`, because `uintptr_t` has to be at least as big as `size_t`).
 */
static_assert(sizeof(size_t) == sizeof(uintptr_t) &&
		      __alignof__(size_t) == __alignof__(uintptr_t),
	      "Rust code expects C `size_t` to match Rust `usize`");
