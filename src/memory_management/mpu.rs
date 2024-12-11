/// Représente une région MPU
#[derive(Clone, Copy)]
pub struct MpuRegion {
    base_address: u32,
    size: u32,
    attributes: u32,
    number: u8,
}

/// Gestionnaire de la MPU
pub struct Mpu {
    regions: [Option<MpuRegion>; 8],
}

impl Mpu {
    /// Crée une nouvelle instance de MPU
    pub const fn new() -> Self {
        Mpu {
            regions: [None; 8],
        }
    }

    /// Configure une région MPU
    pub fn configure_region(&mut self, 
        number: u8, 
        base_address: u32, 
        _size: sizeRegion, 
        attributes: u32
    ) -> Result<(), &'static str> {
        if number >= 8 {
            return Err("Numéro de région invalide");
        }
        let size : u32 = _size as u32;
        let region = MpuRegion {
            base_address,
            size,
            attributes,
            number,
        };

        self.regions[number as usize] = Some(region);
        Ok(())
    }

    /// Active la MPU
    pub fn enable(&self) {
        unsafe {
            // Configure et active chaque région définie
            for region in self.regions.iter().flatten() {
                // Sélectionne la région
                write_mpu_rnr(region.number);
                // Configure la base et les attributs
                write_mpu_rbar(region.base_address | 1);
                write_mpu_rasr(region.attributes | ((region.size - 1) << 1) | 1);
            }

            // Active la MPU et autorise les interruptions pendant les fault handlers
            let mut ctrl = 1u32 | (1 << 2);
            ctrl |= 1 << 1; // Enable MPU during hard fault, NMI, and FAULTMASK handlers
            write_mpu_ctrl(ctrl);
        }
    }

    /// Désactive la MPU
    pub fn disable(&self) {
        unsafe {
            write_mpu_ctrl(0);
        }
    }
}

/// Configurer les registres
#[inline(always)]
unsafe fn write_mpu_rnr(region_number: u8) {
    unsafe{
        core::ptr::write_volatile(0xE000ED98 as *mut u32, region_number as u32);
        
    }
}

#[inline(always)]
unsafe fn write_mpu_ctrl(value: u32) {
    unsafe{
        core::ptr::write_volatile(0xE000ED94 as *mut u32, value);  
    }
}

#[inline(always)]
unsafe fn write_mpu_rbar(value: u32) {
    unsafe{
        core::ptr::write_volatile(0xE000ED9C as *mut u32, value);
    }
}

#[inline(always)]
fn write_mpu_rasr(value: u32) {
    unsafe {
        core::ptr::write_volatile(0xE000EDA0 as *mut u32, value);
    }  
}

// Constantes pour les attributs de région
#[allow(dead_code)]
pub const MPU_REGION_ENABLE: u32 = 1;


#[allow(non_camel_case_types, dead_code)]
pub enum sizeRegion {
    SIZE_32B = 4,
    SIZE_64B = 5,
    SIZE_128B = 6,
    SIZE_256B = 7,
    SIZE_512B = 8,
    SIZE_1KB = 9,
    SIZE_2KB = 10,
    SIZE_4KB = 11,
    SIZE_8KB = 12,
    SIZE_16KB = 13,
    SIZE_32KB = 14,
    SIZE_64KB = 15,
    SIZE_128KB = 16,
    SIZE_256KB = 17,
    SIZE_512KB = 18,
    SIZE_1MB = 19,
    SIZE_2MB = 20,
    SIZE_4MB = 21,
    SIZE_8MB = 22,
    SIZE_16MB = 23,
    SIZE_32MB = 24,
    SIZE_64MB = 25,
    SIZE_128MB = 26,
    SIZE_256MB = 27,
    SIZE_512MB = 28
}

// Constantes pour les attributs de permission

#[allow(non_camel_case_types, dead_code)] 
pub mod mpu_perm {
    pub const NO_ACCESS: u32 = 0x0 << 24;
    pub const PRIVILEGED_RW: u32 = 0x1 << 24;
    pub const PRIVILEGED_RW_UNPRIVILEGED_RO: u32 = 0x2 << 24;
    pub const FULL_ACCESS: u32 = 0x3 << 24;
} 

#[allow(non_camel_case_types, dead_code)] 
pub mod mpu_type {
    pub const TYPE_STRONGLY_ORDERED: u32 = 0x0 << 16;
    pub const TYPE_NORMAL: u32 = 0x1 << 16;
    pub const SHAREABLE: u32 = 1 << 17;
}

