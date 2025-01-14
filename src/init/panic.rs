use core::panic::PanicInfo;
use crate::log_debug;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log_debug!("== SYSTEM PANIC ==");
    log_debug!("{}",info);
    loop {}
}