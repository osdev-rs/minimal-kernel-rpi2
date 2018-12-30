#![feature(global_asm, asm)]
#![feature(alloc, alloc_error_handler)]
#![feature(core_intrinsics, lang_items)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

global_asm!(include_str!("boot.S"));

extern crate alloc;

use core::panic::PanicInfo;
use core::intrinsics::volatile_load;
use crate::alloc::boxed::Box;

mod uart;
mod hw;
mod mem;

const CORE0_INTERRUPT_SOURCE: u32 = 0x40000060;
const IRQ_PEND2: u32 = 0x3F00B208;
const UART0_MIS: u32 = 0x3f201040;

#[global_allocator]
static GLOBAL: mem::KernelAllocator = mem::KernelAllocator;

#[no_mangle]
pub extern fn kernel_main() {
    uart::write("Hello Rust Kernel world!\n");
    hw::enable_uart_irq();
    enable_irq();

    let v = Box::new(4);

//    uart::write("irq_enable\n");
    loop {
        //uart::writec(uart::getc())
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

//#[no_mangle]
//pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

//#[allow(non_snake_case)]
//#[no_mangle]
//pub extern fn _Unwind_Resume() { loop {} }

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
    }

    uart::write("\nirq_handler\n");
    enable_irq();
}

// for custom allocator
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}
