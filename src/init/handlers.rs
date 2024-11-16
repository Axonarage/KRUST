use core::sync::atomic::{compiler_fence, Ordering};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler() -> ! {
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}