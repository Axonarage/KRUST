//! Main logic of Krust kernel

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_runner"]

mod init;
mod utils;
mod test;
mod memory_management;

use memory_management::heap;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    log_debug!("KRUST");

    unsafe {
        enable_system_handler_fault();
    }
    
    #[cfg(test)]
    test_runner();

    init::start_sys_tick();   

    loop {}
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