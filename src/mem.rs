use core::alloc::{GlobalAlloc, Layout};
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

static mut FREES: usize = 0;
static mut FREE: [FreeInfo; 4090] = [FreeInfo{addr:0,size:0}; 4090];

use super::uart;

pub unsafe fn init() {

    FREES = 1;
    FREE[0] = FreeInfo{addr: 0x2000000,
                       size: 0x100000};

//    FREE[0] = FreeInfo{
//        addr: unsafe {__kernel_heap_start__ as u32},
//        size: unsafe {__kernel_heap_end__.wrapping_offset_from(
//            __kernel_heap_start__) as usize } * size_of::<u32>()};

}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        for i in 0..FREES {
            let size = layout.align();

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
