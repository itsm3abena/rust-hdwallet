//! Rust equivalent of python-hdwallet `tests/conftest.py`.
#![allow(dead_code, unused_imports)]

#[allow(dead_code, unused_imports)]
mod assert;
#[allow(dead_code, unused_imports)]
mod address_helpers;
#[allow(dead_code, unused_imports)]
mod data;
#[allow(dead_code, unused_imports)]
mod derivation_helpers;
#[allow(dead_code, unused_imports)]
mod ecc_helpers;
#[allow(dead_code, unused_imports)]
mod hd_helpers;
#[allow(dead_code, unused_imports)]
mod hdwallet;
#[allow(dead_code, unused_imports)]
mod json_helpers;
#[allow(dead_code, unused_imports)]
mod test_error;

pub use assert::*;
pub use address_helpers::*;
pub use data::TestData;
pub use derivation_helpers::*;
pub use ecc_helpers::*;
pub use hd_helpers::*;
pub use test_error::TestError;
pub use hdwallet::*;
pub use json_helpers::*;

/// Mirrors pytest `cli_tester` fixture (`CliRunner`).
pub fn cli_tester(args: &[&str]) -> Result<String, i32> {
    rust_hdwallet::hdwallet::cli::run_from(args)
}

pub fn get_bytes(hex: &str) -> Vec<u8> {
    rust_hdwallet::hdwallet::utils::hex_to_bytes(hex).expect("valid hex")
}
