use bitcoin::Network;

use crate::hdwallet::addresses::iaddress::{EncodeKwargs, EncodePublicKey, IAddress};
use crate::hdwallet::addresses::{P2PKHAddress, P2WPKHAddress};
use crate::hdwallet::consts::{Modes, PublicKeyTypes};
use crate::hdwallet::derivations::{CustomDerivation, IDerivation};
use crate::hdwallet::exceptions::{Error, Result as HdResult};
use crate::hdwallet::hds::bip32::Bip32Hd;
use crate::hdwallet::hds::ihd::Ihd;

#[derive(Debug, Clone)]
pub struct ElectrumV2Hd {
    mode: String,
    bip32: Bip32Hd,
    change_index: u32,
    address_index: u32,
}

impl ElectrumV2Hd {
    pub fn new(
        network: Network,
        mode: &str,
        public_key_type: &str,
        wif_prefix: Option<u8>,
    ) -> HdResult<Self> {
        if mode != Modes::STANDARD && mode != Modes::SEGWIT {
            return Err(Error::new(
                "Invalid Electrum-V2 mode",
                None,
                Some(&Modes::get_modes().join(", ")),
                Some(mode),
            ));
        }
        let mut bip32 = Bip32Hd::from_seed(network, &[0u8; 32])?;
        bip32.configure(wif_prefix, public_key_type, false);
        Ok(Self {
            mode: mode.to_string(),
            bip32,
            change_index: 0,
            address_index: 0,
        })
    }

    pub fn from_seed_bytes(&mut self, seed: &[u8]) -> HdResult<()> {
        self.bip32.from_seed(seed)?;
        self.drive(self.change_index, self.address_index)
    }

    pub fn drive(&mut self, change_index: u32, address_index: u32) -> HdResult<()> {
        self.change_index = change_index;
        self.address_index = address_index;

        let path = if self.mode == Modes::SEGWIT {
            format!("m/0'/{change_index}/{address_index}")
        } else {
            format!("m/{change_index}/{address_index}")
        };
        let derivation = CustomDerivation::new(Some(&path), None)
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?;
        self.bip32.from_derivation(derivation.path())?;
        Ok(())
    }

    pub fn from_derivation_indices(&mut self, change_index: u32, address_index: u32) -> HdResult<()> {
        self.drive(change_index, address_index)
    }

    pub fn encode_address(
        &self,
        public_key_address_prefix: Option<u32>,
        hrp: Option<&'static str>,
        witness_version: Option<u8>,
    ) -> HdResult<String> {
        let public_key = self
            .bip32
            .public_key()
            .ok_or_else(|| Error::new("Public key is not available", None, None, None))?;
        let public_key_type = if self.bip32.public_key_type().as_deref() == Some(PublicKeyTypes::UNCOMPRESSED) {
            PublicKeyTypes::UNCOMPRESSED
        } else {
            PublicKeyTypes::COMPRESSED
        };
        let kwargs = EncodeKwargs {
            public_key_type: Some(public_key_type),
            public_key_address_prefix,
            hrp,
            witness_version,
            ..Default::default()
        };
        let input = EncodePublicKey::Str(&public_key);
        if self.mode == Modes::SEGWIT {
            P2WPKHAddress
                .encode(input, &kwargs)
                .map_err(|error| {
                    Error::new("Address encoding failed", Some(&error.to_string()), None, None)
                })
        } else {
            P2PKHAddress
                .encode(input, &kwargs)
                .map_err(|error| {
                    Error::new("Address encoding failed", Some(&error.to_string()), None, None)
                })
        }
    }
}

impl Ihd for ElectrumV2Hd {
    fn registry_name(&self) -> &'static str {
        "Electrum-V2"
    }

    fn from_seed(&mut self, seed: &[u8]) -> HdResult<()> {
        self.from_seed_bytes(seed)
    }

    fn from_derivation(&mut self, _derivation: &str) -> HdResult<()> {
        Ok(())
    }

    fn update_derivation(&mut self, _derivation: &str) -> HdResult<()> {
        Ok(())
    }

    fn seed_hex(&self) -> Option<String> {
        self.bip32.seed_hex()
    }

    fn mode(&self) -> Option<String> {
        Some(self.mode.clone())
    }

    fn public_key_type(&self) -> Option<String> {
        self.bip32.public_key_type()
    }

    fn wif_type(&self) -> Option<String> {
        self.bip32.wif_type()
    }

    fn master_private_key(&self) -> Option<String> {
        self.bip32.root_private_key()
    }

    fn master_wif(&self) -> Option<String> {
        self.bip32.root_wif()
    }

    fn master_public_key(&self) -> Option<String> {
        self.bip32.root_public_key()
    }

    fn private_key(&self) -> Option<String> {
        self.bip32.private_key()
    }

    fn wif(&self) -> Option<String> {
        self.bip32.wif()
    }

    fn public_key(&self) -> Option<String> {
        self.bip32.public_key()
    }

    fn uncompressed(&self) -> Option<String> {
        self.bip32.uncompressed()
    }

    fn compressed(&self) -> Option<String> {
        self.bip32.compressed()
    }
}
