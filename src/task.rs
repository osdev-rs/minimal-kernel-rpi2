
use super::alloc::alloc::Layout;

use super::uart;
use super::util;

use lazy_static::lazy_static;
use alloc::prelude::*;
use spin::Mutex;

pub struct Tcb {
    _stack_addr: *mut u8,
    _stack_size: usize,
    sp: *mut u32,
    r: [u32; 13],
    lr: *mut u8,
    pc: *mut u8,
    cpsr: u32,
}

unsafe impl Send for Tcb {}
unsafe impl Sync for Tcb {}

fn task_exit() {
    uart::write(&format!("task_exit\n"));
    loop {}
}

const STACK_SIZE: usize = 4096;

impl Tcb {
    pub fn new(entry: unsafe extern "C" fn()) -> Tcb {
        unsafe {
            let addr = alloc::alloc::alloc(Layout::new::<[u8; STACK_SIZE]>());
            let sp = (addr.offset(STACK_SIZE as isize) as u32 & !(7u32)) as *mut u32;
            Tcb {
                _stack_addr: addr,
                _stack_size: STACK_SIZE,
                sp: sp,
                r: [0xdeadbeef; 13],
                lr: task_exit as *mut u8,
                pc: entry as *mut u8,
                cpsr: 0x10, /* User mode */
            }
        }
    }
}

extern "C" fn entry1() {
    loop {
        uart::write("task::entry1()\n");
        util::delay(200_000_000);
    }
}

extern "C" fn entry2() {
    loop {
        uart::write("task::entry2()\n");
        util::delay(200_000_000);
    }
}

lazy_static! {
    static ref TCBS: Mutex<Vec<Tcb>> = {
        Mutex::new(Vec::new())
    };
}

static mut CURRENT_TASK: usize = 0;

#[no_mangle]
extern "C" fn demo_setup_switch(sp: *mut u32) {
    unsafe {
        let tcbs = (*TCBS).lock();
        {
            let mut sp = sp;
            for i in 0..13 {
                *sp = (*tcbs)[CURRENT_TASK].r[i];
                sp = sp.offset(1);
            }
            *sp = (*tcbs)[CURRENT_TASK].pc as u32;

            let cpsr = (*tcbs)[CURRENT_TASK].cpsr;
            asm!("msr spsr_cxsf, $0" :: "r"(cpsr));
        }

        asm!("cps #31 @ SYSTEM MODE
             mov lr, $0
             mov sp, $1
             cps #19  @ SVC MODE"
             ::"r"((*tcbs)[CURRENT_TASK].lr), "r"((*tcbs)[CURRENT_TASK].sp));
    }
}

pub fn demo_start() {
    unsafe {
        {
            let mut tcbs = (*TCBS).lock();
            (*tcbs).push(Tcb::new(entry1));
            (*tcbs).push(Tcb::new(entry2));

        }

        asm!(
            "stmfd sp!, {r0-r12, lr}
              mov r0, sp

              bl demo_setup_switch
              ldmfd sp!, {r0-r12, pc}^"
        );
    }
}

pub fn demo_context_switch(sp: *mut u32) {
    unsafe {
        let current = CURRENT_TASK;
        CURRENT_TASK += 1;
        CURRENT_TASK %= 2;
        let next = CURRENT_TASK;

        let mut tcbs = (*TCBS).lock();
        {
            let mut sp = sp;
            for i in 0..13 {
                (*tcbs)[current].r[i] = *sp;
                sp = sp.offset(1);
            }
            (*tcbs)[current].pc = *sp as *mut u8;

            let mut cpsr:u32;
            asm!("mrs $0, spsr" : "=r"(cpsr));
            (*tcbs)[current].cpsr = cpsr;
        }

        let mut lr_tmp: u32;
        let mut sp_tmp: u32;
        asm!("cps #31
              mov $0, lr
              mov $1, sp
              cps #18" : "=r"(lr_tmp), "=r"(sp_tmp));
        (*tcbs)[current].lr = lr_tmp as *mut u8;
        (*tcbs)[current].sp = sp_tmp as *mut u32;

        {
            let mut sp = sp;
            for i in 0..13 {
                *sp = (*tcbs)[next].r[i];
                sp = sp.offset(1);
            }
            *sp = (*tcbs)[next].pc as u32;

            let cpsr = (*tcbs)[next].cpsr;
            asm!("msr spsr_cxsf, $0" :: "r"(cpsr));
        }

        asm!("cps #31
              mov lr, $0
              mov sp, $1
              cps #18" ::"r"((*tcbs)[next].lr), "r"((*tcbs)[next].sp));

        uart::write("demo_context_switch end\n");
    }
}
