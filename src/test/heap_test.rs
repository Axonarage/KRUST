use core::arch::asm;
use crate::memory_management::heap;
use crate::log_debug;

#[test_case]
#[inline(never)]
fn alloc_heap(){
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
    }
}

#[test_case]
#[inline(never)]
fn free_heap(){
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

        // Libération de la première allocation
        heap::v_port_free(ptr1);
        log_debug!("Mémoire libérée pour le premier bloc");

        // Réallocation pour vérifier le recyclage de la mémoire
        let ptr2 = heap::pv_port_malloc(16);
        if ptr2.is_null() {
            log_debug!("Réallocation de 16 octets échouée");
        } else {
            log_debug!("Réallocation réussie : adresse {:?}", ptr2);
        }
    }
}