use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::{PublicKey as SecpPublicKey, SecretKey};
use num_bigint::BigUint;
use sha2::{Digest, Sha256};

use crate::hdwallet::addresses::iaddress::{EncodeKwargs, EncodePublicKey, IAddress};
use crate::hdwallet::addresses::P2PKHAddress;
use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::wif::{decode_wif, encode_wif};

const SECP256K1_ORDER: &str =
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

#[derive(Debug, Clone)]
pub struct ElectrumV1Hd {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    wif_prefix: Option<u8>,
    compressed: bool,
    seed: Option<Vec<u8>>,
    master_private: Option<SecretKey>,
    master_public: SecpPublicKey,
    derived_private: Option<SecretKey>,
    derived_public: SecpPublicKey,
    change_index: u32,
    address_index: u32,
}

impl ElectrumV1Hd {
    pub fn new(public_key_type: &str, wif_prefix: Option<u8>) -> HdResult<Self> {
        let compressed = public_key_type != PublicKeyTypes::UNCOMPRESSED;
        let secp = Secp256k1::new();
        let master_public = SecpPublicKey::from_secret_key(&secp, &SecretKey::from_slice(&[1u8; 32]).map_err(|error| {
            Error::new("Invalid default key", Some(&error.to_string()), None, None)
        })?);
        Ok(Self {
            secp,
            wif_prefix,
            compressed,
            seed: None,
            master_private: None,
            master_public,
            derived_private: None,
            derived_public: master_public,
            change_index: 0,
            address_index: 0,
        })
    }

    pub fn from_seed_bytes(&mut self, seed: &[u8]) -> HdResult<()> {
        self.seed = Some(seed.to_vec());
        self.from_private_key_bytes(seed)
    }

    pub fn from_private_key_hex(&mut self, private_key: &str) -> HdResult<()> {
        let bytes = hex::decode(private_key.trim()).map_err(|error| {
            Error::new("Invalid private key hex", Some(&error.to_string()), None, None)
        })?;
        self.from_private_key_bytes(&bytes)
    }

    pub fn from_wif_str(&mut self, wif: &str) -> HdResult<()> {
        let prefix = self.wif_prefix.ok_or_else(|| {
            Error::new("WIF prefix is not configured for this network", None, None, None)
        })?;
        let (private_key, wif_type, _) = decode_wif(wif, prefix).map_err(|error| {
            Error::new("Invalid WIF", Some(&error.to_string()), None, None)
        })?;
        self.compressed = wif_type == "wif-compressed";
        self.from_private_key_bytes(&private_key)
    }

    fn from_private_key_bytes(&mut self, private_key: &[u8]) -> HdResult<()> {
        let secret = SecretKey::from_slice(private_key).map_err(|error| {
            Error::new("Invalid private key", Some(&error.to_string()), None, None)
        })?;
        self.master_private = Some(secret);
        self.master_public = SecpPublicKey::from_secret_key(&self.secp, &secret);
        self.drive(self.change_index, self.address_index)
    }

    pub fn drive(&mut self, change_index: u32, address_index: u32) -> HdResult<()> {
        self.change_index = change_index;
        self.address_index = address_index;

        let uncompressed = self.master_public.serialize_uncompressed();
        let body = &uncompressed[1..];
        let sequence_input = format!("{address_index}:{change_index}:");
        let mut payload = sequence_input.into_bytes();
        payload.extend_from_slice(body);
        let sequence = double_sha256(&payload);

        if let Some(master_private) = self.master_private {
            let private_int = bytes_to_biguint(master_private.secret_bytes().as_ref());
            let sequence_int = bytes_to_biguint(&sequence);
            let order = BigUint::parse_bytes(SECP256K1_ORDER.as_bytes(), 16).unwrap();
            let derived_int = (private_int + sequence_int) % &order;
            let derived_bytes = biguint_to_32_bytes(&derived_int);
            let derived_secret = SecretKey::from_slice(&derived_bytes).map_err(|error| {
                Error::new("Invalid derived private key", Some(&error.to_string()), None, None)
            })?;
            self.derived_private = Some(derived_secret);
            self.derived_public = SecpPublicKey::from_secret_key(&self.secp, &derived_secret);
        } else {
            let sequence_int = bytes_to_biguint(&sequence);
            let mut scalar = [0u8; 32];
            let seq_bytes = biguint_to_32_bytes(&sequence_int);
            scalar.copy_from_slice(&seq_bytes);
            let offset = PublicKey::from_secret_key(&self.secp, &SecretKey::from_slice(&scalar).map_err(|error| {
                Error::new("Invalid sequence scalar", Some(&error.to_string()), None, None)
            })?);
            self.derived_public = self.master_public.combine(&offset).map_err(|error| {
                Error::new("Public key derivation failed", Some(&error.to_string()), None, None)
            })?;
            self.derived_private = None;
        }
        Ok(())
    }

    pub fn from_derivation_indices(&mut self, change_index: u32, address_index: u32) -> HdResult<()> {
        self.drive(change_index, address_index)
    }

    pub fn encode_address(
        &self,
        public_key_address_prefix: Option<u32>,
    ) -> HdResult<String> {
        let public_key_type = if self.compressed {
            PublicKeyTypes::COMPRESSED
        } else {
            PublicKeyTypes::UNCOMPRESSED
        };
        let kwargs = EncodeKwargs {
            public_key_type: Some(public_key_type),
            public_key_address_prefix,
            ..Default::default()
        };
        P2PKHAddress
            .encode(
                EncodePublicKey::Str(&hex::encode(self.derived_public.serialize())),
                &kwargs,
            )
            .map_err(|error| Error::new("Address encoding failed", Some(&error.to_string()), None, None))
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

    fn public_key_hex(&self, key: &SecpPublicKey, compressed: bool) -> String {
        if compressed {
            hex::encode(key.serialize())
        } else {
            hex::encode(key.serialize_uncompressed())
        }
    }
}

use bitcoin::secp256k1::PublicKey;

impl Ihd for ElectrumV1Hd {
    fn registry_name(&self) -> &'static str {
        "Electrum-V1"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        self.from_seed_bytes(seed)
    }

    fn from_wif(&mut self, wif: &str) -> HdResult<()> {
        self.from_wif_str(wif)
    }

    fn from_private_key(&mut self, private_key: &str) -> HdResult<()> {
        self.from_private_key_hex(private_key)
    }

    fn from_derivation(&mut self, _derivation: &str) -> HdResult<()> {
        Ok(())
    }

    fn update_derivation(&mut self, _derivation: &str) -> HdResult<()> {
        Ok(())
    }

    fn seed_hex(&self) -> Option<String> {
        self.seed.as_ref().map(hex::encode)
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

    fn master_private_key(&self) -> Option<String> {
        self.master_private
            .as_ref()
            .map(|key| hex::encode(key.secret_bytes()))
    }

    fn master_wif(&self) -> Option<String> {
        self.master_private
            .as_ref()
            .and_then(|key| self.encode_wif_for_key(&key.secret_bytes()).ok())
    }

    fn master_public_key(&self) -> Option<String> {
        Some(self.public_key_hex(&self.master_public, self.compressed))
    }

    fn private_key(&self) -> Option<String> {
        self.derived_private
            .as_ref()
            .map(|key| hex::encode(key.secret_bytes()))
    }

    fn wif(&self) -> Option<String> {
        self.derived_private
            .as_ref()
            .and_then(|key| self.encode_wif_for_key(&key.secret_bytes()).ok())
    }

    fn public_key(&self) -> Option<String> {
        Some(self.public_key_hex(&self.derived_public, self.compressed))
    }

    fn uncompressed(&self) -> Option<String> {
        Some(self.public_key_hex(&self.derived_public, false))
    }

    fn compressed(&self) -> Option<String> {
        Some(self.public_key_hex(&self.derived_public, true))
    }

    fn address(&self) -> Option<String> {
        None
    }
}

fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first = Sha256::digest(data);
    Sha256::digest(first).to_vec()
}

fn bytes_to_biguint(bytes: &[u8]) -> BigUint {
    BigUint::from_bytes_be(bytes)
}

fn biguint_to_32_bytes(value: &BigUint) -> [u8; 32] {
    let bytes = value.to_bytes_be();
    let mut out = [0u8; 32];
    let start = 32usize.saturating_sub(bytes.len());
    out[start..].copy_from_slice(&bytes);
    out
}
