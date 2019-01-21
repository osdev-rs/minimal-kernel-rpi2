use core::intrinsics::{volatile_load, volatile_store};

#[allow(dead_code)]
mod timer_registers {
    pub const SYSTEM_TIMER_BASE:u32 = 0x3F003000;
    pub const SYSTEM_TIMER_CS:u32 = SYSTEM_TIMER_BASE + 0x00;
    pub const SYSTEM_TIMER_CLO:u32 = SYSTEM_TIMER_BASE + 0x04;
    pub const SYSTEM_TIMER_CHI:u32 = SYSTEM_TIMER_BASE + 0x08;
    pub const SYSTEM_TIMER_C0:u32 = SYSTEM_TIMER_BASE + 0x0C;
    pub const SYSTEM_TIMER_C1:u32 = SYSTEM_TIMER_BASE + 0x10;
    pub const SYSTEM_TIMER_C2:u32 = SYSTEM_TIMER_BASE + 0x14;
    pub const SYSTEM_TIMER_C3:u32 = SYSTEM_TIMER_BASE + 0x18;
}

pub use self::timer_registers::*;
use super::util::{mmio_write, mmio_read, enable_irq_no, IRQ_ENABLE1};
use super::uart;

pub fn timer_isr() {
    unsafe {
        uart::write("tick!!\n");
    }
}

pub fn init() {
    unsafe {
        let now = mmio_read(SYSTEM_TIMER_CLO);
        uart::write(&format!("SYSTEM_TIMER_CLO: {}\n", now));
        mmio_write(SYSTEM_TIMER_C1, now + 1_000_000);
        mmio_write(SYSTEM_TIMER_CS, 0); // Timer 1 match detected

        mmio_write(IRQ_ENABLE1, 1<<1);
    }
}
