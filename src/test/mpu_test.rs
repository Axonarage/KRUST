use core::any::Any;
use core::arch::asm;
use crate::memory_management::mpu;
use crate::log_debug;

#[test_case]
#[inline(never)]
pub fn createRegion() {
    let mut mpu = mpu::Mpu::new();

    // Région 0 : FULL_ACCESS 
    let full_access_addr: u32 = 0x20020000; // Début de la SRAM
    let attributes_full_access = mpu::MPU_REGION_ENABLE |
                                mpu::mpu_perm::FULL_ACCESS |
                                mpu::mpu_type::TYPE_NORMAL |
                                mpu::mpu_type::SHAREABLE;

    match mpu.configure_region(0, full_access_addr, mpu::sizeRegion::SIZE_1KB, attributes_full_access) {
        Ok(_) => log_debug!("Région 0 (FULL_ACCESS) configurée"),
        Err(e) => log_debug!("Erreur config région 2: {}", e),
    }.expect("REASON");

    // Région 1 : NO_ACCESS
    let no_access_addr: u32 = 0x20010000; // Début de la SRAM
    let attributes_no_access = mpu::MPU_REGION_ENABLE |
                                mpu::mpu_perm::NO_ACCESS |
                                mpu::mpu_type::TYPE_NORMAL |
                                mpu::mpu_type::SHAREABLE;

    match mpu.configure_region(1, no_access_addr, mpu::sizeRegion::SIZE_1KB, attributes_no_access) {
        Ok(_) => log_debug!("Région 1 (NO_ACCESS) configurée"),
        Err(e) => log_debug!("Erreur config région 2: {}", e),
    }.expect("REASON");


    // Active la MPU
    mpu.enable();

    // Test : write in FULL_ACCESS region
    log_debug!("Test: Tentative d'accès à la région FULL_ACCESS");
    unsafe {
        let ptr = full_access_addr as *mut u32;
        log_debug!("Tentative d'écriture à l'adresse {:p}", ptr);
        *ptr = 42; // Devrait réussir
        let val = *ptr;
        log_debug!("Accès FULL_ACCESS réussi, valeur lue: {}", val);
    }
/*
    // Test : write in NO_ACCESS region 
    log_debug!("Test: Tentative d'accès à la région NO_ACCESS");
    unsafe {
        let ptr = no_access_addr as *mut u32;
        log_debug!("Tentative d'écriture à l'adresse {:p}", ptr);
        *ptr = 42; // Fault 
        let val = *ptr;
        log_debug!("Accès FULL_ACCESS réussi, valeur lue: {}", val);
    }*/
}