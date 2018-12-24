#![feature(global_asm)]
#![feature(core_intrinsics, lang_items)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

global_asm!(include_str!("boot.S"));

use core::panic::PanicInfo;
use core::intrinsics::abort;

mod uart;

#[no_mangle]
pub extern fn kernel_main() {
    uart::write("Hello Rust Kernel world!\n");
    loop {
        uart::writec(uart::getc())
    }
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

#[no_mangle]
pub extern fn irq_handler() {
    uart::write("irq_handler\n");
}
