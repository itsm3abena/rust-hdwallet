use std::collections::HashMap;

use hmac::{Hmac, KeyInit, Mac};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha512, Sha512_256};
use sha3::Sha3_256;

type HmacSha512 = Hmac<Sha512>;

pub fn generate_passphrase(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn get_hmac(ecc_name: &str) -> Option<&[u8]> {
    match ecc_name {
        "Kholaw-Ed25519" | "SLIP10-Ed25519" | "SLIP10-Ed25519-Blake2b" | "SLIP10-Ed25519-Monero" => Some(b"ed25519 seed"),
        "SLIP10-Nist256p1" => Some(b"Nist256p1 seed"),
        "SLIP10-Secp256k1" => Some(b"Bitcoin seed"),
        _ => None,
    }
}

fn exclude_keys_map(
    nested: &serde_json::Map<String, serde_json::Value>,
    keys: &[String],
) -> serde_json::Map<String, serde_json::Value> {
    let mut new_map = serde_json::Map::new();
    for (k, v) in nested.iter() {
        if let serde_json::Value::Object(obj) = v {
            new_map.insert(
                k.clone(),
                serde_json::Value::Object(exclude_keys_map(obj, keys)),
            );
        } else if !keys.contains(&k.replace('-', "_")) {
            new_map.insert(k.clone(), v.clone());
        }
    }
    new_map
}

pub fn exclude_keys(
    nested: &HashMap<String, serde_json::Value>,
    keys: &[String],
) -> HashMap<String, serde_json::Value> {
    let mut new_map = HashMap::new();
    for (k, v) in nested.iter() {
        if let serde_json::Value::Object(obj) = v {
            new_map.insert(
                k.clone(),
                serde_json::Value::Object(exclude_keys_map(obj, keys)),
            );
        } else if !keys.contains(&k.replace('-', "_")) {
            new_map.insert(k.clone(), v.clone());
        }
    }
    new_map
}

pub fn path_to_indexes(path: &str) -> Result<Vec<u32>, String> {
    if path == "m" || path == "m/" {
        return Ok(vec![]);
    }
    if !path.starts_with("m/") {
        return Err(format!("Bad path format: {}", path));
    }
    let mut indexes = Vec::new();
    for index in path.trim_start_matches("m/").split('/') {
        if index.ends_with('\'') {
            let val: u32 = index[..index.len() - 1].parse().map_err(|_| "Invalid index")?;
            indexes.push(val + 0x80000000);
        } else {
            let val: u32 = index.parse().map_err(|_| "Invalid index")?;
            indexes.push(val);
        }
    }
    Ok(indexes)
}

pub fn indexes_to_path(indexes: &[u32]) -> String {
    let mut path = String::from("m");
    for &index in indexes {
        if index & 0x80000000 != 0 {
            path.push_str(&format!("/{}'", index - 0x80000000));
        } else {
            path.push_str(&format!("/{}", index));
        }
    }
    path
}

pub fn xor(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    data1.iter().zip(data2.iter()).map(|(a,b)| a ^ b).collect()
}

pub fn add_no_carry(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    data1.iter().zip(data2.iter()).map(|(a,b)| a.wrapping_add(*b)).collect()
}

pub fn multiply_scalar_no_carry(data: &[u8], scalar: u8) -> Vec<u8> {
    data.iter().map(|b| b.wrapping_mul(scalar)).collect()
}

pub fn is_bits_set(value: u32, bit_num: u32) -> bool {
    value & (1 << bit_num) != 0
}

pub fn set_bit(value: u32, bit_num: u32) -> u32 {
    value | (1 << bit_num)
}

pub fn reset_bit(value: u32, bit_num: u32) -> u32 {
    value & !(1 << bit_num)
}

pub fn bytes_to_integer(data: &[u8], endianness: &str) -> u32 {
    match endianness {
        "little" => u32::from_le_bytes(data.try_into().unwrap()),
        "big" => u32::from_be_bytes(data.try_into().unwrap()),
        _ => panic!("Invalid endianness"),
    }
}

pub fn integer_to_bytes(value: u32, endianness: &str) -> Vec<u8> {
    match endianness {
        "little" => value.to_le_bytes().to_vec(),
        "big" => value.to_be_bytes().to_vec(),
        _ => panic!("Invalid endianness"),
    }
}

pub fn bytes_reverse(data: &[u8]) -> Vec<u8> {
    let mut tmp = data.to_vec();
    tmp.reverse();
    tmp
}

pub fn bytes_chunk_to_words(bytes_chunk: &[u8], words_list: &[String], endianness: &str) -> Vec<String> {
    let chunk_val = bytes_to_integer(bytes_chunk, endianness);
    let len = words_list.len() as u32;
    let w1 = chunk_val % len;
    let w2 = ((chunk_val / len) + w1) % len;
    let w3 = ((chunk_val / len / len) + w2) % len;
    vec![
        words_list[w1 as usize].clone(),
        words_list[w2 as usize].clone(),
        words_list[w3 as usize].clone(),
    ]
}

pub fn words_to_bytes_chunk(word1: &str, word2: &str, word3: &str, words_list: &[String], endianness: &str) -> Vec<u8> {
    let len = words_list.len() as u32;
    let idx_map: HashMap<String, u32> = words_list.iter().enumerate().map(|(i, w)| (w.clone(), i as u32)).collect();
    let i1 = *idx_map.get(word1).unwrap();
    let i2 = *idx_map.get(word2).unwrap();
    let i3 = *idx_map.get(word3).unwrap();
    let chunk_val = i1 + len * ((i2 + len - i1) % len) + len * len * ((i3 + len - i2) % len);
    integer_to_bytes(chunk_val, endianness)
}

// --- Mnemonic / crypto helpers (embedded wordlists, Electrum, Monero, Algorand) ---

pub fn crc32_ieee(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

pub fn sha512_256(data: &[u8]) -> [u8; 32] {
    Sha512_256::digest(data).into()
}

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    use sha3::digest::Digest;
    let mut h = Sha3_256::new();
    Digest::update(&mut h, data);
    Digest::finalize(h).into()
}

pub fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha512::new_from_slice(key).expect("HMAC key length");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key length");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn sha512(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha512};
    Sha512::digest(data).to_vec()
}

pub fn are_bits_set(value: u8, mask: u8) -> bool {
    value & mask != 0
}

pub fn chacha20_poly1305_encrypt(
    key: &[u8],
    nonce: &[u8],
    associated_data: &[u8],
    plaintext: &[u8],
) -> Result<Vec<u8>, String> {
    use chacha20poly1305::aead::{Aead, KeyInit, Payload};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    cipher
        .encrypt(
            nonce,
            Payload {
                msg: plaintext,
                aad: associated_data,
            },
        )
        .map_err(|error| error.to_string())
}

pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, dklen: usize) -> Vec<u8> {
    let mut out = vec![0u8; dklen];
    let mut offset = 0usize;
    let mut block_num = 1u32;
    while offset < dklen {
        let mut mac = HmacSha512::new_from_slice(password).expect("HMAC key length");
        mac.update(salt);
        mac.update(&block_num.to_be_bytes());
        let mut u = mac.finalize().into_bytes();
        let mut t = u;
        for _ in 1..iterations {
            let mut mac2 = HmacSha512::new_from_slice(password).expect("HMAC key length");
            mac2.update(&u);
            u = mac2.finalize().into_bytes();
            for j in 0..64 {
                t[j] ^= u[j];
            }
        }
        let len = (dklen - offset).min(64);
        out[offset..offset + len].copy_from_slice(&t[..len]);
        offset += len;
        block_num = block_num.wrapping_add(1);
    }
    out
}

pub fn bytes_to_u32_be(data: &[u8]) -> u32 {
    let mut a = [0u8; 4];
    let n = data.len().min(4);
    a[4 - n..].copy_from_slice(&data[data.len() - n..]);
    u32::from_be_bytes(a)
}

pub fn u32_to_bytes_be(value: u32) -> [u8; 4] {
    value.to_be_bytes()
}

/// Big-endian minimal-length encoding (MSB first, no leading zero bytes except single 0).
pub fn u128_to_be_bytes_min(value: u128) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }
    let mut v = value;
    let mut tmp = Vec::new();
    while v > 0 {
        tmp.push(v as u8);
        v >>= 8;
    }
    tmp.reverse();
    tmp
}

pub fn bytes_be_to_u128(data: &[u8]) -> u128 {
    assert!(data.len() <= 16, "u128 overflow");
    let mut buf = [0u8; 16];
    buf[16 - data.len()..].copy_from_slice(data);
    u128::from_be_bytes(buf)
}

/// Monero / Electrum-style: 4 bytes little-endian → 3 words from list.
pub fn bytes_chunk_to_words_le(chunk: &[u8], words_list: &[String]) -> Vec<String> {
    let chunk_val = u32::from_le_bytes(chunk.try_into().unwrap());
    let len = words_list.len() as u32;
    let w1 = chunk_val % len;
    let w2 = ((chunk_val / len) + w1) % len;
    let w3 = ((chunk_val / len / len) + w2) % len;
    vec![
        words_list[w1 as usize].clone(),
        words_list[w2 as usize].clone(),
        words_list[w3 as usize].clone(),
    ]
}

pub fn words_to_bytes_chunk_le(word1: &str, word2: &str, word3: &str, words_list: &[String]) -> Vec<u8> {
    let len = words_list.len() as u32;
    let idx: HashMap<_, _> = words_list
        .iter()
        .enumerate()
        .map(|(i, w)| (w.as_str(), i as u32))
        .collect();
    let i1 = *idx.get(word1).expect("word1");
    let i2 = *idx.get(word2).expect("word2");
    let i3 = *idx.get(word3).expect("word3");
    let chunk_val = i1 + len * ((i2 + len - i1) % len) + len * len * ((i3 + len - i2) % len);
    chunk_val.to_le_bytes().to_vec()
}

/// Python `convert_bits` parity (LSB-first accumulator).
pub fn convert_bits(data: &[u32], from_bits: u32, to_bits: u32) -> Option<Vec<u32>> {
    let max_out_val = (1u32 << to_bits) - 1;
    let from_mask = if from_bits >= 32 {
        u32::MAX
    } else {
        (1u32 << from_bits) - 1
    };
    let mut acc = 0u32;
    let mut bits = 0u32;
    let mut ret = Vec::new();
    for &value in data {
        if value > from_mask {
            return None;
        }
        acc |= value << bits;
        bits += from_bits;
        while bits >= to_bits {
            ret.push(acc & max_out_val);
            acc >>= to_bits;
            bits -= to_bits;
        }
    }
    if bits != 0 {
        ret.push(acc & max_out_val);
    }
    Some(ret)
}

/// Algorand-style 8→11 bit conversion (outputs word indices, 0..2048).
pub fn convert_bits_8_to_11(data: &[u8]) -> Option<Vec<u32>> {
    let input: Vec<u32> = data.iter().map(|&byte| u32::from(byte)).collect();
    convert_bits(&input, 8, 11)
}

/// 11→8 for Algorand decode (input: word indices).
pub fn convert_bits_11_to_8(data: &[u32]) -> Option<Vec<u8>> {
    convert_bits(data, 11, 8).map(|values| values.into_iter().map(|value| value as u8).collect())
}

pub fn parse_wordlist_text(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}

pub fn integer_to_bytes_python(value: u32, bytes_num: Option<usize>) -> Vec<u8> {
    let bytes_num = bytes_num.unwrap_or_else(|| {
        let bit_len = if value == 0 { 1 } else { 32 - value.leading_zeros() };
        ((bit_len + 7) / 8) as usize
    });
    let be = value.to_be_bytes();
    be[4 - bytes_num..].to_vec()
}

pub fn blake2b_512(data: &[u8]) -> [u8; 64] {
    use blake2::digest::{consts::U64, Digest};
    use blake2::Blake2b;
    Blake2b::<U64>::digest(data).into()
}

pub fn blake2b_224(data: &[u8]) -> [u8; 28] {
    use blake2::digest::{consts::U28, Digest};
    use blake2::Blake2b;
    Blake2b::<U28>::digest(data).into()
}

pub fn blake2b_256(data: &[u8]) -> [u8; 32] {
    use blake2::digest::{consts::U32, Digest};
    use blake2::Blake2b;
    Blake2b::<U32>::digest(data).into()
}

pub fn blake2b_32(data: &[u8]) -> [u8; 4] {
    use blake2::digest::{consts::U4, Digest};
    use blake2::Blake2b;
    Blake2b::<U4>::digest(data).into()
}

pub fn blake2b_160(data: &[u8]) -> [u8; 20] {
    use blake2::digest::{consts::U20, Digest};
    use blake2::Blake2b;
    Blake2b::<U20>::digest(data).into()
}

pub fn blake2b_40(data: &[u8]) -> [u8; 5] {
    use blake2::digest::{consts::U5, Digest};
    use blake2::Blake2b;
    Blake2b::<U5>::digest(data).into()
}

pub fn crc32_be_bytes(data: &[u8]) -> [u8; 4] {
    crc32fast::hash(data).to_be_bytes()
}

pub fn crc32_to_u32(data: &[u8]) -> u32 {
    u32::from_be_bytes(crc32_be_bytes(data))
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}