use super::conftest::{assert_address_name, assert_simple_encode_decode, TestData};
use hdwallet_rust::hdwallet::addresses::NeoAddress;

#[test]
fn test_neo_address() {
    let data = TestData::load();
    assert_address_name::<NeoAddress>(data, "SLIP10-Nist256p1", "Neo");
    assert_simple_encode_decode::<NeoAddress>(data, "SLIP10-Nist256p1", "Neo", "public-key");
}
