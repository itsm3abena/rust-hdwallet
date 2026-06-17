use hdwallet_rust::hdwallet::addresses::{
    CardanoAddress, MoneroAddress, ADDRESS_TYPE_STANDARD,
};
use hdwallet_rust::hdwallet::addresses::iaddress::{DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress};
use hdwallet_rust::hdwallet::consts::PublicKeyTypes;
use serde_json::Value;

use super::json_helpers::json_hex_prefix;
use super::{get_bytes, json_str, TestData};

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
        "atom" => "atom",
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
        "secp256k1" => "secp256k1",
        "bls" => "bls",
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
        "p2pkh" => "p2pkh",
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
        "p-chain" => "p-chain",
        "x-chain" => "x-chain",
        other => panic!("unsupported avalanche address type {other}"),
    });
    encode.public_key_type = Some(public_key_type(&json_str(&case["args"]["public_key_type"])));
    let mut decode = DecodeKwargs::default();
    decode.address_type = encode.address_type;
    let _ = kind;
    (encode, decode)
}

pub fn assert_cardano_address_name() {
    assert_eq!(CardanoAddress::default().name(), "Cardano");
}

pub fn assert_cardano_byron_icarus(data: &TestData) {
    let root = &data.addresses["Kholaw-Ed25519"];
    let fixture = &root["addresses"]["byron-icarus"];
    let public_key = json_str(&root["public-key"]);
    let chain_code = get_bytes(json_str(&fixture["args"]["chain_code"]).as_str());
    let scheme = CardanoAddress::default();

    let mut encode = EncodeKwargs::default();
    encode.encode_type = Some(CardanoAddress::ENCODE_BYRON_ICARUS);
    encode.chain_code = Some(&chain_code);
    let encoded = scheme
        .encode(EncodePublicKey::Str(&public_key), &encode)
        .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));

    let mut decode = DecodeKwargs::default();
    decode.decode_type = Some(CardanoAddress::ENCODE_BYRON_ICARUS);
    let decoded = scheme
        .decode(json_str(&fixture["encode"]).as_str(), &decode)
        .expect("decode");
    assert_eq!(decoded, json_str(&fixture["decode"]));
}

pub fn assert_cardano_byron_legacy(data: &TestData) {
    let root = &data.addresses["Kholaw-Ed25519"];
    let fixture = &root["addresses"]["byron-legacy"];
    let public_key = json_str(&root["public-key"]);
    let path = json_str(&fixture["args"]["path"]);
    let path_key = get_bytes(json_str(&fixture["args"]["path_key"]).as_str());
    let chain_code = get_bytes(json_str(&fixture["args"]["chain_code"]).as_str());
    let scheme = CardanoAddress::default();

    let mut encode = EncodeKwargs::default();
    encode.encode_type = Some(CardanoAddress::ENCODE_BYRON_LEGACY);
    encode.path = Some(path.as_str());
    encode.path_key = Some(&path_key);
    encode.chain_code = Some(&chain_code);
    let encoded = scheme
        .encode(EncodePublicKey::Str(&public_key), &encode)
        .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));

    let mut decode = DecodeKwargs::default();
    decode.decode_type = Some(CardanoAddress::ENCODE_BYRON_LEGACY);
    let decoded = scheme
        .decode(json_str(&fixture["encode"]).as_str(), &decode)
        .expect("decode");
    assert_eq!(decoded, json_str(&fixture["decode"]));
}

pub fn assert_cardano_shelley_payment(data: &TestData) {
    let root = &data.addresses["Kholaw-Ed25519"];
    let fixture = &root["addresses"]["shelley"];
    let public_key = json_str(&root["public-key"]);
    let staking_public_key = get_bytes(json_str(&fixture["args"]["staking_public_key"]).as_str());
    let network = json_str(&fixture["args"]["network"]);
    let scheme = CardanoAddress::default();

    let mut encode = EncodeKwargs::default();
    encode.encode_type = Some(CardanoAddress::ENCODE_PAYMENT);
    encode.staking_public_key = Some(&staking_public_key);
    encode.network = Some(match network.as_str() {
        "mainnet" => "mainnet",
        other => panic!("unsupported cardano network {other}"),
    });
    let encoded = scheme
        .encode(EncodePublicKey::Str(&public_key), &encode)
        .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));

    let mut decode = DecodeKwargs::default();
    decode.decode_type = Some(CardanoAddress::ENCODE_PAYMENT);
    decode.network = encode.network;
    let decoded = scheme
        .decode(json_str(&fixture["encode"]).as_str(), &decode)
        .expect("decode");
    assert_eq!(decoded, json_str(&fixture["decode"]));
}

pub fn assert_cardano_shelley_staking(data: &TestData) {
    let root = &data.addresses["Kholaw-Ed25519"];
    let fixture = &root["addresses"]["shelley-staking"];
    let public_key = json_str(&root["public-key"]);
    let network = json_str(&fixture["args"]["network"]);
    let scheme = CardanoAddress::default();

    let mut encode = EncodeKwargs::default();
    encode.encode_type = Some(CardanoAddress::ENCODE_STAKING);
    encode.network = Some(match network.as_str() {
        "mainnet" => "mainnet",
        other => panic!("unsupported cardano network {other}"),
    });
    let encoded = scheme
        .encode(EncodePublicKey::Str(&public_key), &encode)
        .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));

    let mut decode = DecodeKwargs::default();
    decode.decode_type = Some(CardanoAddress::ENCODE_STAKING);
    decode.network = encode.network;
    let decoded = scheme
        .decode(json_str(&fixture["encode"]).as_str(), &decode)
        .expect("decode");
    assert_eq!(decoded, json_str(&fixture["decode"]));
}

pub fn assert_monero_address(data: &TestData) {
    let fixture = &data.addresses["SLIP10-Ed25519-Monero"];
    assert_eq!(MoneroAddress::default().name(), json_str(&fixture["name"]));
    let spend_public_key = json_str(&fixture["spend-public-key"]);
    let view_public_key = json_str(&fixture["view-public-key"]);
    let payment_id = fixture["args"]["payment_id"].as_str().map(|value| get_bytes(value));
    let encoded = MoneroAddress::encode(
        hdwallet_rust::hdwallet::eccs::PublicKeyInput::Hex(&spend_public_key),
        hdwallet_rust::hdwallet::eccs::PublicKeyInput::Hex(&view_public_key),
        "mainnet",
        ADDRESS_TYPE_STANDARD,
        payment_id.as_deref(),
    )
    .expect("encode");
    assert_eq!(encoded, json_str(&fixture["encode"]));
    let (spend, view) = MoneroAddress::decode(
        json_str(&fixture["encode"]).as_str(),
        "mainnet",
        ADDRESS_TYPE_STANDARD,
        payment_id.as_deref(),
    )
    .expect("decode");
    assert_eq!(spend, spend_public_key);
    assert_eq!(view, view_public_key);
}
