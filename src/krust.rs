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

/// Krust main function called by the Reset handler
pub fn main() -> ! {

    unsafe {
        
        asm!(
            "LDR r0, =0xdeadc0de"
        );
        mpu::initialize_mpu();
        memory_management::paging::map_page(0x3000_0000, 0x1000, 0b111);
        log_debug!("coucou");
    }


    loop {}
}