use crate::proc::SystemProcess;
use crate::utils::LinkedList;

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

// Dummy entry points for processes
extern "C" fn process_1_entry() {
    loop {
        // Process 1 logic
    }
}

extern "C" fn process_2_entry() {
    loop {
        // Process 2 logic
    }
}
