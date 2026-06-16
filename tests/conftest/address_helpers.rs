use hdwallet_rust::hdwallet::addresses::iaddress::{DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress};
use hdwallet_rust::hdwallet::consts::PublicKeyTypes;
use serde_json::Value;

use super::json_helpers::json_hex_prefix;
use super::{json_str, TestData};

pub fn assert_address_name<A: IAddress + Default>(data: &TestData, ecc_key: &str, address_key: &str) {
    let fixture = &data.addresses[ecc_key]["addresses"][address_key];
    assert_eq!(A::default().name(), json_str(&fixture["name"]));
}

pub fn assert_simple_encode_decode<A: IAddress + Default>(
    data: &TestData,
    ecc_key: &str,
    address_key: &str,
    public_key_field: &str,
) {
    let root = &data.addresses[ecc_key];
    let fixture = &root["addresses"][address_key];
    let public_key = json_str(&root[public_key_field]);
    let scheme = A::default();
    let encoded = scheme
        .encode(EncodePublicKey::Str(&public_key), &EncodeKwargs::default())
        .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));
    let decoded = scheme
        .decode(json_str(&fixture["encode"]).as_str(), &DecodeKwargs::default())
        .expect("decode");
    assert_eq!(decoded, json_str(&fixture["decode"]));
}

pub fn assert_compressed_uncompressed<A: IAddress + Default, F>(
    data: &TestData,
    ecc_key: &str,
    address_key: &str,
    encode_kwargs: F,
) where
    F: Fn(&Value, &str) -> (EncodeKwargs<'static>, DecodeKwargs),
{
    let root = &data.addresses[ecc_key];
    let fixture = &root["addresses"][address_key];
    let scheme = A::default();

    for kind in ["compressed", "uncompressed"] {
        let public_key = json_str(&root[format!("{kind}-public-key")]);
        let case = &fixture[kind];
        let (encode_kwargs, decode_kwargs) = encode_kwargs(case, kind);
        let encoded = scheme
            .encode(EncodePublicKey::Str(&public_key), &encode_kwargs)
            .expect("encode");
        assert_eq!(encoded, json_str(&case["encode"]));
        let decoded = scheme
            .decode(json_str(&case["encode"]).as_str(), &decode_kwargs)
            .expect("decode");
        assert_eq!(decoded, json_str(&case["decode"]));
    }
}

fn public_key_type(value: &str) -> &'static str {
    if value == PublicKeyTypes::UNCOMPRESSED {
        PublicKeyTypes::UNCOMPRESSED
    } else {
        PublicKeyTypes::COMPRESSED
    }
}

pub fn p2pkh_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let prefix = json_hex_prefix(json_str(&case["args"]["public_key_address_prefix"]).as_str());
    let mut encode = EncodeKwargs::default();
    encode.public_key_address_prefix = Some(prefix);
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.public_key_address_prefix = Some(prefix);
    let _ = kind;
    (encode, decode)
}

pub fn p2sh_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let prefix = json_hex_prefix(json_str(&case["args"]["script_address_prefix"]).as_str());
    let mut encode = EncodeKwargs::default();
    encode.script_address_prefix = Some(prefix);
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.script_address_prefix = Some(prefix);
    let _ = kind;
    (encode, decode)
}

pub fn p2wpkh_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let hrp = json_str(&case["args"]["hrp"]);
    let mut encode = EncodeKwargs::default();
    encode.hrp = Some(match hrp.as_str() {
        "bc" => "bc",
        "tb" => "tb",
        other => panic!("unsupported hrp {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.hrp = encode.hrp;
    let _ = kind;
    (encode, decode)
}

pub fn skip_checksum_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let skip = case["args"]["skip_checksum_encode"].as_bool().unwrap_or(false);
    let mut encode = EncodeKwargs::default();
    encode.skip_checksum_encode = Some(skip);
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.skip_checksum_encode = Some(skip);
    let _ = kind;
    (encode, decode)
}

pub fn public_key_type_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let mut encode = EncodeKwargs::default();
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let _ = kind;
    (encode, DecodeKwargs::default())
}

pub fn ripple_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let prefix = json_hex_prefix(json_str(&case["args"]["public_key_address_prefix"]).as_str());
    let mut encode = EncodeKwargs::default();
    encode.public_key_address_prefix = Some(prefix);
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.public_key_address_prefix = Some(prefix);
    let _ = kind;
    (encode, decode)
}

pub fn filecoin_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let address_type = json_str(&case["args"]["address_type"]);
    let mut encode = EncodeKwargs::default();
    encode.address_type = Some(match address_type.as_str() {
        "f1" => "f1",
        "f3" => "f3",
        "f4" => "f4",
        other => panic!("unsupported filecoin address type {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.address_type = encode.address_type;
    let _ = kind;
    (encode, decode)
}

pub fn cosmos_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let hrp = json_str(&case["args"]["hrp"]);
    let mut encode = EncodeKwargs::default();
    encode.hrp = Some(match hrp.as_str() {
        "cosmos" => "cosmos",
        "osmo" => "osmo",
        other => panic!("unsupported cosmos hrp {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.hrp = encode.hrp;
    let _ = kind;
    (encode, decode)
}

pub fn ergo_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let address_type = json_str(&case["args"]["address_type"]);
    let network_type = json_str(&case["args"]["network_type"]);
    let mut encode = EncodeKwargs::default();
    encode.address_type = Some(match address_type.as_str() {
        "P2PK" => "P2PK",
        other => panic!("unsupported ergo address type {other}"),
    });
    encode.network = Some(match network_type.as_str() {
        "mainnet" => "mainnet",
        "testnet" => "testnet",
        other => panic!("unsupported ergo network {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.address_type = encode.address_type;
    decode.network = encode.network;
    let _ = kind;
    (encode, decode)
}

pub fn p2tr_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    public_key_type_kwargs(case, kind)
}

pub fn avalanche_kwargs(case: &Value, kind: &str) -> (EncodeKwargs<'static>, DecodeKwargs) {
    let address_type = json_str(&case["args"]["address_type"]);
    let mut encode = EncodeKwargs::default();
    encode.address_type = Some(match address_type.as_str() {
        "X" => "X",
        "P" => "P",
        "C" => "C",
        other => panic!("unsupported avalanche address type {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.address_type = encode.address_type;
    let _ = kind;
    (encode, decode)
}
