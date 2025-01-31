mod linked_list;
pub use linked_list::LinkedList; 

pub mod macros {
    #![macro_use]

    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)*) => {
            cortex_m_semihosting::hprintln!("{}", format_args!($($arg)*)).ok()
        };
    }

    #[macro_export]
    macro_rules! check_cookie {
        ($ptr:expr) => {
            if(!crate::memory_management::heap::check_cookie($ptr)){
                panic!("Heap corruption detected!");
            }
        }
    }
    
    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)*) => {
            cortex_m_semihosting::hprintln!("{}", format_args!($($arg)*)).ok()
        };
    }
}