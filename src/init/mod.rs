//! Initialization of Cortex-M device
//! 
//! Cortex-M devices are using a vector table for initialization.
//! It is located at the start of the code region in memory.
//! 
//! The first 2 elements of the vector table are :
//!   - the **initial Stack Pointer**
//!   - the **Reset vector** (pointer to the Reset handler)
//! 
//! This crates takes care of RAM initialization, and populates the vector table like this :
//! 
//! ```
//! Vector Table
//! +------------------+ 0x0
//! | Initial SP       |
//! +------------------+ 0x4
//! | (1) Reset        |
//! +------------------+ 0x8
//! | (2) NMI          |
//! +------------------+ 0xc
//! | (3) HardFault    |
//! +------------------+ 0x10
//! | (4) MemManage    |
//! +------------------+ 0x14
//! | (5) BusFault     |
//! +------------------+ 0x18
//! | (6) UsageFault   |
//! +------------------+ 0x1c
//! | (7-10) Reserved  |
//! +------------------+ 0x2c
//! | (11) SVCall      |
//! +------------------+ 0x30
//! | (12-13) Reserved |
//! +------------------+ 0x38
//! | (14) PendSV      |
//! +------------------+ 0x3c
//! | (15) SysTick     |
//! +------------------+ 0x40
//! | IRQ_n            |
//! | ...              |
//! ```


use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};

mod panic;
mod handlers;
mod systick;
pub use crate::init::systick::SysTick;
use crate::main;
pub use crate::init::handlers::{CURRENT_PROCESS_SP, NEXT_PROCESS_SP};

#[repr(C)]
#[allow(non_snake_case)]
pub struct ExceptionsHandlers {
    NMI: unsafe extern "C" fn() -> !,
    HardFault: unsafe extern "C" fn() -> !,
    MemManage: unsafe extern "C" fn() -> !,
    BusFault: unsafe extern "C" fn() -> !,
    UsageFault: unsafe extern "C" fn() -> !,
    Reserved_7: u32,
    Reserved_8: u32,
    Reserved_9: u32,
    Reserved_10: u32,
    SVCall: unsafe extern "C" fn(),
    Reserved_12: u32,
    Reserved_13: u32,
    PendSV: unsafe extern "C" fn(),
    SysTick: unsafe extern "C" fn()
}

/// Reset vector, part of the Vector table, points to our Reset handler : Reset()
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// Exceptions vectors, part of the Vector table
#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static _EXCEPTIONS: ExceptionsHandlers = ExceptionsHandlers {
    NMI: handlers::NMIHandler,
    HardFault: handlers::HardFaultHandler,
    MemManage: handlers::MemoryManagementFaultHandler,
    BusFault: handlers::BusFaultHandler,
    UsageFault: handlers::UsageFaultHandler,
    Reserved_7: 0,
    Reserved_8: 0,
    Reserved_9: 0,
    Reserved_10: 0,
    SVCall: handlers::SVCallHandler,
    Reserved_12: 0,
    Reserved_13: 0,
    PendSV: handlers::PendSV_Handler,
    SysTick: handlers::SysTickHandler
};

pub unsafe fn enable_system_handler_fault() {
    unsafe {
        const SHCSR_ADDR: u32 = 0xE000ED24; // Coprocessor Access Control Register
        let mut shcsr_value: u32 = core::ptr::read_volatile(SHCSR_ADDR as *const u32);

        shcsr_value |= 1 << 18; // Set the USGFAULTENA bit
        shcsr_value |= 1 << 17; // Set the BUSFAULTENA bit
        shcsr_value |= 1 << 16; // Set the MEMFAULTENA bit

        core::ptr::write_volatile(SHCSR_ADDR as *mut u32, shcsr_value);
    }
}

pub unsafe fn setup_priority_handler() {
    /*
        System Handler Priority Registers (SHPRx)

        SHPR1 :
            Memory Management Fault : PRI_4  - Bits 0-7
            Bus Fault               : PRI_5  - Bits 15-8
            Usage Fault             : PRI_6  - Bits 23-16
        SHPR2 :
            SVCall                  : PRI_11 - Bits 31-24
        SHPR3 :
            PendSV                  : PRI_14 - Bits 23-16
            SysTick                 : PRI_15 - Bits 31-24
     */

     /*
        PendSV : 0
        SysTick : 0
        SVCall : 2
      */

    #[allow(dead_code)]
    unsafe {
        const SHPR1_ADDR: u32 = 0xE000ED18;
        const SHPR2_ADDR: u32 = 0xE000ED1C;
        const SHPR3_ADDR: u32 = 0xE000ED20;


        let mut shpr3_value: u32 = core::ptr::read_volatile(SHPR3_ADDR as *const u32);

        // Modify SHPR3:
        // PendSV (PRI_14: Bits 23-16) -> Priority 0
        // SysTick (PRI_15: Bits 31-24) -> Priority 0
        shpr3_value &= !(0xFF << 16); // Clear bits for PendSV
        shpr3_value &= !(0xFF << 24); // Clear bits for SysTick
        shpr3_value |= 0x00 << 16;    // Set PendSV to 0
        shpr3_value |= 0x00 << 24;    // Set SysTick to 0
        core::ptr::write_volatile(SHPR3_ADDR as *mut u32, shpr3_value);

        // Modify SHPR2:
        // SVCall (PRI_11: Bits 31-24) -> Priority 2
        let mut shpr2_value: u32 = core::ptr::read_volatile(SHPR2_ADDR as *const u32);
        shpr2_value &= !(0xFF << 24); // Clear bits for SVCall
        shpr2_value |= 0x02 << 24;    // Set SVCall to 2
        core::ptr::write_volatile(SHPR2_ADDR as *mut u32, shpr2_value);
    }
}

/// Initialization of the .bss section by zeroing  out memory
unsafe fn init_bss(start_bss: *mut u8, count: usize) {
    unsafe {
        ptr::write_bytes(start_bss, 0, count);
    }
    compiler_fence(Ordering::SeqCst);
}

/// Initialization of the .data section by populating it with data from sidata
unsafe fn init_data(start_data: *mut u8, sidata: *const u8, count: usize){
    unsafe {
        ptr::copy_nonoverlapping(sidata,start_data, count);
    }
    compiler_fence(Ordering::SeqCst);
}

/// Our Reset handler, wich initializes RAM and calls main
#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    
    unsafe extern "C" {
        static _vector_table: u32;

        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    // Setting VTOR
    const VTOR: *mut u32 = 0xe000ed08 as *mut u32;
    unsafe {
        ptr::write_volatile(VTOR, ptr::addr_of!(_vector_table) as u32);
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