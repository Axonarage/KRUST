//! Main logic of Krust kernel

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_runner"]

mod init;
mod utils;
mod test;
mod proc;
mod memory_management;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    log_debug!("KRUST");
    
    unsafe {
        init::enable_system_handler_fault();
        memory_management::heap::initialize_heap();
    }
    
    #[cfg(test)]
    test_runner();

    init::start_sys_tick();   

    loop {}
}
