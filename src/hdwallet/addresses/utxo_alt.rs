//! P2PKH / P2WPKH for Bitcoin-fork networks (Litecoin, etc.).

use bech32::{Bech32m, Hrp};
use bitcoin::hashes::{hash160, Hash};
use bitcoin::secp256k1::PublicKey;
use bitcoin::base58;

use crate::hdwallet::exceptions::{Error, Result};

/// Version bytes and Bech32 HRP for a UTXO network.
#[derive(Debug, Clone, Copy)]
pub struct UtxoAddressParams {
    pub p2pkh_version: u8,
    pub p2sh_version: u8,
    pub bech32_hrp: &'static str,
}

pub const LITECOIN_MAINNET: UtxoAddressParams = UtxoAddressParams {
    p2pkh_version: 0x30,
    p2sh_version: 0x32,
    bech32_hrp: "ltc",
};

pub const LITECOIN_TESTNET: UtxoAddressParams = UtxoAddressParams {
    p2pkh_version: 0x6f,
    p2sh_version: 0x3a,
    bech32_hrp: "tltc",
};

pub const DOGECOIN_MAINNET: UtxoAddressParams = UtxoAddressParams {
    p2pkh_version: 0x1e,
    p2sh_version: 0x16,
    bech32_hrp: "",
};

pub const DOGECOIN_TESTNET: UtxoAddressParams = UtxoAddressParams {
    p2pkh_version: 0x71,
    p2sh_version: 0xc4,
    bech32_hrp: "",
};

fn p2pkh(params: &UtxoAddressParams, pk: &PublicKey) -> String {
    let h = hash160::Hash::hash(&pk.serialize());
    let mut data = vec![params.p2pkh_version];
    data.extend(h.as_byte_array());
    base58::encode_check(&data)
}

fn p2wpkh(params: &UtxoAddressParams, pk: &PublicKey) -> Result<String> {
    if params.bech32_hrp.is_empty() {
        return Err(Error::Address(
            "native segwit not defined for this coin".into(),
        ));
    }
    let h = hash160::Hash::hash(&pk.serialize());
    let hrp = Hrp::parse(params.bech32_hrp).map_err(|e| Error::Address(e.to_string()))?;
    let mut wit = Vec::with_capacity(22);
    wit.push(0);
    wit.extend_from_slice(h.as_byte_array());
    let addr = bech32::encode::<Bech32m>(hrp, &wit).map_err(|e| Error::Address(e.to_string()))?;
    Ok(addr)
}

/// `semantic` uses the same strings as Bitcoin (`p2pkh`, `p2wpkh`, …).
pub fn encode_utxo_coin_address(
    params: &UtxoAddressParams,
    semantic: &str,
    pk: &PublicKey,
) -> Result<String> {
    use crate::hdwallet::consts::semantics;
    match semantic {
        semantics::P2PKH => Ok(p2pkh(params, pk)),
        semantics::P2WPKH => p2wpkh(params, pk),
        semantics::P2WPKH_IN_P2SH => {
            let w = hash160::Hash::hash(&pk.serialize());
            let mut redeem = Vec::with_capacity(22);
            redeem.push(0x00);
            redeem.push(0x14);
            redeem.extend_from_slice(w.as_byte_array());
            let script_hash = hash160::Hash::hash(&redeem);
            let mut data = vec![params.p2sh_version];
            data.extend(script_hash.as_byte_array());
            Ok(base58::encode_check(&data))
        }
        _ => Err(Error::Address(format!(
            "semantic {semantic} not supported for this altcoin"
        ))),
    }
}
