//! Main logic of Krust kernel

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_runner"]

extern crate alloc;

mod init;
mod utils;
mod test;
mod proc;
mod memory_management;

use crate::proc::SystemProcess;
use core::arch::asm;
use init::SysTick;


use lazy_static::lazy_static;
use spin::Mutex;
use cortex_m::interrupt;

lazy_static! {
    pub static ref SYSTEM_PROCESS: Mutex<SystemProcess> = Mutex::new(SystemProcess::new());
}
/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    log_debug!("KRUST");
    
    unsafe {
        init::enable_system_handler_fault();
        init::setup_priority_handler();
        memory_management::heap::initialize_heap();
    }

    let vec = alloc::vec![1, 2, 3, 4];
    log_debug!("{:?}", vec);

    #[cfg(test)]
    test_runner();

    let mut sys_tick: SysTick;

    sys_tick = init::SysTick::new();
    sys_tick.init_sys_tick();
    sys_tick.set_sys_tick_reload_us(10_000_000);

    // Create some processes
    interrupt::free(|_cs| {
        let mut system_process = SYSTEM_PROCESS.lock(); // Lock the Mutex
        system_process.create_process("proc_test", test_process as u32);
        system_process.create_process("Process 1", process_1_entry as u32);
        system_process.create_process("Process 2", process_2_entry as u32);
    });
    
    log_debug!("Processes created");

    sys_tick.start_sys_tick();
    test_process();

    loop{}
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

#[allow(named_asm_labels)]
extern "C" fn test_process(){
    unsafe {
        asm!(
        "
            MOV R0, #0              // Initialize counter to 0
            MOV R7, #1 
            MOV R8, #2               
            LDR R1, =10000000       // Set the limit to 10,000,000

            loop:
                ADD R0, R0, #1          // Increment counter (R0)
                ADD R7, R7, #1          
                ADD R8, R8, #1          
                CMP R0, R1              // Compare counter with the limit
                BLT loop                // If counter < 10,000,000, loop again  
        "
        )
    }
}