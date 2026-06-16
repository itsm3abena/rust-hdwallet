use hdwallet_rust::hdwallet::eccs::{
    Slip10Secp256k1Ecc, Slip10Secp256k1Point, Slip10Secp256k1PrivateKey, Slip10Secp256k1PublicKey,
    IPrivateKey, IPublicKey,
};

use super::conftest::{
    assert_ecc_metadata, assert_point_roundtrip, assert_private_key_bytes, assert_public_key_roundtrip,
    get_bytes, json_str, TestData,
};

fn assert_secp256k1_private_key_roundtrip(fixture: &serde_json::Value) {
    assert_private_key_bytes::<Slip10Secp256k1PrivateKey>(fixture);
    let private_key = Slip10Secp256k1PrivateKey::from_bytes(
        &get_bytes(&json_str(&fixture["private-key"])),
    )
    .expect("from_bytes");
    let public_key = private_key.public_key();
    assert_eq!(
        public_key.raw_uncompressed(),
        get_bytes(&json_str(&fixture["uncompressed"]["public-key"]))
    );
    assert_eq!(
        public_key.raw_compressed(),
        get_bytes(&json_str(&fixture["compressed"]["public-key"]))
    );
}

#[test]
fn slip10_secp256k1_ecc_coincurve() {
    let data = TestData::load();
    assert_ecc_metadata::<Slip10Secp256k1Ecc>(data, "SLIP10-Secp256k1");
}

#[test]
fn slip10_secp256k1_ecc_point_coincurve() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Secp256k1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<Slip10Secp256k1Point>(fixture, public_key_type);
    }
}

#[test]
fn slip10_secp256k1_ecc_public_key_coincurve() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Secp256k1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<Slip10Secp256k1PublicKey>(fixture, public_key_type);
    }
}

#[test]
fn slip10_secp256k1_ecc_private_key_coincurve() {
    let data = TestData::load();
    assert_secp256k1_private_key_roundtrip(&data.eccs["SLIP10-Secp256k1"]);
}

#[test]
fn slip10_secp256k1_ecc_ecdsa() {
    let data = TestData::load();
    assert_ecc_metadata::<Slip10Secp256k1Ecc>(data, "SLIP10-Secp256k1");
}

#[test]
fn slip10_secp256k1_ecc_point_ecdsa() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Secp256k1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_point_roundtrip::<Slip10Secp256k1Point>(fixture, public_key_type);
    }
}

#[test]
fn slip10_secp256k1_ecc_public_key_ecdsa() {
    let data = TestData::load();
    let fixture = &data.eccs["SLIP10-Secp256k1"];
    for public_key_type in ["uncompressed", "compressed"] {
        assert_public_key_roundtrip::<Slip10Secp256k1PublicKey>(fixture, public_key_type);
    }
}

#[test]
fn slip10_secp256k1_ecc_private_key() {
    let data = TestData::load();
    assert_secp256k1_private_key_roundtrip(&data.eccs["SLIP10-Secp256k1"]);
}
