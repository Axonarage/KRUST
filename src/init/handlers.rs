use core::sync::atomic::{compiler_fence, Ordering};
use cortex_m_semihosting::hprintln;


#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler() -> ! {
    hprintln!("Default Handler").ok();
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}


macro_rules! debug_log {
    ($($arg:tt)*) => {
        hprintln!("{}", format_args!($($arg)*)).ok();
    };
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
        debug_log!("Debug is used.");
    }
    if forced == 1 {
        // inspect other fault status registers
        debug_log!("Forced hard fault. Need to inspect the other fault status registers.");

        unsafe {
            FaultHandler();
        }
    }
    if vecttbl == 1 {
        debug_log!("Bus fault while trying to read the vector table.");
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
unsafe fn FaultHandler() -> ! {
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
        debug_log!("Usage Fault.");
        
        if (cfsr_value >> DIVBYZERO_BIT) & 1 == 1 {
            debug_log!("Divide by zero usage fault.");
        } else if (cfsr_value >> UNALIGNED_BIT) & 1 == 1 {
            debug_log!("Unaligned access usage fault.");
        } else if (cfsr_value >> NOCP_BIT) & 1 == 1 {
            debug_log!("No coprocessor usage fault.");
        } else if (cfsr_value >> INVPC_BIT) & 1 == 1 {
            debug_log!("Invalid PC load usage fault, caused by an invalid PC load by EXC_RETURN.");
        } else if (cfsr_value >> INVSTATE_BIT) & 1 == 1 {
            debug_log!("Invalid state usage fault.");
        } else if (cfsr_value >> UNDEFINSTR_BIT) & 1 == 1 {
            debug_log!("Undefined instruction usage fault.");
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
        debug_log!("Bus Fault.");
        
        if (cfsr_value >> LSPEERR_BIT) & 1 == 1 {
            debug_log!("Bus fault on floating-point lazy state preservation.");
        } else if (cfsr_value >> STKERR_BIT) & 1 == 1 {
            debug_log!("Bus fault on stacking for exception entry.");
        } else if (cfsr_value >> UNSTKERR_BIT) & 1 == 1 {
            debug_log!("Bus fault on unstacking for a return from exception.");
        } else if (cfsr_value >> IMPRECISERR_BIT) & 1 == 1 {
            debug_log!("Imprecise data bus error.");
        } else if (cfsr_value >> PRECISERR_BIT) & 1 == 1 {
            debug_log!("Precise data bus error.");
        } else if (cfsr_value >> IBUERR_BIT) & 1 == 1 {
            debug_log!("Instruction bus error.");
        } 
    }

    // Bus Fault Address Register (BFAR) valid flag.
    if (cfsr_value >> BFARVALID_BIT) & 1 == 1 {
        unsafe {
            bfar_value = GetFaultAddress(BFAR_ADDR);
        }
        debug_log!("Fault at address {:#X}", bfar_value);
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
        debug_log!("Memory Management Fault.");
        
        if (cfsr_value >> MLSPEERR_BIT) & 1 == 1 {
            debug_log!("MemManage fault occurred during floating-point lazy state preservation.");
        } else if (cfsr_value >> MSTKERR_BIT) & 1 == 1 {
            debug_log!("Memory manager fault on stacking for exception entry.");
        } else if (cfsr_value >> MUNSTKERR_BIT) & 1 == 1 {
            debug_log!("Memory manager fault on unstacking for a return from exception.");
        } else if (cfsr_value >> DACCVIOL_BIT) & 1 == 1 {
            debug_log!("Data access violation flag.");
        } else if (cfsr_value >> IACCVIOL_BIT) & 1 == 1 {
            debug_log!("Instruction access violation flag.");
        }
    }

    // Memory Management Fault Address Register (MMAR) valid flag.
    if (cfsr_value >> MMARVALID_BIT) & 1 == 1 {
        unsafe {
            mmfar_value = GetFaultAddress(MMFAR_ADDR);
        }
        debug_log!("Fault at address {:#X}", mmfar_value);
    }

    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[allow(non_snake_case)]
unsafe fn GetFaultAddress(ADDR: u32) -> u32 {
    let addr_value: u32;

    unsafe {
        addr_value = core::ptr::read_volatile(ADDR as *const u32);
    }

    addr_value
}
