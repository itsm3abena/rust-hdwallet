//! Ethereum checksummed-style hex address (0x + 20 bytes).

use bitcoin::secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

use crate::hdwallet::exceptions::{Error, Result};

/// `Keccak256(pubkey_x || pubkey_y)` with uncompressed SEC format **without** the `0x04` prefix.
pub fn ethereum_address_from_pubkey(pk: &PublicKey) -> Result<String> {
    let u = pk.serialize_uncompressed();
    if u.len() != 65 || u[0] != 0x04 {
        return Err(Error::Address("expected uncompressed public key".into()));
    }
    let mut hasher = Keccak256::new();
    hasher.update(&u[1..]);
    let hash = hasher.finalize();
    let addr_bytes = &hash[12..];
    Ok(format!("0x{}", hex::encode(addr_bytes)))
}
