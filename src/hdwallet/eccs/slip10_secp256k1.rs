//! SLIP-10 secp256k1 (mirrors `hdwallet/eccs/slip10_secp256k1.py` — type tag for API parity).

/// Marker matching Python `SLIP10Secp256k1ECC` used on cryptocurrency definitions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Slip10Secp256k1Ecc;
