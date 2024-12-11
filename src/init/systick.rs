use crate::log_debug;

const SYST_CSR_ADDR: u32 = 0xE000_E010;
const SYST_RVR_ADDR: u32 = 0xE000_E014;
const SYST_CALIB_ADDR: u32 = 0xE000_E01C;

pub struct SysTick {
    freq: u32
}

impl SysTick {
    pub fn new() -> SysTick {
        SysTick {
            freq: 0
        }
    }

    /// Initialization of SysTick : setup SYST_CSR and get TENMS in SYST_CALIB
    pub fn init_sys_tick(&mut self) {
        let syst_csr: u32;
        let syst_calib: u32;

        unsafe {
            //Set TICKINT in SysTick Control and Status Register to use SysTickHandler
            syst_csr = core::ptr::read_volatile(SYST_CSR_ADDR as *const u32);
            core::ptr::write_volatile(SYST_CSR_ADDR as *mut u32, syst_csr | 0b10);
            
            //Get TENMS in SysTick Calibration value Register
            syst_calib = core::ptr::read_volatile(SYST_CALIB_ADDR as *const u32);
            self.freq = (syst_calib & 0x00FF_FFFF) * 100;
            log_debug!("SysTick Freq (Hz) : {}",self.freq);
        }
    }

    /// Enable SysTick
    pub fn start_sys_tick(&self) {
        let syst_csr: u32;

        unsafe {
            //Set ENABLE in SysTick Control and Status Register
            syst_csr = core::ptr::read_volatile(SYST_CSR_ADDR as *const u32);
            core::ptr::write_volatile(SYST_CSR_ADDR as *mut u32, syst_csr | 0b1);
        }
    }

    /// Set reload value in SYST_RVR (use microseconds)
    pub fn set_sys_tick_reload_us(&self, us_time: u64){
        let mut syst_rvr: u32;
        let mut reload_value: u64;
        
        reload_value = (self.freq as u64 * us_time) / 1_000_000;
        // Sanitize reload_value
        reload_value &= 0x00FF_FFFF;

        unsafe {
            //Set SysTick Reload Value Register
            syst_rvr = core::ptr::read_volatile(SYST_RVR_ADDR as *const u32);
            syst_rvr &= 0xFF00_0000; //Clear RELOAD field of SYST_RVR
            core::ptr::write_volatile(SYST_RVR_ADDR as *mut u32, syst_rvr | (reload_value as u32));
        }
    }
}