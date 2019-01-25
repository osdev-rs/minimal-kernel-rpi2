
use core::mem::align_of;
use super::alloc::alloc::Layout;

use super::uart;

pub struct Tcb {
    stack_addr: *mut u8,
    stack_size: usize,
    pub sp: *mut u8,
}

fn task_exit() {
    uart::write(&format!("task_exit\n"));
    loop {}
}

const STACK_SIZE: usize =  4096;

impl Tcb {
    pub fn new(entry: unsafe extern "C" fn()) -> Tcb {
        unsafe {
            let addr = alloc::alloc::alloc(Layout::new::<[u8; STACK_SIZE]>());
            let sp = stack_init(entry, addr.offset(STACK_SIZE as isize));
            Tcb{stack_addr: addr, stack_size: STACK_SIZE, sp: sp}
        }
    }
}

#[rustfmt::skip]
unsafe fn stack_init(entry: unsafe extern "C" fn(),
                     stack_addr: *mut u8) -> *mut u8 {
//    sa = sa.offset(-1isize * align_of::<u64>() as isize);
    uart::write(&format!("stack_addr: {}\n", stack_addr as u32));

    let mut sa = (stack_addr as u32 & !(7u32)) as *mut u32;

//    uart::write(&format!("sa: {}\n", sa as u32));

    sa = sa.offset(-1); *sa = entry as u32; // entry point
    sa = sa.offset(-1); *sa = task_exit as u32; // lr
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r12
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r11
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r10
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r9
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r8
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r7
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r6
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r5
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r4
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r3
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r2
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r1
    sa = sa.offset(-1); *sa = 0xdeadbeef;   // r0 TODO: entry function argument

    // cpsr
    sa = sa.offset(-1); *sa = 0x13; // SVCMODE

//    uart::write(&format!("sa: {}\n", sa as *mut u8 as u32));
    uart::write(&format!("*sa: {}\n", *sa));

    sa as *mut u8
}
