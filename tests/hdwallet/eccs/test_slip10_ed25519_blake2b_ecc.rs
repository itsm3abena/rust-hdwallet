use rust_hdwallet::hdwallet::eccs::{
    Slip10Ed25519Blake2bEcc, Slip10Ed25519Blake2bPoint, Slip10Ed25519Blake2bPrivateKey, Slip10Ed25519Blake2bPublicKey, IPrivateKey, IPublicKey,
};

use super::conftest::{
    assert_ecc_metadata, assert_point_roundtrip, assert_private_key_bytes, assert_public_key_roundtrip,
    TestData,
};

#[test]
fn slip10_ed25519_blake2b_ecc_ecc() {
    let data = TestData::load();
    assert_ecc_metadata::<Slip10Ed25519Blake2bEcc>(data, "SLIP10-Ed25519-Blake2b");
}

#[test]
fn slip10_ed25519_blake2b_ecc_ecc_point() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519-Blake2b"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<Slip10Ed25519Blake2bPoint>(fixture, public_key_type);
    }
}

#[test]
fn slip10_ed25519_blake2b_ecc_ecc_public_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519-Blake2b"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<Slip10Ed25519Blake2bPublicKey>(fixture, public_key_type);
    }
}

#[test]
fn slip10_ed25519_blake2b_ecc_ecc_private_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519-Blake2b"];
    assert_private_key_bytes::<Slip10Ed25519Blake2bPrivateKey>(fixture);
    let private_key = Slip10Ed25519Blake2bPrivateKey::from_bytes(
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
