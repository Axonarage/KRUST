searchState.loadedDescShard("krust", 0, "Main logic of Krust kernel\nHold reference to SystemProcess object\nproc_1 bytecode\nproc_2 bytecode\nReturns the argument unchanged.\nInitialization of Cortex-M device\nCalls <code>U::from(self)</code>.\nKrust main function called by the Reset handler\nReset vector, part of the Vector table, points to our …\nOur Reset handler, wich initializes RAM and calls main\nExceptions vectors, part of the Vector table\nReturns the argument unchanged.\nInitialization of the .bss section by zeroing  out memory\nInitialization of the .data section by populating it with …\nCalls <code>U::from(self)</code>.\nPendSV_Handler performing context switch\nHandles system calls (SVC) by processing the syscall …\nReturns the argument unchanged.\nInitialization of SysTick : setup SYST_CSR and get TENMS …\nCalls <code>U::from(self)</code>.\nSet reload value in SYST_RVR (use microseconds)\nEnable SysTick\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nGestionnaire de la MPU\nReprésente une région MPU\nConfigure une région MPU\nDésactive la MPU\nActive la MPU\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCrée une nouvelle instance de MPU\nConfigurer les registres\nThis struct is the kernel representation of a process …\nThis struct hold reference to the Process List of the …\nInitializes the stack frame for a new process. This …\nCreates a new process, assigns a process ID, allocates …\nMark the running process as Finished, so that this process …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet priority of the running process\nGet non mutable reference of the running process\nGet free PID for new process\nGet non mutable reference of a process from a PID\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nKill a specific process based on a PID\nList process in the Process List with the following format …\nLoads the code for a new process into heap memory and …\nSchedules the next process to run. This function also …\nAdd node at the end of the linked list\nDelete specific node in the linked list\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")