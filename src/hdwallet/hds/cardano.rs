use num_bigint::BigUint;
use num_traits::{One, Zero};
use sha2::Digest;

use crate::hdwallet::addresses::CardanoAddress;
use crate::hdwallet::addresses::iaddress::{EncodeKwargs, EncodePublicKey, IAddress};
use crate::hdwallet::eccs::{
    IPrivateKey, IPublicKey, KholawEd25519Ecc, KholawEd25519PrivateKey, KholawEd25519PublicKey,
};
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::extended_keys::{
    cardano_xprivate_version, cardano_xpublic_version, deserialize_extended, fingerprint_from_public_key_hex,
    is_root_extended_key, serialize_extended,
};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::libs::ed25519::{point_add, point_scalar_mul_base, PointRef, COORD_BYTE_LEN};
use crate::hdwallet::seeds::{CardanoSeed, CardanoTypes};
use crate::hdwallet::utils::{
    add_no_carry, are_bits_set, get_hmac, hmac_sha256, hmac_sha512, multiply_scalar_no_carry,
    path_to_indexes, pbkdf2_hmac_sha512, sha512,
};

const HMAC_HALF: usize = 32;

#[derive(Debug, Clone)]
pub struct CardanoHd {
    cardano_type: String,
    network: String,
    seed: Option<Vec<u8>>,
    strict: bool,
    root_private: Option<KholawEd25519PrivateKey>,
    root_public: Option<KholawEd25519PublicKey>,
    root_chain_code: Option<[u8; COORD_BYTE_LEN]>,
    private_key: Option<KholawEd25519PrivateKey>,
    public_key: Option<KholawEd25519PublicKey>,
    chain_code: Option<[u8; COORD_BYTE_LEN]>,
    path: Option<String>,
    staking_public_key: Option<Vec<u8>>,
    address_type: Option<String>,
    depth: u8,
    index: u32,
    parent_fingerprint: [u8; 4],
    fingerprint: [u8; 4],
}

impl CardanoHd {
    pub fn new(cardano_type: &str, network: &str) -> HdResult<Self> {
        if !CardanoTypes::all().contains(&cardano_type) {
            return Err(Error::new(
                "Invalid Cardano type",
                None,
                Some(&CardanoTypes::all().join(", ")),
                Some(cardano_type),
            ));
        }
        Ok(Self {
            cardano_type: cardano_type.to_string(),
            network: network.to_string(),
            seed: None,
            strict: true,
            root_private: None,
            root_public: None,
            root_chain_code: None,
            private_key: None,
            public_key: None,
            chain_code: None,
            path: None,
            staking_public_key: None,
            address_type: None,
            depth: 0,
            index: 0,
            parent_fingerprint: [0u8; 4],
            fingerprint: [0u8; 4],
        })
    }

    pub fn set_staking_public_key(&mut self, value: Option<&[u8]>) {
        self.staking_public_key = value.map(|bytes| bytes.to_vec());
    }

    pub fn set_address_type(&mut self, value: Option<String>) {
        self.address_type = value;
    }

    pub fn from_seed_bytes(&mut self, seed: &[u8], passphrase: Option<&str>) -> HdResult<()> {
        self.seed = Some(seed.to_vec());
        match self.cardano_type.as_str() {
            CardanoTypes::BYRON_ICARUS | CardanoTypes::SHELLEY_ICARUS => {
                self.from_icarus_seed(seed, passphrase)
            }
            CardanoTypes::BYRON_LEDGER | CardanoTypes::SHELLEY_LEDGER => {
                self.from_ledger_seed(seed, passphrase)
            }
            CardanoTypes::BYRON_LEGACY => self.from_byron_legacy_seed(seed),
            other => Err(Error::new(
                &format!("Cardano {other} HD is not implemented yet"),
                None,
                None,
                Some(other),
            )),
        }
    }

    fn from_ledger_seed(&mut self, seed: &[u8], _passphrase: Option<&str>) -> HdResult<()> {
        if seed.len() < 16 {
            return Err(Error::new("Invalid seed length", None, Some("< 16"), None));
        }
        let hmac_key = get_hmac(KholawEd25519Ecc::NAME).ok_or_else(|| {
            Error::new("HMAC key is not configured for Cardano", None, None, None)
        })?;
        let mut hmac_data = seed.to_vec();
        let hmac = loop {
            let hmac = hmac_sha512(hmac_key, &hmac_data);
            if !are_bits_set(hmac[HMAC_HALF - 1], 0x20) {
                break hmac;
            }
            hmac_data = hmac.clone();
        };
        let kl = tweak_ledger_master_key(&hmac[..HMAC_HALF]);
        let kr = hmac[HMAC_HALF..].to_vec();
        let mut private_bytes = kl;
        private_bytes.extend_from_slice(&kr);
        let private = KholawEd25519PrivateKey::from_signing_and_extended(
            private_bytes[..COORD_BYTE_LEN].try_into().unwrap(),
            private_bytes[COORD_BYTE_LEN..].try_into().unwrap(),
        );
        let chain_code = hmac_sha256(hmac_key, &[&[0x01], seed].concat());
        self.root_private = Some(private.clone());
        self.root_public = Some(private.public_key());
        self.root_chain_code = Some(chain_code[..COORD_BYTE_LEN].try_into().unwrap());
        self.reset_derived_to_root();
        self.strict = true;
        Ok(())
    }

    fn from_byron_legacy_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        if seed.len() != 32 {
            return Err(Error::new("Invalid seed length", None, Some("32"), None));
        }
        let data = CardanoSeed::cbor_encode_bytes(seed).map_err(|error| {
            Error::new("CBOR encoding failed", Some(&error.to_string()), None, None)
        })?;
        let mut iteration = 1u32;
        let (il, ir) = loop {
            let message = format!("Root Seed Chain {iteration}");
            let i = hmac_sha512(&data, message.as_bytes());
            let il = tweak_ledger_master_key(&sha512(&i[..HMAC_HALF]));
            let ir = i[HMAC_HALF..].to_vec();
            if !are_bits_set(il[31], 0x20) {
                break (il, ir);
            }
            iteration += 1;
        };
        let private = KholawEd25519PrivateKey::from_signing_and_extended(
            il[..COORD_BYTE_LEN].try_into().unwrap(),
            il[COORD_BYTE_LEN..].try_into().unwrap(),
        );
        self.root_private = Some(private.clone());
        self.root_public = Some(private.public_key());
        self.root_chain_code = Some(ir[..COORD_BYTE_LEN].try_into().unwrap());
        self.reset_derived_to_root();
        self.strict = true;
        Ok(())
    }

    pub fn from_private_key_hex(&mut self, private_key: &str) -> HdResult<()> {
        if matches!(
            self.cardano_type.as_str(),
            CardanoTypes::BYRON_ICARUS | CardanoTypes::BYRON_LEGACY | CardanoTypes::BYRON_LEDGER
        ) {
            return Err(Error::new(
                &format!(
                    "Conversion from private key is not implemented for Cardano {} type",
                    self.cardano_type
                ),
                None,
                None,
                None,
            ));
        }
        let bytes = hex::decode(private_key.trim()).map_err(|error| {
            Error::new("Invalid private key hex", Some(&error.to_string()), None, None)
        })?;
        let private = KholawEd25519PrivateKey::from_bytes(&bytes).map_err(|error| {
            Error::new("Invalid private key", Some(&error.to_string()), None, None)
        })?;
        self.private_key = Some(private.clone());
        self.public_key = Some(private.public_key());
        self.strict = false;
        self.parent_fingerprint = [0u8; 4];
        self.update_fingerprint();
        Ok(())
    }

    pub fn from_public_key_hex(&mut self, public_key: &str) -> HdResult<()> {
        if matches!(
            self.cardano_type.as_str(),
            CardanoTypes::BYRON_ICARUS | CardanoTypes::BYRON_LEGACY | CardanoTypes::BYRON_LEDGER
        ) {
            return Err(Error::new(
                &format!(
                    "Conversion from public key is not implemented for Cardano {} type",
                    self.cardano_type
                ),
                None,
                None,
                None,
            ));
        }
        let bytes = hex::decode(public_key.trim()).map_err(|error| {
            Error::new("Invalid public key hex", Some(&error.to_string()), None, None)
        })?;
        let public = KholawEd25519PublicKey::from_bytes(&bytes).map_err(|error| {
            Error::new("Invalid public key", Some(&error.to_string()), None, None)
        })?;
        self.public_key = Some(public);
        self.private_key = None;
        self.strict = false;
        self.parent_fingerprint = [0u8; 4];
        self.update_fingerprint();
        Ok(())
    }

    pub fn path_key(&self) -> Option<String> {
        if self.cardano_type.as_str() != CardanoTypes::BYRON_LEGACY {
            return None;
        }
        let root_public = self.root_public.as_ref()?;
        let root_chain_code = self.root_chain_code?;
        let mut salt = root_public.encoded_point().to_vec();
        salt.extend_from_slice(&root_chain_code);
        Some(hex::encode(pbkdf2_hmac_sha512(
            &salt,
            b"address-hashing",
            500,
            32,
        )))
    }

    pub fn cardano_type(&self) -> &str {
        &self.cardano_type
    }

    fn from_icarus_seed(&mut self, seed: &[u8], passphrase: Option<&str>) -> HdResult<()> {
        if seed.len() < 16 {
            return Err(Error::new("Invalid seed length", None, Some("< 16"), None));
        }
        let mut pbkdf2 = pbkdf2_hmac_sha512(
            passphrase.unwrap_or("").as_bytes(),
            seed,
            4096,
            96,
        );
        tweak_icarus_master_key(&mut pbkdf2[..COORD_BYTE_LEN]);
        let signing: [u8; COORD_BYTE_LEN] = pbkdf2[..COORD_BYTE_LEN]
            .try_into()
            .map_err(|_| Error::new("Invalid signing key", None, None, None))?;
        let extended: [u8; COORD_BYTE_LEN] = pbkdf2[COORD_BYTE_LEN..COORD_BYTE_LEN * 2]
            .try_into()
            .map_err(|_| Error::new("Invalid extended key", None, None, None))?;
        let private = KholawEd25519PrivateKey::from_signing_and_extended(signing, extended);
        let chain_code: [u8; COORD_BYTE_LEN] = pbkdf2[COORD_BYTE_LEN * 2..]
            .try_into()
            .map_err(|_| Error::new("Invalid chain code", None, None, None))?;
        let public = private.public_key();
        self.root_private = Some(private.clone());
        self.root_public = Some(public);
        self.root_chain_code = Some(chain_code);
        self.reset_derived_to_root();
        self.strict = true;
        Ok(())
    }

    fn reset_derived_to_root(&mut self) {
        if let Some(root_private) = self.root_private.clone() {
            self.private_key = Some(root_private.clone());
            self.public_key = Some(root_private.public_key());
        }
        if let Some(root_chain_code) = self.root_chain_code {
            self.chain_code = Some(root_chain_code);
        }
        self.depth = 0;
        self.index = 0;
        self.parent_fingerprint = [0u8; 4];
        self.update_fingerprint();
        self.path = Some("m".to_string());
    }

    fn is_leaf_import(&self) -> bool {
        self.root_chain_code.is_none()
    }

    fn update_fingerprint(&mut self) {
        if let Some(public_key) = self.public_key.as_ref() {
            let hex_key = hex_public_key(public_key);
            if let Ok(fp) = fingerprint_from_public_key_hex(&hex_key) {
                self.fingerprint = fp;
            }
        }
    }

    pub fn from_xpublic_key(&mut self, xpublic_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        if self.cardano_type.as_str() == CardanoTypes::BYRON_LEGACY {
            return Err(Error::new(
                &format!(
                    "Conversion from xpublic key is not implemented for Cardano {} type",
                    self.cardano_type
                ),
                None,
                None,
                None,
            ));
        }
        let parts = deserialize_extended(xpublic_key, encoded)?;
        if parts.key.len() != 33 {
            return Err(Error::new("Invalid extended public key", None, None, None));
        }
        if strict && !is_root_extended_key(xpublic_key, encoded)? {
            return Err(Error::new("Invalid root extended public key", None, None, None));
        }

        let public = KholawEd25519PublicKey::from_bytes(&parts.key).map_err(|error| {
            Error::new("Invalid extended public key", Some(&error.to_string()), None, None)
        })?;

        self.root_public = Some(public.clone());
        self.root_chain_code = Some(parts.chain_code);
        self.root_private = None;
        self.public_key = Some(public);
        self.private_key = None;
        self.chain_code = Some(parts.chain_code);
        self.depth = parts.depth;
        self.index = parts.index;
        self.parent_fingerprint = parts.parent_fingerprint;
        self.update_fingerprint();
        self.path = Some("m".to_string());
        self.strict = is_root_extended_key(xpublic_key, encoded)?;
        Ok(())
    }

    pub fn from_xprivate_key(&mut self, xprivate_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        let parts = deserialize_extended(xprivate_key, encoded)?;
        if parts.key.len() != 65 || parts.key[0] != 0x00 {
            return Err(Error::new("Invalid extended private key", None, None, None));
        }
        if strict && !is_root_extended_key(xprivate_key, encoded)? {
            return Err(Error::new("Invalid root extended private key", None, None, None));
        }

        let private = KholawEd25519PrivateKey::from_bytes(&parts.key[1..]).map_err(|error| {
            Error::new("Invalid extended private key", Some(&error.to_string()), None, None)
        })?;
        let public = private.public_key();

        self.root_private = Some(private.clone());
        self.root_public = Some(public.clone());
        self.root_chain_code = Some(parts.chain_code);
        self.private_key = Some(private);
        self.public_key = Some(public);
        self.chain_code = Some(parts.chain_code);
        self.depth = parts.depth;
        self.index = parts.index;
        self.parent_fingerprint = parts.parent_fingerprint;
        self.update_fingerprint();
        self.path = Some("m".to_string());
        self.strict = is_root_extended_key(xprivate_key, encoded)?;
        Ok(())
    }

    fn xprivate_key_encoded(&self, version: u32, root: bool, encoded: bool) -> Option<String> {
        let private = if root {
            self.root_private.as_ref()?
        } else {
            self.private_key.as_ref()?
        };
        let chain_code = if root {
            self.root_chain_code?
        } else {
            self.chain_code?
        };
        let depth = if root { 0 } else { self.depth };
        let index = if root { 0 } else { self.index };
        let parent_fingerprint = if root {
            [0u8; 4]
        } else {
            self.parent_fingerprint
        };
        let mut key = vec![0x00];
        key.extend_from_slice(private.raw_bytes().as_ref());
        serialize_extended(
            version,
            depth,
            &parent_fingerprint,
            index,
            &chain_code,
            &key,
            encoded,
        )
        .ok()
    }

    fn xpublic_key_encoded(&self, version: u32, root: bool, encoded: bool) -> Option<String> {
        let public = if root {
            self.root_public.as_ref()?
        } else {
            self.public_key.as_ref()?
        };
        let chain_code = if root {
            self.root_chain_code?
        } else {
            self.chain_code?
        };
        let depth = if root { 0 } else { self.depth };
        let index = if root { 0 } else { self.index };
        let parent_fingerprint = if root {
            [0u8; 4]
        } else {
            self.parent_fingerprint
        };
        serialize_extended(
            version,
            depth,
            &parent_fingerprint,
            index,
            &chain_code,
            &public.raw_compressed(),
            encoded,
        )
        .ok()
    }

    pub fn derive_path(&mut self, path: &str) -> HdResult<()> {
        self.reset_derived_to_root();
        let indexes = path_to_indexes(path).map_err(|error| {
            Error::new("Invalid derivation path", Some(&error), None, None)
        })?;
        for index in indexes {
            self.drive(index)?;
        }
        self.path = Some(path.to_string());
        Ok(())
    }

    fn drive(&mut self, index: u32) -> HdResult<()> {
        let index_bytes = if self.cardano_type.as_str() == CardanoTypes::BYRON_LEGACY {
            index.to_be_bytes()
        } else {
            index.to_le_bytes()
        };

        if let Some(private) = self.private_key.clone() {
            let public = private.public_key();
            let chain_code = self
                .chain_code
                .ok_or_else(|| Error::new("Chain code is not set", None, None, None))?;

            let (z, h) = if index & 0x8000_0000 != 0 {
                let signing = private.raw_bytes();
                (
                    hmac_sha512(&chain_code, &[&[0x00], signing.as_ref(), &index_bytes].concat()),
                    hmac_sha512(&chain_code, &[&[0x01], signing.as_ref(), &index_bytes].concat()),
                )
            } else {
                let public_body = public.encoded_point();
                (
                    hmac_sha512(&chain_code, &[&[0x02], public_body.as_slice(), &index_bytes].concat()),
                    hmac_sha512(&chain_code, &[&[0x03], public_body.as_slice(), &index_bytes].concat()),
                )
            };

            let derived = if self.cardano_type.as_str() == CardanoTypes::BYRON_LEGACY {
                derive_child_private_legacy(&private, &z[..HMAC_HALF], &z[HMAC_HALF..])?
            } else {
                derive_child_private_icarus(&private, &z[..HMAC_HALF], &z[HMAC_HALF..])?
            };
            self.private_key = Some(derived.clone());
            self.public_key = Some(derived.public_key());
            self.chain_code = Some(h[HMAC_HALF..].try_into().unwrap());
            self.parent_fingerprint = self.fingerprint;
            self.depth += 1;
            self.index = index;
            self.update_fingerprint();
            return Ok(());
        }

        let public = self
            .public_key
            .as_ref()
            .ok_or_else(|| Error::new("Public key is not set", None, None, None))?
            .clone();
        let chain_code = self
            .chain_code
            .ok_or_else(|| Error::new("Chain code is not set", None, None, None))?;

        if index & 0x8000_0000 != 0 {
            return Err(Error::new(
                "Hardened derivation path is invalid for watch-only key",
                None,
                None,
                None,
            ));
        }

        let public_body = public.encoded_point();
        let z = hmac_sha512(&chain_code, &[&[0x02], public_body.as_slice(), &index_bytes].concat());
        let h = hmac_sha512(&chain_code, &[&[0x03], public_body.as_slice(), &index_bytes].concat());
        let child_public = if self.cardano_type.as_str() == CardanoTypes::BYRON_LEGACY {
            derive_child_public_legacy(&public, &z[..HMAC_HALF])?
        } else {
            derive_child_public_icarus(&public, &z[..HMAC_HALF])?
        };
        self.public_key = Some(child_public);
        self.private_key = None;
        self.chain_code = Some(h[HMAC_HALF..].try_into().unwrap());
        self.parent_fingerprint = self.fingerprint;
        self.depth += 1;
        self.index = index;
        self.update_fingerprint();
        Ok(())
    }

    pub fn staking_public_key_bytes(&self) -> HdResult<Vec<u8>> {
        if let Some(staking) = self.staking_public_key.as_ref() {
            return Ok(staking.clone());
        }

        let path = self
            .path
            .as_deref()
            .ok_or_else(|| Error::new("Derivation path is not set", None, None, None))?;
        let staking_path = staking_path_from_payment(path).ok_or_else(|| {
            Error::new(
                "staking_public_key is required for Shelley payment addresses",
                None,
                None,
                None,
            )
        })?;

        let seed = self
            .seed
            .as_ref()
            .ok_or_else(|| Error::new("Seed is not set", None, None, None))?;
        let mut staking_hd = CardanoHd::new(&self.cardano_type, &self.network)?;
        staking_hd.from_seed_bytes(seed, None)?;
        staking_hd.derive_path(&staking_path)?;
        let public_key = staking_hd
            .public_key
            .as_ref()
            .ok_or_else(|| Error::new("Staking public key is not available", None, None, None))?;
        Ok(public_key.encoded_point().to_vec())
    }

    pub fn encode_address(&self) -> HdResult<String> {
        let public_key = self
            .public_key
            .as_ref()
            .ok_or_else(|| Error::new("Public key is not available", None, None, None))?;

        let encode_type = match self.cardano_type.as_str() {
            CardanoTypes::BYRON_LEGACY => CardanoAddress::ENCODE_BYRON_LEGACY,
            CardanoTypes::SHELLEY_ICARUS | CardanoTypes::SHELLEY_LEDGER => {
                match self.address_type.as_deref() {
                    Some("staking") | Some("reward") => CardanoAddress::ENCODE_STAKING,
                    _ => CardanoAddress::ENCODE_PAYMENT,
                }
            }
            CardanoTypes::BYRON_ICARUS | CardanoTypes::BYRON_LEDGER => CardanoAddress::ENCODE_BYRON_ICARUS,
            _ => CardanoAddress::ENCODE_BYRON_ICARUS,
        };

        let chain_code = match encode_type {
            CardanoAddress::ENCODE_BYRON_LEGACY | CardanoAddress::ENCODE_BYRON_ICARUS => {
                Some(
                    self.chain_code
                        .ok_or_else(|| Error::new("Chain code is not available", None, None, None))?,
                )
            }
            _ => self.chain_code,
        };

        let path_key_bytes = self
            .path_key()
            .and_then(|value| hex::decode(value).ok());
        let path_key_ref = path_key_bytes.as_deref();

        let staking_public_key = if encode_type == CardanoAddress::ENCODE_PAYMENT {
            Some(self.staking_public_key_bytes()?)
        } else {
            None
        };

        let kwargs = EncodeKwargs {
            network: Some(if self.network == "testnet" {
                "testnet"
            } else {
                "mainnet"
            }),
            encode_type: Some(encode_type),
            address_type: cardano_address_type_label(self.address_type.as_deref()),
            staking_public_key: staking_public_key.as_deref(),
            chain_code: chain_code.as_ref().map(|code| code.as_slice()),
            path: self.path.as_deref(),
            path_key: path_key_ref,
            ..Default::default()
        };

        CardanoAddress
            .encode(
                EncodePublicKey::Bytes(public_key.raw_compressed().as_slice()),
                &kwargs,
            )
            .map_err(|error: crate::hdwallet::addresses::iaddress::AddressSchemeError| {
                Error::new("Address encoding failed", Some(&error.to_string()), None, None)
            })
    }
}

impl Ihd for CardanoHd {
    fn registry_name(&self) -> &'static str {
        "Cardano"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        self.from_seed_bytes(seed, None)
    }

    fn from_private_key(&mut self, private_key: &str) -> HdResult<()> {
        self.from_private_key_hex(private_key)
    }

    fn from_public_key(&mut self, public_key: &str) -> HdResult<()> {
        self.from_public_key_hex(public_key)
    }

    fn from_xprivate_key(&mut self, xprivate_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        self.from_xprivate_key(xprivate_key, encoded, strict)
    }

    fn from_xpublic_key(&mut self, xpublic_key: &str, encoded: bool, strict: bool) -> HdResult<()> {
        self.from_xpublic_key(xpublic_key, encoded, strict)
    }

    fn path_key(&self) -> Option<String> {
        CardanoHd::path_key(self)
    }

    fn from_derivation(&mut self, derivation: &str) -> HdResult<()> {
        self.derive_path(derivation)
    }

    fn update_derivation(&mut self, derivation: &str) -> HdResult<()> {
        self.derive_path(derivation)
    }

    fn seed_hex(&self) -> Option<String> {
        self.seed.as_ref().map(hex::encode)
    }

    fn strict(&self) -> Option<bool> {
        if self.is_leaf_import() {
            return None;
        }
        Some(self.strict)
    }

    fn root_xprivate_key(&self, encoded: bool) -> Option<String> {
        self.xprivate_key_encoded(cardano_xprivate_version(&self.network), true, encoded)
    }

    fn root_xpublic_key(&self, encoded: bool) -> Option<String> {
        self.xpublic_key_encoded(cardano_xpublic_version(&self.network), true, encoded)
    }

    fn xprivate_key(&self, encoded: bool) -> Option<String> {
        self.xprivate_key_encoded(cardano_xprivate_version(&self.network), false, encoded)
    }

    fn xpublic_key(&self, encoded: bool) -> Option<String> {
        self.xpublic_key_encoded(cardano_xpublic_version(&self.network), false, encoded)
    }

    fn root_private_key(&self) -> Option<String> {
        self.root_private
            .as_ref()
            .map(|key| hex::encode(key.raw_bytes().as_ref()))
    }

    fn root_public_key(&self) -> Option<String> {
        self.root_public.as_ref().map(hex_public_key)
    }

    fn root_chain_code(&self) -> Option<String> {
        self.root_chain_code
            .as_ref()
            .map(|code| hex::encode(code))
    }

    fn private_key(&self) -> Option<String> {
        self.private_key
            .as_ref()
            .map(|key| hex::encode(key.raw_bytes().as_ref()))
    }

    fn public_key(&self) -> Option<String> {
        self.public_key.as_ref().map(hex_public_key)
    }

    fn hash(&self) -> Option<String> {
        self.public_key
            .as_ref()
            .map(|key| hex::encode(hash160(key.raw_compressed().as_slice())))
    }

    fn chain_code(&self) -> Option<String> {
        self.chain_code
            .as_ref()
            .map(|code| hex::encode(code))
    }

    fn path(&self) -> Option<String> {
        self.path.clone()
    }

    fn fingerprint(&self) -> Option<String> {
        Some(hex::encode(self.fingerprint))
    }

    fn parent_fingerprint(&self) -> Option<String> {
        if self.is_leaf_import() {
            return None;
        }
        Some(hex::encode(self.parent_fingerprint))
    }

    fn depth(&self) -> Option<u32> {
        Some(self.depth as u32)
    }

    fn index(&self) -> Option<u32> {
        Some(self.index)
    }

    fn address(&self) -> Option<String> {
        self.encode_address().ok()
    }
}

fn tweak_icarus_master_key(key: &mut [u8]) {
    key[0] &= 0b1111_1000;
    key[31] &= 0b0001_1111;
    key[31] |= 0b0100_0000;
}

fn tweak_ledger_master_key(key: &[u8]) -> Vec<u8> {
    let mut out = key.to_vec();
    out[0] &= 0b1111_1000;
    out[31] &= 0b0111_1111;
    out[31] |= 0b0100_0000;
    out
}

fn le_bytes_to_uint256(bytes: &[u8]) -> BigUint {
    let mut padded = [0u8; 32];
    let len = bytes.len().min(32);
    padded[..len].copy_from_slice(&bytes[..len]);
    BigUint::from_bytes_le(&padded)
}

fn uint256_to_le_32(value: &BigUint) -> [u8; 32] {
    let modulus = BigUint::one() << 256usize;
    let reduced = value % modulus;
    let bytes = reduced.to_bytes_le();
    let mut out = [0u8; 32];
    let len = bytes.len().min(32);
    out[..len].copy_from_slice(&bytes[..len]);
    out
}

fn derive_child_private_icarus(
    parent: &KholawEd25519PrivateKey,
    zl: &[u8],
    zr: &[u8],
) -> HdResult<KholawEd25519PrivateKey> {
    let parent_raw = parent.raw_bytes();
    let kl = &parent_raw[..COORD_BYTE_LEN];
    let kr = &parent_raw[COORD_BYTE_LEN..];

    let zl_int = le_bytes_to_uint256(&zl[..28.min(zl.len())]);
    let kl_int = le_bytes_to_uint256(kl);
    let left = (zl_int * BigUint::from(8u32)) + kl_int;
    if &left % KholawEd25519Ecc::order() == BigUint::zero() {
        return Err(Error::new("Computed child key is not valid", None, None, None));
    }

    let zr_int = le_bytes_to_uint256(zr);
    let kr_int = le_bytes_to_uint256(kr);
    let right = (zr_int + kr_int) % (BigUint::one() << 256usize);

    let mut bytes = [0u8; COORD_BYTE_LEN * 2];
    bytes[..COORD_BYTE_LEN].copy_from_slice(&uint256_to_le_32(&left));
    bytes[COORD_BYTE_LEN..].copy_from_slice(&uint256_to_le_32(&right));
    let signing: [u8; COORD_BYTE_LEN] = bytes[..COORD_BYTE_LEN].try_into().unwrap();
    let extended: [u8; COORD_BYTE_LEN] = bytes[COORD_BYTE_LEN..].try_into().unwrap();
    Ok(KholawEd25519PrivateKey::from_signing_and_extended(signing, extended))
}

fn derive_child_public_icarus(
    parent: &KholawEd25519PublicKey,
    zl: &[u8],
) -> HdResult<KholawEd25519PublicKey> {
    let zl_int = le_bytes_to_uint256(&zl[..28.min(zl.len())]);
    let scalar = uint256_to_le_32(&(zl_int * BigUint::from(8u32)));
    let tweak = point_scalar_mul_base(&scalar);
    let child = point_add(
        PointRef::Encoded(parent.encoded_point()),
        PointRef::Encoded(&tweak),
    )
    .map_err(|error| {
        Error::new("Invalid derived public key", Some(&error.to_string()), None, None)
    })?;
    if child == [0u8; 32] {
        return Err(Error::new(
            "Computed public child key is not valid",
            None,
            None,
            None,
        ));
    }
    Ok(KholawEd25519PublicKey {
        enc: child,
        _v: std::marker::PhantomData,
    })
}

fn derive_child_private_legacy(
    parent: &KholawEd25519PrivateKey,
    zl: &[u8],
    zr: &[u8],
) -> HdResult<KholawEd25519PrivateKey> {
    let parent_raw = parent.raw_bytes();
    let kl = &parent_raw[..HMAC_HALF];
    let kr = &parent_raw[HMAC_HALF..];
    let zl_int = le_bytes_to_uint256(&multiply_scalar_no_carry(zl, 8));
    let kl_int = le_bytes_to_uint256(kl);
    let left = uint256_to_le_32(&((zl_int + kl_int) % KholawEd25519Ecc::order()));
    let right = add_no_carry(zr, kr);
    KholawEd25519PrivateKey::from_bytes(&[left.to_vec(), right].concat()).map_err(|error| {
        Error::new("Invalid derived private key", Some(&error.to_string()), None, None)
    })
}

fn derive_child_public_legacy(
    parent: &KholawEd25519PublicKey,
    zl: &[u8],
) -> HdResult<KholawEd25519PublicKey> {
    let zl_int = le_bytes_to_uint256(&multiply_scalar_no_carry(zl, 8));
    let scalar = uint256_to_le_32(&zl_int);
    let tweak = point_scalar_mul_base(&scalar);
    let child = point_add(
        PointRef::Encoded(parent.encoded_point()),
        PointRef::Encoded(&tweak),
    )
    .map_err(|error| {
        Error::new("Invalid derived public key", Some(&error.to_string()), None, None)
    })?;
    if child == [0u8; 32] {
        return Err(Error::new(
            "Computed public child key is not valid",
            None,
            None,
            None,
        ));
    }
    Ok(KholawEd25519PublicKey {
        enc: child,
        _v: std::marker::PhantomData,
    })
}

fn hex_public_key(public_key: &KholawEd25519PublicKey) -> String {
    hex::encode(public_key.raw_compressed())
}

fn hash160(data: &[u8]) -> Vec<u8> {
    let digest = sha2::Sha256::digest(data);
    crate::hdwallet::libs::ripemd160::ripemd160(&digest).to_vec()
}

fn cardano_address_type_label(address_type: Option<&str>) -> Option<&'static str> {
    match address_type {
        Some("payment") => Some("payment"),
        Some("staking") => Some("staking"),
        Some("reward") => Some("reward"),
        Some("public-key") => Some("public-key"),
        Some("redemption") => Some("redemption"),
        _ => None,
    }
}

fn staking_path_from_payment(path: &str) -> Option<String> {
    let indexes = path_to_indexes(path).ok()?;
    if indexes.len() != 5 {
        return None;
    }
    let mut parts: Vec<String> = indexes
        .iter()
        .map(|index| {
            if *index & 0x8000_0000 != 0 {
                format!("{}'", index & !0x8000_0000)
            } else {
                index.to_string()
            }
        })
        .collect();
    parts[3] = "2".to_string();
    Some(format!("m/{}", parts.join("/")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byron_legacy_seed_fixture() {
        let seed = hex::decode(
            "cce7ebb0de741cb8829075d35bb2e7cacf21d994e50a1e68635498b4007e03af",
        )
        .unwrap();
        let mut hd = CardanoHd::new("byron-legacy", "mainnet").unwrap();
        hd.set_address_type(Some("public-key".to_string()));
        hd.from_seed_bytes(&seed, None).unwrap();
        assert_eq!(
            hd.root_private_key().as_deref(),
            Some("1850d3bdca31799bb2b43814227f921c04eb92c0ed39d9f0c31067bb4014904e141bd05618158045ecbe7fbd25e2954c60fa86c366b28e859ffc875f31b599a2")
        );
        assert_eq!(
            hd.root_chain_code().as_deref(),
            Some("6076a1706cdc45faed7fd7626b6ad4dbcdd1f5a19d1872c3b9de0884ddb99724")
        );
        assert_eq!(
            hd.root_public_key().as_deref(),
            Some("003ac7e2918f8565adf9c16b25bb147a260e35bd1f7f4bc10839dbcf5ee5d53906")
        );
        assert_eq!(
            hd.path_key().as_deref(),
            Some("aaf5ba7df4510e29d1128b4dd75c67fd9c93118b0944a6f04c93149bbae7c676")
        );
        hd.derive_path("m/1852'/1815'/0'/0/0").unwrap();
        assert_eq!(
            hd.public_key().as_deref(),
            Some("00dec61c3987a6826a1b5b50c077c4747cec92a376450934aff0e3d2d22f1ad051")
        );
        assert_eq!(
            hd.chain_code().as_deref(),
            Some("f9c5be63fb16428b341e7335e55637bdea3e216ad671f354cf5709017f8a8817")
        );
        assert_eq!(
            hd.encode_address().unwrap(),
            "37btjrVyb4KFsdvxLakJZbS1HPVZwfrAGD8e1ZWYKvhkp6DXAvzDiA9inYcaGkpbuaErq2fMjwTyUwU8KLhVxrCHaVyBXU5ZABMTCo4wiZfZUVfoGj"
        );
    }

    #[test]
    fn byron_icarus_payment_address() {
        let seed = hex::decode("46cee0c87543fb17ac72fc1a4f545dac0f533fe67c7a48c51c46c401").unwrap();
        let mut hd = CardanoHd::new("byron-icarus", "mainnet").unwrap();
        hd.set_address_type(Some("public-key".to_string()));
        hd.from_seed_bytes(&seed, None).unwrap();
        hd.derive_path("m/1852'/1815'/0'/0/0").unwrap();
        let addr = hd.encode_address().unwrap();
        assert_eq!(
            addr,
            "Ae2tdPwUPEZHRJzGNXCFGU8N4KJj91cGVfmobV26Jmm3GcrVfnyMAtkTCGs"
        );
    }
}
