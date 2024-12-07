use crate::utils::LinkedList;

type Unknown = u8;

#[derive(Default,PartialEq)]
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

impl SystemProcess {
    fn get_new_proc_id(&mut self) -> u16 {
        self.last_proc_id += 1;
        self.last_proc_id
    }

    fn get_new_proc_state(&mut self) -> ProcessState {
        ProcessState::default()
    }

    pub fn create_process(&mut self, name: &'static str) {
        let new_proc = Process::new(name, self.get_new_proc_id(), self.get_new_proc_state());
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
struct Process {
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