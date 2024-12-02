#![no_std]

use core::mem::{align_of, size_of};
use core::ptr;

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
}

const BLOCK_HEADER_SIZE: usize = size_of::<BlockLink>();
const MINIMUM_BLOCK_SIZE: usize = BLOCK_HEADER_SIZE * 2;

// Structures globales pour suivre l'état du heap
static mut START: BlockLink = BlockLink {
    next_free: ptr::null_mut(),
    block_size: 0,
};

static mut END: BlockLink = BlockLink {
    next_free: ptr::null_mut(),
    block_size: 0,
};

static mut FREE_BYTES_REMAINING: usize = HEAP_SIZE;

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

pub unsafe fn pv_port_malloc(mut wanted_size: usize) -> *mut u8 {
    if wanted_size == 0 {
        return ptr::null_mut();
    }

    // Ajouter la taille du header
    wanted_size += BLOCK_HEADER_SIZE;

    // Aligner la taille demandée
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
        return ptr::null_mut(); // Allocation échouée
    }

    let allocated_block = current_block;

    if (*allocated_block).block_size - wanted_size >= MINIMUM_BLOCK_SIZE {
        // Découper le bloc si possible
        let new_block = (allocated_block as usize + wanted_size) as *mut BlockLink;

        (*new_block).block_size = (*allocated_block).block_size - wanted_size;
        (*new_block).next_free = (*allocated_block).next_free;

        (*allocated_block).block_size = wanted_size;
        (*previous_block).next_free = new_block;
    } else {
        // Sinon, utiliser tout le bloc
        (*previous_block).next_free = (*allocated_block).next_free;
    }

    FREE_BYTES_REMAINING -= (*allocated_block).block_size;

    // Retourner un pointeur au début de la mémoire utilisateur (après le header)
    (allocated_block as *mut u8).add(BLOCK_HEADER_SIZE)
}

pub unsafe fn v_port_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    // Reculer pour trouver le header
    let block_to_free = (ptr as usize - BLOCK_HEADER_SIZE) as *mut BlockLink;

    // memset
    let user_memory_start = ptr;
    let user_memory_size = (*block_to_free).block_size - BLOCK_HEADER_SIZE;
    ptr::write_bytes(user_memory_start, 0, user_memory_size);


    // Ajouter le bloc dans la liste des blocs libres
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

pub unsafe fn get_free_heap_size() -> usize {
    FREE_BYTES_REMAINING
}

pub unsafe fn reset_heap() {
    HEAP_INIT = false;
    initialize_heap();
}
