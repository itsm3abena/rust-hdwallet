use sha3::{Digest, Keccak256};

use crate::hdwallet::addresses::{
    MoneroAddress, ADDRESS_TYPE_INTEGRATED, ADDRESS_TYPE_STANDARD, ADDRESS_TYPE_SUB_ADDRESS,
};
use crate::hdwallet::eccs::{
    EccsError, IPrivateKey, IPublicKey, IPoint, PublicKeyInput, Slip10Ed25519MoneroPoint,
    Slip10Ed25519MoneroPrivateKey, Slip10Ed25519MoneroPublicKey,
};
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::libs::ed25519::{int_decode, scalar_reduce, G_ENC_BYTES, COORD_BYTE_LEN};

#[derive(Debug, Clone)]
pub struct MoneroHd {
    network: String,
    seed: Option<Vec<u8>>,
    private_key: Option<Vec<u8>>,
    spend_private: Option<Slip10Ed25519MoneroPrivateKey>,
    view_private: Option<Slip10Ed25519MoneroPrivateKey>,
    spend_public: Option<Slip10Ed25519MoneroPublicKey>,
    view_public: Option<Slip10Ed25519MoneroPublicKey>,
    minor_index: u32,
    major_index: u32,
}

impl MoneroHd {
    pub fn new(network: &str) -> HdResult<Self> {
        Ok(Self {
            network: network.to_string(),
            seed: None,
            private_key: None,
            spend_private: None,
            view_private: None,
            spend_public: None,
            view_public: None,
            minor_index: 1,
            major_index: 0,
        })
    }

    pub fn from_seed_bytes(&mut self, seed: &[u8]) -> HdResult<()> {
        self.seed = Some(seed.to_vec());
        let spend = if seed.len() == COORD_BYTE_LEN {
            seed.to_vec()
        } else {
            keccak256(seed).to_vec()
        };
        self.from_spend_private_key_bytes(&spend)
    }

    pub fn from_private_key_hex(&mut self, private_key: &str) -> HdResult<()> {
        let bytes = hex::decode(private_key.trim()).map_err(|error| {
            Error::new("Invalid private key hex", Some(&error.to_string()), None, None)
        })?;
        self.private_key = Some(bytes.clone());
        let spend = keccak256(&bytes);
        self.from_spend_private_key_bytes(spend.as_slice())
    }

    pub fn from_spend_private_key_hex(&mut self, spend_private_key: &str) -> HdResult<()> {
        let bytes = hex::decode(spend_private_key.trim()).map_err(|error| {
            Error::new("Invalid spend private key hex", Some(&error.to_string()), None, None)
        })?;
        self.from_spend_private_key_bytes(&bytes)
    }

    fn from_spend_private_key_bytes(&mut self, spend_private_key: &[u8]) -> HdResult<()> {
        let reduced = scalar_reduce(spend_private_key);
        let spend_private = Slip10Ed25519MoneroPrivateKey::from_bytes(&reduced)
            .map_err(map_ecc_error)?;
        let view_bytes = scalar_reduce(&keccak256(spend_private.raw_bytes().as_ref()));
        let view_private = Slip10Ed25519MoneroPrivateKey::from_bytes(&view_bytes)
            .map_err(map_ecc_error)?;
        self.spend_private = Some(spend_private.clone());
        self.view_private = Some(view_private.clone());
        self.spend_public = Some(spend_private.public_key());
        self.view_public = Some(view_private.public_key());
        Ok(())
    }

    pub fn from_watch_only_keys(
        &mut self,
        view_private_key: &str,
        spend_public_key: &str,
    ) -> HdResult<()> {
        let view_bytes = hex::decode(view_private_key.trim()).map_err(|error| {
            Error::new("Invalid view private key hex", Some(&error.to_string()), None, None)
        })?;
        let spend_bytes = hex::decode(spend_public_key.trim()).map_err(|error| {
            Error::new("Invalid spend public key hex", Some(&error.to_string()), None, None)
        })?;
        let view_private = Slip10Ed25519MoneroPrivateKey::from_bytes(&view_bytes)
            .map_err(map_ecc_error)?;
        let spend_public = Slip10Ed25519MoneroPublicKey::from_bytes(&spend_bytes)
            .map_err(map_ecc_error)?;
        self.spend_private = None;
        self.view_private = Some(view_private.clone());
        self.spend_public = Some(spend_public);
        self.view_public = Some(view_private.public_key());
        Ok(())
    }

    pub fn set_derivation_indices(&mut self, minor: u32, major: u32) {
        self.minor_index = minor;
        self.major_index = major;
    }

    fn spend_public(&self) -> HdResult<&Slip10Ed25519MoneroPublicKey> {
        self.spend_public
            .as_ref()
            .ok_or_else(|| Error::new("Spend public key is not set", None, None, None))
    }

    fn view_private(&self) -> HdResult<&Slip10Ed25519MoneroPrivateKey> {
        self.view_private
            .as_ref()
            .ok_or_else(|| Error::new("View private key is not set", None, None, None))
    }

    fn view_public(&self) -> HdResult<&Slip10Ed25519MoneroPublicKey> {
        self.view_public
            .as_ref()
            .ok_or_else(|| Error::new("View public key is not set", None, None, None))
    }

    fn drive(
        &self,
        minor_index: u32,
        major_index: u32,
    ) -> HdResult<(Slip10Ed25519MoneroPublicKey, Slip10Ed25519MoneroPublicKey)> {
        if minor_index == 0 && major_index == 0 {
            return Ok((self.spend_public()?.clone(), self.view_public()?.clone()));
        }

        let mut payload = Vec::with_capacity(7 + COORD_BYTE_LEN + 8);
        payload.extend_from_slice(b"SubAddr\x00");
        payload.extend_from_slice(self.view_private()?.raw_bytes().as_ref());
        payload.extend_from_slice(&major_index.to_le_bytes());
        payload.extend_from_slice(&minor_index.to_le_bytes());

        let reduced = scalar_reduce(&keccak256(&payload));
        let m = int_decode(&reduced);
        let spend_point = self.spend_public()?.point();
        let generator = Slip10Ed25519MoneroPoint::from_bytes(G_ENC_BYTES.as_slice())
            .map_err(map_ecc_error)?;
        let offset = generator.mul_scalar(&m).map_err(map_ecc_error)?;
        let sub_spend_point = spend_point.add(&offset).map_err(map_ecc_error)?;

        let view_scalar = int_decode(
            self.view_private()?
                .raw_bytes()
                .as_ref()
                .try_into()
                .map_err(|_| Error::new("Invalid view private key", None, None, None))?,
        );
        let sub_view_point = sub_spend_point.mul_scalar(&view_scalar).map_err(map_ecc_error)?;

        Ok((
            Slip10Ed25519MoneroPublicKey::from_point(&sub_spend_point).map_err(map_ecc_error)?,
            Slip10Ed25519MoneroPublicKey::from_point(&sub_view_point).map_err(map_ecc_error)?,
        ))
    }

    pub fn primary_address(&self) -> HdResult<String> {
        MoneroAddress::encode(
            PublicKeyInput::Instance(self.spend_public()?.clone()),
            PublicKeyInput::Instance(self.view_public()?.clone()),
            &self.network,
            ADDRESS_TYPE_STANDARD,
            None,
        )
        .map_err(|error| Error::new("Address encoding failed", Some(&error.to_string()), None, None))
    }

    pub fn integrated_address(&self, payment_id: &[u8]) -> HdResult<String> {
        MoneroAddress::encode(
            PublicKeyInput::Instance(self.spend_public()?.clone()),
            PublicKeyInput::Instance(self.view_public()?.clone()),
            &self.network,
            ADDRESS_TYPE_INTEGRATED,
            Some(payment_id),
        )
        .map_err(|error| Error::new("Address encoding failed", Some(&error.to_string()), None, None))
    }

    pub fn sub_address(&self, minor: u32, major: u32) -> HdResult<String> {
        if minor == 0 && major == 0 {
            return self.primary_address();
        }
        let (spend_public, view_public) = self.drive(minor, major)?;
        MoneroAddress::encode(
            PublicKeyInput::Instance(spend_public),
            PublicKeyInput::Instance(view_public),
            &self.network,
            ADDRESS_TYPE_SUB_ADDRESS,
            None,
        )
        .map_err(|error| Error::new("Address encoding failed", Some(&error.to_string()), None, None))
    }

    pub fn sub_address_for_indices(&self) -> HdResult<String> {
        self.sub_address(self.minor_index, self.major_index)
    }
}

impl Ihd for MoneroHd {
    fn registry_name(&self) -> &'static str {
        "Monero"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        self.from_seed_bytes(seed)
    }

    fn from_private_key(&mut self, private_key: &str) -> HdResult<()> {
        self.from_private_key_hex(private_key)
    }

    fn from_spend_private_key(&mut self, spend_private_key: &str) -> HdResult<()> {
        self.from_spend_private_key_hex(spend_private_key)
    }

    fn from_watch_only(
        &mut self,
        view_private_key: &str,
        spend_public_key: &str,
    ) -> HdResult<()> {
        self.from_watch_only_keys(view_private_key, spend_public_key)
    }

    fn seed_hex(&self) -> Option<String> {
        self.seed.as_ref().map(hex::encode)
    }

    fn private_key(&self) -> Option<String> {
        self.private_key.as_ref().map(hex::encode)
    }

    fn spend_private_key(&self) -> Option<String> {
        self.spend_private
            .as_ref()
            .map(|key| hex::encode(key.raw_bytes().as_ref()))
    }

    fn view_private_key(&self) -> Option<String> {
        self.view_private
            .as_ref()
            .map(|key| hex::encode(key.raw_bytes().as_ref()))
    }

    fn spend_public_key(&self) -> Option<String> {
        self.spend_public
            .as_ref()
            .map(|key| hex::encode(key.raw_compressed()))
    }

    fn view_public_key(&self) -> Option<String> {
        self.view_public
            .as_ref()
            .map(|key| hex::encode(key.raw_compressed()))
    }

    fn primary_address(&self) -> Option<String> {
        MoneroHd::primary_address(self).ok()
    }

    fn integrated_address(&self) -> Option<String> {
        None
    }

    fn sub_address(&self) -> Option<String> {
        self.sub_address_for_indices().ok()
    }

    fn address(&self) -> Option<String> {
        MoneroHd::primary_address(self).ok()
    }
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn map_ecc_error(error: EccsError) -> Error {
    Error::new("Invalid Monero key", Some(&error.to_string()), None, None)
}
