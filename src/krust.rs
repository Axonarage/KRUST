//! Main logic of Krust kernel

#![no_std]
#![no_main]

mod init;
use core::arch::asm;

/// Krust main function called by the Reset handler
pub fn main() -> ! {

    unsafe {
        asm!(
            "LDR r0, =0xdeadc0de"
        );
    }

    loop {}
}