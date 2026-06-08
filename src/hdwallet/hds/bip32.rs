use std::str::FromStr;

use bitcoin::bip32::{ChainCode, ChildNumber, DerivationPath, Fingerprint, Xpriv, Xpub};
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::{PublicKey as SecpPublicKey, SecretKey};
use bitcoin::Network;

use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::wif::{decode_wif, encode_wif};

#[derive(Debug, Clone)]
pub struct Bip32Hd {
    network: Network,
    master: Xpriv,
    derived: Option<Xpriv>,
    watch_xpub: Option<Xpub>,
    derived_xpub: Option<Xpub>,
    secp: Secp256k1<bitcoin::secp256k1::All>,
    wif_prefix: Option<u8>,
    compressed: bool,
    strict: bool,
    seed: Option<Vec<u8>>,
    derivation_path: Option<String>,
}

impl Bip32Hd {
    pub fn from_seed(network: Network, seed: &[u8]) -> HdResult<Self> {
        let secp = Secp256k1::new();
        let master = Xpriv::new_master(network, seed).map_err(|error| {
            Error::new("Invalid BIP32 master key", Some(&error.to_string()), None, None)
        })?;
        Ok(Self {
            network,
            master,
            derived: None,
            watch_xpub: None,
            derived_xpub: None,
            secp,
            wif_prefix: None,
            compressed: true,
            strict: false,
            seed: Some(seed.to_vec()),
            derivation_path: None,
        })
    }

    pub fn configure(
        &mut self,
        wif_prefix: Option<u8>,
        public_key_type: &str,
        strict: bool,
    ) {
        self.wif_prefix = wif_prefix;
        self.compressed = public_key_type != PublicKeyTypes::UNCOMPRESSED;
        self.strict = strict;
    }

    #[must_use]
    pub fn master_xpriv(&self) -> &Xpriv {
        &self.master
    }

    pub fn derive_at_path(&mut self, path: &str) -> HdResult<()> {
        let derivation_path = DerivationPath::from_str(path).map_err(|error| {
            Error::new("Invalid derivation path", Some(&error.to_string()), None, None)
        })?;
        if let Some(xpub) = self.watch_xpub.as_ref() {
            let derived = xpub.derive_pub(&self.secp, &derivation_path).map_err(|error| {
                Error::new("BIP32 public derivation failed", Some(&error.to_string()), None, None)
            })?;
            self.derived_xpub = Some(derived);
            self.derived = None;
        } else {
            let derived = self
                .master
                .derive_priv(&self.secp, &derivation_path)
                .map_err(|error| {
                    Error::new("BIP32 derivation failed", Some(&error.to_string()), None, None)
                })?;
            self.derived = Some(derived);
            self.derived_xpub = None;
        }
        self.derivation_path = Some(path.to_string());
        Ok(())
    }

    pub fn from_xprivate_key_str(
        &mut self,
        xprivate_key: &str,
        encoded: bool,
        strict: bool,
    ) -> HdResult<()> {
        let master = if encoded {
            xprivate_key.parse::<Xpriv>().map_err(|error| {
                Error::new("Invalid xprivate key", Some(&error.to_string()), None, None)
            })?
        } else {
            let bytes = hex::decode(xprivate_key).map_err(|error| {
                Error::new("Invalid xprivate key hex", Some(&error.to_string()), None, None)
            })?;
            Xpriv::decode(&bytes).map_err(|error| {
                Error::new("Invalid xprivate key bytes", Some(&error.to_string()), None, None)
            })?
        };
        self.master = master;
        self.derived = None;
        self.derived_xpub = None;
        self.watch_xpub = None;
        self.derivation_path = None;
        self.seed = None;
        self.strict = strict;
        Ok(())
    }

    pub fn from_wif_str(&mut self, wif: &str) -> HdResult<()> {
        let prefix = self.wif_prefix.ok_or_else(|| {
            Error::new("WIF prefix is not configured for this network", None, None, None)
        })?;
        let (private_key, wif_type, _) = decode_wif(wif, prefix).map_err(|error| {
            Error::new("Invalid WIF", Some(&error.to_string()), None, None)
        })?;
        self.compressed = wif_type == "wif-compressed";
        let secret = SecretKey::from_slice(&private_key).map_err(|error| {
            Error::new("Invalid WIF private key", Some(&error.to_string()), None, None)
        })?;
        self.master = Xpriv {
            network: self.network.into(),
            depth: 0,
            parent_fingerprint: Fingerprint::default(),
            child_number: ChildNumber::from_normal_idx(0).map_err(|error| {
                Error::new("Invalid child number", Some(&error.to_string()), None, None)
            })?,
            private_key: secret,
            chain_code: ChainCode::from([0u8; 32]),
        };
        self.derived = None;
        self.derived_xpub = None;
        self.watch_xpub = None;
        self.derivation_path = None;
        self.seed = None;
        Ok(())
    }

    pub fn from_xpublic_key_str(
        &mut self,
        xpublic_key: &str,
        encoded: bool,
        strict: bool,
    ) -> HdResult<()> {
        let xpub = if encoded {
            xpublic_key.parse::<Xpub>().map_err(|error| {
                Error::new("Invalid xpublic key", Some(&error.to_string()), None, None)
            })?
        } else {
            let bytes = hex::decode(xpublic_key).map_err(|error| {
                Error::new("Invalid xpublic key hex", Some(&error.to_string()), None, None)
            })?;
            Xpub::decode(&bytes).map_err(|error| {
                Error::new("Invalid xpublic key bytes", Some(&error.to_string()), None, None)
            })?
        };
        self.watch_xpub = Some(xpub);
        self.derived_xpub = None;
        self.derived = None;
        self.derivation_path = None;
        self.seed = None;
        self.strict = strict;
        Ok(())
    }

    pub fn from_private_key_str(&mut self, private_key: &str) -> HdResult<()> {
        let bytes = hex::decode(private_key.trim()).map_err(|error| {
            Error::new("Invalid private key hex", Some(&error.to_string()), None, None)
        })?;
        let secret = SecretKey::from_slice(&bytes).map_err(|error| {
            Error::new("Invalid private key", Some(&error.to_string()), None, None)
        })?;
        self.master = Xpriv {
            network: self.network.into(),
            depth: 0,
            parent_fingerprint: Fingerprint::default(),
            child_number: ChildNumber::from_normal_idx(0).map_err(|error| {
                Error::new("Invalid child number", Some(&error.to_string()), None, None)
            })?,
            private_key: secret,
            chain_code: ChainCode::from([0u8; 32]),
        };
        self.derived = None;
        self.derived_xpub = None;
        self.watch_xpub = None;
        self.derivation_path = None;
        self.seed = None;
        Ok(())
    }

    fn active_xpriv(&self) -> Option<&Xpriv> {
        if self.watch_xpub.is_some() {
            return None;
        }
        self.derived.as_ref().or(Some(&self.master))
    }

    fn active_xpub(&self) -> Option<&Xpub> {
        self.derived_xpub
            .as_ref()
            .or(self.watch_xpub.as_ref())
    }

    fn xpub_public_key_bytes(&self, xpub: &Xpub, compressed: bool) -> String {
        if compressed {
            hex::encode(xpub.public_key.serialize())
        } else {
            hex::encode(xpub.public_key.serialize_uncompressed())
        }
    }

    fn encode_wif_for_key(&self, private_key: &[u8]) -> HdResult<String> {
        let prefix = self.wif_prefix.ok_or_else(|| {
            Error::new("WIF prefix is not configured for this network", None, None, None)
        })?;
        let (wif, wif_compressed) = encode_wif(private_key, prefix).map_err(|error| {
            Error::new("WIF encoding failed", Some(&error.to_string()), None, None)
        })?;
        Ok(if self.compressed {
            wif_compressed
        } else {
            wif
        })
    }

    fn public_key_bytes(&self, xpriv: &Xpriv, compressed: bool) -> String {
        let public_key = SecpPublicKey::from_secret_key(&self.secp, &xpriv.private_key);
        if compressed {
            hex::encode(public_key.serialize())
        } else {
            hex::encode(public_key.serialize_uncompressed())
        }
    }
}

impl Ihd for Bip32Hd {
    fn registry_name(&self) -> &'static str {
        "BIP32"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        *self = Self::from_seed(self.network, seed)?;
        Ok(())
    }

    fn from_xprivate_key(&mut self, xprivate_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        self.from_xprivate_key_str(xprivate_key, encoded, strict)
    }

    fn from_xpublic_key(&mut self, xpublic_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        self.from_xpublic_key_str(xpublic_key, encoded, strict)
    }

    fn from_wif(&mut self, wif: &str) -> HdResult<()> {
        self.from_wif_str(wif)
    }

    fn from_private_key(&mut self, private_key: &str) -> HdResult<()> {
        self.from_private_key_str(private_key)
    }

    fn from_derivation(&mut self, derivation: &str) -> HdResult<()> {
        self.derive_at_path(derivation)
    }

    fn update_derivation(&mut self, derivation: &str) -> HdResult<()> {
        self.derive_at_path(derivation)
    }

    fn seed_hex(&self) -> Option<String> {
        self.seed.as_ref().map(|seed| hex::encode(seed))
    }

    fn strict(&self) -> Option<bool> {
        Some(self.strict)
    }

    fn public_key_type(&self) -> Option<String> {
        Some(if self.compressed {
            PublicKeyTypes::COMPRESSED.to_string()
        } else {
            PublicKeyTypes::UNCOMPRESSED.to_string()
        })
    }

    fn wif_type(&self) -> Option<String> {
        Some(if self.compressed {
            "wif-compressed".to_string()
        } else {
            "wif".to_string()
        })
    }

    fn root_xprivate_key(&self, _encoded: bool) -> Option<String> {
        Some(self.master.to_string())
    }

    fn root_xpublic_key(&self, _encoded: bool) -> Option<String> {
        if let Some(xpub) = self.watch_xpub.as_ref() {
            return Some(xpub.to_string());
        }
        Some(Xpub::from_priv(&self.secp, &self.master).to_string())
    }

    fn root_private_key(&self) -> Option<String> {
        Some(hex::encode(self.master.private_key.secret_bytes()))
    }

    fn root_wif(&self) -> Option<String> {
        self.encode_wif_for_key(&self.master.private_key.secret_bytes()).ok()
    }

    fn root_chain_code(&self) -> Option<String> {
        Some(hex::encode(self.master.chain_code.as_ref() as &[u8]))
    }

    fn root_public_key(&self) -> Option<String> {
        if let Some(xpub) = self.watch_xpub.as_ref() {
            return Some(self.xpub_public_key_bytes(xpub, true));
        }
        Some(self.public_key_bytes(&self.master, true))
    }

    fn xprivate_key(&self, _encoded: bool) -> Option<String> {
        self.active_xpriv().map(|xpriv| xpriv.to_string())
    }

    fn xpublic_key(&self, _encoded: bool) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(xpub.to_string());
        }
        self.active_xpriv()
            .map(|xpriv| Xpub::from_priv(&self.secp, xpriv).to_string())
    }

    fn private_key(&self) -> Option<String> {
        self.active_xpriv()
            .map(|xpriv| hex::encode(xpriv.private_key.secret_bytes()))
    }

    fn wif(&self) -> Option<String> {
        self.active_xpriv()
            .and_then(|xpriv| self.encode_wif_for_key(&xpriv.private_key.secret_bytes()).ok())
    }

    fn chain_code(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(hex::encode(xpub.chain_code.as_ref() as &[u8]));
        }
        self.active_xpriv()
            .map(|xpriv| hex::encode(xpriv.chain_code.as_ref() as &[u8]))
    }

    fn public_key(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, self.compressed));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, self.compressed))
    }

    fn compressed(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, true));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, true))
    }

    fn uncompressed(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, false));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, false))
    }

    fn hash(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(hex::encode(hash160(&xpub.public_key.serialize())));
        }
        self.active_xpriv().map(|xpriv| {
            let public_key = SecpPublicKey::from_secret_key(&self.secp, &xpriv.private_key);
            hex::encode(hash160(&public_key.serialize()))
        })
    }

    fn fingerprint(&self) -> Option<String> {
        self.hash().map(|hash| hash.chars().take(8).collect())
    }

    fn parent_fingerprint(&self) -> Option<String> {
        if let Some(xpub) = self.active_xpub() {
            return Some(hex::encode(xpub.parent_fingerprint.to_bytes()));
        }
        self.active_xpriv().map(|xpriv| {
            hex::encode(xpriv.parent_fingerprint.to_bytes())
        })
    }

    fn depth(&self) -> Option<u32> {
        if let Some(xpub) = self.active_xpub() {
            return Some(u32::from(xpub.depth));
        }
        self.active_xpriv().map(|xpriv| u32::from(xpriv.depth))
    }

    fn path(&self) -> Option<String> {
        self.derivation_path.clone()
    }

    fn indexes(&self) -> Option<Vec<u32>> {
        self.derivation_path.as_ref().and_then(|path| {
            crate::hdwallet::utils::path_to_indexes(path).ok()
        })
    }
}
