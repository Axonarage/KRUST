use crate::mpu::MpuRegion;

/// Emulate paging by dynamically reconfiguring MPU regions
pub fn map_page(base_address: u32, size: u32, attributes: u32, region : u32) {
    // Define the logic to re-map an MPU region dynamically
    unsafe {
        // Dynamically update MPU configuration for the new "page"
        cortex_m::peripheral::Peripherals::steal().MPU.rnr.write(region); // Registre indiquant la région ou écrire 
        cortex_m::peripheral::Peripherals::steal().MPU.rbar.write(base_address & 0xFFFFFFE0); // RBAR = @ de base de la région
        cortex_m::peripheral::Peripherals::steal().MPU.rasr.write(  // RASR = Region attribute and size register
            (1 << 0) // enable la zone
            | (attributes << 24) // Permissions sur la région
            | ((size.trailing_zeros() - 1) << 1), // défini taille de la région
        );
    }
}
