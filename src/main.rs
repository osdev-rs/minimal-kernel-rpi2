#![feature(global_asm)]
#![feature(core_intrinsics, lang_items)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

global_asm!(include_str!("boot.S"));

use core::panic::PanicInfo;
use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi2 and raspi3 have peripheral base address 0x3F000000,
// but raspi1 has peripheral base address 0x20000000. Ensure
// you are using the correct peripheral address for your
// hardware.
const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    write("Hello Rust Kernel world!\n");
    loop {
        writec(getc())
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
