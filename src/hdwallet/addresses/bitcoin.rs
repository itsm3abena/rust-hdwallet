use bitcoin::key::{Secp256k1, TapTweak, TweakedPublicKey};
use bitcoin::secp256k1::{Keypair, PublicKey, SecretKey};
use bitcoin::{Address, CompressedPublicKey, KnownHrp, Network, PublicKey as BtcPublicKey};

use crate::hdwallet::consts::semantics;
use crate::hdwallet::exceptions::{Error, Result};

pub fn encode_bitcoin_address(
    network: Network,
    semantic: &str,
    secret_key: &SecretKey,
    public_key: &PublicKey,
) -> Result<String> {
    let secp = Secp256k1::new();
    let hrp = KnownHrp::from(network);

    match semantic {
        semantics::P2PKH => {
            let addr = Address::p2pkh(BtcPublicKey::from(*public_key), network);
            Ok(addr.to_string())
        }
        semantics::P2WPKH => {
            let cpk = CompressedPublicKey::from_slice(&public_key.serialize()).map_err(|e| {
                Error::new(&format!("invalid public key: {e}"), None, None, None)
            })?;
            let addr = Address::p2wpkh(&cpk, hrp);
            Ok(addr.to_string())
        }
        semantics::P2WPKH_IN_P2SH => {
            let cpk = CompressedPublicKey::from_slice(&public_key.serialize()).map_err(|e| {
                Error::new(&format!("invalid public key: {e}"), None, None, None)
            })?;
            let addr = Address::p2shwpkh(&cpk, network);
            Ok(addr.to_string())
        }
        semantics::P2TR => {
            let kp = Keypair::from_secret_key(&secp, secret_key);
            let tweaked_kp = kp.tap_tweak(&secp, None);
            let output_key = TweakedPublicKey::from_keypair(tweaked_kp);
            let addr = Address::p2tr_tweaked(output_key, hrp);
            Ok(addr.to_string())
        }
        semantics::P2SH => Err(Error::new(
            "P2SH needs redeem script; use P2WPKH-in-P2SH for standard nested segwit",
            None,
            None,
            None,
        )),
        semantics::P2WSH | semantics::P2WSH_IN_P2SH => Err(Error::new(
            "P2WSH requires witness script; not implemented in this helper",
            None,
            None,
            None,
        )),
        _ => Err(Error::new(
            &format!("unknown semantic {semantic}"),
            None,
            None,
            None,
        )),
    }
}
