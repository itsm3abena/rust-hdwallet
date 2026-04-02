use std::convert::TryInto;
use std::error::Error;

use blake2::{Blake2b, Digest};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce
};
use crc::{Crc, CRC_32_XMODEM};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use ripemd160::Ripemd160;
use sha2::{Sha256, Sha512, Sha512_256, Digest as ShaDigest};
use sha3::Sha3_256;

/// Convert input to bytes
fn to_bytes(data: impl AsRef<[u8]>) -> &[u8] {
    data.as_ref()
}

/// HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// HMAC-SHA512
fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha512>::new_from_slice(key).unwrap();
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// Blake2b with configurable output size
fn blake2b(data: &[u8], digest_size: usize) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.update(data);
    let result = hasher.finalize();
    result[..digest_size].to_vec()
}

/// SHA256
fn sha256(data: &[u8]) -> Vec<u8> {
    Sha256::digest(data).to_vec()
}

/// Double SHA256
fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first = sha256(data);
    sha256(&first)
}

/// HASH160 = RIPEMD160(SHA256(data))
fn hash160(data: &[u8]) -> Vec<u8> {
    let sha = sha256(data);
    Ripemd160::digest(&sha).to_vec()
}

/// CRC32
fn crc32(data: &[u8]) -> Vec<u8> {
    (binascii::crc32(data) as u32).to_be_bytes().to_vec()
}

/// XMODEM CRC
fn xmodem_crc(data: &[u8]) -> Vec<u8> {
    let crc = Crc::<u32>::new(&CRC_32_XMODEM);
    crc.checksum(data).to_be_bytes().to_vec()
}

/// PBKDF2-HMAC-SHA512
fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, dklen: usize) -> Vec<u8> {
    let mut out = vec![0u8; dklen];
    pbkdf2::<Hmac<Sha512>>(password, salt, iterations as usize, &mut out);
    out
}

/// Keccak256
fn keccak256(data: &[u8]) -> Vec<u8> {
    use tiny_keccak::Keccak;
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output.to_vec()
}

/// ChaCha20-Poly1305 encrypt
fn chacha20_poly1305_encrypt(key: &[u8], nonce: &[u8], assoc_data: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let ciphertext = cipher.encrypt(nonce, plaintext)?;
    Ok((ciphertext, vec![])) // tag is part of ciphertext in Rust crate
}

/// ChaCha20-Poly1305 decrypt
fn chacha20_poly1305_decrypt(key: &[u8], nonce: &[u8], assoc_data: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext)?;
    Ok(plaintext)
}

/// SHA512
fn sha512(data: &[u8]) -> Vec<u8> {
    Sha512::digest(data).to_vec()
}

/// SHA512/256
fn sha512_256(data: &[u8]) -> Vec<u8> {
    Sha512_256::digest(data).to_vec()
}

/// SHA3-256
fn sha3_256(data: &[u8]) -> Vec<u8> {
    Sha3_256::digest(data).to_vec()
}