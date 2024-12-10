use crate::utils::LinkedList;
use crate::init::{trigger_pendsv, CURRENT_PROCESS_STATE, NEXT_PROCESS_STATE};

type Unknown = u8;

#[derive(Default,PartialEq)]
pub enum ProcStatus{
    #[default] Idle,
    Running,
    Waiting,
    Finished
}
pub struct SystemProcess {
    pub last_proc_id: u16,
    pub process_list: LinkedList<Process>
}

impl SystemProcess {
    fn get_new_proc_id(&mut self) -> u16 {
        self.last_proc_id += 1;
        self.last_proc_id
    }

    fn get_new_proc_state(&mut self) -> ProcessState {
        ProcessState::default()
    }

    pub fn create_process(&mut self, name: &'static str,  entry_point: usize) {
        let stack = [0; 1024]; // Allocate stack memory
        let sp = stack.as_ptr() as usize + 1024; // Calculate initial SP

        let mut new_state = self.get_new_proc_state();
        new_state.sp = sp; // Set the stack pointer
        new_state.pc = entry_point; // Set the program counter

        let new_proc = Process::new(name, self.get_new_proc_id(), new_state);
        self.process_list.add(new_proc);
    }

    pub fn kill_process(&mut self, proc_id: u16) {

        for process in self.process_list.iter() {
            if process.proc_id == proc_id {
                //process.status = ProcStatus::Finished;

                self.process_list.delete(process);
                break;
            }
        }
    }


    pub fn schedule_next_process(&mut self) {
        // Find the next idle process and the current running process
        let mut next_process = None;
        let mut current_process = None;

        // Iterate over the LinkedList to find the idle and running processes
        for process in self.process_list.iter_mut() {
            if process.status == ProcStatus::Idle && next_process.is_none() {
                next_process = Some(process);
            }
            else if process.status == ProcStatus::Running {
                current_process = Some(process);
            }
        }

        // If we found both a running process and an idle process, schedule the next process
        if let (Some(next_process), Some(current_process)) = (next_process, current_process) {
            // Mark the current process as Idle
            current_process.status = ProcStatus::Idle;

            // Save the current process state
            unsafe {
                CURRENT_PROCESS_STATE = current_process.stored_state.sp;
            }

            // Mark the next process as Running
            next_process.status = ProcStatus::Running;

            // Restore the next process state
            unsafe {
                NEXT_PROCESS_STATE = next_process.stored_state.sp;
                trigger_pendsv(); // Trigger the context switch
            }
        }
    }

}

#[derive(Default,PartialEq)]
struct ProcessState {
    registers: [usize; 13], // r0 - r12
    pc: usize, // Program Counter
    sp: usize, // Stack Pointer
    lr: usize, // Link Register
    psr: usize // Program Status Registers
}

/// This struct is the kernel representation of a process
/// Using this struct, the scheduler can transfert the execution flow to the represented process
#[derive(Default,PartialEq)]
pub struct Process {
    proc_name: &'static str,
    proc_id: u16,
    status: ProcStatus,
    mem_regions: [Unknown; 8],
    stored_state : ProcessState
}

impl Process {
    pub fn new(name: &'static str,proc_id: u16, proc_state: ProcessState) -> Self {
        Process {
            proc_name: name,
            proc_id,
            status: ProcStatus::Idle,
            mem_regions: [0;8],
            stored_state: proc_state
        }
    }
}