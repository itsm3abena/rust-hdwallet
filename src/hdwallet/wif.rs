use base58::{ToBase58, FromBase58};
use sha2::{Sha256, Digest};
use thiserror::Error;

pub const SLIP10_SECP256K1_PRIVATE_KEY_COMPRESSED_PREFIX: u8 = 0x01;

#[derive(Debug, Error)]
pub enum WIFError {
    #[error("Invalid private key length: expected {expected}, got {got}")]
    InvalidKeyLength { expected: usize, got: usize },
    #[error("Invalid WIF prefix: expected {expected:?}, got {got:?}")]
    InvalidPrefix { expected: Vec<u8>, got: Vec<u8> },
    #[error("Invalid WIF string")]
    InvalidWIF,
    #[error("Invalid WIF type: {got}, expected one of {expected:?}")]
    WrongType { expected: Vec<String>, got: String },
}

pub fn get_checksum(payload: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(&Sha256::digest(payload));
    hash[0..4].to_vec()
}

pub fn integer_to_bytes(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

pub fn bytes_to_string(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn get_bytes(data: impl AsRef<[u8]>) -> Vec<u8> {
    data.as_ref().to_vec()
}

pub fn encode_wif(private_key: &[u8], wif_prefix: u8) -> Result<(String, String), WIFError> {
    if private_key.len() != 32 {
        return Err(WIFError::InvalidKeyLength { expected: 32, got: private_key.len() });
    }

    let mut payload = vec![wif_prefix];
    payload.extend_from_slice(private_key);
    let checksum = get_checksum(&payload);
    let wif = [payload.clone(), checksum].concat().to_base58();

    let mut payload_compressed = vec![wif_prefix];
    payload_compressed.extend_from_slice(private_key);
    payload_compressed.push(SLIP10_SECP256K1_PRIVATE_KEY_COMPRESSED_PREFIX);
    let checksum_compressed = get_checksum(&payload_compressed);
    let wif_compressed = [payload_compressed.clone(), checksum_compressed].concat().to_base58();

    Ok((wif, wif_compressed))
}

pub fn decode_wif(wif: &str, wif_prefix: u8) -> Result<(Vec<u8>, String, Vec<u8>), WIFError> {
    let raw = wif.from_base58().map_err(|_| WIFError::InvalidWIF)?;

    if raw[0] != wif_prefix {
        return Err(WIFError::InvalidPrefix { expected: vec![wif_prefix], got: vec![raw[0]] });
    }

    let prefix_len = 1;
    let raw_without_prefix = &raw[prefix_len..];
    let checksum = &raw_without_prefix[raw_without_prefix.len() - 4..];
    let mut private_key = raw_without_prefix[..raw_without_prefix.len() - 4].to_vec();

    let mut wif_type = "wif".to_string();
    if private_key.len() == 33 && private_key[32] == SLIP10_SECP256K1_PRIVATE_KEY_COMPRESSED_PREFIX {
        private_key.pop();
        wif_type = "wif-compressed".to_string();
    }

    if private_key.len() != 32 {
        return Err(WIFError::InvalidKeyLength { expected: 32, got: private_key.len() });
    }

    Ok((private_key, wif_type, checksum.to_vec()))
}

pub fn private_key_to_wif(private_key: &[u8], wif_type: &str, wif_prefix: u8) -> Result<String, WIFError> {
    let (wif, wif_compressed) = encode_wif(private_key, wif_prefix)?;
    match wif_type {
        "wif" => Ok(wif),
        "wif-compressed" => Ok(wif_compressed),
        _ => Err(WIFError::WrongType { expected: vec!["wif".to_string(), "wif-compressed".to_string()], got: wif_type.to_string() }),
    }
}

pub fn wif_to_private_key(wif: &str, wif_prefix: u8) -> Result<String, WIFError> {
    let (private_key, _wif_type, _checksum) = decode_wif(wif, wif_prefix)?;
    Ok(bytes_to_string(&private_key))
}

pub fn get_wif_type(wif: &str, wif_prefix: u8) -> Result<String, WIFError> {
    let (_private_key, wif_type, _checksum) = decode_wif(wif, wif_prefix)?;
    Ok(wif_type)
}

pub fn get_wif_checksum(wif: &str, wif_prefix: u8) -> Result<String, WIFError> {
    let (_private_key, _wif_type, checksum) = decode_wif(wif, wif_prefix)?;
    Ok(bytes_to_string(&checksum))
}