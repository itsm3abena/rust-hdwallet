use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::libs::base58::{check_decode, check_encode};

#[derive(Debug, Clone)]
pub struct ExtendedKeyParts {
    pub version: [u8; 4],
    pub depth: u8,
    pub parent_fingerprint: [u8; 4],
    pub index: u32,
    pub chain_code: [u8; 32],
    pub key: Vec<u8>,
}

fn write_u32_be(value: u32) -> [u8; 4] {
    value.to_be_bytes()
}

fn read_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn integer_to_version_bytes(version: u32) -> [u8; 4] {
    write_u32_be(version)
}

pub fn serialize_extended(
    version: u32,
    depth: u8,
    parent_fingerprint: &[u8; 4],
    index: u32,
    chain_code: &[u8; 32],
    key: &[u8],
    encoded: bool,
) -> HdResult<String> {
    let mut raw = Vec::with_capacity(45 + key.len());
    raw.extend_from_slice(&integer_to_version_bytes(version));
    raw.push(depth);
    raw.extend_from_slice(parent_fingerprint);
    raw.extend_from_slice(&write_u32_be(index));
    raw.extend_from_slice(chain_code);
    raw.extend_from_slice(key);

    if encoded {
        Ok(check_encode(&raw, None))
    } else {
        Ok(hex::encode(raw))
    }
}

pub fn deserialize_extended(key: &str, encoded: bool) -> HdResult<ExtendedKeyParts> {
    let raw = if encoded {
        check_decode(key.as_bytes(), None).map_err(|error| {
            Error::new("Invalid extended key", Some(&error.to_string()), None, None)
        })?
    } else {
        hex::decode(key.trim()).map_err(|error| {
            Error::new("Invalid extended key hex", Some(&error.to_string()), None, None)
        })?
    };

    if raw.len() < 46 {
        return Err(Error::new("Invalid extended key length", None, None, None));
    }

    Ok(ExtendedKeyParts {
        version: raw[0..4].try_into().unwrap(),
        depth: raw[4],
        parent_fingerprint: raw[5..9].try_into().unwrap(),
        index: read_u32_be(&raw[9..13]),
        chain_code: raw[13..45].try_into().unwrap(),
        key: raw[45..].to_vec(),
    })
}

pub fn is_root_extended_key(key: &str, encoded: bool) -> HdResult<bool> {
    let parts = deserialize_extended(key, encoded)?;
    Ok(parts.depth == 0 && parts.parent_fingerprint == [0u8; 4] && parts.index == 0)
}

pub fn fingerprint_from_public_key_hex(public_key_hex: &str) -> HdResult<[u8; 4]> {
    let bytes = hex::decode(public_key_hex.trim()).map_err(|error| {
        Error::new("Invalid public key hex", Some(&error.to_string()), None, None)
    })?;
    let digest = sha2::Sha256::digest(bytes);
    let hash = crate::hdwallet::libs::ripemd160::ripemd160(&digest);
    Ok(hash[0..4].try_into().unwrap())
}

pub fn cardano_xprivate_version(network: &str) -> u32 {
    match network {
        "testnet" => 0x0435_8394,
        _ => 0x0f43_31d4,
    }
}

pub fn cardano_xpublic_version(network: &str) -> u32 {
    match network {
        "testnet" => 0x0435_87cf,
        _ => 0x0488_b21e,
    }
}

pub fn algorand_xprivate_version(_network: &str) -> u32 {
    0x0f43_31d4
}

pub fn algorand_xpublic_version(_network: &str) -> u32 {
    0x0488_b21e
}

use sha2::Digest;
