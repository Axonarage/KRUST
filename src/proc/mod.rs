use core::u8;

use crate::log_debug;
use core::ptr;
use crate::memory_management::heap;
use crate::utils::LinkedList;
use crate::init::{CURRENT_PROCESS_SP, NEXT_PROCESS_SP};

type Unknown = u8;

#[derive(Default,PartialEq)]
#[allow(dead_code)]
pub enum ProcStatus{
    #[default] Idle,
    Running,
    Waiting,
    Finished
}

const DEFAULT_STACK_SIZE: usize = 1024; 
const INIT_STACK_FRAME_SIZE: usize = size_of::<usize>() * 16;

pub struct SystemProcess {
    last_proc_id: u16,
    process_list: LinkedList<Process>,
    current_process_id: u16
}

unsafe impl Send for SystemProcess {}
unsafe impl Sync for SystemProcess {}

impl SystemProcess {
    pub fn new() -> SystemProcess {
        SystemProcess {
            last_proc_id: 0,
            process_list: LinkedList::new(),
            current_process_id: 0
        }
    }

    fn get_new_proc_id(&mut self) -> u16 {
        self.last_proc_id += 1;
        self.last_proc_id
    }

    pub fn get_process_by_id(&mut self, proc_id: u16) -> Option<Process>{
        for process in self.process_list.iter() {
            if process.proc_id == proc_id {
                return Some(process);
            }
        }
        return None;
    }

    pub fn get_current_process(&mut self) -> Option<Process> {
        self.get_process_by_id(self.current_process_id)
    }

    pub fn create_process(&mut self, name: &'static str, code_ptr: &[u8], code_len: usize) -> u16 {
        let pid = self.get_new_proc_id();

        let entry_point = self.load_process_code(code_ptr, code_len);

        let stack: *mut u8;
        unsafe { 
            stack = heap::allocate(DEFAULT_STACK_SIZE);
        }
        let sp = stack as usize + DEFAULT_STACK_SIZE - INIT_STACK_FRAME_SIZE; // Calculate initial SP
        self.create_init_stack_frame(sp as *mut u8,entry_point);

        let new_proc = Process::new(name, pid,stack, sp as u32, entry_point);
        self.process_list.add(new_proc);
        return pid;

    }

    fn load_process_code(&mut self, code_ptr: &[u8], code_len: usize) -> *mut u8{
        let heap_ptr: *mut u8;
        unsafe { 
            heap_ptr = heap::allocate(code_len);
            ptr::copy_nonoverlapping(code_ptr.as_ptr(), heap_ptr, code_len);
        }
        return heap_ptr;
    }

    fn create_init_stack_frame(&mut self, stack_ptr: *mut u8, entry_point: *mut u8){
        unsafe {
            ptr::write_bytes(stack_ptr, 0, size_of::<usize>() * 14);
            ptr::write(stack_ptr.add(size_of::<usize>() * 14) as *mut *mut u8, entry_point); // SP
            ptr::write(stack_ptr.add(size_of::<usize>() * 15) as *mut usize, 0x01000000); // xPSR with Thumb instruction set
        }
        
    }

    pub fn kill_process(&mut self, proc_id: u16) {

        for process in self.process_list.iter() {
            if process.proc_id == proc_id {
                //process.status = ProcStatus::Finished;
                
                unsafe { 
                    heap::deallocate(process.entry_point);
                    heap::deallocate(process.stack); 
                }

                self.process_list.delete(process);
                break;
            }
        }
    }

    pub fn kill_current_process(&mut self) {
        self.kill_process(self.current_process_id)
    }

    pub fn schedule_next_process(&mut self) {

        log_debug!("\n### CALL TO SCHED ###");

        // Find the next idle process and the current running process
        let mut next_process = None;
        let mut current_process = None;

        // Iterate over the LinkedList to find the idle and running processes
        for process in self.process_list.iter_mut() {
            if process.status == ProcStatus::Idle && next_process.is_none() {
                log_debug!("Next Process : {}",process.proc_name);
                next_process = Some(process);
            }
            else if process.status == ProcStatus::Running {
                log_debug!("Current Process : {}",process.proc_name);
                current_process = Some(process);
            }
        }

        if let Some(ref mut current_process) = current_process {
            // Mark the current process as Idle
            current_process.status = ProcStatus::Idle;

            // Save the current process state
            unsafe {
                current_process.stored_sp = CURRENT_PROCESS_SP;
            }
        }

        // If we found both a running process and an idle process, schedule the next process
        if let Some(ref mut next_process) = next_process {
            // Mark the next process as Running
            next_process.status = ProcStatus::Running;

            // Restore the next process state
            unsafe {
                NEXT_PROCESS_SP = next_process.stored_sp;
            }
            self.current_process_id = next_process.proc_id;
        } else if let Some(ref mut current_process) = current_process {
            // Set current_process as next_process
            current_process.status = ProcStatus::Running;

            unsafe {
                NEXT_PROCESS_SP = current_process.stored_sp;
            }
            self.current_process_id = current_process.proc_id;
        } else {
            self.current_process_id = 0;
            panic!("NOTHING TO DO");
        }
    }

}

/// This struct is the kernel representation of a process
/// Using this struct, the scheduler can transfert the execution flow to the represented process
#[derive(PartialEq)]
pub struct Process {
    proc_name: &'static str,
    proc_id: u16,
    status: ProcStatus,
    mem_regions: [Unknown; 8],
    stack: *mut u8,
    stored_sp: u32,
    entry_point: *mut u8
}

impl Process {
    fn new(name: &'static str,proc_id: u16, stack_ptr: *mut u8, init_sp: u32, entry_point: *mut u8) -> Self {
        Process {
            proc_name: name,
            proc_id,
            status: ProcStatus::Idle,
            mem_regions: [0;8],
            stack: stack_ptr,
            stored_sp: init_sp,
            entry_point: entry_point
        }
    }

    pub fn get_stack_ptr(&self) -> u32 {
        self.stored_sp
    }

    pub fn get_entry_point(&self) -> *mut u8 {
        self.entry_point
    }
}