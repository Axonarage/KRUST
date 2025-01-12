#![allow(unused_imports)]
use crate::log_debug;
//mod scheduler;
//mod exception_test;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {

    for (i,test) in tests.into_iter().enumerate() {
        log_debug!("Run test {}",i);
        test();
    }
}
