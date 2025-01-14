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
use init::SysTick;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// Hold reference to SystemProcess object
    pub static ref SYSTEM_PROCESS: Mutex<SystemProcess> = Mutex::new(SystemProcess::new());
}

/// Krust main function called by the Reset handler
/// 
/// Through function calls, enables System Handlers, System Heap and SysTick
/// Create 2 process, proc_1 and proc_2, then start SysTick
pub fn main() -> ! {
    
    log_debug!("=== KRUST ===");
    
    unsafe {
        init::enable_system_handler_fault();
        init::setup_priority_handler();
        memory_management::heap::initialize_heap();
    }

    #[cfg(test)]
    test_runner();

    let mut sys_tick: SysTick;

    sys_tick = init::SysTick::new();
    sys_tick.init_sys_tick();
    sys_tick.set_sys_tick_reload_us(10_000_000); //10_000_000

    let mut pid: u16;

    log_debug!("\n### NEW PROC 1 ###");

    // Create PROC 1
    {
        let mut system_process = SYSTEM_PROCESS.lock(); // Lock the Mutex
        pid = system_process.create_process("proc_1", TEST_1_PROC_BYTE_CODE, TEST_1_PROC_BYTE_CODE.len(), 1);

        let proc = system_process.get_process_by_id(pid).expect("No process with this ID");

        log_debug!("PID : {}",pid);
        log_debug!("Entry Point : {:p}",proc.get_entry_point());
        log_debug!("PSP : {:#x}",proc.get_stack_ptr());
    }

    log_debug!("\n### NEW PROC 2 ###");

    // Create PROC 2
    {
        let mut system_process = SYSTEM_PROCESS.lock(); // Lock the Mutex
        pid = system_process.create_process("proc_2", TEST_2_PROC_BYTE_CODE, TEST_2_PROC_BYTE_CODE.len(), 0);

        let proc = system_process.get_process_by_id(pid).expect("No process with this ID");

        log_debug!("PID : {}",pid);
        log_debug!("Entry Point : {:p}",proc.get_entry_point());
        log_debug!("PSP : {:#x}",proc.get_stack_ptr());
    }

    demo_hook();
    
    sys_tick.start_sys_tick();
   
    loop{}
}

#[inline(never)]
fn demo_hook() {
    log_debug!("hook");
}

const TEST_1_PROC_BYTE_CODE: &[u8;28] = b"\x4f\xf0\x00\x04\x03\x4d\x4f\xf0\x01\x00\x03\x49\x00\xdf\xac\x42\xf6\xd0\xfc\xe7\xdd\xcc\xbb\xaa\x01\x70\xad\x0b";
const TEST_2_PROC_BYTE_CODE: &[u8;28] = b"\x4f\xf0\x00\x04\x03\x4d\x4f\xf0\x01\x00\x03\x49\x00\xdf\xac\x42\xf6\xd0\xfc\xe7\xdd\xcc\xbb\xaa\x01\x70\xad\xde";

// print_call:
//     MOV R4, #0
//     MOV R0, #1
//     LDR R1, =0xbad7001
//     SVC 0

// loop:
//     CMP     R4, #0xAABBCCDD
//     BEQ     print_call
//     B       loop 
