use core::alloc::{GlobalAlloc, Layout};
use spin::Mutex;
use core::mem::size_of;

#[derive(Clone,Copy)]
struct FreeInfo {
    addr: u32,
    size: usize
}

pub struct KernelAllocator;

extern {
    static __kernel_heap_start__: *mut u8;
    static __kernel_heap_end__: *mut u8;
}

static mut FREES: Mutex<usize> = Mutex::new(0usize);
static mut FREE: Mutex<[FreeInfo; 4090]> = Mutex::new([FreeInfo{addr:0,size:0}; 4090]);

use super::uart;

pub unsafe fn init() {
    let mut fs = FREES.lock();
    let mut free = FREE.lock();

    *fs = 1;
    free[0] = FreeInfo{addr: 0x2000000,
//addr: unsafe {__kernel_heap_start__ as u32},
//                       size: unsafe {__kernel_heap_end__.wrapping_offset_from(
//                           __kernel_heap_start__) as usize } * size_of::<u32>()};
                       size: 0x100000};

//    if free[0].size <= 0 {
//        uart::write("free[0].size <= 0\n");
//    }

//    uart::write("mem::init()\n");

}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut fs = FREES.lock();
        let mut free = FREE.lock();

        for i in 0..*fs {
            let size = layout.align();

            if free[i].size >= size {
                let addr = free[i].addr as *mut u8;
                free[i].addr += size as u32;
                free[i].size -= size;
                if free[i].size == 0 {
                    *fs -= 1;
                    for j in i..*fs {
                        free[j] = free[j+1];
                    }
                }
                return addr
            }
        }

        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}

#[alloc_error_handler]
fn foo(_: Layout) -> ! {
    uart::write("alloc_error");
    loop {}
}
