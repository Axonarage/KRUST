use core::arch::asm;

#[test_case]
#[inline(never)]
fn trigger_nocp() {
    unsafe {
        asm!(
            "MRC p15, 0, r0, c15, c0, 0", // Access a system control register via coprocessor
            options(nostack)
        );
    }
}

#[test_case]
#[inline(never)]
fn trigger_lsperr() {
    let _ = 1.0f32 + 2.0f32; // Perform floating-point operation
    unsafe {
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("LDR R1, [R0]");        // Access invalid memory
    }
}

#[test_case]
#[inline(never)]
fn trigger_nmi() {
    const NVIC_ICSR: *mut u32 = 0xE000ED04 as *mut u32; // Address of ICSR
    unsafe {
        core::ptr::write_volatile(NVIC_ICSR, 1 << 31); // Set NMIPENDSET bit
    }
}

#[test_case]
#[inline(never)]
fn trigger_hardfault() {
    // Trigger HardFaultHandler with Undefined instruction usage fault.
    unsafe {
        asm!("udf #0");
    }
}

#[test_case]
#[inline(never)]
fn trigger_invpc() {
    unsafe {
        // Trigger INVPC by manually loading an invalid value into the PC
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("BX R0"); // Branch to the invalid address
    }
}