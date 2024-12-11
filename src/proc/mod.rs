use crate::utils::LinkedList;
use crate::init::{trigger_pendsv, CURRENT_PROCESS_SP, NEXT_PROCESS_SP};

type Unknown = u8;

#[derive(Default,PartialEq)]
#[allow(dead_code)]
pub enum ProcStatus{
    #[default] Idle,
    Running,
    Waiting,
    Finished
}

pub struct SystemProcess {
    last_proc_id: u16,
    process_list: LinkedList<Process>
}

unsafe impl Send for SystemProcess {}
unsafe impl Sync for SystemProcess {}

impl SystemProcess {
    pub fn new() -> SystemProcess {
        SystemProcess {
            last_proc_id: 0,
            process_list: LinkedList::new()
        }
    }

    fn get_new_proc_id(&mut self) -> u16 {
        self.last_proc_id += 1;
        self.last_proc_id
    }

    pub fn create_process(&mut self, name: &'static str,  entry_point: u32) {
        let stack = [0; 1024]; // Allocate stack memory
        let sp = stack.as_ptr() as u32 + 1024; // Calculate initial SP

        let new_proc = Process::new(name, self.get_new_proc_id(), sp,entry_point);
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
                current_process.stored_sp = CURRENT_PROCESS_SP;
            }

            // Mark the next process as Running
            next_process.status = ProcStatus::Running;

            // Restore the next process state
            unsafe {
                NEXT_PROCESS_SP = next_process.stored_sp;
                trigger_pendsv(); // Trigger the context switch
            }
        }
    }

}

/// This struct is the kernel representation of a process
/// Using this struct, the scheduler can transfert the execution flow to the represented process
#[derive(Default,PartialEq)]
struct Process {
    proc_name: &'static str,
    proc_id: u16,
    status: ProcStatus,
    mem_regions: [Unknown; 8],
    stored_sp: u32,
    entry_point: u32
}

impl Process {
    fn new(name: &'static str,proc_id: u16, init_sp: u32, entry_point: u32) -> Self {
        Process {
            proc_name: name,
            proc_id,
            status: ProcStatus::Idle,
            mem_regions: [0;8],
            stored_sp: init_sp,
            entry_point: entry_point
        }
    }
}