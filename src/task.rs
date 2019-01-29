
use core::mem::align_of;
use super::alloc::alloc::Layout;

use super::uart;
use super::util;

use lazy_static::lazy_static;
use alloc::prelude::*;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use spin::Mutex;
use core::borrow::BorrowMut;

extern "C" {
    fn context_switch_to(sp: *mut *mut u8);
}

pub struct Tcb {
    stack_addr: *mut u8,
    stack_size: usize,
    pub sp: *mut u8,
    pub r: [u32; 13],
    pub lr: u32,
    pub cpsr: u32,
    pub pc: u32,
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
            let sp = (addr.offset(STACK_SIZE as isize) as u32 & !(7u32)) as *mut u8;
            Tcb{stack_addr: addr, stack_size: STACK_SIZE, sp: sp, r: [0xdeadbeef; 13], lr: task_exit as u32, cpsr: 0x10 , pc: entry as u32}
        }
    }
}

unsafe impl Send for Tcb{}

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
    static ref TCBS: Arc<Mutex<UnsafeCell<Vec<Tcb>>>> = {
        Arc::new(Mutex::new(UnsafeCell::new(Vec::new())))
    };
}

static mut current_task:usize = 0;

#[no_mangle]
extern "C" fn demo_setup_switch(sp: *mut u32) {
    unsafe {
        let tcbs = (*TCBS).lock().get();
        {
            let mut sp = sp;
            for i in 0..13 {
                *sp = (*tcbs)[current_task].r[i];
                sp = sp.offset(1);
            }
            *sp = (*tcbs)[current_task].pc;

            let cpsr = (*tcbs)[current_task].cpsr;
            asm!("msr spsr_cxsf, $0" :: "r"(cpsr));
        }

        asm!("cps #31 @ SYSTEM MODE
             mov lr, $0
             mov sp, $1
             cps #19  @ SVC MODE"
             ::"r"((*tcbs)[current_task].lr), "r"((*tcbs)[current_task].sp));
    }
}

pub fn demo_start() {
    unsafe {
        {
            let tcbs = (*TCBS).lock().get();
            (*tcbs).push(Tcb::new(entry1));
            (*tcbs).push(Tcb::new(entry2));

        }

        asm!("stmfd sp!, {r0-r12, lr}
              mov r0, sp

              bl demo_setup_switch
              ldmfd sp!, {r0-r12, pc}^");
    }
}

pub fn demo_context_switch(sp: *mut u32) {
    unsafe {
        let current = current_task;
        current_task += 1;
        current_task %= 2;
        let next = current_task;

        let tcbs = (*TCBS).lock().get();
        {
            let mut sp = sp;
            for i in 0..13 {
                (*tcbs)[current].r[i] = *sp;
                sp = sp.offset(1);
            }
            (*tcbs)[current].pc = *sp;

            let mut cpsr = 0u32;
            asm!("mrs $0, spsr" : "=r"(cpsr));
            (*tcbs)[current].cpsr = cpsr;
        }

        let mut lr_tmp = 0u32;
        let mut sp_tmp = 0u32;
        asm!("cps #31
                  mov $0, lr
                  mov $1, sp
                  cps #18" : "=r"(lr_tmp), "=r"(sp_tmp));
        (*tcbs)[current].lr = lr_tmp;
        (*tcbs)[current].sp = sp_tmp as *mut u8;

        {
            let mut sp = sp;
            for i in 0..13 {
                *sp = (*tcbs)[next].r[i];
                sp = sp.offset(1);
            }
            *sp = (*tcbs)[next].pc;

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
