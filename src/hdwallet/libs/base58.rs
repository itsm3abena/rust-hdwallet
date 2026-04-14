use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use thiserror::Error;

pub const DEFAULT_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub const MONERO_BLOCK_ENC_BYTE_LENS: [usize; 9] = [0, 2, 3, 5, 6, 7, 9, 10, 11];

const MONERO_BLOCK_ENC_LEN: usize = 11;
const MONERO_BLOCK_DEC_LEN: usize = 8;

#[derive(Debug, Error)]
pub enum Base58Error {
    #[error("base58 decoding checksum error")]
    Checksum,
    #[error("invalid base58 character")]
    InvalidCharacter,
    #[error("invalid UTF-8 string")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("invalid hex address")]
    InvalidAddress,
    #[error("invalid Monero base58 payload length")]
    InvalidMoneroLength,
}

pub fn string_to_uint(data: &[u8]) -> BigUint {
    BigUint::from_bytes_be(data)
}

pub fn ensure_string(data: impl AsRef<[u8]>) -> Result<String, Base58Error> {
    String::from_utf8(data.as_ref().to_vec()).map_err(Into::into)
}

pub fn encode(data: &[u8], alphabet: Option<&str>) -> String {
    let alphabet = alphabet.unwrap_or(DEFAULT_ALPHABET);
    let base = BigUint::from(alphabet.len());
    let mut val = string_to_uint(data);
    let mut out_rev: Vec<char> = Vec::new();

    while val >= base {
        let r = &val % &base;
        let idx = r.to_usize().expect("remainder fits usize");
        out_rev.push(alphabet.chars().nth(idx).expect("index in alphabet"));
        val /= &base;
    }
    if !val.is_zero() {
        let idx = val.to_usize().expect("remainder fits usize");
        out_rev.push(alphabet.chars().nth(idx).expect("index in alphabet"));
    }
    out_rev.reverse();
    let enc: String = out_rev.into_iter().collect();

    let n = data.iter().take_while(|&&b| b == 0).count();
    let lead = alphabet.chars().next().expect("non-empty alphabet");
    std::iter::repeat(lead).take(n).collect::<String>() + &enc
}

fn double_sha256_checksum_prefix(data: &[u8]) -> [u8; 4] {
    use sha2::{Digest, Sha256};
    let first = Sha256::digest(data);
    let second = Sha256::digest(first);
    second[..4].try_into().expect("4 bytes")
}

pub fn check_encode(raw: &[u8], alphabet: Option<&str>) -> String {
    let chk = double_sha256_checksum_prefix(raw);
    let mut payload = raw.to_vec();
    payload.extend_from_slice(&chk);
    encode(&payload, alphabet)
}

pub fn decode(data: &[u8], alphabet: Option<&str>) -> Result<Vec<u8>, Base58Error> {
    let alphabet = alphabet.unwrap_or(DEFAULT_ALPHABET);
    let b_len = BigUint::from(alphabet.len());
    let mut val = BigUint::zero();
    let mut prefix = 0usize;

    for &c in data {
        let idx = alphabet
            .as_bytes()
            .iter()
            .position(|&x| x == c)
            .ok_or(Base58Error::InvalidCharacter)?;
        val = &val * &b_len + BigUint::from(idx);
        if val.is_zero() {
            prefix += 1;
        }
    }

    let mut dec: Vec<u8> = Vec::new();
    let mut v = val;
    let base256 = BigUint::from(256u32);
    while v > BigUint::zero() {
        let r = &v % &base256;
        let byte = r.to_u8().expect("byte");
        dec.push(byte);
        v /= &base256;
    }
    dec.extend(std::iter::repeat(0u8).take(prefix));
    dec.reverse();
    Ok(dec)
}

pub fn check_decode(enc: &[u8], alphabet: Option<&str>) -> Result<Vec<u8>, Base58Error> {
    let dec = decode(enc, alphabet)?;
    if dec.len() < 4 {
        return Err(Base58Error::Checksum);
    }
    let split = dec.len() - 4;
    let raw = &dec[..split];
    let chk = &dec[split..];
    let expect = double_sha256_checksum_prefix(raw);
    if chk != expect.as_slice() {
        return Err(Base58Error::Checksum);
    }
    Ok(raw.to_vec())
}

pub fn checksum_encode(address: &str, crypto: &str) -> Result<String, Base58Error> {
    let addr = if crypto == "eth" {
        address.to_lowercase().replace("0x", "")
    } else {
        address.to_lowercase().replace("xdc", "")
    };
    if addr.len() != 40 || !addr.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Base58Error::InvalidAddress);
    }

    let hash = {
        use sha3::{Digest, Keccak256};
        Keccak256::digest(addr.as_bytes())
    };
    let hash_hex = hex::encode(hash);

    let mut out = String::with_capacity(addr.len());
    for (i, c) in addr.chars().enumerate() {
        let h = u8::from_str_radix(&hash_hex[i..i + 1], 16).map_err(|_| Base58Error::InvalidAddress)?;
        let c = if h >= 8 {
            c.to_ascii_uppercase()
        } else {
            c
        };
        out.push(c);
    }

    let prefixed = if crypto == "eth" {
        format!("0x{}", out)
    } else {
        format!("xdc{}", out)
    };
    Ok(prefixed)
}

pub fn pad(enc: &str, pad_len: usize, alphabet: Option<&str>) -> String {
    let alphabet = alphabet.unwrap_or(DEFAULT_ALPHABET);
    let lead = alphabet.chars().next().expect("alphabet");
    let mut s = String::with_capacity(pad_len);
    let pad = pad_len.saturating_sub(enc.len());
    s.extend(std::iter::repeat(lead).take(pad));
    s.push_str(enc);
    s
}

pub fn unpad(dec: &[u8], unpad_len: usize) -> Vec<u8> {
    if dec.len() <= unpad_len {
        dec.to_vec()
    } else {
        dec[dec.len() - unpad_len..].to_vec()
    }
}

pub fn encode_monero(data: &[u8], alphabet: Option<&str>) -> String {
    let data_len = data.len();
    let (tot_block_cnt, last_block_dec_len) = (data_len / MONERO_BLOCK_DEC_LEN, data_len % MONERO_BLOCK_DEC_LEN);
    let mut enc = String::new();

    for i in 0..tot_block_cnt {
        let chunk = &data[i * MONERO_BLOCK_DEC_LEN..(i + 1) * MONERO_BLOCK_DEC_LEN];
        let block_enc = encode(chunk, alphabet);
        enc.push_str(&pad(&block_enc, MONERO_BLOCK_ENC_LEN, alphabet));
    }

    if last_block_dec_len > 0 {
        let start = tot_block_cnt * MONERO_BLOCK_DEC_LEN;
        let chunk = &data[start..start + last_block_dec_len];
        let block_enc = encode(chunk, alphabet);
        let pad_len = MONERO_BLOCK_ENC_BYTE_LENS[last_block_dec_len];
        enc.push_str(&pad(&block_enc, pad_len, alphabet));
    }

    enc
}

pub fn decode_monero(data: &str, alphabet: Option<&str>) -> Result<Vec<u8>, Base58Error> {
    let data_len = data.len();
    let (tot_block_cnt, last_block_enc_len) = (data_len / MONERO_BLOCK_ENC_LEN, data_len % MONERO_BLOCK_ENC_LEN);

    let last_block_dec_len = MONERO_BLOCK_ENC_BYTE_LENS
        .iter()
        .position(|&l| l == last_block_enc_len)
        .ok_or(Base58Error::InvalidMoneroLength)?;

    let mut dec = Vec::new();
    for i in 0..tot_block_cnt {
        let start = i * MONERO_BLOCK_ENC_LEN;
        let chunk = data[start..start + MONERO_BLOCK_ENC_LEN].as_bytes();
        let block_dec = decode(chunk, alphabet)?;
        dec.extend_from_slice(&unpad(&block_dec, MONERO_BLOCK_DEC_LEN));
    }

    if last_block_enc_len > 0 {
        let start = tot_block_cnt * MONERO_BLOCK_ENC_LEN;
        let chunk = data[start..start + last_block_enc_len].as_bytes();
        let block_dec = decode(chunk, alphabet)?;
        dec.extend_from_slice(&unpad(&block_dec, last_block_dec_len));
    }

    Ok(dec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base58check_roundtrip() {
        let raw = b"hello world";
        let enc = check_encode(raw, None);
        let dec = check_decode(enc.as_bytes(), None).unwrap();
        assert_eq!(dec, raw);
    }

    #[test]
    fn eip55_checksum_encode() {
        let out = checksum_encode("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed", "eth").unwrap();
        assert_eq!(out, "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    }

    #[test]
    fn monero_blocks_roundtrip() {
        let data = b"abcdefghABCDEFGH";
        let enc = encode_monero(data, None);
        let dec = decode_monero(&enc, None).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn checksum_error_on_bad_base58check() {
        let mut bad = check_encode(b"test", None).into_bytes();
        if let Some(last) = bad.last_mut() {
            *last ^= 1;
        }
        assert!(matches!(
            check_decode(&bad, None),
            Err(Base58Error::Checksum)
        ));
    }
}
