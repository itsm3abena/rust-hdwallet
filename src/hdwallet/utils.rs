use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::collections::HashMap;

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

pub fn exclude_keys(nested: &HashMap<String, serde_json::Value>, keys: &Vec<String>) -> HashMap<String, serde_json::Value> {
    let mut new_map = HashMap::new();
    for (k, v) in nested.iter() {
        if let serde_json::Value::Object(obj) = v {
            new_map.insert(k.clone(), serde_json::Value::Object(exclude_keys(obj, keys)));
        } else if !keys.contains(&k.replace("-", "_")) {
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