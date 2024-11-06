//! Initialization of Cortex-M device
//! 
//! Cortex-M devices are using a vector table for initialization.
//! It is located at the start of the code region in memory.
//! 
//! The first 2 elements of the vector table are :
//!   - the **initial Stack Pointer**
//!   - the **Reset vector** (pointer to the Reset handler)
//! 
//! This crates takes care of RAM initialization, and will populates the vector table.

use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};

mod panic;
use crate::main;

/// Here is the Reset vector, a pointer to our Reset handler : Reset()
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


/// Initialization of the .bss section by zeroing  out memory
pub unsafe fn init_bss(start_bss: *mut u8, count: usize) {
    unsafe {
        ptr::write_bytes(start_bss, 0, count);
    }
    compiler_fence(Ordering::SeqCst);
}

/// Initialization of the .data section by populating it with data from sidata
pub unsafe fn init_data(start_data: *mut u8, sidata: *const u8, count: usize){
    unsafe {
        ptr::copy_nonoverlapping(sidata,start_data, count);
    }
    compiler_fence(Ordering::SeqCst);
}


/// Our Reset handler, wich initializes RAM and calls main
#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    
    unsafe extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }
    
    // RAM initialization
    let mut start: *mut u8 = &raw mut _sbss;
    let mut end: *mut u8 = &raw mut _ebss;
    unsafe{
        init_bss(start,end.offset_from(start) as usize);
    }

    start = &raw mut _sdata;
    end = &raw mut _edata;
    unsafe {
        init_data(start,&_sidata,end.offset_from(start) as usize);
    }

    // main() trampoline
    #[inline(never)]
    fn trampoline() -> ! {
        main();
    }
    trampoline();
}