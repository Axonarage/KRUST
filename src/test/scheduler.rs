use crate::proc::SystemProcess;
use crate::utils::LinkedList;
use crate::log_debug;

#[test_case]
#[inline(never)]
fn test_scheduler() {
    // Initialize the system process manager
    let mut system_process = SystemProcess {
        last_proc_id: 0,
        process_list: LinkedList::new(),
    };

    // Create some processes
    system_process.create_process("Process 1", process_1_entry as usize);
    system_process.create_process("Process 2", process_2_entry as usize);

    // Main scheduler loop
    loop {
        system_process.schedule_next_process();
    }
}

#[test_case]
#[inline(never)]
fn test_list_proc(){
    // Initialize the system process manager
    let mut system_process = SystemProcess {
        last_proc_id: 0,
        process_list: LinkedList::new(),
    };

    // Create some processes
    system_process.create_process("Process 1", process_1_entry as usize);
    system_process.create_process("Process 2", process_2_entry as usize);

    for process in system_process.process_list.iter_mut() {
        log_debug!("{}", process.proc_name);
    }
    log_debug!("{}", system_process.last_proc_id);
}


#[test_case]
#[inline(never)]
fn test_kill_proc(){
    // Initialize the system process manager
    let mut system_process = SystemProcess {
        last_proc_id: 0,
        process_list: LinkedList::new(),
    };

    // Create some processes
    system_process.create_process("Process 1", process_1_entry as usize);
    system_process.create_process("Process 2", process_2_entry as usize);

    for process in system_process.process_list.iter_mut() {
        log_debug!("{}", process.proc_name);
    }
    log_debug!("{}", system_process.last_proc_id);

    system_process.kill_process(1);

    for process in system_process.process_list.iter_mut() {
        log_debug!("{}", process.proc_name);
    }
    log_debug!("{}", system_process.last_proc_id);
}


// Dummy entry points for processes
extern "C" fn process_1_entry() {
    loop {
        log_debug!("Process 1 running");
    }
}

extern "C" fn process_2_entry() {
    loop {
        log_debug!("Process 2 running");
    }
}
