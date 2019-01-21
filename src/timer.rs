use core::intrinsics::{volatile_load, volatile_store};

#[allow(dead_code)]
mod timer_registers {
    pub const CORE0_TIMER_IRQCNTL:u32 = 0x4000_0040;
    pub const CORE0_IRQ_SOURCE:u32 = 0x4000_0060;
}

pub use self::timer_registers::*;
use super::util::{mmio_write, mmio_read, enable_irq_no, IRQ_ENABLE1};
use super::uart;

static mut cnt_1sec:u32 = 0;

pub fn timer_isr() {
    unsafe {
        uart::write("tick!!\n");
        write_cntv_tval(cnt_1sec);
    }
}

fn routing_core0cntv_to_core0irq() {
    unsafe {
        mmio_write(CORE0_TIMER_IRQCNTL, 0x08);
    }
}

pub fn read_core0timer_pending() -> u32 {
    unsafe {
        mmio_read(CORE0_IRQ_SOURCE)
    }
}

fn enable_cntv() {
    unsafe {
        let cntv_ctl = 1u32;
        asm!("mcr p15, 0, $0, c14, c3, 1" :: "r"(cntv_ctl));
    }
}

fn read_cntfrq() -> u32 {
    unsafe {
        let mut val:u32 = 0;
        asm!("mrc p15, 0, $0, c14, c0, 0" : "=r"(val) );
        val
    }
}

fn write_cntv_tval(val: u32) {
    unsafe {
        asm!("mcr p15, 0, $0, c14, c3, 0" :: "r"(val));
    }
}

pub fn init() {
    unsafe {
        cnt_1sec = read_cntfrq();
        write_cntv_tval(cnt_1sec);

        routing_core0cntv_to_core0irq();
        enable_cntv();
    }
}
