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
    sys_tick.set_sys_tick_reload_us(10_000); //10_000_000

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
    
    sys_tick.start_sys_tick();
   
    loop{}
}

/// proc_1 bytecode
const TEST_1_PROC_BYTE_CODE: &[u8;64] = b"\x0b\x4a\x0c\x49\x08\xf1\x01\x08\x88\x45\xfb\xdb\x09\xf1\x01\x09\x89\x45\xf7\xdb\x4f\xf0\x01\x00\x07\x49\x00\xdf\x00\xf1\x01\x00\x88\x42\xef\xdb\x4f\xf0\x00\x00\x04\x49\x00\xdf\xfe\xe7\x00\xbf\xde\xc0\xad\x0b\xff\xff\xff\xff\x01\x70\xad\x0b\x01\xc0\xad\x0b";
// LDR R2, =0xbadc0de
// LDR R1, =0xffffffff

// loop_0:
//     loop_1:
//         loop_2:
//             ADD R8, R8, #1
//             CMP R8, R1
//             BLT loop_2
//         ADD R9, R9, #1
//         CMP R9, R1
//         BLT loop_1
//     MOV R0, #1
//     LDR R1, =0xbad7001
//     SVC 0
//     ADD R0, R0, #1
//     CMP R0, R1
//     BLT loop_0

// MOV R0, #0
// LDR R1, =0xbadc001
// SVC 0

// end_loop:
//     B end_loop

/// proc_2 bytecode
const TEST_2_PROC_BYTE_CODE: &[u8;64] = b"\x0b\x4a\x0c\x49\x08\xf1\x01\x08\x88\x45\xfb\xdb\x09\xf1\x01\x09\x89\x45\xf7\xdb\x4f\xf0\x01\x00\x07\x49\x00\xdf\x00\xf1\x01\x00\x88\x42\xef\xdb\x4f\xf0\x00\x00\x04\x49\x00\xdf\xfe\xe7\x00\xbf\xde\xc0\xad\xde\xff\xff\xff\xff\x01\x70\xad\xde\x01\xc0\xad\xde";
// LDR R2, =0xdeadc0de
// LDR R1, = 0xffffffff

// loop_0:
//     loop_1:
//         loop_2:
//             ADD R8, R8, #1
//             CMP R8, R1
//             BLT loop_2
//         ADD R9, R9, #1
//         CMP R9, R1
//         BLT loop_1
//     MOV R0, #1
//     LDR R1, =0xdead7001
//     SVC 0
//     ADD R0, R0, #1
//     CMP R0, R1
//     BLT loop_0

// MOV R0, #0
// LDR R1, =0xdeadc001
// SVC 0

// end_loop:
//     B end_loop