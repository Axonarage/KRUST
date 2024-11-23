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
        enable_system_handler_fault();
    }
    
    init::start_sys_tick();
    
    /*
    // Trigger HardFaultHandler with Undefined instruction usage fault.
    unsafe {
        asm!("udf #0");
    }
    */
    /*
    unsafe {
        // Trigger INVPC by manually loading an invalid value into the PC
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("BX R0"); // Branch to the invalid address

        log_debug!("This line will not be reached due to INVPC fault.");
    }
    */
    // unsafe {
    //     // Attempt to access an unimplemented coprocessor
    //     trigger_lsperr();

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

unsafe fn enable_system_handler_fault() {
    unsafe {
        const SHCSR_ADDR: u32 = 0xE000ED24; // Coprocessor Access Control Register
        let mut shcsr_value: u32 = core::ptr::read_volatile(SHCSR_ADDR as *const u32);

        shcsr_value |= 1 << 18; // Set the USGFAULTENA bit
        shcsr_value |= 1 << 17; // Set the BUSFAULTENA bit
        shcsr_value |= 1 << 16; // Set the MEMFAULTENA bit

        core::ptr::write_volatile(SHCSR_ADDR as *mut u32, shcsr_value);
    }
}
