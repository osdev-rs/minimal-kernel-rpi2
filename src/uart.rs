
use super::util::{mmio_write, mmio_read, enable_irq_no};

#[allow(dead_code)]
mod constval {
// The GPIO registers base address.
pub const GPIO_BASE: u32 = 0x3F200000; // for raspi2 & 3, 0x20200000 for raspi1

// The offsets for reach register.

// Controls actuation of pull up/down to ALL GPIO pins.
pub const GPPUD: u32 = (GPIO_BASE + 0x94);

// Controls actuation of pull up/down for specific GPIO pin.
pub const GPPUDCLK0: u32 = (GPIO_BASE + 0x98);

// The base address for UART.
pub const UART0_BASE: u32 = 0x3F201000; // for raspi2 & 3, 0x20201000 for raspi1

// The offsets for reach register for the UART.
pub const UART0_DR: u32 = (UART0_BASE + 0x00);
pub const UART0_RSRECR: u32 = (UART0_BASE + 0x04);
pub const UART0_FR: u32 = (UART0_BASE + 0x18);
pub const UART0_ILPR: u32 = (UART0_BASE + 0x20);
pub const UART0_IBRD: u32 = (UART0_BASE + 0x24);
pub const UART0_FBRD: u32 = (UART0_BASE + 0x28);
pub const UART0_LCRH: u32 = (UART0_BASE + 0x2C);
pub const UART0_CR: u32 = (UART0_BASE + 0x30);
pub const UART0_IFLS: u32 = (UART0_BASE + 0x34);
pub const UART0_IMSC: u32 = (UART0_BASE + 0x38);
pub const UART0_RIS: u32 = (UART0_BASE + 0x3C);
pub const UART0_MIS: u32 = (UART0_BASE + 0x40);
pub const UART0_ICR: u32 = (UART0_BASE + 0x44);
pub const UART0_DMACR: u32 = (UART0_BASE + 0x48);
pub const UART0_ITCR: u32 = (UART0_BASE + 0x80);
pub const UART0_ITIP: u32 = (UART0_BASE + 0x84);
pub const UART0_ITOP: u32 = (UART0_BASE + 0x88);
pub const UART0_TDR: u32 = (UART0_BASE + 0x8C);

}

use self::constval::*;
use super::util::delay;

fn transmit_fifo_full() -> bool {
    unsafe {mmio_read(UART0_FR) & (1 << 5) > 0}
}

fn receive_fifo_empty() -> bool {
    unsafe {mmio_read(UART0_FR) & (1 << 4) > 0}
}

pub fn writec(c: u8) {
    while transmit_fifo_full() {}
    unsafe {mmio_write(UART0_DR, c as u32);}
}

pub fn getc() -> u8 {
    while receive_fifo_empty() {}
    unsafe {mmio_read(UART0_DR) as u8}
}

pub fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

pub fn init() {
    unsafe {
        // Disable UART0.
        mmio_write(UART0_CR, 0x00000000);
        // Setup the GPIO pin 14 && 15.

        // Disable pull up/down for all GPIO pins & delay for 150 cycles.
        mmio_write(GPPUD, 0x00000000);
        delay(150);

        // Disable pull up/down for pin 14,15 & delay for 150 cycles.
        mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
        delay(150);

        // Write 0 to GPPUDCLK0 to make it take effect.
        mmio_write(GPPUDCLK0, 0x00000000);

        // Clear pending interrupts.
        mmio_write(UART0_ICR, 0x7FF);

        // Set integer & fractional part of baud rate.
        // Divider = UART_CLOCK/(16 * Baud)
        // Fraction part register = (Fractional part * 64) + 0.5
        // UART_CLOCK = 3000000; Baud = 115200.

        // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
        mmio_write(UART0_IBRD, 1);
        // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
        mmio_write(UART0_FBRD, 40);

        // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
        mmio_write(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

        // Mask all interrupts.
//        mmio_write(
//            UART0_IMSC,
//            (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10),
//        );

        // Enable UART0, receive & transfer part of UART.
        mmio_write(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));

        // unmask RX IRQ
        mmio_write(UART0_IMSC, 1 << 4);

        // enable uart irq
        enable_irq_no(57);
    }
}
