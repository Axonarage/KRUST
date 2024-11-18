// Gère l'initialisation de la MPU et la configuration des régions
 
use cortex_m::peripheral::MPU;

use crate::memory_management::memory_layout;

// Represents an MPU region's configuration
pub struct MpuRegion {
    pub base_address: u32,
    pub size: u32, 
    pub attributes: u32,
}

#[inline(always)]
pub unsafe fn initialize_mpu() {

    let peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    let mut mpu = peripherals.MPU;

    // Configure FLASH
    configure_region(
        &mut mpu,
        0,
        MpuRegion {
            base_address: memory_layout::REGION_FLASH_ADDR,
            size: memory_layout::REGION_FLASH_LEN,
            attributes: memory_layout::ATTR_READ_EXEC,
        },
    );

    // Configure RAM (heap, bss, ...)
    configure_region(
        &mut mpu,
        1,
        MpuRegion {
            base_address: memory_layout::REGION_RAM_ADDR,
            size: memory_layout::REGION_RAM_LEN,
            attributes: memory_layout::ATTR_READ_WRITE,
        },
    );
    
    // Configure CCRAM (stack)
    configure_region(
        &mut mpu,
        2,
        MpuRegion {
            base_address: memory_layout::REGION_CCRAM_ADDR,
            size: memory_layout::REGION_CCRAM_LEN,
            attributes: memory_layout::ATTR_READ_WRITE,
        },
    );
    enable_mpu(&mut mpu);
    
}

unsafe fn configure_region(mpu: &mut MPU, region_index: usize, region: MpuRegion) {
    mpu.rnr.write(region_index as u32); // Select region
    mpu.rbar.write(region.base_address & 0xFFFFFFE0); // Base address
    mpu.rasr.write(
        (1 << 0)                     // Enable region
        | (region.attributes << 24)  // Attributes
        | ((region.size.trailing_zeros() - 1) << 1), // Size encoding
    );
}

unsafe fn enable_mpu(mpu: &mut MPU) {
    mpu.ctrl.write(1 | (1 << 2)); // Enable MPU with default memory map
}
