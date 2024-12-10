use core::mem::{align_of, size_of};
use core::ptr;
use crate::check_cookie;

const HEAP_SIZE: usize = 0x10000; // Taille totale de la heap (RAM/2)
const ALIGNMENT: usize = align_of::<usize>();
const ALIGNMENT_MASK: usize = !(ALIGNMENT - 1);

#[unsafe(link_section = ".ram_heap")]
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

static mut HEAP_INIT: bool = false;

#[derive(Debug)]
struct BlockLink {
    next_free: *mut BlockLink,
    block_size: usize,
    cookie: usize,
}

const BLOCK_HEADER_SIZE: usize = size_of::<BlockLink>();
const MINIMUM_BLOCK_SIZE: usize = BLOCK_HEADER_SIZE * 2;

// Structures globales pour suivre l'état du heap
static mut START: BlockLink = BlockLink {
    next_free: ptr::null_mut(),
    block_size: 0,
    cookie: 0,
};

static mut END: BlockLink = BlockLink {
    next_free: ptr::null_mut(),
    block_size: 0,
    cookie: 0,
};

static mut FREE_BYTES_REMAINING: usize = HEAP_SIZE;

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn initialize_heap() -> () {
    if HEAP_INIT {
        return;
    }

    let aligned_heap_start = (&raw const HEAP as *const _ as usize + (ALIGNMENT - 1)) & ALIGNMENT_MASK;
    let aligned_heap_end = &raw const HEAP as *const _ as usize + HEAP_SIZE;
    let heap_size = aligned_heap_end - aligned_heap_start;

    let first_free = aligned_heap_start as *mut BlockLink;
    
    (*first_free).next_free = &raw const END as *mut _;
    (*first_free).block_size = heap_size - BLOCK_HEADER_SIZE;

    START.next_free = first_free;
    START.block_size = 0;

    END.next_free = ptr::null_mut();
    END.block_size = 0;

    FREE_BYTES_REMAINING = heap_size - BLOCK_HEADER_SIZE;
    HEAP_INIT = true;
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn allocate(mut wanted_size: usize) -> *mut u8 {
    if wanted_size == 0 {
        return ptr::null_mut();
    }

    // Add the header size and space for the end cookie
    wanted_size += BLOCK_HEADER_SIZE + size_of::<usize>();

    // Align the requested size
    if wanted_size & ALIGNMENT_MASK != 0 {
        wanted_size = (wanted_size + ALIGNMENT) & ALIGNMENT_MASK;
    }

    let mut previous_block = &raw const START as *mut _;
    let mut current_block = START.next_free;

    while !current_block.is_null() && (*current_block).block_size < wanted_size {
        previous_block = current_block;
        current_block = (*current_block).next_free;
    }

    if current_block.is_null() || (*current_block).block_size < wanted_size {
        return ptr::null_mut(); // Allocation failed
    }

    let allocated_block = current_block;

    if (*allocated_block).block_size - wanted_size >= MINIMUM_BLOCK_SIZE {
        // Split the block if possible
        let new_block = (allocated_block as usize + wanted_size) as *mut BlockLink;

        (*new_block).block_size = (*allocated_block).block_size - wanted_size;
        (*new_block).next_free = (*allocated_block).next_free;

        (*allocated_block).block_size = wanted_size;
        (*previous_block).next_free = new_block;
    } else {
        // Otherwise, use the entire block
        (*previous_block).next_free = (*allocated_block).next_free;
    }

    FREE_BYTES_REMAINING -= (*allocated_block).block_size;

    // Generate a unique COOKIE for this block
    let cookie = generate_random();
    (*allocated_block).cookie = cookie; // Store the first cookie in the structure

    let user_memory = (allocated_block as *mut u8).add(BLOCK_HEADER_SIZE);
    *(user_memory.add((*allocated_block).block_size - size_of::<usize>()) as *mut usize) = cookie;

    // Return a pointer to the start of the user memory (after the header)
    user_memory
}

/// Checks the integrity of a memory block by verifying its cookies.
/// 
/// Each allocated block has a unique `COOKIE` stored at the start (within the `BlockLink` structure)
/// and at the end of the user memory. This function ensures that both cookies match, indicating
/// that the block has not been corrupted.
pub fn check_cookie(ptr: *mut u8) -> bool {
    unsafe {
        let block_link = (ptr as usize - BLOCK_HEADER_SIZE) as *mut BlockLink;
        let block_size = (*block_link).block_size; // Retrieve block size from BlockLink
        let start_cookie = (*block_link).cookie;
        let end_cookie = *(ptr.add(block_size - size_of::<usize>()) as *mut usize);
        start_cookie == end_cookie
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn deallocate(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    // Move back to find the header
    let block_to_free = (ptr as usize - BLOCK_HEADER_SIZE) as *mut BlockLink;

    // Check cookies using the new function
    check_cookie!(ptr);

    // Add the block to the list of free blocks
    let mut previous_block = &raw const START as *mut _;
    let mut current_block = START.next_free;

    while !current_block.is_null() && current_block < block_to_free {
        previous_block = current_block;
        current_block = (*current_block).next_free;
    }

    (*block_to_free).next_free = current_block;
    (*previous_block).next_free = block_to_free;

    FREE_BYTES_REMAINING += (*block_to_free).block_size;
}

pub unsafe fn zeroes_region(block: *mut u8) {
    unsafe {
        let user_memory_size = (*(block as *mut BlockLink)).block_size - BLOCK_HEADER_SIZE;
        ptr::write_bytes(block, 0, user_memory_size);
    }
}

#[allow(dead_code)]
pub fn get_free_heap_size() -> usize {
    unsafe {
        FREE_BYTES_REMAINING
    }
}

#[allow(dead_code)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn reset_heap() {
    HEAP_INIT = false;
    initialize_heap();
}

pub fn generate_random() -> usize {
    return 0xdeadbeef;
}