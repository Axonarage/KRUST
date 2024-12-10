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

use crate::proc::SystemProcess;
use crate::utils::LinkedList;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    log_debug!("KRUST");
    
    unsafe {
        init::enable_system_handler_fault();
        memory_management::heap::initialize_heap();
    }
    
    #[cfg(test)]
    test_runner();

    // Initialize the system process manager
    let mut system_process = SystemProcess {
        last_proc_id: 0,
        process_list: LinkedList::new(),
    };

    // Create some processes
    system_process.create_process("Process 1", process_1_entry as usize);
    system_process.create_process("Process 2", process_2_entry as usize);

    log_debug!("Processes created");

    // Main scheduler loop
    loop {
        system_process.schedule_next_process();
    } 
}

// Dummy entry points for processes
extern "C" fn process_1_entry() {
    loop {
        log_debug!("Process 1");
    }
}

extern "C" fn process_2_entry() {
    loop {
        log_debug!("Process 2");
    }
}
