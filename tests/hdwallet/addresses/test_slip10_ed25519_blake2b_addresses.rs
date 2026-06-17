use super::conftest::{assert_address_name, assert_simple_encode_decode, TestData};
use hdwallet_rust::hdwallet::addresses::NanoAddress;

#[test]
fn test_nano_address() {
    let data = TestData::load();
    assert_address_name::<NanoAddress>(data, "SLIP10-Ed25519-Blake2b", "Nano");
    assert_simple_encode_decode::<NanoAddress>(data, "SLIP10-Ed25519-Blake2b", "Nano", "public-key");
}
