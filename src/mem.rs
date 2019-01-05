use core::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;

#[derive(Clone,Copy)]
struct FreeInfo {
    addr: u32,
    size: usize
}

pub struct KernelAllocator;

static mut FREES: usize = 0;
static mut FREE: [FreeInfo; 4090] = [FreeInfo{addr:0,size:0}; 4090];

use super::uart;

extern {
    fn kernel_heap_start() -> u32;
    fn kernel_heap_end() -> u32;
}

pub unsafe fn init() {

    FREES = 1;
    FREE[0] = FreeInfo{
        addr: kernel_heap_start() ,
        size: (kernel_heap_end() - kernel_heap_start()) as usize
    };
}

unsafe impl GlobalAlloc for KernelAllocator {
    // FIXME: ensure that return address is multiple of layout.align()
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        for i in 0..FREES {
            let s = layout.size();
            let a = layout.align();
            let size = a * ((s / a) + if s % a > 0 {1} else {0});

            if FREE[i].size >= size {
                let addr = FREE[i].addr as *mut u8;
                FREE[i].addr += size as u32;
                FREE[i].size -= size;
                if FREE[i].size == 0 {
                    FREES -= 1;
                    for j in i..FREES {
                        FREE[j] = FREE[j+1];
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
