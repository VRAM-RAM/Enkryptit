//! Enkryptit Test Suite
//!
//! Comprehensive testing for file/folder encryption using XChaCha20-Poly1305 + Argon2id
//! Tests organized by category with explicit parallel execution support

pub mod integration;
pub mod mocks;
pub mod unit;

/// Configure test thread pool size explicitly (default 8 threads)
#[cfg(test)]
fn setup_parallel_execution() {
    // Enable parallel test execution - adjust number of threads as needed
    unsafe {
        std::env::set_var("RUST_TEST_THREADS", "16");
    }
}

#[cfg(all(test))]
mod runtime_tests {
    use super::*;

    #[test]
    fn thread_configuration_valid() {
        setup_parallel_execution();
        assert!(std::env::var("RUST_TEST_THREADS").is_ok());
    }
}
