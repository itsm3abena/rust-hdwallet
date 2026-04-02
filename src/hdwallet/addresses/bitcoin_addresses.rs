//! Bitcoin-style addresses (P2PKH, P2WPKH, P2TR, P2SH-P2WPKH).

use bitcoin::key::{Secp256k1, TapTweak};
use bitcoin::secp256k1::{Keypair, PublicKey, SecretKey, XOnlyPublicKey};
use bitcoin::{Address, KnownHrp, Network};

use crate::hdwallet::consts::semantics;
use crate::hdwallet::exceptions::{Error, Result};

/// Encode a pay-to-public-key-hash style address using `Network` (Bitcoin / signet / regtest).
pub fn encode_bitcoin_address(
    network: Network,
    semantic: &str,
    secret_key: &SecretKey,
    public_key: &PublicKey,
) -> Result<String> {
    let secp = Secp256k1::new();
    match semantic {
        semantics::P2PKH => {
            let addr = Address::p2pkh(public_key, network);
            Ok(addr.to_string())
        }
        semantics::P2WPKH => {
            let addr = Address::p2wpkh(public_key, network);
            Ok(addr.to_string())
        }
        semantics::P2WPKH_IN_P2SH => {
            let addr = Address::p2shp2wpkh(public_key, network);
            Ok(addr.to_string())
        }
        semantics::P2TR => {
            let kp = Keypair::from_secret_key(&secp, secret_key);
            let (xonly, _) = kp.x_only_public_key();
            let tweak = TapTweak::from_keypair(kp);
            let tweaked = xonly.tap_tweak(&secp, tweak.merkle_root());
            let hrp = match network {
                Network::Bitcoin => KnownHrp::Mainnet,
                Network::Testnet | Network::Signet | Network::Regtest => KnownHrp::Testnets,
                _ => KnownHrp::Testnets,
            };
            let addr = Address::p2tr_tweaked(hrp, tweaked.0);
            Ok(addr.to_string())
        }
        semantics::P2SH => Err(Error::Address(
            "P2SH needs redeem script; use P2WPKH-in-P2SH for standard nested segwit".into(),
        )),
        semantics::P2WSH | semantics::P2WSH_IN_P2SH => Err(Error::Address(
            "P2WSH requires witness script; not implemented in this helper".into(),
        )),
        _ => Err(Error::Address(format!("unknown semantic {semantic}"))),
    }
}
