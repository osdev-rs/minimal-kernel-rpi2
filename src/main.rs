#![feature(global_asm, asm)]
#![feature(alloc, alloc_error_handler)]
#![feature(core_intrinsics, lang_items)]
#![feature(ptr_wrapping_offset_from)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

global_asm!(include_str!("boot.S"));

#[macro_use]
extern crate alloc;

use core::panic::PanicInfo;
use core::intrinsics::volatile_load;
//use alloc::prelude::*;

mod uart;
mod mem;
mod util;
mod timer;

use self::util::{mmio_write, mmio_read};

#[allow(dead_code)]
mod constval {
    pub const CORE0_INTERRUPT_SOURCE: u32 = 0x40000060;
    pub const IRQ_PEND_BASIC: u32 = 0x3F00B200;
    pub const IRQ_PEND1: u32 = 0x3F00B204;
    pub const IRQ_PEND2: u32 = 0x3F00B208;
    pub const UART0_MIS: u32 = 0x3f201040;
    pub const GPU_INTERRUPTS_ROUTING: u32 = 0x4000000C;
}

use self::constval::*;

#[global_allocator]
static GLOBAL: mem::KernelAllocator = mem::KernelAllocator;

#[no_mangle]
pub extern fn kernel_main() {

    unsafe {mem::init()};
    uart::init();
    timer::init();

    // route IRQ to CORE0
    unsafe {mmio_write(GPU_INTERRUPTS_ROUTING, 0u32);};

    enable_irq();

//    uart::write(&format!("{}\n", "hello, rust-os"));

    loop {
//        uart::write(&format!("SYSTEM_TIMER_C1: {}\n", unsafe { mmio_read(timer::SYSTEM_TIMER_CLO) }));
    }
}

#[inline]
fn enable_irq() {
    unsafe { asm!("cpsie i");}
}

#[inline]
fn disable_irq() {
    unsafe {asm!("cpsid i");}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn check_flag(addr: u32, val: u32) -> bool {
    return volatile_load(addr as *mut u32) & val > 0;
}

#[no_mangle]
pub extern fn irq_handler() {
    disable_irq();

    if unsafe { check_flag(CORE0_INTERRUPT_SOURCE, 1<<8) } {
        if unsafe { check_flag(IRQ_PEND2, 1 << 25)} {
            if unsafe { check_flag(UART0_MIS, 1 << 4) } {
                uart::writec(uart::getc());
            }
        }

        if unsafe { check_flag(IRQ_PEND1, 1 << 1) } {
            timer::timer_isr();
        }
    }

    uart::write("\nirq_handler\n");
    enable_irq();
}

// for custom allocator
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}
