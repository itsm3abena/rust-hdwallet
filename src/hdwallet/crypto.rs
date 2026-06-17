use crc::{Crc, CRC_16_XMODEM};
use sha2::{Digest, Sha256};
use sha3::Keccak256;

use crate::hdwallet::libs::ripemd160::ripemd160 as ripemd160_hash;
use crate::hdwallet::utils;

pub fn hmac_sha256(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::hmac_sha256(key.as_ref(), data.as_ref())
}

pub fn hmac_sha512(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::hmac_sha512(key.as_ref(), data.as_ref())
}

pub fn blake2b(data: impl AsRef<[u8]>, digest_size: usize) -> Vec<u8> {
    let data = data.as_ref();
    match digest_size {
        4 => blake2b_32(data).to_vec(),
        5 => blake2b_40(data).to_vec(),
        20 => blake2b_160(data).to_vec(),
        28 => blake2b_224(data).to_vec(),
        32 => blake2b_256(data).to_vec(),
        64 => blake2b_512(data).to_vec(),
        other => {
            use blake2::digest::{Update, VariableOutput};
            use blake2::Blake2bVar;
            let mut hasher = Blake2bVar::new(other).expect("valid BLAKE2b digest size");
            hasher.update(data);
            let mut out = vec![0u8; other];
            hasher
                .finalize_variable(&mut out)
                .expect("BLAKE2b finalize");
            out
        }
    }
}

pub fn blake2b_32(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_32(data.as_ref()).to_vec()
}

pub fn blake2b_40(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_40(data.as_ref()).to_vec()
}

pub fn blake2b_160(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_160(data.as_ref()).to_vec()
}

pub fn blake2b_224(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_224(data.as_ref()).to_vec()
}

pub fn blake2b_256(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_256(data.as_ref()).to_vec()
}

pub fn blake2b_512(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::blake2b_512(data.as_ref()).to_vec()
}

pub fn chacha20_poly1305_encrypt(
    key: impl AsRef<[u8]>,
    nonce: impl AsRef<[u8]>,
    assoc_data: impl AsRef<[u8]>,
    plain_text: impl AsRef<[u8]>,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let encrypted = utils::chacha20_poly1305_encrypt(
        key.as_ref(),
        nonce.as_ref(),
        assoc_data.as_ref(),
        plain_text.as_ref(),
    )?;
    let tag_len = 16;
    let split = encrypted
        .len()
        .checked_sub(tag_len)
        .ok_or_else(|| "invalid ChaCha20-Poly1305 output".to_string())?;
    Ok((
        encrypted[..split].to_vec(),
        encrypted[split..].to_vec(),
    ))
}

pub fn chacha20_poly1305_decrypt(
    key: impl AsRef<[u8]>,
    nonce: impl AsRef<[u8]>,
    assoc_data: impl AsRef<[u8]>,
    cipher_text: impl AsRef<[u8]>,
    tag: impl AsRef<[u8]>,
) -> Result<Vec<u8>, String> {
    use chacha20poly1305::aead::{Aead, KeyInit, Payload};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let nonce = Nonce::from_slice(nonce.as_ref());
    let mut combined = cipher_text.as_ref().to_vec();
    combined.extend_from_slice(tag.as_ref());
    cipher
        .decrypt(
            nonce,
            Payload {
                msg: &combined,
                aad: assoc_data.as_ref(),
            },
        )
        .map_err(|error| error.to_string())
}

pub fn sha256(data: impl AsRef<[u8]>) -> Vec<u8> {
    Sha256::digest(data.as_ref()).to_vec()
}

pub fn double_sha256(data: impl AsRef<[u8]>) -> Vec<u8> {
    sha256(sha256(data))
}

pub fn hash160(data: impl AsRef<[u8]>) -> Vec<u8> {
    ripemd160_hash(&sha256(data)).to_vec()
}

pub fn crc32(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::crc32_be_bytes(data.as_ref()).to_vec()
}

pub fn xmodem_crc(data: impl AsRef<[u8]>) -> Vec<u8> {
    let crc = Crc::<u16>::new(&CRC_16_XMODEM);
    crc.checksum(data.as_ref()).to_be_bytes().to_vec()
}

pub fn pbkdf2_hmac_sha512(
    password: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    iteration_num: u32,
    derived_key_length: Option<usize>,
) -> Vec<u8> {
    utils::pbkdf2_hmac_sha512(
        password.as_ref(),
        salt.as_ref(),
        iteration_num,
        derived_key_length.unwrap_or(64),
    )
}

pub fn kekkak256(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha3::Digest;
    Keccak256::digest(data.as_ref()).to_vec()
}

pub fn ripemd160(data: impl AsRef<[u8]>) -> Vec<u8> {
    ripemd160_hash(data.as_ref()).to_vec()
}

pub fn sha512(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::sha512(data.as_ref())
}

pub fn sha512_256(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::sha512_256(data.as_ref()).to_vec()
}

pub fn sha3_256(data: impl AsRef<[u8]>) -> Vec<u8> {
    utils::sha3_256(data.as_ref()).to_vec()
}
