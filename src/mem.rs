use core::alloc::{GlobalAlloc, Layout};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Clone,Copy)]
struct FreeInfo {
    addr: u32,
    size: u32
}

pub struct KernelAllocator;

static frees: Mutex<u32> = Mutex::new(1);

extern {
    static __kernel_heap_start__: *mut u8;
    static __kernel_heap_end__: *mut u8;
}

lazy_static! {
    static ref FREE: Mutex<[FreeInfo; 4090]> = {
        let mut fs = [FreeInfo{addr:0, size:0}; 4090];
        fs[0] = FreeInfo{addr: unsafe {__kernel_heap_start__ as u32},
                         size: unsafe {__kernel_heap_end__.wrapping_offset_from(
                             __kernel_heap_start__) as u32}};
        Mutex::new(fs)
    };
}

use super::uart;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut fs = frees.lock();
        for i in 0..*fs as usize {
            let size = layout.align();
            let mut free = FREE.lock();
            if free[i].size >= size as u32 {
                let ret = free[i].addr as *mut u8;
                free[i].addr += size as u32;
                free[i].size -= size as u32;
                if free[i].size == 0 {
                    *fs -= 1;
                    for j in i..*fs as usize {
                        free[j] = free[j+1];
                    }
                }
                return ret as *mut u8
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
