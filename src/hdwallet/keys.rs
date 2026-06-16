use std::convert::TryInto;
use std::error::Error;

use crate::hdwallet::libs::base58::check_decode;

#[derive(Debug)]
pub struct ExtendedKeyError(pub String);

impl std::fmt::Display for ExtendedKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExtendedKeyError: {}", self.0)
    }
}

impl Error for ExtendedKeyError {}

pub fn serialize(
    version: &[u8],
    depth: u8,
    parent_fingerprint: &[u8],
    index: u32,
    chain_code: &[u8],
    key: &[u8],
    encoded: bool,
) -> Result<String, Box<dyn Error>> {
    if version.len() != 4 || parent_fingerprint.len() != 4 || chain_code.len() != 32 || key.len() != 33 {
        return Err(Box::new(ExtendedKeyError("Invalid key component length".into())));
    }

    let mut raw = Vec::with_capacity(78);
    raw.extend_from_slice(version);
    raw.push(depth);
    raw.extend_from_slice(parent_fingerprint);
    let mut index_bytes = [0u8; 4];
    index_bytes.copy_from_slice(&index.to_be_bytes());
    raw.extend_from_slice(&index_bytes);
    raw.extend_from_slice(chain_code);
    raw.extend_from_slice(key);

    Ok(if encoded {
        crate::hdwallet::libs::base58::check_encode(&raw, None)
    } else {
        String::from_utf8(raw)?
    })
}

pub fn deserialize(key: &str, encoded: bool) -> Result<([u8;4], u8, [u8;4], u32, [u8;32], [u8;33]), Box<dyn Error>> {
    let raw = if encoded {
        check_decode(key.as_bytes(), None).map_err(|_| {
            Box::new(ExtendedKeyError("Invalid extended(x) key".into())) as Box<dyn Error>
        })?
    } else {
        key.as_bytes().to_vec()
    };

    if raw.len() < 78 {
        return Err(Box::new(ExtendedKeyError("Invalid extended key length".into())));
    }

    let version: [u8; 4] = raw[0..4].try_into().unwrap();
    let depth: u8 = raw[4];
    let parent_fingerprint: [u8; 4] = raw[5..9].try_into().unwrap();
    let index: u32 = u32::from_be_bytes(raw[9..13].try_into().unwrap());
    let chain_code: [u8; 32] = raw[13..45].try_into().unwrap();
    let key_end = 45 + 33.min(raw.len() - 45);
    let mut key_bytes = [0u8; 33];
    key_bytes[..key_end - 45].copy_from_slice(&raw[45..key_end]);

    Ok((version, depth, parent_fingerprint, index, chain_code, key_bytes))
}

pub fn is_valid_key(key: &str, encoded: bool) -> bool {
    deserialize(key, encoded).is_ok()
}

pub fn is_root_key(key: &str, encoded: bool) -> Result<bool, Box<dyn Error>> {
    if !is_valid_key(key, encoded) {
        return Err(Box::new(ExtendedKeyError("Invalid extended(x) key".into())));
    }

    let (_, depth, parent_fingerprint, index, _, _) = deserialize(key, encoded)?;
    Ok(depth == 0 && parent_fingerprint == [0u8; 4] && index == 0)
}