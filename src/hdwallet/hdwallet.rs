//! Hierarchical deterministic wallet (Python `hdwallet` parity, in progress).
//!
//! The **full** field list and `use` graph from the port live in the block comment on
//! [`HDWallet`] below. Uncomment and wire each piece as the corresponding module is
//! enabled in [`crate::hdwallet`]. The previous ~850-line `impl` is still in git:
//! `git show HEAD:src/hdwallet/hdwallet.rs` (or restore that file and fix paths when ready).
//!
//! Import paths below are corrected for this crate (`crate::hdwallet::…`, not `crate::…`).

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::hdwallet::entropies::IEntropy;

// use crate::hdwallet::cryptocurrencies::icryptocurrency::{ICryptocurrency, INetwork};
// use crate::hdwallet::hds::{IHD, HDS};
// use crate::hdwallet::addresses::{IAddress, ADDRESSES};
// use crate::hdwallet::entropies::Entropies;
// use crate::hdwallet::mnemonics::{IMnemonic, ElectrumV2Mnemonic, MoneroMnemonic, MNEMONICS};
// use crate::hdwallet::seeds::{ISeed, BIP39Seed, CardanoSeed, ElectrumV2Seed, SEEDS};
// use crate::hdwallet::eccs::IEllipticCurveCryptography;
// use crate::hdwallet::exceptions::{Error, NetworkError, AddressError, CryptocurrencyError};
// use crate::hdwallet::consts::{MODES};

pub struct HDWallet {
    /*
    cryptocurrency: Rc<dyn ICryptocurrency>,
    network: Rc<dyn INetwork>,
    address: Rc<dyn IAddress>,
    address_type: Option<String>,
    address_prefix: Option<String>,
    ecc: Rc<dyn IEllipticCurveCryptography>,
    */

    /// Active entropy scheme + value once [`HDWallet::from_entropy`] (or mnemonic flow) is implemented.
    pub entropy: Option<Rc<dyn IEntropy>>,

    /*
    language: Option<String>,
    passphrase: Option<String>,
    mnemonic: Option<Rc<dyn IMnemonic>>,
    seed: Option<Rc<dyn ISeed>>,
    derivation: Option<String>,

    semantic: Option<String>,

    // Electrum-V2
    mode: Option<String>,
    mnemonic_type: Option<String>,

    public_key_type: Option<String>,
    cardano_type: Option<String>,
    use_default_path: bool,
    checksum: bool,
    */

    pub kwargs: HashMap<String, Option<String>>,

    // hd: Rc<dyn IHD>,
}

impl fmt::Debug for HDWallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HDWallet")
            .field(
                "entropy",
                &self.entropy.as_ref().map(|e| e.name()),
            )
            .field("kwargs", &self.kwargs)
            .finish()
    }
}

impl Default for HDWallet {
    fn default() -> Self {
        Self {
            entropy: None,
            kwargs: HashMap::new(),
        }
    }
}

impl HDWallet {
    pub fn new(kwargs: HashMap<String, Option<String>>) -> Self {
        Self {
            entropy: None,
            kwargs,
        }
    }
}
