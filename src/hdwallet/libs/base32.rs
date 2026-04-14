use data_encoding::BASE32;
use thiserror::Error;

pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[derive(Debug, Error)]
pub enum Base32Error {
    #[error("Invalid Base32 string")]
    Invalid,
    #[error("Invalid hex string")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("alphabet lengths must match")]
    AlphabetMismatch,
}

pub fn add_padding(data: &str) -> String {
    let last_block_width = data.len() % 8;
    if last_block_width != 0 {
        format!("{}{}", data, "=".repeat(8 - last_block_width))
    } else {
        data.to_owned()
    }
}

pub fn translate_alphabet(data: &str, from_alphabet: &str, to_alphabet: &str) -> Result<String, Base32Error> {
    if from_alphabet.len() != to_alphabet.len() {
        return Err(Base32Error::AlphabetMismatch);
    }
    let mut table: [u8; 256] = std::array::from_fn(|i| i as u8);
    for (a, b) in from_alphabet.bytes().zip(to_alphabet.bytes()) {
        table[a as usize] = b;
    }
    let out: Vec<u8> = data.bytes().map(|byte| table[byte as usize]).collect();
    String::from_utf8(out).map_err(|_| Base32Error::Invalid)
}

pub fn decode(data: &str, custom_alphabet: Option<&str>) -> Result<String, Base32Error> {
    let mut data_decode = add_padding(data);
    if let Some(custom) = custom_alphabet {
        data_decode = translate_alphabet(&data_decode, custom, ALPHABET)?;
    }
    let bytes = BASE32
        .decode(data_decode.as_bytes())
        .map_err(|_| Base32Error::Invalid)?;
    Ok(hex::encode(bytes))
}

pub fn encode(data: &str, custom_alphabet: Option<&str>) -> Result<String, Base32Error> {
    let bytes = hex::decode(data)?;
    let mut b32_encode = BASE32.encode(&bytes);
    if let Some(custom) = custom_alphabet {
        b32_encode = translate_alphabet(&b32_encode, ALPHABET, custom)?;
    }
    Ok(b32_encode)
}

pub fn encode_no_padding(data: &str, custom_alphabet: Option<&str>) -> Result<String, Base32Error> {
    Ok(encode(data, custom_alphabet)?.trim_end_matches('=').to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_standard() {
        let hex_in = "66";
        let b32 = encode(hex_in, None).unwrap();
        assert_eq!(b32, "MY======");
        assert_eq!(decode(&b32, None).unwrap(), hex_in);
    }

    #[test]
    fn encode_no_padding_matches() {
        assert_eq!(encode_no_padding("66", None).unwrap(), "MY");
    }

    #[test]
    fn custom_alphabet_roundtrip() {
        let custom = "ZYXWVUTSRQPONMLKJIHGFEDCBA234567";
        let hex_in = "deadbeef";
        let enc = encode(hex_in, Some(custom)).unwrap();
        let dec = decode(&enc, Some(custom)).unwrap();
        assert_eq!(dec, hex_in);
    }

    #[test]
    fn invalid_base32() {
        assert!(matches!(decode("!!!!", None), Err(Base32Error::Invalid)));
    }
}
