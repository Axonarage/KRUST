use crate::mpu::MpuRegion;

/// Emulate paging by dynamically reconfiguring MPU regions
pub fn map_page(base_address: u32, size: u32, attributes: u32) {
    // Define the logic to re-map an MPU region dynamically
    unsafe {
        // Dynamically update MPU configuration for the new "page"
        cortex_m::peripheral::Peripherals::steal().MPU.rnr.write(0); // Example: Region 0
        cortex_m::peripheral::Peripherals::steal().MPU.rbar.write(base_address & 0xFFFFFFE0);
        cortex_m::peripheral::Peripherals::steal().MPU.rasr.write(
            (1 << 0) // Enable region
            | (attributes << 24)
            | ((size.trailing_zeros() - 1) << 1), // Encode size
        );
    }
}
