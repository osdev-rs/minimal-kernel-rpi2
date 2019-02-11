
#[allow(dead_code)]
#[cfg_attr(rstfmt, rustfmt_skip)]
mod constant {
const DESC_SEC: u32 = 0b10;
const CB: u32       = 0b11 << 2;
const CNB: u32      = 0b10 << 2;
const NCNB: u32     = 0b00 << 2;

const AP_RW: u32    = 0b11 << 10; // privileged=RW, user=RW
const AP_RO: u32    = 0b10 << 10; // privileged=RW, user=RO

const XN: u32       = 0b1 << 4;   // execute never

const DOMAIN0: u32  = 0 << 5;
const DOMAIN1: u32  = 1 << 5;

pub const RW_CB: u32     = AP_RW | DOMAIN0 | CB   | DESC_SEC;
pub const RW_NCNBXN: u32 = AP_RW | DOMAIN0 | NCNB | DESC_SEC;

}

use self::constant::*;

#[inline]
unsafe fn disable_mmu() {
    //    asm!("mcr p15, 0, $0, c8, c7, 0" :: "r"(0) :: "volatile"); // invalidate tlb

    let mut sctlr: u32 = 0;
    asm!("mrc p15, 0, $0, c1, c0, 0" : "=r"(sctlr) ::: "volatile");
    sctlr &= !1u32;
    asm!("mcr p15, 0, $0, c1, c0, 0" :: "r"(sctlr) :: "volatile");
    asm!("dsb" :::: "volatile");
}

#[inline]
unsafe fn enable_mmu() {
    let mut sctlr: u32 = 0;
    asm!("mrc p15, 0, $0, c1, c0, 0" : "=r"(sctlr) ::: "volatile");
    sctlr |= 1u32;
    asm!("mcr p15, 0, $0, c1, c0, 0" :: "r"(sctlr) :: "volatile");
    asm!("dsb" :::: "volatile");
}

#[repr(align(0x4000))]
struct MmuTable([u32; 4 * 1024]);
static mut MMU_TABLE: MmuTable = MmuTable([0; 4 * 1024]);

unsafe fn set_mtt(vaddr_start: u32, vaddr_end: u32, paddr_start: u32, attr: u32) {
    let num_sec = ((vaddr_end >> 20) - (vaddr_start >> 20)) as usize;
    let offset = (vaddr_start >> 20) as usize;
    for i in 0..num_sec {
        MMU_TABLE.0[offset + i] = attr | (((paddr_start >> 20) + i as u32) << 20);
    }
}

unsafe fn set_domain_register(attr: u32) {
    asm!("mcr p15, 0, $0, c3, c0" :: "r"(attr) :: "volatile");
}

unsafe fn set_tlb(tlb_addr: u32) {
    asm!("mcr p15, 0, $0, c2, c0, 0" :: "r"(tlb_addr) :: "volatile");
}

pub fn init() {
    unsafe {
        disable_mmu();

        set_mtt(0, 0xFFFF_FFFF - 1, 0, RW_CB);

        set_mtt(0x4400_0000, 0x8000_0000 - 1, 0x4400_0000, RW_NCNBXN);

        set_domain_register(0x5555_5555);

        set_tlb(&MMU_TABLE.0[0] as *const _ as u32);

        enable_mmu();
    }
}
