use core::arch::asm;
use crate::memory_management::heap;
use crate::log_debug;
extern crate alloc;
use crate::check_cookie;

/// Test case for heap allocation
#[test_case]
#[inline(never)]
fn alloc_heap(){
    unsafe {
        // Allocate a block of 32 bytes
        let ptr1 = heap::allocate(32);
        if ptr1.is_null() {
            // Allocation failed
            log_debug!("Allocation of 32 bytes failed");
        } else {
            log_debug!("Allocation successful: address {:?}", ptr1);
        }
        *ptr1 = 0x13;

        // Allocate another block of 64 bytes
        let ptr2 = heap::allocate(64);
        if ptr2.is_null() {
            log_debug!("Allocation of 64 bytes failed");
        } else {
            log_debug!("Allocation successful: address {:?}", ptr2);
        }
        *ptr2 = 0x37;
    }
}

/// Test case for heap deallocation
#[test_case]
#[inline(never)]
fn free_heap(){
    unsafe {
        // Allocate a block of 32 bytes
        let ptr1 = heap::allocate(32);
        if ptr1.is_null() {
            // Allocation failed
            log_debug!("Allocation of 32 bytes failed");
        } else {
            log_debug!("Allocation successful: address {:?}", ptr1);
        }
        *ptr1 = 0x13;

        // Free the first allocation
        heap::deallocate(ptr1);
        log_debug!("Memory freed for the first block");

        // Reallocate to verify memory recycling
        let ptr2 = heap::allocate(16);
        if ptr2.is_null() {
            log_debug!("Reallocation of 16 bytes failed");
        } else {
            log_debug!("Reallocation successful: address {:?}", ptr2);
        }
    }
}

/// Test case for verifying heap cookie
#[test_case]
#[inline(never)]
fn cookie_macro(){
    unsafe{
        log_debug!("Testing cookie (nominal case).");
        // Allocate 32 bytes
        let ptr = heap::allocate(32);
        if ptr.is_null() {
            log_debug!("Allocation failed");
            return;
        }

        // Get the block header to access cookie
        let block_link = (ptr as usize - size_of::<usize>()) as *mut usize;
        let initial_cookie = unsafe { *block_link };
        log_debug!("Initial cookie: {:#x}", initial_cookie);

        // Write some data
        *ptr = 0x42;

        check_cookie!(ptr);
    }
}


/// Test case for verifying heap cookie
#[test_case]
#[inline(never)]
fn cookie_overflow(){
    unsafe{
        log_debug!("Testing cookie overflow.");
        // Allocate 32 bytes
        let ptr = heap::allocate(32);
        if ptr.is_null() {
            log_debug!("Allocation failed");
            return;
        }

        // Get the block header to access cookie
        let block_size = *((ptr as usize - (size_of::<usize>()*2) ) as *mut usize);
        log_debug!("Block size is: {:#x}", block_size);

        let initial_cookie_ptr = (ptr as usize - size_of::<usize>()) as *mut usize;
        log_debug!("Initial cookie: {:#x}", *initial_cookie_ptr);

        // Write some data beyond the allocated size to corrupt the cookie
        let final_cookie_ptr = ptr.add(block_size).sub(size_of::<usize>()-3); // Point to the cookie location
        *final_cookie_ptr = 0x42; // This should corrupt the cookie

        // Get cookie value again
        log_debug!("Final cookie: {:#x}", *final_cookie_ptr);
        
        // Check if cookie is still valid (should be false due to corruption)
        let is_valid = heap::check_cookie(ptr);
        log_debug!("Cookie check result after corruption: {}", is_valid);
        assert!(!is_valid, "Cookie should be invalid after corruption");
    }
}