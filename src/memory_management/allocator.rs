use core::alloc::{GlobalAlloc, Layout};
use super::heap;

pub struct SimpleAllocator;

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        heap::allocate(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        heap::deallocate(ptr);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: SimpleAllocator = SimpleAllocator;