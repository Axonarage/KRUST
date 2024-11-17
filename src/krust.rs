//! Main logic of Krust kernel

#![no_std]
#![no_main]

mod init;
use core::arch::asm;
use cortex_m_semihosting::hprintln;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    unsafe {
        asm!(
            "LDR r0, =0xdeadc0de"
        );
    }
    hprintln!("Hello, world!").unwrap();

    unsafe {
        enable_system_handler_fault();
    }
    
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

        hprintln!("This line will not be reached due to INVPC fault.").unwrap();
    }
    */
    unsafe {
        // Attempt to access an unimplemented coprocessor
        trigger_nocp();

        hprintln!("This line will not be reached.").unwrap();
    }

    
    loop {}
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

unsafe fn enable_system_handler_fault() {
    unsafe {
        const CPACR_ADDR: u32 = 0xE000ED24; // Coprocessor Access Control Register
        let mut cpacr_value: u32 = core::ptr::read_volatile(CPACR_ADDR as *const u32);

        cpacr_value |= 1 << 18; // Set the USGFAULTENA bit
        cpacr_value |= 1 << 17; // Set the BUSFAULTENA bit
        cpacr_value |= 1 << 16; // Set the MEMFAULTENA bit

        core::ptr::write_volatile(CPACR_ADDR as *mut u32, cpacr_value);
    }
}