use rust_hdwallet::hdwallet::eccs::{
    Slip10Nist256p1Ecc, Slip10Nist256p1Point, Slip10Nist256p1PrivateKey, Slip10Nist256p1PublicKey, IPrivateKey, IPublicKey,
};

use super::conftest::{
    assert_ecc_metadata, assert_point_roundtrip, assert_private_key_bytes, assert_public_key_roundtrip,
    TestData,
};

#[test]
fn slip10_nist256p1_ecc_ecc() {
    let data = TestData::load();
    assert_ecc_metadata::<Slip10Nist256p1Ecc>(data, "SLIP10-Nist256p1");
}

#[test]
fn slip10_nist256p1_ecc_ecc_point() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Nist256p1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<Slip10Nist256p1Point>(fixture, public_key_type);
    }
}

#[test]
fn slip10_nist256p1_ecc_ecc_public_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Nist256p1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<Slip10Nist256p1PublicKey>(fixture, public_key_type);
    }
}

#[test]
fn slip10_nist256p1_ecc_ecc_private_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Nist256p1"];
    assert_private_key_bytes::<Slip10Nist256p1PrivateKey>(fixture);
    let private_key = Slip10Nist256p1PrivateKey::from_bytes(
        &super::conftest::get_bytes(&super::conftest::json_str(&fixture["private-key"])),
    )
    .expect("from_bytes");
    let public_key = private_key.public_key();
    assert_eq!(
        public_key.raw_uncompressed(),
        super::conftest::get_bytes(&super::conftest::json_str(&fixture["uncompressed"]["public-key"]))
    );
    assert_eq!(
        public_key.raw_compressed(),
        super::conftest::get_bytes(&super::conftest::json_str(&fixture["compressed"]["public-key"]))
    );
}
