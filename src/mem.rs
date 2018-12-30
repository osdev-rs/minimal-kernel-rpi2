use core::alloc::{GlobalAlloc, Layout};

pub struct KernelAllocator;

use super::uart;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
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
