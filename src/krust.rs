//! Main logic of Krust kernel

#![no_std]
#![no_main]

mod init;
mod utils;
mod memory_management;

use memory_management::mpu;
use memory_management::paging;
use memory_management::memory_layout;
use core::arch::asm;

// Inclure le module de l'allocateur
use memory_management::heap;

/// Krust main function called by the Reset handler
pub fn main() -> ! {
    
    unsafe {
        enable_system_handler_fault();
    }
    
    init::start_sys_tick();

    unsafe {
        
        asm!(
            "LDR r0, =0xdeadc0de"
        );
   // Initialisation du tas avec l'espace mémoire
   unsafe {
    heap::initialize_heap();
   }

// Test de l'allocation
unsafe {
    // Allouer un bloc de mémoire de 32 octets
    let ptr1 = heap::pv_port_malloc(32);
    if ptr1.is_null() {
        // Échec de l'allocation
        log_debug!("Allocation de 32 octets échouée");
    } else {
        log_debug!("Allocation réussie : adresse {:?}", ptr1);
    }
    *ptr1 = 0x13;
    // Allouer un autre bloc de 64 octets
    let ptr2 = heap::pv_port_malloc(64);
    if ptr2.is_null() {
        log_debug!("Allocation de 64 octets échouée");
    } else {
        log_debug!("Allocation réussie : adresse {:?}", ptr2);
    }
    *ptr2 = 0x37;

    // Libération de la première allocation
    heap::v_port_free(ptr1);
    log_debug!("Mémoire libérée pour le premier bloc");

    // Réallocation pour vérifier le recyclage de la mémoire
    let ptr3 = heap::pv_port_malloc(16);
    if ptr3.is_null() {
        log_debug!("Réallocation de 16 octets échouée");
    } else {
        log_debug!("Réallocation réussie : adresse {:?}", ptr3);
    }
}









        //mpu::initialize_mpu();
        //memory_management::paging::map_page(0x2000_0000, 0x1000, 0b001,0x0);
        //memory_management::paging::map_page(0x08000000, 0x2000, 0b000,0x1);
        /*let ptr = 0x2000_0010 as *mut u32;
        *ptr = 42;*/
    }
    //log_debug!("KRUST");

    
    
    /*
    // Trigger HardFaultHandler with Undefined instruction usage fault.
    unsafe {
        asm!("udf #0");
    }
    */
    /*
    unsafe {
        // Trigger INVPC by manually loading an invalid value into the PC
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("BX R0"); // Branch to the invalid address

        log_debug!("This line will not be reached due to INVPC fault.");
    }
    */
    // unsafe {
    //     // Attempt to access an unimplemented coprocessor
    //     trigger_lsperr();

    log_debug!("KRUST");

    loop {}
}


#[inline(never)]
unsafe fn trigger_nmi() {
    const NVIC_ICSR: *mut u32 = 0xE000ED04 as *mut u32; // Address of ICSR
    unsafe {
        core::ptr::write_volatile(NVIC_ICSR, 1 << 31); // Set NMIPENDSET bit
    }
}


#[inline(never)]
unsafe fn trigger_nocp() {
    unsafe {
        asm!(
            "MRC p15, 0, r0, c15, c0, 0", // Access a system control register via coprocessor
            options(nostack)
        );
    }
}

#[inline(never)]
unsafe fn trigger_lsperr() {
    let _ = 1.0f32 + 2.0f32; // Perform floating-point operation
    unsafe {
        asm!("LDR R0, =0xFFFFFFFF"); // Load an invalid address
        asm!("LDR R1, [R0]");        // Access invalid memory
    }
}

unsafe fn enable_system_handler_fault() {
    unsafe {
        const SHCSR_ADDR: u32 = 0xE000ED24; // Coprocessor Access Control Register
        let mut shcsr_value: u32 = core::ptr::read_volatile(SHCSR_ADDR as *const u32);

        shcsr_value |= 1 << 18; // Set the USGFAULTENA bit
        shcsr_value |= 1 << 17; // Set the BUSFAULTENA bit
        shcsr_value |= 1 << 16; // Set the MEMFAULTENA bit

        core::ptr::write_volatile(SHCSR_ADDR as *mut u32, shcsr_value);
    }
}
