use hdwallet_rust::hdwallet::eccs::{
    Slip10Ed25519Ecc, Slip10Ed25519PointStd, Slip10Ed25519PrivateKeyStd, Slip10Ed25519PublicKeyStd, IPrivateKey, IPublicKey,
};

use super::conftest::{
    assert_ecc_metadata, assert_point_roundtrip, assert_private_key_bytes, assert_public_key_roundtrip,
    TestData,
};

#[test]
fn slip10_ed25519_ecc_ecc() {
    let data = TestData::load();
    assert_ecc_metadata::<Slip10Ed25519Ecc>(data, "SLIP10-Ed25519");
}

#[test]
fn slip10_ed25519_ecc_ecc_point() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<Slip10Ed25519PointStd>(fixture, public_key_type);
    }
}

#[test]
fn slip10_ed25519_ecc_ecc_public_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<Slip10Ed25519PublicKeyStd>(fixture, public_key_type);
    }
}

#[test]
fn slip10_ed25519_ecc_ecc_private_key() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Ed25519"];
    assert_private_key_bytes::<Slip10Ed25519PrivateKeyStd>(fixture);
    let private_key = Slip10Ed25519PrivateKeyStd::from_bytes(
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
