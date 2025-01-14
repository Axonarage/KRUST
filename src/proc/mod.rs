use core::u8;

use alloc::vec::Vec;

use crate::log_debug;
use core::ptr;
use crate::memory_management::heap;
use crate::utils::LinkedList;
use crate::init::{CURRENT_PROCESS_SP, NEXT_PROCESS_SP};

type Unknown = u8;

#[derive(Default,PartialEq,Clone,Copy)]
#[allow(dead_code)]
pub enum ProcStatus{
    #[default] Idle,
    Running,
    Waiting,
    Finished
}

const DEFAULT_STACK_SIZE: usize = 1024; 
const INIT_STACK_FRAME_SIZE: usize = size_of::<usize>() * 16;

/// This struct hold reference to the Process List of the system, and the PID of the running process
/// 
/// All operation performed on process are implemented here (create, kill, schedule, ...) 
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

    /// Get free PID for new process
    fn get_new_proc_id(&mut self) -> u16 {
        self.last_proc_id += 1;
        self.last_proc_id
    }

    /// Get non mutable reference of a process from a PID
    pub fn get_process_by_id(&mut self, proc_id: u16) -> Option<Process>{
        for process in self.process_list.iter() {
            if process.proc_id == proc_id {
                return Some(process);
            }
        }
        return None;
    }

    /// Get non mutable reference of the running process
    pub fn get_current_process(&mut self) -> Option<Process> {
        self.get_process_by_id(self.current_process_id)
    }

    /// Get priority of the running process
    pub fn get_current_priority_process(&mut self) -> u8 {
        for process in self.process_list.iter() {
            if process.proc_id == self.current_process_id {
                return process.priority;
            }
        }
        return 255;
    }


    /// Creates a new process, assigns a process ID, allocates memory for its code and stack, 
    /// initializes the stack frame, and adds the new process to the process list.
    /// 
    /// # Arguments
    /// * `name` - A string representing the name of the new process.
    /// * `code_ptr` - A byte slice containing the code to be loaded into the process.
    /// * `code_len` - The length of the code to be loaded.
    /// 
    /// # Returns
    /// * A unique process ID (PID) for the newly created process.
    ///
    /// # IMPORTANT
    /// Process code MUST end with a SYS_EXIT then an infinite loop
    pub fn create_process(&mut self, name: &'static str, code_ptr: &[u8], code_len: usize, priority: u8) -> u16 {
        let pid = self.get_new_proc_id();

        let entry_point = self.load_process_code(code_ptr, code_len);

        let stack: *mut u8;
        unsafe { 
            stack = heap::allocate(DEFAULT_STACK_SIZE);
        }
        let sp = stack as usize + DEFAULT_STACK_SIZE - INIT_STACK_FRAME_SIZE; // Calculate initial SP
        self.create_init_stack_frame(sp as *mut u8,entry_point);

        let new_proc = Process::new(name, pid,stack, sp as u32, entry_point, priority);
        self.process_list.add(new_proc);
        return pid;

    }

    /// Loads the code for a new process into heap memory and returns the pointer to the allocated space.
    ///
    /// # Arguments
    /// * `code_ptr` - A byte slice containing the process code to be loaded.
    /// * `code_len` - The length of the code to be loaded.
    ///
    /// # Returns
    /// * A pointer to the allocated memory containing the process code.
    fn load_process_code(&mut self, code_ptr: &[u8], code_len: usize) -> *mut u8{
        let heap_ptr: *mut u8;
        unsafe { 
            heap_ptr = heap::allocate(code_len);
            ptr::copy_nonoverlapping(code_ptr.as_ptr(), heap_ptr, code_len);
        }
        return heap_ptr;
    }

    /// Initializes the stack frame for a new process. This function sets up the initial values 
    /// on the stack, such as the entry point and xPSR value, which are required when the process 
    /// starts execution.
    /// 
    /// The 14 first u32 (save of register r0 to r12 + LR) are set to 0x00 so that new process starts with clean registers
    /// Then we set up the entry point and the xPSR.
    /// 
    /// When the scheduler will pick up a process for the fisrt time, it'll unwrap the INIT STACK, setting up all registers properly 
    /// 
    /// INIT STACK
    /// ```
    /// +--------+ < SP
    /// | 0x00   |
    /// +--------+
    /// | ...    |
    /// +--------+ < SP + 14 * REG_SIZE
    /// | PC     |
    /// +--------+ < SP + 15 * REG_SIZE
    /// | xPSR   |
    /// +--------+
    /// ```
    fn create_init_stack_frame(&mut self, stack_ptr: *mut u8, entry_point: *mut u8){
        unsafe {
            ptr::write_bytes(stack_ptr, 0, size_of::<usize>() * 14);
            ptr::write(stack_ptr.add(size_of::<usize>() * 14) as *mut *mut u8, entry_point); // PC
            ptr::write(stack_ptr.add(size_of::<usize>() * 15) as *mut usize, 0x01000000); // xPSR
        }
    }

    /// Kill a specific process based on a PID
    pub fn kill_process(&mut self, proc_id: u16) {
        log_debug!("> KILL PID {}",proc_id);
        for process in self.process_list.iter() {
            if process.proc_id == proc_id {                
                unsafe { 
                    heap::deallocate(process.entry_point);
                    heap::deallocate(process.stack); 
                }

                self.process_list.delete(process);
                break;
            }
        }
    }

    /// Mark the running process as Finished, so that this process get killed on next scheduler call
    pub fn exit_current_process(&mut self) {
        for process in self.process_list.iter_mut() {
            if process.proc_id == self.current_process_id {
                process.status = ProcStatus::Finished;
                break;
            }
        }
    }

    /// List process in the Process List with the following format : 
    /// [PID] PROC_NAME (PROC_STATUS)
    pub fn list_proc(&mut self) {
        for process in self.process_list.iter_mut() {
            log_debug!("> [{}] {} ({})",process.proc_id,process.proc_name,process.status as u8);
        }
    }

    /// Schedules the next process to run.
    /// This function also handles killing processes that have finished execution.
    ///
    /// It performs the following:
    /// - Collects the processes that are finished and calls `kill_process` on them.
    /// - Iterates over the list of processes to identify the next idle process and the currently running process based on priority.
    /// - Marks the current running process as idle, saves its state, and schedules the next process.
    ///
    /// # Panics
    /// This function will panic with the message `"NOTHING TO DO"` if it can't find a next process to schedule.
    pub fn schedule_next_process(&mut self) {

        log_debug!("\n### CALL TO SCHED ###");

        // Find the next idle process and the current running process
        let mut next_process = None;
        let mut current_process = None;
        
        
        let to_kill: Vec<u16> = self.process_list.iter()
        .filter_map(|process| {
            if process.status == ProcStatus::Finished {
                Some(process.proc_id)  // Collect the process ID
            } else {
                None
            }
        }).collect();

        for proc_id in to_kill {
            self.kill_process(proc_id);  // Call kill_process outside of the loop
        }

        // Filter processes by priority
        let highest_priority = self.process_list.iter()
            .filter(|process| process.status == ProcStatus::Idle)
            .min_by_key(|process| process.priority)
            .map(|process| process.priority);
        
        // Iterate over the LinkedList to find the idle and running processes
        for process in self.process_list.iter_mut() {
            if process.status == ProcStatus::Running {
                log_debug!("Current Process: {}", process.proc_name);
                log_debug!("Priority : {}", process.priority);
                current_process = Some(process);
            } else if process.status == ProcStatus::Idle {
                if let Some(priority) = highest_priority {
                    if process.priority == priority && next_process.is_none() {
                        log_debug!("Next Process: {}", process.proc_name);
                        next_process = Some(process);
                    }
                }
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
            if next_process.stored_sp != 0 {
                // Mark the next process as Running
                next_process.status = ProcStatus::Running;

                // Restore the next process state
                unsafe {
                    NEXT_PROCESS_SP = next_process.stored_sp;
                }
                self.current_process_id = next_process.proc_id;
            } else {
                next_process.status = ProcStatus::Finished;
                self.schedule_next_process();
            }
            
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
    entry_point: *mut u8,
    priority: u8
}

impl Process {
    fn new(name: &'static str,proc_id: u16, stack_ptr: *mut u8, init_sp: u32, entry_point: *mut u8, priority: u8) -> Self {
        Process {
            proc_name: name,
            proc_id,
            status: ProcStatus::Idle,
            mem_regions: [0;8],
            stack: stack_ptr,
            stored_sp: init_sp,
            entry_point: entry_point,
            priority: priority
        }
    }

    pub fn get_stack_ptr(&self) -> u32 {
        self.stored_sp
    }

    pub fn get_entry_point(&self) -> *mut u8 {
        self.entry_point
    }
}