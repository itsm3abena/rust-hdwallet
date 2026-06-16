use hdwallet_rust::hdwallet::eccs::{
    KholawEd25519Ecc, KholawEd25519Point, KholawEd25519PrivateKey, KholawEd25519PublicKey, IPrivateKey, IPublicKey,
};

use super::conftest::{
    assert_ecc_metadata, assert_point_roundtrip, assert_private_key_bytes, assert_public_key_roundtrip,
    TestData,
};

#[test]
fn kholaw_ed25519_ecc_ecc() {
    let data = TestData::load();
    assert_ecc_metadata::<KholawEd25519Ecc>(data, "Kholaw-Ed25519");
}

#[test]
fn kholaw_ed25519_ecc_ecc_point() {
    let data = TestData::load();
    let fixture = &data.eccs["Kholaw-Ed25519"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<KholawEd25519Point>(fixture, public_key_type);
    }
}

#[test]
fn kholaw_ed25519_ecc_ecc_public_key() {
    let data = TestData::load();
    let fixture = &data.eccs["Kholaw-Ed25519"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<KholawEd25519PublicKey>(fixture, public_key_type);
    }
}

#[test]
fn kholaw_ed25519_ecc_ecc_private_key() {
    let data = TestData::load();
    let fixture = &data.eccs["Kholaw-Ed25519"];
    assert_private_key_bytes::<KholawEd25519PrivateKey>(fixture);
    let private_key = KholawEd25519PrivateKey::from_bytes(
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
