use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NestedNamespace {
    pub data: HashMap<String, NestedValue>,
}

#[derive(Debug, Clone)]
pub enum NestedValue {
    String(String),
    Nested(Box<NestedNamespace>),
}

impl NestedNamespace {
    pub fn new_from_dict(data: HashMap<String, serde_json::Value>) -> Self {
        let mut ns = NestedNamespace { data: HashMap::new() };
        for (k, v) in data {
            let value = match v {
                serde_json::Value::Object(obj) => NestedValue::Nested(Box::new(
                    NestedNamespace::new_from_dict(
                        obj.into_iter()
                            .map(|(k, v)| (k, v))
                            .collect(),
                    ),
                )),
                _ => NestedValue::String(v.to_string()),
            };
            ns.data.insert(k, value);
        }
        ns
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<NestedValue> {
        self.data.values().cloned().collect()
    }
}

pub struct SLIP10Ed25519Const;
impl SLIP10Ed25519Const {
    pub const PRIVATE_KEY_BYTE_LENGTH: usize = 32;
    pub const PUBLIC_KEY_PREFIX: u8 = 0x00;
    pub const PUBLIC_KEY_BYTE_LENGTH: usize = 32;
}

pub struct KholawEd25519Const;
impl KholawEd25519Const {
    pub const PRIVATE_KEY_BYTE_LENGTH: usize = 64;
    pub const PUBLIC_KEY_PREFIX: u8 = 0x00;
    pub const PUBLIC_KEY_BYTE_LENGTH: usize = 32;
}

pub struct SLIP10Secp256k1Const;
impl SLIP10Secp256k1Const {
    pub const USE: &'static str = "coincurve";
    pub const POINT_COORDINATE_BYTE_LENGTH: usize = 32;
    pub const PRIVATE_KEY_BYTE_LENGTH: usize = 32;
    pub const PRIVATE_KEY_UNCOMPRESSED_PREFIX: u8 = 0x00;
    pub const PRIVATE_KEY_COMPRESSED_PREFIX: u8 = 0x01;
    pub const PUBLIC_KEY_UNCOMPRESSED_PREFIX: u8 = 0x04;
    pub const PUBLIC_KEY_COMPRESSED_BYTE_LENGTH: usize = 33;
    pub const PUBLIC_KEY_UNCOMPRESSED_BYTE_LENGTH: usize = 65;
    pub const CHECKSUM_BYTE_LENGTH: usize = 4;
}

pub struct PublicKeyTypes;
impl PublicKeyTypes {
    pub const UNCOMPRESSED: &'static str = "uncompressed";
    pub const COMPRESSED: &'static str = "compressed";
    pub fn get_types() -> Vec<&'static str> {
        vec![Self::UNCOMPRESSED, Self::COMPRESSED]
    }
}

pub struct WifTypes;
impl WifTypes {
    pub const WIF: &'static str = "wif";
    pub const WIF_COMPRESSED: &'static str = "wif-compressed";
    pub fn get_types() -> Vec<&'static str> {
        vec![Self::WIF, Self::WIF_COMPRESSED]
    }
}

pub struct Semantics;
impl Semantics {
    pub const P2WPKH: &'static str = "p2wpkh";
    pub const P2WPKH_IN_P2SH: &'static str = "p2wpkh-in-p2sh";
    pub const P2WSH: &'static str = "p2wsh";
    pub const P2WSH_IN_P2SH: &'static str = "p2wsh-in-p2sh";
    pub fn get_types() -> Vec<&'static str> {
        vec![
            Self::P2WPKH,
            Self::P2WPKH_IN_P2SH,
            Self::P2WSH,
            Self::P2WSH_IN_P2SH,
        ]
    }
}

pub struct Modes;
impl Modes {
    pub const STANDARD: &'static str = "standard";
    pub const SEGWIT: &'static str = "segwit";
    pub fn get_modes() -> Vec<&'static str> {
        vec![Self::STANDARD, Self::SEGWIT]
    }
}

pub mod semantics {
    pub const P2PKH: &str = "p2pkh";
    pub const P2SH: &str = "p2sh";
    pub const P2WPKH: &str = "p2wpkh";
    pub const P2WPKH_IN_P2SH: &str = "p2wpkh-in-p2sh";
    pub const P2WSH: &str = "p2wsh";
    pub const P2WSH_IN_P2SH: &str = "p2wsh-in-p2sh";
    pub const P2TR: &str = "p2tr";
}