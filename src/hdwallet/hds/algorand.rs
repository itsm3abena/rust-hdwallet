//! Algorand hierarchical deterministic keys (mirrors `hdwallet/hds/algorand.py`).
#![allow(non_snake_case)] // kL/kR/zL mirror Python / spec notation

use hmac::{Hmac, KeyInit, Mac};
use num_bigint::BigUint;
use num_traits::One;
use sha2::{Digest, Sha256, Sha512};

use crate::hdwallet::exceptions::{Error, Result, SeedError};
use crate::hdwallet::libs::base32;
use crate::hdwallet::libs::ed25519::{
    point_add, point_scalar_mul_base, scalar_reduce_from_int, PointRef, COORD_BYTE_LEN,
};
use crate::hdwallet::libs::ripemd160::ripemd160;
use crate::hdwallet::seeds::iseed::ISeed;
use crate::hdwallet::utils::sha512_256;

use super::ihd::Ihd;

type HmacSha512 = Hmac<Sha512>;

const PEIKERT_G: usize = 9;

#[inline]
fn two_pow_255() -> BigUint {
    BigUint::one() << 255usize
}

#[inline]
fn two_pow_256() -> BigUint {
    BigUint::one() << 256usize
}

fn hmac_sha512_key_msg(key: &[u8], msg: &[u8]) -> [u8; 64] {
    let mut mac = HmacSha512::new_from_slice(key).expect("HMAC key length");
    mac.update(msg);
    let out = mac.finalize().into_bytes();
    out.as_slice().try_into().expect("HMAC-SHA512 is 64 bytes")
}

fn clamp_kl(kl: &mut [u8; 32]) {
    kl[0] &= 0b1111_1000;
    kl[31] &= 0b0111_1111;
    kl[31] |= 0b0100_0000;
}

fn trunc_256_minus_g_bits(buf: &[u8; 32], g: usize) -> [u8; 32] {
    assert!(g <= 256);
    let mut out = *buf;
    let mut remaining = g;
    for i in (0..32).rev() {
        if remaining >= 8 {
            out[i] = 0;
            remaining -= 8;
        } else if remaining > 0 {
            out[i] &= 0xFFu8 >> remaining;
            break;
        } else {
            break;
        }
    }
    out
}

fn le_bytes_to_uint256(b: &[u8; 32]) -> BigUint {
    BigUint::from_bytes_le(b.as_ref())
}

fn uint256_to_le_32(x: &BigUint) -> [u8; 32] {
    let m = two_pow_256();
    let v = x % m;
    let mut out = [0u8; 32];
    let bytes = v.to_bytes_le();
    let n = bytes.len().min(32);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

/// Public key bytes used for BIP32-style fingerprint (SHA256 → RIPEMD160, first 4 bytes).
fn fingerprint_4(pub_compressed: &[u8]) -> [u8; 4] {
    let mut s = sha2::Sha256::new();
    s.update(pub_compressed);
    let inner = ripemd160(&s.finalize());
    inner[0..4].try_into().expect("ripemd160 is 20 bytes")
}

/// Algorand address (Python `AlgorandAddress.encode`): Base32 no padding over `pubkey32 || sha512_256(pubkey32)[28..32]`.
pub fn encode_address(pub_key_32: &[u8; COORD_BYTE_LEN]) -> Result<String> {
    let h = sha512_256(pub_key_32);
    let mut payload = Vec::with_capacity(36);
    payload.extend_from_slice(pub_key_32);
    payload.extend_from_slice(&h[28..32]);
    let hex_payload = hex::encode(payload);
    base32::encode_no_padding(&hex_payload, None).map_err(|e| {
        Error::new(
            "Algorand address encoding failed",
            Some(&e.to_string()),
            None,
            None,
        )
    })
}

/// Kholo-style extended secret `kL || kR` (64 bytes) → compressed public key (first 32 bytes as scalar mod order, base multiply).
fn public_from_extended_secret(ext: &[u8; 64]) -> [u8; COORD_BYTE_LEN] {
    let mut kl = [0u8; 32];
    kl.copy_from_slice(&ext[..32]);
    point_scalar_mul_base(&kl)
}

/// Algorand HD root + child derivation (Python `AlgorandHD`).
#[derive(Debug, Clone)]
#[allow(dead_code)] // root_* retained for future xprv / parity with Python state
pub struct AlgorandHd {
    seed: Vec<u8>,
    root_private_64: [u8; 64],
    root_chain_code: [u8; 32],
    /// `Some` when hardened-capable (root / hardened-derived); cleared after non-hardened `drive`.
    private_64: Option<[u8; 64]>,
    chain_code: [u8; 32],
    public_key: [u8; COORD_BYTE_LEN],
    depth: u32,
    child_index: u32,
    parent_fingerprint: [u8; 4],
    fingerprint: [u8; 4],
}

impl AlgorandHd {
    #[must_use]
    pub fn registry_name() -> &'static str {
        "Algorand"
    }

    /// Build from raw seed bytes (Python `from_seed` with `bytes`).
    pub fn from_seed_bytes(seed: &[u8]) -> Result<Self> {
        if seed.len() < 16 {
            return Err(Error::new(
                "Invalid seed length",
                Some("need at least 16 bytes"),
                Some(">= 16"),
                Some(&format!("{}", seed.len())),
            ));
        }

        let digest = Sha512::digest(seed);
        let mut kL = [0u8; 32];
        let mut kR = [0u8; 32];
        kL.copy_from_slice(&digest[..32]);
        kR.copy_from_slice(&digest[32..64]);

        while (kL[31] & 0b0010_0000) != 0 {
            let k = hmac_sha512_key_msg(&kL, &kR);
            kL.copy_from_slice(&k[..32]);
            kR.copy_from_slice(&k[32..64]);
        }

        clamp_kl(&mut kL);

        let mut chain_input = Vec::with_capacity(1 + seed.len());
        chain_input.push(0x01);
        chain_input.extend_from_slice(seed);
        let root_chain_code: [u8; 32] = Sha256::digest(&chain_input).into();

        let mut root_private_64 = [0u8; 64];
        root_private_64[..32].copy_from_slice(&kL);
        root_private_64[32..].copy_from_slice(&kR);

        let public_key = public_from_extended_secret(&root_private_64);

        let parent_fingerprint = [0u8; 4];
        let fingerprint = fingerprint_4(&public_key);

        Ok(Self {
            seed: seed.to_vec(),
            root_private_64,
            root_chain_code,
            private_64: Some(root_private_64),
            chain_code: root_chain_code,
            public_key,
            depth: 0,
            child_index: 0,
            parent_fingerprint,
            fingerprint,
        })
    }

    /// Hex seed string (Python `get_bytes` on str).
    pub fn from_seed_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex.trim())
            .map_err(|_| SeedError::new("Invalid seed data", None, None, None))?;
        Self::from_seed_bytes(&bytes)
    }

    /// `ISeed::seed()` hex (e.g. [`AlgorandSeed`](crate::hdwallet::seeds::AlgorandSeed)).
    pub fn from_is_seed<S: ISeed>(seed: &S) -> Result<Self> {
        Self::from_seed_hex(seed.seed())
    }

    #[must_use]
    pub fn seed(&self) -> &[u8] {
        &self.seed
    }

    #[must_use]
    pub fn tree_depth(&self) -> u32 {
        self.depth
    }

    #[must_use]
    pub fn child_index_value(&self) -> u32 {
        self.child_index
    }

    #[must_use]
    pub fn chain_code_bytes(&self) -> &[u8; 32] {
        &self.chain_code
    }

    #[must_use]
    pub fn pub_key_bytes(&self) -> &[u8; COORD_BYTE_LEN] {
        &self.public_key
    }

    #[must_use]
    pub fn private_extended(&self) -> Option<&[u8; 64]> {
        self.private_64.as_ref()
    }

    /// Derive one child step (Python `drive`).
    pub fn drive(&mut self, index: u32) -> Result<&mut Self> {
        let index_bytes = index.to_le_bytes();
        let cc = self.chain_code;

        if index & 0x8000_0000 != 0 {
            let sk = self.private_64.ok_or_else(|| {
                Error::new(
                    "Private key required for hardened derivation",
                    None,
                    None,
                    None,
                )
            })?;
            let mut kL = [0u8; 32];
            let mut kR = [0u8; 32];
            kL.copy_from_slice(&sk[..32]);
            kR.copy_from_slice(&sk[32..]);

            let mut data = Vec::with_capacity(1 + 64 + 4);
            data.push(0x00);
            data.extend_from_slice(&kL);
            data.extend_from_slice(&kR);
            data.extend_from_slice(&index_bytes);
            let z = hmac_sha512_key_msg(&cc, &data);

            data[0] = 0x01;
            let z2 = hmac_sha512_key_msg(&cc, &data);
            let mut child_cc = [0u8; 32];
            child_cc.copy_from_slice(&z2[32..64]);

            let (zL, zR) = z.split_at(32);
            let zL: [u8; 32] = zL.try_into().unwrap();
            let zR: [u8; 32] = zR.try_into().unwrap();

            let truncated_zL = trunc_256_minus_g_bits(&zL, PEIKERT_G);
            let kL_int = le_bytes_to_uint256(&kL);
            let zL_int = le_bytes_to_uint256(&truncated_zL);
            let eight = BigUint::from(8u32);
            let child_kL_int = kL_int + eight * zL_int;
            if child_kL_int >= two_pow_255() {
                return Err(Error::new(
                    "zL * 8 + kL exceeds Ed25519 scalar limit",
                    None,
                    None,
                    None,
                ));
            }
            let child_kL = uint256_to_le_32(&child_kL_int);
            let kR_int = le_bytes_to_uint256(&kR);
            let zR_int = le_bytes_to_uint256(&zR);
            let child_kR_int = (kR_int + zR_int) % two_pow_256();
            let child_kR = uint256_to_le_32(&child_kR_int);

            let mut child_64 = [0u8; 64];
            child_64[..32].copy_from_slice(&child_kL);
            child_64[32..].copy_from_slice(&child_kR);

            self.parent_fingerprint = self.fingerprint;
            self.public_key = public_from_extended_secret(&child_64);
            self.private_64 = Some(child_64);
            self.chain_code = child_cc;
        } else {
            let a = self.public_key;
            let mut data = Vec::with_capacity(1 + 32 + 4);
            data.push(0x02);
            data.extend_from_slice(&a);
            data.extend_from_slice(&index_bytes);
            let z = hmac_sha512_key_msg(&cc, &data);

            data[0] = 0x03;
            let z2 = hmac_sha512_key_msg(&cc, &data);
            let mut child_cc = [0u8; 32];
            child_cc.copy_from_slice(&z2[32..64]);

            let zL: [u8; 32] = z[..32].try_into().unwrap();
            let truncated_zL = trunc_256_minus_g_bits(&zL, PEIKERT_G);
            let zL_int = le_bytes_to_uint256(&truncated_zL);
            let scalar_big = BigUint::from(8u32) * zL_int;
            let scalar_bytes = scalar_reduce_from_int(&scalar_big);
            let g_mul = point_scalar_mul_base(&scalar_bytes);
            self.parent_fingerprint = self.fingerprint;
            self.public_key = point_add(
                PointRef::Encoded(&a),
                PointRef::Encoded(&g_mul),
            )
            .map_err(|e| {
                Error::new(
                    "Algorand non-hardened point add",
                    Some(&e.to_string()),
                    None,
                    None,
                )
            })?;
            self.private_64 = None;
            self.chain_code = child_cc;
        }

        self.depth += 1;
        self.child_index = index;
        self.fingerprint = fingerprint_4(&self.public_key);
        Ok(self)
    }

    /// Payment address for the current public key.
    pub fn address(&self) -> Result<String> {
        encode_address(&self.public_key)
    }

    /// Fingerprint hex string (Python `fingerprint()`): first 8 hex chars of RIPEMD160(SHA256(pub)).
    pub fn fingerprint_hex(&self) -> String {
        let fp = fingerprint_4(&self.public_key);
        hex::encode(fp)
    }
}

impl Ihd for AlgorandHd {
    fn registry_name(&self) -> &'static str {
        Self::registry_name()
    }

    fn from_seed(&mut self, seed: &[u8]) -> Result<()> {
        *self = Self::from_seed_bytes(seed)?;
        Ok(())
    }

    fn seed_hex(&self) -> Option<String> {
        Some(hex::encode(&self.seed))
    }

    fn depth(&self) -> Option<u32> {
        Some(self.depth)
    }

    fn index(&self) -> Option<u32> {
        Some(self.child_index)
    }

    fn chain_code(&self) -> Option<String> {
        Some(hex::encode(self.chain_code))
    }

    fn public_key(&self) -> Option<String> {
        Some(hex::encode(self.public_key))
    }

    fn fingerprint(&self) -> Option<String> {
        Some(self.fingerprint_hex())
    }

    fn parent_fingerprint(&self) -> Option<String> {
        Some(hex::encode(self.parent_fingerprint))
    }

    fn address(&self) -> Option<String> {
        self.address().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_drive_hardened_address_stable() {
        let seed = [7u8; 64];
        let mut hd = AlgorandHd::from_seed_bytes(&seed).expect("seed");
        let addr0 = hd.address().expect("addr");
        hd.drive(0x80000000).expect("hardened 0");
        let addr1 = hd.address().expect("addr1");
        assert_ne!(addr0, addr1);
        assert!(addr0.len() >= 50);
        assert!(addr1.len() >= 50);
    }
}
