use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

#[allow(dead_code)]
mod constval {
    pub const IRQ_ENABLE1: u32 = 0x3F00B210;
    pub const IRQ_ENABLE2: u32 = 0x3F00B214;
}

use self::constval::*;

pub unsafe fn mmio_write(reg: u32, val: u32) {
    volatile_store(reg as *mut u32, val);
}

pub unsafe fn mmio_read(reg: u32) -> u32 {
    volatile_load(reg as *const u32)
}

pub fn enable_irq_no(irq_no: u8) {
    unsafe {
        let (irq_no, addr) = if irq_no < 32 { (irq_no, IRQ_ENABLE1) } else { (irq_no-32, IRQ_ENABLE2) };

        let mut irq_val = mmio_read(addr);
        irq_val |= 1 << irq_no;
        mmio_write(addr, irq_val);
    }
}
