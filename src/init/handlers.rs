use core::sync::atomic::{compiler_fence, Ordering};
use crate::{log_debug, log_info};
use core::arch::asm;
use cortex_m::interrupt;
use crate::SYSTEM_PROCESS;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler() -> ! {
    log_debug!("Default Handler");
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn NMIHandler() -> ! {
    log_debug!("NMI Handler");
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn HardFaultHandler() -> ! {

    /*
        Hard Fault status Register

        Address offset : 0x2C
    
        Bit 31    : DEBUG_VT : Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is unpredictable.
        Bit 30    : FORCED   : Forced hard fault. Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot 
                               be handles, either because of priority or because it is disabled.
                               When this bit is set to 1, the hard fault handler must read the other fault status registers to find the cause of the fault.
        Bits 29:2 : Reserved
        Bit 1     : VECTTBL  : Vector table hard fault. Indicates a bus fault on a vector table read during exception processing. This error is always handled 
                               by the hard fault handler.
                               When this bit is set to 1, the PC value stacked for the exception return points to the instruction that was preempted by the exception.
        Bit 0     : Reserved

        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.14
     */

    const HFSR_ADDR: u32 = 0xE000ED2C;
    let hfsr_value: u32;
    unsafe {
        // Read the value from the HFSR address
        hfsr_value = core::ptr::read_volatile(HFSR_ADDR as *const u32);
    }
    let debug_vt = (hfsr_value >> 31) & 1;
    let forced = (hfsr_value >> 30) & 1;
    let vecttbl = (hfsr_value >> 1) & 1;

    if debug_vt == 1 {
        log_debug!("Debug is used.");
    }
    if forced == 1 {
        // inspect other fault status registers
        log_debug!("Forced hard fault. Need to inspect the other fault status registers.");

        FaultHandler();
    }
    if vecttbl == 1 {
        log_debug!("Bus fault while trying to read the vector table.");
        //asm!(
        //    "BKPT #0"
        //);
    }

    // Keep the program in an infinite loop
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}


#[allow(non_snake_case)]
fn FaultHandler() -> ! {
    const CFSR_ADDR: u32 = 0xE000ED28;
    let cfsr_value: u32;

    unsafe {
        cfsr_value = core::ptr::read_volatile(CFSR_ADDR as *const u32);
    }

    // UFSR : Usage Fault status register
    let ufsr: u32 = cfsr_value >> 16;
    let ufsr_mask: u32 = 0b1100001111;

    // BFSR : Bus Fault status register
    let bfsr: u32 = (cfsr_value & 0xffff) >> 8;
    let bfsr_mask: u32 = 0b10111111;

    // MMFSR : Memory Management Fault status register
    let mmfsr: u32 = cfsr_value & 0xff;
    let mmfsr_mask: u32 = 0b10111011;

    if (ufsr & ufsr_mask) != 0 {
        unsafe {
            UsageFaultHandler();
        }
    } else if (bfsr & bfsr_mask) != 0 {
        unsafe {
            BusFaultHandler();
        }
    } else if (mmfsr & mmfsr_mask) != 0 {
        unsafe {
            MemoryManagementFaultHandler();
        }
    }

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn UsageFaultHandler() -> ! {
    /*
        Configurable Fault Status Register

        Address offset : 0x28
    
        Usage Fault Status Register

        Bits 31:26 : Reserved   :
        Bit 25     : DIVBYZERO  : Divide by zero usage fault. When the processor sets this bit to 1, the PC value stacked for the exception return points to 
                                  the instruction that performed the divide by zero.
        Bit 24     : UNALIGNED  : Unaligned access usage fault. Enable trapping of unaligned accesses by setting the UNALIGN_TRP bit in the CCR to 1.
        Bits 23:20 : Reserved
        Bit 19     : NOCP       : No coprocessor usage fault. The processor does not support coprocessor instructions.
        Bit 18     : INVPC      : Invalid PC load usage fault, caused by an invalid PC load by EXC_RETURN. When this bit is set to 1, the PC value stacked for 
                                  the exception return points to the instruction that tried to perform the illegal load of the PC.
        Bit 17     : INVSTATE   : Invalid state usage fault. When this bit is set to 1, the PC value stacked for the exception return points to the instruction 
                                  that attempted the illegal use of the EPSR. This bit is not set to 1 if an undefined instruction uses the EPSR.
        Bit 16     : UNDEFINSTR : Undefined instruction usage fault. When this bit is set to 1, the PC value stacked for the exception return points to the 
                                  undefined instruction. An undefined instruction is an instruction that the processor cannot decode.

        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.11
    */

    const CFSR_ADDR: u32 = 0xE000ED28;
    let cfsr_value: u32;

    unsafe {
        cfsr_value = core::ptr::read_volatile(CFSR_ADDR as *const u32);
    }

    // UFSR : Usage Fault status register
    let ufsr: u32 = cfsr_value >> 16;
    let ufsr_mask: u32 = 0b1100001111;

    const DIVBYZERO_BIT: u32 = 25;
    const UNALIGNED_BIT: u32 = 24;
    const NOCP_BIT: u32 = 19;
    const INVPC_BIT: u32 = 18;
    const INVSTATE_BIT: u32 = 17;
    const UNDEFINSTR_BIT: u32 = 16;

    if (ufsr & ufsr_mask) != 0 {
        log_debug!("Usage Fault.");
        
        if (cfsr_value >> DIVBYZERO_BIT) & 1 == 1 {
            log_debug!("Divide by zero usage fault.");
        } else if (cfsr_value >> UNALIGNED_BIT) & 1 == 1 {
            log_debug!("Unaligned access usage fault.");
        } else if (cfsr_value >> NOCP_BIT) & 1 == 1 {
            log_debug!("No coprocessor usage fault.");
        } else if (cfsr_value >> INVPC_BIT) & 1 == 1 {
            log_debug!("Invalid PC load usage fault, caused by an invalid PC load by EXC_RETURN.");
        } else if (cfsr_value >> INVSTATE_BIT) & 1 == 1 {
            log_debug!("Invalid state usage fault.");
        } else if (cfsr_value >> UNDEFINSTR_BIT) & 1 == 1 {
            log_debug!("Undefined instruction usage fault.");
        } 
    }

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn BusFaultHandler() -> ! {
    /*
        Configurable Fault Status Register

        Address offset : 0x28
    
        Bus Fault Status Register

        Bit 15     : BFARVALID  : Bus Fault Address Register (BFAR) valid flag. The processor sets this bit to 1 after a bus fault where the address is known. 
                                  Other faults can set this bit to 0, such as a memory management fault occurring later. If a bus fault occurs and is escalated 
                                  to a hard fault because of priority, the hard fault handler must set this bit to 0. This prevents problems if returning to a 
                                  stacked active bus fault handler whose BFAR value is overwritten.
        Bit 14     : Reserved
        Bit 13     : LSPERR     : Bus fault on floating-point lazy state preservation.
        Bit 12     : STKERR     : Bus fault on stacking for exception entry. When the processor sets this bit to 1, the SP is still adjusted but the values in 
                                  the context area on the stack might be incorrect. The processor does not write a fault address to the BFAR.
        Bit 11     : UNSTKERR   : Bus fault on unstacking for a return from exception. This fault is chained to the handler. This means that when the processor 
                                  sets this bit to 1, the original return stack is still present. The processor does not adjust the SP from the failing return, 
                                  does not performed a new save, and does not write a fault address to the BFAR.
        Bit 10     : IMPRECISERR: Imprecise data bus error. When the processor sets this bit to 1, it does not write a fault address to the BFAR. This is an 
                                  asynchronous fault. Therefore, if it is detected when the priority of the current process is higher than the bus fault priority, 
                                  the bus fault becomes pending and becomes active only when the processor returns from all higher priority processes. If a precise 
                                  fault occurs before the processor enters the handler for the imprecise bus fault, the handler detects both IMPRECISERR set to 1 
                                  and one of the precise fault status bits set to 1.
        Bit 9      : PRECISERR  : Precise data bus error. When the processor sets this bit is 1, it writes the faulting address to the BFAR.
        Bit 8      : IBUSERR    : Instruction bus error. The processor detects the instruction bus error on prefetching an instruction, but it sets the IBUSERR 
                                  flag to 1 only if it attempts to issue the faulting instruction.
        
        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.12


        Bus Fault Address Register

        Address offset : 0x38

        Bits 31-0  : BFAR       : Bus fault address. When the BFARVALID bit of the BFSR is set to 1, this field holds the address of the location that generated the 
                                  bus fault. When an unaligned access faults the address in the BFAR is the one requested by the instruction, even if it is not the 
                                  address of the fault. 

        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.16
    */

    const CFSR_ADDR: u32 = 0xE000ED28;
    const BFAR_ADDR: u32 = 0xE000ED38;

    let cfsr_value: u32;
    let bfar_value: u32;

    unsafe {
        cfsr_value = core::ptr::read_volatile(CFSR_ADDR as *const u32);
    }

    // BFSR : Bus Fault status register
    let bfsr: u32 = (cfsr_value & 0xffff) >> 8;
    let bfsr_mask: u32 = 0b10111111;

    const BFARVALID_BIT: u32 = 15;
    const LSPEERR_BIT: u32 = 13;
    const STKERR_BIT: u32 = 12;
    const UNSTKERR_BIT: u32 = 11;
    const IMPRECISERR_BIT: u32 = 10;
    const PRECISERR_BIT: u32 = 9;
    const IBUERR_BIT: u32 = 8;

    if (bfsr & bfsr_mask) != 0 {
        log_debug!("Bus Fault.");
        
        if (cfsr_value >> LSPEERR_BIT) & 1 == 1 {
            log_debug!("Bus fault on floating-point lazy state preservation.");
        } else if (cfsr_value >> STKERR_BIT) & 1 == 1 {
            log_debug!("Bus fault on stacking for exception entry.");
        } else if (cfsr_value >> UNSTKERR_BIT) & 1 == 1 {
            log_debug!("Bus fault on unstacking for a return from exception.");
        } else if (cfsr_value >> IMPRECISERR_BIT) & 1 == 1 {
            log_debug!("Imprecise data bus error.");
        } else if (cfsr_value >> PRECISERR_BIT) & 1 == 1 {
            log_debug!("Precise data bus error.");
        } else if (cfsr_value >> IBUERR_BIT) & 1 == 1 {
            log_debug!("Instruction bus error.");
        } 
    }

    // Bus Fault Address Register (BFAR) valid flag.
    if (cfsr_value >> BFARVALID_BIT) & 1 == 1 {
        bfar_value = GetFaultAddress(BFAR_ADDR);
        log_debug!("Fault at address {:#X}", bfar_value);
        // TO CHECK, PRINT LR/ EXC_RETURN value. (Seems to be but not referenced in the table exception return behavior)
        // OUTPUT : 
        //      Bus Fault.
        //      Precise data bus error.
        //      Fault at address 0xFFFFFFFC
    }

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn MemoryManagementFaultHandler() -> ! {
    /*
        Configurable Fault Status Register

        Address offset : 0x28
    
        Memory Management Fault Status Address Register

        Bit 7      : MMARVALID  : Memory Management Fault Address Register (MMAR) valid flag. If a memory management fault occurs and is escalated to a hard fault 
                                  because of priority, the hard fault handler must set this bit to 0. This prevents problems on return to a stacked active memory 
                                  management fault handler whose MMAR value is overwritten.
        Bit 6      : Reserved 
        Bit 5      : MLSPERR    : A MemManage fault occurred during floating-point lazy state preservation. 
        Bit 4      : MSTKERR    : Memory manager fault on stacking for exception entry. When this bit is 1, the SP is still adjusted but the values in the context 
                                  area on the stack might be incorrect. The processor has not written a fault address to the MMAR.
        Bit 3      : MUNSTKERR  : Memory manager fault on unstacking for a return from exception. This fault is chained to the handler. This means that when this 
                                  bit is 1, the original return stack is still present. The processor has not adjusted the SP from the failing return, and has not 
                                  performed a new save. The processor has not written a fault address to the MMAR.
        Bit 2      : Reserved
        Bit 1      : DACCVIOL   : Data access violation flag. When this bit is 1, the PC value stacked for the exception return points to the faulting instruction. 
                                  The processor has loaded the MMAR with the address of the attempted access.
        Bit 0      : IACCVIOL   : Instruction access violation flag. This fault occurs on any access to an XN region, even the MPU is disabled or not present. When 
                                  this bit is 1, the PC value stacked for the exception return points to the faulting instruction. The processor has not written a 
                                  fault address to the MMAR.

        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.13


        Memory Management Fault Address Register

        Address offset : 0x34

        Bits 31-0  : MMFAR      : Memory management fault address. When the MMARVALID bit of the MMFSR is set to 1, this field holds the address of the location that 
                                  generated the memory management fault. When an unaligned access faults, the address is the actual address that faulted. Because a 
                                  single read or write instruction can be split into multiple aligned accesses, the fault address can be any address in the range of 
                                  the requested access size.

        https://developer.arm.com/documentation/dui0552/a/cortex-m3-peripherals/system-control-block/hardfault-status-register - Section 4.4.15
    */

    const CFSR_ADDR: u32 = 0xE000ED28;
    const MMFAR_ADDR: u32 = 0xE000ED34;

    let cfsr_value: u32;
    let mmfar_value: u32;

    unsafe {
        cfsr_value = core::ptr::read_volatile(CFSR_ADDR as *const u32);
    }

    // MMFSR : Memory Management Fault status register
    let mmfsr: u32 = cfsr_value & 0xff;
    let mmfsr_mask: u32 = 0b10111011;

    const MMARVALID_BIT: u32 = 7;
    const MLSPEERR_BIT: u32 = 5;
    const MSTKERR_BIT: u32 = 4;
    const MUNSTKERR_BIT: u32 = 3;
    const DACCVIOL_BIT: u32 = 1;
    const IACCVIOL_BIT: u32 = 0;

    if (mmfsr & mmfsr_mask) != 0 {
        log_debug!("Memory Management Fault.");
        
        if (cfsr_value >> MLSPEERR_BIT) & 1 == 1 {
            log_debug!("MemManage fault occurred during floating-point lazy state preservation.");
        } else if (cfsr_value >> MSTKERR_BIT) & 1 == 1 {
            log_debug!("Memory manager fault on stacking for exception entry.");
        } else if (cfsr_value >> MUNSTKERR_BIT) & 1 == 1 {
            log_debug!("Memory manager fault on unstacking for a return from exception.");
        } else if (cfsr_value >> DACCVIOL_BIT) & 1 == 1 {
            log_debug!("Data access violation flag.");
        } else if (cfsr_value >> IACCVIOL_BIT) & 1 == 1 {
            log_debug!("Instruction access violation flag.");
        }
    }

    // Memory Management Fault Address Register (MMAR) valid flag.
    if (cfsr_value >> MMARVALID_BIT) & 1 == 1 {
        mmfar_value = GetFaultAddress(MMFAR_ADDR);
        log_debug!("Fault at address {:#X}", mmfar_value);
    }

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}


#[allow(non_snake_case)]
fn GetFaultAddress(ADDR: u32) -> u32 {
    let addr_value: u32;

    unsafe {
        addr_value = core::ptr::read_volatile(ADDR as *const u32);
    }

    addr_value
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn SysTickHandler() {
    trigger_pendsv();
}


/// Handles system calls (SVC) by processing the syscall number and its associated arguments.
/// 
/// This function is typically invoked when a system call (SVC) instruction is executed in user mode,
/// and it performs the corresponding action based on the syscall number.
///
/// # Call Convention
///     R0 <- SYSCALL_ID
///     R1 <- ARG0
///     R2 <- ARG1
///     R3 <- ARG2
/// 
/// # Syscalls
/// - `0`: SYS_EXIT - Terminates the current process.
/// - `1`: SYS_PRINT - Prints arg0 in hex
#[unsafe(no_mangle)]
#[allow(unused_variables,unused_assignments)]
pub unsafe extern "C" fn SVCallHandler() {
    
    let syscall_n: u32;
    let arg0: u32;
    let arg1: u32;
    let arg2: u32;

    unsafe { 
        asm!(
            "mov {0}, r0", 
            "mov {1}, r1", 
            "mov {2}, r2", 
            "mov {3}, r3", 
            out(reg) syscall_n, out(reg) arg0, out(reg) arg1, out(reg) arg2
        ); 
    }

    log_debug!("\n### SVCAll Handler ###");

    // Handle the syscall based on the value of R0
    match syscall_n {
        0 => {
            // SYS_EXIT
            log_debug!("[SYS_EXIT] Return code {:#x}",arg0);
            interrupt::free(|_cs| {
                let mut system_process = SYSTEM_PROCESS.lock();
                system_process.exit_current_process();
            });
        }
        1 => {
            // SYS_PRINT
            log_info!("[SYS_PRINT] {:#x}",arg0);
        }
        _ => {
            log_debug!("Unknown syscall : {}", syscall_n);
        }
    }
}

/// PendSV_Handler performing context switch
/// 
/// This function saves the current process state, call the scheduler to get the next process, 
/// and restores the state of this process.
/// It also handles the enabling  and disabling of interrupts during the context switch to ensure atomicity.
/// 
/// PendSV_Handler saves registers r4 to r11 on top of the Exception frame
/// 
/// Exception frame on stack
/// ```
/// +--------+ < SP
/// | R0     |
/// +--------+
/// | R1     |
/// +--------+
/// | R2     |
/// +--------+
/// | R3     |
/// +--------+
/// | R12    |
/// +--------+
/// | LR     |
/// +--------+
/// | PC     |
/// +--------+
/// | RETPSR |
/// +--------+
/// ```
///
/// # Process Flow
/// - Disables interrupts to ensure atomicity during context switching.
/// - Saves the state (stack pointer and callee-saved registers) of the current process.
/// - Schedules the next process using the scheduler.
/// - Restores the state (stack pointer and callee-saved registers) of the next process.
/// - Re-enables interrupts and returns to the process that will resume execution.
#[unsafe(no_mangle)]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn PendSV_Handler() {
    log_debug!("\n### PENDSV Handler ###");

    // Disable interrupts
    unsafe {
        asm!(
            "CPSID I",
            "isb"
        );  
    }

    unsafe{
        if CURRENT_PROCESS_SP != 0 {
            // Save the current process state
            asm!(
                "
                mrs r0, psp             // Get the process stack pointer
                stmdb r0!, {{r4-r11}}   // Store callee-saved registers on process stack
                ldr r1, =CURRENT_PROCESS_SP
                str r0, [r1]            // Save PSP to current process state
            ");
        }

        interrupt::free(|_cs| {
            let mut system_process = SYSTEM_PROCESS.lock(); // Lock the Mutex
            system_process.schedule_next_process();
        });
        log_debug!("CURRENT_PROCESS_SP : {:#x}",CURRENT_PROCESS_SP);
        log_debug!("NEXT_PROCESS_SP : {:#x}",NEXT_PROCESS_SP);

        if NEXT_PROCESS_SP != 0 {
            // Load the next process state
            asm!(
                "ldr r2, =NEXT_PROCESS_SP",
                "ldr r0, [r2]",             // Load PSP of next process
                "ldmia r0!, {{r4-r11}}",// Restore callee-saved registers
                "msr psp, r0",              // Update process stack pointer
                "ldr r2, =CURRENT_PROCESS_SP",
                "str r0, [r2]"            // Save PSP to current process state
            );
        }

        asm!(
            "CPSIE I",  // Enable interrupts
            "isb",
            "ldr lr, =0xFFFFFFFD", // EXC_RETURN = 0xFFFFFFFD => return to Thread mode with Process stack
            "bx lr", // Return from exception
            options(noreturn)
        );
    }
}

// Pointers to the current and next process stack pointer
#[unsafe(no_mangle)]
pub static mut CURRENT_PROCESS_SP: u32 = 0;
#[unsafe(no_mangle)]
pub static mut NEXT_PROCESS_SP: u32 = 0;

pub fn trigger_pendsv() {
    const ICSR_ADDR: u32 = 0xE000ED04; // Interrupt Control State Register
    const PENDSVSET: u32 = 1 << 28; // PendSV Set-Pending bit

    let icsr = ICSR_ADDR as *mut u32;
    unsafe {
    core::ptr::write_volatile(icsr, PENDSVSET);
    }
}