use num_bigint::BigUint;
use serde_json::Value;

use super::{json_str, json_usize};

pub fn json_kebab_str(value: &Value, key: &str) -> String {
    json_str(&value[key])
}

pub fn json_biguint(value: &Value) -> BigUint {
    if let Some(s) = value.as_str() {
        return s.parse::<BigUint>().unwrap_or_else(|error| {
            panic!("expected numeric JSON value, got {value}: {error}")
        });
    }
    if let Some(n) = value.as_u64() {
        return BigUint::from(n);
    }
    if let Some(number) = value.as_number() {
        if let Some(n) = number.as_u64() {
            return BigUint::from(n);
        }
        return number.as_str().parse::<BigUint>().unwrap_or_else(|error| {
            panic!("expected numeric JSON value, got {value}: {error}")
        });
    }
    panic!("expected numeric JSON value, got {value}")
}

pub fn json_u32(value: &Value) -> u32 {
    json_usize(value) as u32
}

pub fn json_hex_prefix(value: &str) -> u32 {
    u32::from_str_radix(value.trim_start_matches("0x"), 16)
        .unwrap_or_else(|error| panic!("invalid hex prefix {value}: {error}"))
}

pub fn json_indexes(value: &Value) -> Vec<u32> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("expected JSON array, got {value}"))
        .iter()
        .map(json_u32)
        .collect()
}
