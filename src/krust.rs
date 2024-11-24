//! Main logic of Krust kernel

#![no_std]
#![no_main]

mod init;
mod utils;
use core::arch::asm;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    unsafe {
        asm!(
            "LDR r0, =0xdeadc0de"
        );
    }
    log_debug!("KRUST");

    unsafe {
        init::enable_system_handler_fault();
    }
    
    init::start_sys_tick();

    log_debug!("KRUST");

    loop {}
}


#[inline(never)]
unsafe fn trigger_nmi() {
    const NVIC_ICSR: *mut u32 = 0xE000ED04 as *mut u32; // Address of ICSR
    unsafe {
        core::ptr::write_volatile(NVIC_ICSR, 1 << 31); // Set NMIPENDSET bit
    }
}


#[inline(never)]
unsafe fn trigger_nocp() {
    unsafe {
        asm!(
            "MRC p15, 0, r0, c15, c0, 0", // Access a system control register via coprocessor
            options(nostack)
        );
    }
}

#[inline(never)]
unsafe fn trigger_lsperr() {
    let _ = 1.0f32 + 2.0f32; // Perform floating-point operation
    unsafe {
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("LDR R1, [R0]");        // Access invalid memory
    }
}
