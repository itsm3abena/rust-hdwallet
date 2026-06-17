use std::str::FromStr;

use bitcoin::bip32::{ChainCode, ChildNumber, DerivationPath, Fingerprint, Xpriv, Xpub};
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::{PublicKey as SecpPublicKey, SecretKey};
use bitcoin::Network;

use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::extended_keys::{deserialize_extended, is_root_extended_key, serialize_extended, ExtendedKeyParts};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::wif::{decode_wif, encode_wif};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bip32ImportMode {
    Extended,
    LeafPrivate,
    LeafPublic,
}

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
    xprivate_version: u32,
    xpublic_version: u32,
    seed: Option<Vec<u8>>,
    derivation_path: Option<String>,
    import_mode: Bip32ImportMode,
    leaf_public_key: Option<SecpPublicKey>,
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
            xprivate_version: 0x0488ade4,
            xpublic_version: 0x0488b21e,
            seed: Some(seed.to_vec()),
            derivation_path: None,
            import_mode: Bip32ImportMode::Extended,
            leaf_public_key: None,
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

    pub fn set_xkey_versions(&mut self, xprivate_version: u32, xpublic_version: u32) {
        self.xprivate_version = xprivate_version;
        self.xpublic_version = xpublic_version;
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

    fn xpriv_from_parts(&self, parts: &ExtendedKeyParts) -> HdResult<Xpriv> {
        if parts.key.is_empty() || parts.key[0] != 0x00 {
            return Err(Error::new("Invalid extended private key", None, None, None));
        }
        let secret = SecretKey::from_slice(&parts.key[1..]).map_err(|error| {
            Error::new("Invalid extended private key", Some(&error.to_string()), None, None)
        })?;
        Ok(Xpriv {
            network: self.network.into(),
            depth: parts.depth,
            parent_fingerprint: Fingerprint::from(parts.parent_fingerprint),
            child_number: ChildNumber::from(parts.index),
            private_key: secret,
            chain_code: ChainCode::from(parts.chain_code),
        })
    }

    fn xpub_from_parts(&self, parts: &ExtendedKeyParts) -> HdResult<Xpub> {
        let public_key = SecpPublicKey::from_slice(&parts.key).map_err(|error| {
            Error::new("Invalid extended public key", Some(&error.to_string()), None, None)
        })?;
        Ok(Xpub {
            network: self.network.into(),
            depth: parts.depth,
            parent_fingerprint: Fingerprint::from(parts.parent_fingerprint),
            child_number: ChildNumber::from(parts.index),
            public_key,
            chain_code: ChainCode::from(parts.chain_code),
        })
    }

    pub fn from_xprivate_key_str(
        &mut self,
        xprivate_key: &str,
        encoded: bool,
        strict: bool,
    ) -> HdResult<()> {
        let parts = deserialize_extended(xprivate_key, encoded)?;
        let master = match if encoded {
            xprivate_key.parse::<Xpriv>()
        } else {
            let bytes = hex::decode(xprivate_key).map_err(|error| {
                Error::new("Invalid xprivate key hex", Some(&error.to_string()), None, None)
            })?;
            Xpriv::decode(&bytes)
        } {
            Ok(xpriv) => xpriv,
            Err(_) => self.xpriv_from_parts(&parts)?,
        };
        if strict && !is_root_extended_key(xprivate_key, encoded)? {
            return Err(Error::new("Invalid root extended private key", None, None, None));
        }
        self.master = master;
        self.derived = None;
        self.derived_xpub = None;
        self.watch_xpub = None;
        self.derivation_path = None;
        self.seed = None;
        self.strict = is_root_extended_key(xprivate_key, encoded)?;
        self.import_mode = Bip32ImportMode::Extended;
        self.leaf_public_key = None;
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
        self.import_mode = Bip32ImportMode::LeafPrivate;
        self.leaf_public_key = None;
        Ok(())
    }

    pub fn from_xpublic_key_str(
        &mut self,
        xpublic_key: &str,
        encoded: bool,
        strict: bool,
    ) -> HdResult<()> {
        let parts = deserialize_extended(xpublic_key, encoded)?;
        let xpub = match if encoded {
            xpublic_key.parse::<Xpub>()
        } else {
            let bytes = hex::decode(xpublic_key).map_err(|error| {
                Error::new("Invalid xpublic key hex", Some(&error.to_string()), None, None)
            })?;
            Xpub::decode(&bytes)
        } {
            Ok(xpub) => xpub,
            Err(_) => self.xpub_from_parts(&parts)?,
        };
        if strict && !is_root_extended_key(xpublic_key, encoded)? {
            return Err(Error::new("Invalid root extended public key", None, None, None));
        }
        self.watch_xpub = Some(xpub);
        self.derived_xpub = None;
        self.derived = None;
        self.derivation_path = None;
        self.seed = None;
        self.strict = is_root_extended_key(xpublic_key, encoded)?;
        self.import_mode = Bip32ImportMode::Extended;
        self.leaf_public_key = None;
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
        self.import_mode = Bip32ImportMode::LeafPrivate;
        self.leaf_public_key = None;
        Ok(())
    }

    pub fn from_public_key_str(&mut self, public_key: &str) -> HdResult<()> {
        let bytes = hex::decode(public_key.trim()).map_err(|error| {
            Error::new("Invalid public key hex", Some(&error.to_string()), None, None)
        })?;
        let public_key = SecpPublicKey::from_slice(&bytes).map_err(|error| {
            Error::new("Invalid public key", Some(&error.to_string()), None, None)
        })?;
        self.compressed = bytes.len() != 65;
        self.derived = None;
        self.derived_xpub = None;
        self.watch_xpub = None;
        self.derivation_path = None;
        self.seed = None;
        self.import_mode = Bip32ImportMode::LeafPublic;
        self.leaf_public_key = Some(public_key);
        Ok(())
    }

    fn active_xpriv(&self) -> Option<&Xpriv> {
        if self.import_mode == Bip32ImportMode::LeafPublic {
            return None;
        }
        if self.watch_xpub.is_some() {
            return None;
        }
        self.derived.as_ref().or(Some(&self.master))
    }

    fn active_leaf_public_key(&self) -> Option<&SecpPublicKey> {
        if self.import_mode == Bip32ImportMode::LeafPublic {
            self.leaf_public_key.as_ref()
        } else {
            None
        }
    }

    fn public_key_hash_bytes(&self) -> Option<Vec<u8>> {
        if let Some(xpub) = self.active_xpub() {
            return Some(if self.compressed {
                xpub.public_key.serialize().to_vec()
            } else {
                xpub.public_key.serialize_uncompressed().to_vec()
            });
        }
        if let Some(xpriv) = self.active_xpriv() {
            let public_key = SecpPublicKey::from_secret_key(&self.secp, &xpriv.private_key);
            return Some(if self.compressed {
                public_key.serialize().to_vec()
            } else {
                public_key.serialize_uncompressed().to_vec()
            });
        }
        self.active_leaf_public_key().map(|public_key| {
            if self.compressed {
                public_key.serialize().to_vec()
            } else {
                public_key.serialize_uncompressed().to_vec()
            }
        })
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

    fn fingerprint_bytes_from_pubkey(&self, public_key: &SecpPublicKey) -> [u8; 4] {
        let bytes = if self.compressed {
            public_key.serialize().to_vec()
        } else {
            public_key.serialize_uncompressed().to_vec()
        };
        let hash = hash160(&bytes);
        hash[0..4].try_into().unwrap()
    }

    fn parent_public_key(&self) -> HdResult<SecpPublicKey> {
        let path = self.derivation_path.as_deref().ok_or_else(|| {
            Error::new("Derivation path is not set", None, None, None)
        })?;
        let parent_path = match path.rfind('/') {
            Some(idx) if idx > 1 => &path[..idx],
            _ => "m",
        };
        if parent_path == "m" {
            if let Some(xpub) = self.watch_xpub.as_ref() {
                return Ok(xpub.public_key);
            }
            return Ok(SecpPublicKey::from_secret_key(
                &self.secp,
                &self.master.private_key,
            ));
        }
        let derivation_path = DerivationPath::from_str(parent_path).map_err(|error| {
            Error::new("Invalid parent derivation path", Some(&error.to_string()), None, None)
        })?;
        if let Some(xpub) = self.watch_xpub.as_ref() {
            let derived = xpub.derive_pub(&self.secp, &derivation_path).map_err(|error| {
                Error::new("BIP32 public derivation failed", Some(&error.to_string()), None, None)
            })?;
            return Ok(derived.public_key);
        }
        let derived = self
            .master
            .derive_priv(&self.secp, &derivation_path)
            .map_err(|error| {
                Error::new("BIP32 derivation failed", Some(&error.to_string()), None, None)
            })?;
        Ok(SecpPublicKey::from_secret_key(
            &self.secp,
            &derived.private_key,
        ))
    }

    fn parent_fingerprint_bytes(&self) -> HdResult<[u8; 4]> {
        let depth = if let Some(xpub) = self.active_xpub() {
            xpub.depth
        } else if let Some(xpriv) = self.active_xpriv() {
            xpriv.depth
        } else {
            return Ok([0u8; 4]);
        };
        if depth == 0 {
            return Ok([0u8; 4]);
        }
        Ok(self.fingerprint_bytes_from_pubkey(&self.parent_public_key()?))
    }

    fn encode_xpriv(&self, xpriv: &Xpriv) -> HdResult<String> {
        let encoded = xpriv.to_string();
        let parts = deserialize_extended(&encoded, true)?;
        let version = self.xprivate_version;
        let parent_fingerprint = if parts.depth == 0 {
            [0u8; 4]
        } else if self.compressed {
            parts.parent_fingerprint
        } else {
            self.parent_fingerprint_bytes()?
        };
        let mut key = vec![0x00];
        key.extend_from_slice(&xpriv.private_key.secret_bytes());
        serialize_extended(
            version,
            parts.depth,
            &parent_fingerprint,
            parts.index,
            &parts.chain_code,
            &key,
            true,
        )
    }

    fn encode_xpub(&self, xpub: &Xpub) -> HdResult<String> {
        let encoded = xpub.to_string();
        let parts = deserialize_extended(&encoded, true)?;
        let version = self.xpublic_version;
        let parent_fingerprint = if parts.depth == 0 {
            [0u8; 4]
        } else if self.compressed {
            parts.parent_fingerprint
        } else {
            self.parent_fingerprint_bytes()?
        };
        serialize_extended(
            version,
            parts.depth,
            &parent_fingerprint,
            parts.index,
            &parts.chain_code,
            &xpub.public_key.serialize(),
            true,
        )
    }
}

impl Ihd for Bip32Hd {
    fn registry_name(&self) -> &'static str {
        "BIP32"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        let network = self.network;
        let wif_prefix = self.wif_prefix;
        let compressed = self.compressed;
        *self = Self::from_seed(network, seed)?;
        self.wif_prefix = wif_prefix;
        self.compressed = compressed;
        self.strict = true;
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

    fn from_public_key(&mut self, public_key: &str) -> HdResult<()> {
        self.from_public_key_str(public_key)
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
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
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
        if self.import_mode == Bip32ImportMode::LeafPublic || self.watch_xpub.is_some() {
            return None;
        }
        if self.active_xpriv().is_none() {
            return None;
        }
        Some(if self.compressed {
            "wif-compressed".to_string()
        } else {
            "wif".to_string()
        })
    }

    fn root_xprivate_key(&self, _encoded: bool) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended || self.watch_xpub.is_some() {
            return None;
        }
        self.encode_xpriv(&self.master).ok()
    }

    fn root_xpublic_key(&self, _encoded: bool) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        if let Some(xpub) = self.watch_xpub.as_ref() {
            return self.encode_xpub(xpub).ok();
        }
        let xpub = Xpub::from_priv(&self.secp, &self.master);
        self.encode_xpub(&xpub).ok()
    }

    fn root_private_key(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended || self.watch_xpub.is_some() {
            return None;
        }
        Some(hex::encode(self.master.private_key.secret_bytes()))
    }

    fn root_wif(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended || self.watch_xpub.is_some() {
            return None;
        }
        self.encode_wif_for_key(&self.master.private_key.secret_bytes()).ok()
    }

    fn root_chain_code(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        if let Some(xpub) = self.watch_xpub.as_ref() {
            return Some(hex::encode(xpub.chain_code.as_ref() as &[u8]));
        }
        Some(hex::encode(self.master.chain_code.as_ref() as &[u8]))
    }

    fn root_public_key(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        if let Some(xpub) = self.watch_xpub.as_ref() {
            return Some(self.xpub_public_key_bytes(xpub, self.compressed));
        }
        Some(self.public_key_bytes(&self.master, self.compressed))
    }

    fn xprivate_key(&self, _encoded: bool) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        self.active_xpriv()
            .and_then(|xpriv| self.encode_xpriv(xpriv).ok())
    }

    fn xpublic_key(&self, _encoded: bool) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        if let Some(xpub) = self.active_xpub() {
            return self.encode_xpub(xpub).ok();
        }
        self.active_xpriv()
            .and_then(|xpriv| {
                let xpub = Xpub::from_priv(&self.secp, xpriv);
                self.encode_xpub(&xpub).ok()
            })
    }

    fn private_key(&self) -> Option<String> {
        if self.import_mode == Bip32ImportMode::LeafPublic {
            return None;
        }
        self.active_xpriv()
            .map(|xpriv| hex::encode(xpriv.private_key.secret_bytes()))
    }

    fn wif(&self) -> Option<String> {
        if self.import_mode == Bip32ImportMode::LeafPublic {
            return None;
        }
        self.active_xpriv()
            .and_then(|xpriv| self.encode_wif_for_key(&xpriv.private_key.secret_bytes()).ok())
    }

    fn chain_code(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        if let Some(xpub) = self.active_xpub() {
            return Some(hex::encode(xpub.chain_code.as_ref() as &[u8]));
        }
        self.active_xpriv()
            .map(|xpriv| hex::encode(xpriv.chain_code.as_ref() as &[u8]))
    }

    fn public_key(&self) -> Option<String> {
        if let Some(public_key) = self.active_leaf_public_key() {
            return Some(if self.compressed {
                hex::encode(public_key.serialize())
            } else {
                hex::encode(public_key.serialize_uncompressed())
            });
        }
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, self.compressed));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, self.compressed))
    }

    fn compressed(&self) -> Option<String> {
        if let Some(public_key) = self.active_leaf_public_key() {
            return Some(hex::encode(public_key.serialize()));
        }
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, true));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, true))
    }

    fn uncompressed(&self) -> Option<String> {
        if let Some(public_key) = self.active_leaf_public_key() {
            return Some(hex::encode(public_key.serialize_uncompressed()));
        }
        if let Some(xpub) = self.active_xpub() {
            return Some(self.xpub_public_key_bytes(xpub, false));
        }
        self.active_xpriv()
            .map(|xpriv| self.public_key_bytes(xpriv, false))
    }

    fn hash(&self) -> Option<String> {
        self.public_key_hash_bytes()
            .map(|bytes| hex::encode(hash160(&bytes)))
    }

    fn fingerprint(&self) -> Option<String> {
        self.hash().map(|hash| hash.chars().take(8).collect())
    }

    fn parent_fingerprint(&self) -> Option<String> {
        if self.import_mode != Bip32ImportMode::Extended {
            return None;
        }
        self.parent_fingerprint_bytes()
            .ok()
            .map(|bytes| hex::encode(bytes))
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

    fn index(&self) -> Option<u32> {
        self.indexes().and_then(|indexes| indexes.last().copied())
    }
}
