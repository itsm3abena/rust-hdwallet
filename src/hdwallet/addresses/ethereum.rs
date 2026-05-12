use bitcoin::secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

use crate::hdwallet::exceptions::{Error, Result};

pub fn ethereum_address_from_pubkey(pk: &PublicKey) -> Result<String> {
    let u = pk.serialize_uncompressed();
    if u.len() != 65 || u[0] != 0x04 {
        return Err(Error::new(
            "expected uncompressed public key",
            None,
            None,
            None,
        ));
    }
    let mut hasher = Keccak256::new();
    hasher.update(&u[1..]);
    let hash = hasher.finalize();
    let addr_bytes = &hash[12..];
    Ok(format!("0x{}", hex::encode(addr_bytes)))
}
