use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

const IRQ_ENABLE2: u32 = 0x3F00B214;
const IRQ_DISABLE2: u32 = 0x3F00B220;
const UART0_IMSC: u32 = 0x3F201038;

const GPU_INTERRUPTS_ROUTING: u32 = 0x4000000C;

pub fn enable_uart_irq() {
    unsafe {
        // enable uart rx irq
        volatile_store(UART0_IMSC as *mut u32, 1u32 << 4);

        let mut irq_val = volatile_load(IRQ_ENABLE2 as *mut u32);
        irq_val |= 1 << 25;
        volatile_store(IRQ_ENABLE2 as *mut u32, irq_val);

        volatile_store(GPU_INTERRUPTS_ROUTING as *mut u32, 0x00u32)
    }
}
