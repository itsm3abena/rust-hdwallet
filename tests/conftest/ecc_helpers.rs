use hdwallet_rust::hdwallet::eccs::{IPoint, IPrivateKey, IPublicKey, IEllipticCurveCryptography};
use num_bigint::BigUint;
use num_traits::Zero;
use serde_json::Value;

use super::json_helpers::json_biguint;
use super::json_helpers::json_u32;
use super::{get_bytes, json_str, TestData};

pub fn assert_ecc_metadata<E: IEllipticCurveCryptography>(data: &TestData, key: &str) {
    let fixture = &data.eccs[key];
    assert_eq!(E::NAME, json_str(&fixture["name"]));
    assert!(!E::order().is_zero());
}

pub fn assert_point_roundtrip<P: IPoint>(fixture: &Value, public_key_type: &str) {
    assert_eq!(P::ecc_name(), json_str(&fixture["name"]));

    let point_fixture = &fixture[public_key_type]["point"];
    let expected_x = json_biguint(&point_fixture["x"]);
    let expected_y = json_biguint(&point_fixture["y"]);
    let encode = get_bytes(json_str(&point_fixture["encode"]).as_str());
    let decode = get_bytes(json_str(&point_fixture["decode"]).as_str());

    let point = P::from_bytes(&encode).expect("from_bytes");
    assert_eq!(point.x(), expected_x);
    assert_eq!(point.y(), expected_y);
    assert_eq!(point.raw_encoded(), encode);
    assert_eq!(point.raw_decoded(), decode);

    let point = P::from_coordinates(&expected_x, &expected_y).expect("from_coordinates");
    assert_eq!(point.x(), expected_x);
    assert_eq!(point.y(), expected_y);
    assert_eq!(point.raw_encoded(), encode);
    assert_eq!(point.raw_decoded(), decode);

    assert_point_arithmetic(&point);
}

pub fn assert_point_arithmetic<P: IPoint>(point: &P) {
    for number in 2..50u32 {
        let scalar = BigUint::from(number);
        let prev = point.mul_scalar(&(scalar.clone() - 1u32)).expect("mul_scalar");
        let point_add = point.add(&prev).expect("add");
        let point_radd = prev.add(point).expect("radd");
        let point_mul = point.mul_scalar(&scalar).expect("point_mul");
        let point_rmul = point.mul_scalar(&scalar).expect("rmul");

        assert_eq!(point_add.raw_encoded(), point_radd.raw_encoded());
        assert_eq!(point_add.raw_encoded(), point_mul.raw_encoded());
        assert_eq!(point_add.raw_encoded(), point_rmul.raw_encoded());
        assert_eq!(point_add.raw_decoded(), point_radd.raw_decoded());
        assert_eq!(point_add.raw_decoded(), point_mul.raw_decoded());
        assert_eq!(point_add.raw_decoded(), point_rmul.raw_decoded());
        assert_eq!(point_add.x(), point_radd.x());
        assert_eq!(point_add.x(), point_mul.x());
        assert_eq!(point_add.x(), point_rmul.x());
        assert_eq!(point_add.y(), point_radd.y());
        assert_eq!(point_add.y(), point_mul.y());
        assert_eq!(point_add.y(), point_rmul.y());
    }
}

pub fn assert_public_key_roundtrip<PK: IPublicKey>(fixture: &Value, public_key_type: &str)
where
    PK::Point: IPoint,
{
    assert_eq!(PK::ecc_name(), json_str(&fixture["name"]));
    assert_eq!(PK::uncompressed_len(), json_u32(&fixture["uncompressed"]["length"]) as usize);
    assert_eq!(PK::compressed_len(), json_u32(&fixture["compressed"]["length"]) as usize);

    let public_key = PK::from_bytes(&get_bytes(
        json_str(&fixture[public_key_type]["public-key"]).as_str(),
    ))
    .expect("from_bytes");
    let expected_uncompressed =
        get_bytes(json_str(&fixture["uncompressed"]["public-key"]).as_str());
    let expected_compressed = get_bytes(json_str(&fixture["compressed"]["public-key"]).as_str());
    assert_eq!(public_key.raw_uncompressed(), expected_uncompressed);
    assert_eq!(public_key.raw_compressed(), expected_compressed);
    let _ = public_key.point();
}

pub fn assert_private_key_bytes<SK: IPrivateKey>(fixture: &Value) {
    assert_eq!(SK::ecc_name(), json_str(&fixture["name"]));
    assert_eq!(SK::byte_len(), json_u32(&fixture["private-key-length"]) as usize);

    let expected = get_bytes(json_str(&fixture["private-key"]).as_str());
    let private_key = SK::from_bytes(&expected).expect("from_bytes");
    assert_eq!(private_key.raw_bytes().as_ref(), expected.as_slice());
}
