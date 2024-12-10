use core::alloc::{GlobalAlloc, Layout};
use super::heap;

pub struct SimpleAllocator;

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            heap::allocate(layout.size() + layout.align())
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            heap::deallocate(ptr);
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let ptr = heap::allocate(layout.size() + layout.align());
            if !ptr.is_null() {
                heap::zeroes_region(ptr);
            }
            ptr
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            let new_ptr = heap::allocate(new_size + layout.align());
            if !new_ptr.is_null() && !ptr.is_null() {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, layout.size());
                heap::deallocate(ptr);
            }
            new_ptr
        }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: SimpleAllocator = SimpleAllocator;
