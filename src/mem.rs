use core::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;

#[derive(Clone,Copy)]
struct FreeInfo {
    addr: u32,
    size: usize
}

pub struct KernelAllocator;

const MAX_FREES:usize = 4090;

static mut FREES: usize = 0;
static mut FREE: [FreeInfo; MAX_FREES] = [FreeInfo{addr:0,size:0}; MAX_FREES];

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

fn aligned_size(layout: &Layout) -> usize {
    let s = layout.size();
    let a = layout.align();
    return a * ((s / a) + if s % a > 0 {1} else {0});
}

unsafe impl GlobalAlloc for KernelAllocator {
    // FIXME: ensure that return address is multiple of layout.align()
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        for i in 0..FREES {
            let size = aligned_size(&layout);

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
        let addr = ptr as u32;
        let size = aligned_size(&layout);

        let mut i = 0usize;
        for j in 0..FREES {
            if FREE[i].addr > addr {
                i = j;
                break;
            }
        }

        if i > 0 {
            if FREE[i-1].addr + FREE[i-1].size as u32 == addr {
                FREE[i-1].size += size;
                if i < FREES {
                    if addr + size as u32 == FREE[i].addr {
                        FREE[i-1].size += FREE[i].size;
                        FREES -= 1;
                        for j in i..FREES {
                            FREE[j] = FREE[j+1];
                        }
                    }
                }
                return;
            }
        }

        if i < FREES {
            if addr + size as u32 == FREE[i].addr {
                FREE[i].addr = addr;
                FREE[i].size += size;
                return;
            }
        }

        if FREES < MAX_FREES {
            for j in (i+1..=FREES).rev() {
                FREE[j] = FREE[j-1];
            }

            FREES -= 1;

            FREE[i].addr = addr;
            FREE[i].size = size;
            return;
        }

        // FIXME: deallocation failed. abort?
        uart::write("dealloc failed\n");
    }
}

#[alloc_error_handler]
fn foo(_: Layout) -> ! {
    uart::write("alloc_error");
    loop {}
}
