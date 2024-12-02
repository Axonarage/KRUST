const SYST_CSR_ADDR: u32 = 0xE000E010;
const SYST_RVR_ADDR: u32 = 0xE000E014;

/// Enable SysTick
pub fn start_sys_tick() {
    let syst_csr: u32;

    unsafe {
        //Set ENABLE in SysTick Control and Status Register
        syst_csr = core::ptr::read_volatile(SYST_CSR_ADDR as *const u32);
        core::ptr::write_volatile(SYST_CSR_ADDR as *mut u32, syst_csr | 0b1);
    }
}

/// Initialization of SysTick : setup SYST_CSR and SYSTRVR
pub fn init_sys_tick(mut reload_value: u32){
    let mut syst_rvr: u32;
    let syst_csr: u32;

    // Sanitize reload_value
    reload_value &= 0x00FF_FFFF;

    unsafe {
        //Set SysTick Reload Value Register
        syst_rvr = core::ptr::read_volatile(SYST_RVR_ADDR as *const u32);
        syst_rvr &= 0xFF00_0000; //Clear RELOAD field of SYST_RVR
        core::ptr::write_volatile(SYST_RVR_ADDR as *mut u32, syst_rvr | reload_value);

        //Set TICKINT in SysTick Control and Status Register to use SysTickHandler
        syst_csr = core::ptr::read_volatile(SYST_CSR_ADDR as *const u32);
        core::ptr::write_volatile(SYST_CSR_ADDR as *mut u32, syst_csr | 0b10);
    }
}