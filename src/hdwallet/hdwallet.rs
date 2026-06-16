use std::collections::HashSet;

use bitcoin::Network;
use serde_json::{json, Map, Value};

use crate::hdwallet::addresses::iaddress::{EncodeKwargs, EncodePublicKey, IAddress};
use crate::hdwallet::addresses::{
    P2PKHAddress, P2TRAddress, P2WPKHAddress, P2WPKHInP2SHAddress, Addresses,
};
use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::icryptocurrency::CryptocurrencyDescriptor;
use crate::hdwallet::cryptocurrencies::{get_cryptocurrency, resolve_network};
use crate::hdwallet::derivations::{
    Bip44Derivation, Bip49Derivation, Bip84Derivation, Bip86Derivation, Cip1852Derivation,
    ChangeInput, ChangeValue, CustomDerivation, Derivations, EccInput, ElectrumDerivation,
    HdwDerivation, IDerivation, IndexInput, MoneroDerivation, RoleInput,
};
use crate::hdwallet::entropies::{ElectrumV2Entropy, Entropies, IEntropy};
use crate::hdwallet::exceptions::Error;
use crate::hdwallet::hds::bip32::Bip32Hd;
use crate::hdwallet::hds::algorand::AlgorandHd;
use crate::hdwallet::hds::cardano::CardanoHd;
use crate::hdwallet::hds::electrum::{ElectrumV1Hd, ElectrumV2Hd};
use crate::hdwallet::hds::ihd::Ihd;
use crate::hdwallet::hds::monero::MoneroHd;
use crate::hdwallet::hds::Hds;
use crate::hdwallet::mnemonics::{
    AlgorandMnemonic, BIP39Mnemonic, ElectrumV1Mnemonic, ElectrumV2Mnemonic, IMnemonic,
    Mnemonics, MoneroMnemonic,
};
use crate::hdwallet::seeds::Seeds;

pub struct HDWallet {
    cryptocurrency: &'static CryptocurrencyDescriptor,
    network: &'static crate::hdwallet::cryptocurrencies::icryptocurrency::NetworkDescriptor,
    hd: String,
    semantic: Option<String>,
    language: String,
    passphrase: Option<String>,
    public_key_type: String,
    cardano_type: String,
    address_type: Option<String>,
    mode: String,
    mnemonic_type: String,
    checksum: bool,
    payment_id: Option<String>,
    staking_public_key: Option<String>,

    entropy: Option<String>,
    strength: Option<usize>,
    mnemonic: Option<String>,
    seed_hex: Option<String>,

    strict: bool,
    encoded: bool,

    derivation: Option<Box<dyn IDerivation>>,
    derivation_name: Option<String>,
    backend: Option<HdBackend>,
}

#[derive(Debug)]
enum HdBackend {
    Bip32(Bip32Hd),
    ElectrumV1(ElectrumV1Hd),
    ElectrumV2(ElectrumV2Hd),
    Cardano(CardanoHd),
    Algorand(AlgorandHd),
    Monero(MoneroHd),
}

pub struct HDWalletConfig {
    pub symbol: String,
    pub hd: String,
    pub network: String,
    pub language: String,
    pub passphrase: Option<String>,
    pub public_key_type: String,
    pub cardano_type: String,
    pub address_type: Option<String>,
    pub mode: String,
    pub mnemonic_type: String,
    pub checksum: bool,
    pub payment_id: Option<String>,
    pub staking_public_key: Option<String>,
    pub semantic: Option<String>,
    pub strict: bool,
    pub encoded: bool,
}

impl HDWallet {
    pub fn new(config: HDWalletConfig) -> Result<Self, Error> {
        let cryptocurrency = get_cryptocurrency(&config.symbol)?;
        if !Hds::is_hd(&config.hd) {
            return Err(Error::new(
                "Wrong HD name",
                None,
                Some(&Hds::names().join(", ")),
                Some(&config.hd),
            ));
        }
        if !cryptocurrency
            .hds
            .iter()
            .any(|hd| hd_name_matches(hd, &config.hd))
        {
            return Err(Error::new(
                &format!(
                    "{} HD not implemented on {} cryptocurrency",
                    config.hd, cryptocurrency.name
                ),
                None,
                None,
                Some(&config.hd),
            ));
        }

        let network = resolve_network(cryptocurrency, &config.network)?;
        let semantic = resolve_semantic(
            &config.hd,
            config.semantic.as_deref(),
            cryptocurrency,
        );

        Ok(Self {
            cryptocurrency,
            network,
            hd: config.hd,
            semantic,
            language: config.language,
            passphrase: config.passphrase,
            public_key_type: config.public_key_type,
            cardano_type: config.cardano_type,
            address_type: config.address_type,
            mode: config.mode,
            mnemonic_type: config.mnemonic_type,
            checksum: config.checksum,
            payment_id: config.payment_id,
            staking_public_key: config.staking_public_key,
            entropy: None,
            strength: None,
            mnemonic: None,
            seed_hex: None,
            strict: config.strict,
            encoded: config.encoded,
            derivation: None,
            derivation_name: None,
            backend: None,
        })
    }

    pub fn from_entropy(&mut self, entropy: &str, client: &str) -> Result<(), Error> {
        if !Entropies::is_entropy(client) {
            return Err(Error::new(
                "Wrong entropy client",
                None,
                Some(&Entropies::names().join(", ")),
                Some(client),
            ));
        }
        if !self
            .cryptocurrency
            .entropies
            .iter()
            .any(|name| client_name_matches(name, client))
        {
            return Err(Error::new(
                &format!("Invalid entropy class for {} cryptocurrency", self.cryptocurrency.name),
                None,
                None,
                None,
            ));
        }

        let entropy_bytes = hex::decode(entropy.trim()).map_err(|error| {
            Error::new("Invalid entropy hex", Some(&error.to_string()), None, None)
        })?;
        self.strength = Some(if client == "Electrum-V2" {
            IEntropy::strength_bits_from_hex(&ElectrumV2Entropy, &hex::encode(&entropy_bytes))
                .unwrap_or(entropy_bytes.len() * 8)
        } else {
            entropy_bytes.len() * 8
        });
        self.entropy = Some(hex::encode(&entropy_bytes));

        let mnemonic = match client {
            "Electrum-V2" => ElectrumV2Mnemonic::from_entropy_with_retries(
                &hex::encode(&entropy_bytes),
                &self.language,
                &self.mnemonic_type,
                u128::MAX,
            )
            .map(|m| m.mnemonic())
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Monero" => MoneroMnemonic::from_entropy_with_checksum(
                &entropy_bytes,
                &self.language,
                self.checksum,
            )
            .map(|m| m.mnemonic())
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "BIP39" => BIP39Mnemonic::from_entropy(&entropy_bytes, &self.language)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Electrum-V1" => ElectrumV1Mnemonic::from_entropy(&entropy_bytes, &self.language)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Algorand" => AlgorandMnemonic::from_entropy(&entropy_bytes, &self.language)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            other => {
                return Err(Error::new(
                    "Wrong entropy client",
                    None,
                    Some(&Entropies::names().join(", ")),
                    Some(other),
                ));
            }
        };

        self.from_mnemonic(&mnemonic, client)
    }

    pub fn from_mnemonic(&mut self, mnemonic: &str, client: &str) -> Result<(), Error> {
        if !Mnemonics::is_mnemonic(client) {
            return Err(Error::new(
                "Wrong mnemonic client",
                None,
                Some(&Mnemonics::names().join(", ")),
                Some(client),
            ));
        }
        if !self
            .cryptocurrency
            .mnemonics
            .iter()
            .any(|name| client_name_matches(name, client))
        {
            return Err(Error::new(
                &format!("Invalid mnemonic class for {} cryptocurrency", self.cryptocurrency.name),
                None,
                None,
                None,
            ));
        }

        let words: Vec<String> = mnemonic.split_whitespace().map(str::to_string).collect();
        let entropy_bytes = match client {
            "BIP39" => BIP39Mnemonic::decode(&words)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Electrum-V2" => ElectrumV2Mnemonic::decode_phrase(mnemonic, &self.mnemonic_type)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Monero" => MoneroMnemonic::decode(&words)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Electrum-V1" => ElectrumV1Mnemonic::decode(&words)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            "Algorand" => AlgorandMnemonic::decode(&words)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
            other => {
                return Err(Error::new(
                    "Wrong mnemonic client",
                    None,
                    Some(&Mnemonics::names().join(", ")),
                    Some(other),
                ));
            }
        };

        self.entropy = Some(hex::encode(&entropy_bytes));
        self.strength = Some(if client == "Electrum-V2" {
            IEntropy::strength_bits_from_hex(&ElectrumV2Entropy, &hex::encode(&entropy_bytes))
                .unwrap_or(entropy_bytes.len() * 8)
        } else {
            entropy_bytes.len() * 8
        });
        self.mnemonic = Some(mnemonic.to_string());

        let seed_client = match self.hd.as_str() {
            "Cardano" => "Cardano",
            "Electrum-V1" | "Electrum-V2" => self.hd.as_str(),
            _ => client,
        };
        let seed_hex = Seeds::seed_from_mnemonic(
            seed_client,
            mnemonic,
            self.passphrase.as_deref(),
            &self.cardano_type,
            &self.mnemonic_type,
        )
        .map_err(|error| Error::new(&error.to_string(), None, None, None))?;

        self.from_seed_hex(&seed_hex)
    }

    pub fn from_seed_hex(&mut self, seed_hex: &str) -> Result<(), Error> {
        let seed_bytes = hex::decode(seed_hex.trim()).map_err(|error| {
            Error::new("Invalid seed hex", Some(&error.to_string()), None, None)
        })?;
        self.seed_hex = Some(seed_hex.to_string());

        self.backend = Some(self.backend_from_seed(&seed_bytes)?);
        Ok(())
    }

    pub fn from_xprivate_key(
        &mut self,
        xprivate_key: &str,
        encoded: bool,
        strict: bool,
    ) -> Result<(), Error> {
        if matches!(self.hd.as_str(), "Electrum-V1" | "Monero") {
            return Err(Error::new(
                &format!("Conversion from xprivate key is not implemented for the {} HD type", self.hd),
                None,
                None,
                None,
            ));
        }
        self.strict = strict;
        self.encoded = encoded;
        match self.hd.as_str() {
            "Cardano" => {
                let mut hd = CardanoHd::new(&self.cardano_type, self.network.name)?;
                self.configure_cardano_hd(&mut hd)?;
                hd.from_xprivate_key(xprivate_key, encoded, strict)?;
                self.backend = Some(HdBackend::Cardano(hd));
                Ok(())
            }
            "Algorand" => {
                let mut hd = AlgorandHd::from_seed_bytes(&[0u8; 32])?;
                hd.from_xprivate_key(xprivate_key, encoded, strict)?;
                self.backend = Some(HdBackend::Algorand(hd));
                Ok(())
            }
            _ => {
                let mut bip32 = Bip32Hd::from_seed(
                    map_bitcoin_network(self.network.name),
                    &[0u8; 32],
                )?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    strict,
                );
                if self.uses_bip32_hd() {
                    bip32.set_xkey_versions(
                        xprivate_version_for_semantic(&self.network, self.semantic.as_deref()),
                        xpublic_version_for_semantic(&self.network, self.semantic.as_deref()),
                    );
                }
                bip32.from_xprivate_key(xprivate_key, encoded, strict)?;
                self.backend = Some(HdBackend::Bip32(bip32));
                Ok(())
            }
        }
    }

    pub fn from_xpublic_key(
        &mut self,
        xpublic_key: &str,
        encoded: bool,
        strict: bool,
    ) -> Result<(), Error> {
        if matches!(self.hd.as_str(), "Electrum-V1" | "Monero") {
            return Err(Error::new(
                &format!("Conversion from xpublic key is not implemented for the {} HD type", self.hd),
                None,
                None,
                None,
            ));
        }
        if self.hd.as_str() == "Cardano" && self.cardano_type == "byron-legacy" {
            return Err(Error::new(
                &format!(
                    "Conversion from xpublic key is not implemented for the {} HD {} type",
                    self.hd, self.cardano_type
                ),
                None,
                None,
                None,
            ));
        }
        self.strict = strict;
        self.encoded = encoded;
        match self.hd.as_str() {
            "Cardano" => {
                let mut hd = CardanoHd::new(&self.cardano_type, self.network.name)?;
                self.configure_cardano_hd(&mut hd)?;
                hd.from_xpublic_key(xpublic_key, encoded, strict)?;
                self.backend = Some(HdBackend::Cardano(hd));
                Ok(())
            }
            "Algorand" => {
                let mut hd = AlgorandHd::from_seed_bytes(&[0u8; 32])?;
                hd.from_xpublic_key(xpublic_key, encoded, strict)?;
                self.backend = Some(HdBackend::Algorand(hd));
                Ok(())
            }
            hd if self.uses_bip32_hd() || hd == "Electrum-V2" => {
                let mut bip32 = Bip32Hd::from_seed(
                    map_bitcoin_network(self.network.name),
                    &[0u8; 32],
                )?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    strict,
                );
                if self.uses_bip32_hd() {
                    bip32.set_xkey_versions(
                        xprivate_version_for_semantic(&self.network, self.semantic.as_deref()),
                        xpublic_version_for_semantic(&self.network, self.semantic.as_deref()),
                    );
                }
                bip32.from_xpublic_key(xpublic_key, encoded, strict)?;
                self.backend = Some(HdBackend::Bip32(bip32));
                Ok(())
            }
            other => Err(Error::new(
                &format!("Conversion from xpublic key is not implemented for the {other} HD type"),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_private_key(&mut self, private_key: &str) -> Result<(), Error> {
        match self.hd.as_str() {
            "Electrum-V1" => {
                let mut hd = ElectrumV1Hd::new(&self.public_key_type, self.network.wif_prefix)?;
                hd.from_private_key(private_key)?;
                self.backend = Some(HdBackend::ElectrumV1(hd));
                Ok(())
            }
            "Monero" => {
                let mut hd = MoneroHd::new(self.network.name)?;
                hd.from_private_key(private_key)?;
                self.backend = Some(HdBackend::Monero(hd));
                Ok(())
            }
            "Cardano" => {
                let mut hd = CardanoHd::new(&self.cardano_type, self.network.name)?;
                self.configure_cardano_hd(&mut hd)?;
                hd.from_private_key_hex(private_key)?;
                self.backend = Some(HdBackend::Cardano(hd));
                Ok(())
            }
            "Algorand" => {
                let mut hd = AlgorandHd::from_seed_bytes(&[0u8; 32])?;
                hd.from_private_key_hex(private_key)?;
                self.backend = Some(HdBackend::Algorand(hd));
                Ok(())
            }
            hd if self.uses_bip32_hd() || hd == "Electrum-V2" => {
                let mut bip32 = Bip32Hd::from_seed(
                    map_bitcoin_network(self.network.name),
                    &[0u8; 32],
                )?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    self.strict,
                );
                bip32.from_private_key(private_key)?;
                self.backend = Some(HdBackend::Bip32(bip32));
                Ok(())
            }
            other => Err(Error::new(
                &format!("Conversion from private key is not implemented for the {other} HD type"),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_wif(&mut self, wif: &str) -> Result<(), Error> {
        match self.hd.as_str() {
            "Electrum-V1" => {
                let mut hd = ElectrumV1Hd::new(&self.public_key_type, self.network.wif_prefix)?;
                hd.from_wif(wif)?;
                self.backend = Some(HdBackend::ElectrumV1(hd));
                Ok(())
            }
            other if self.uses_bip32_hd() || other == "Electrum-V2" => {
                let mut bip32 = Bip32Hd::from_seed(
                    map_bitcoin_network(self.network.name),
                    &[0u8; 32],
                )?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    self.strict,
                );
                bip32.from_wif(wif)?;
                self.backend = Some(HdBackend::Bip32(bip32));
                Ok(())
            }
            other => Err(Error::new(
                &format!("Conversion from WIF is not implemented for the {other} HD type"),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_public_key(&mut self, public_key: &str) -> Result<(), Error> {
        match self.hd.as_str() {
            "Electrum-V1" => {
                let mut hd = ElectrumV1Hd::new(&self.public_key_type, self.network.wif_prefix)?;
                hd.from_public_key_hex(public_key)?;
                self.backend = Some(HdBackend::ElectrumV1(hd));
                Ok(())
            }
            "Cardano" => {
                let mut hd = CardanoHd::new(&self.cardano_type, self.network.name)?;
                self.configure_cardano_hd(&mut hd)?;
                hd.from_public_key_hex(public_key)?;
                self.backend = Some(HdBackend::Cardano(hd));
                Ok(())
            }
            hd if self.uses_bip32_hd() || hd == "Electrum-V2" => {
                let mut bip32 = Bip32Hd::from_seed(
                    map_bitcoin_network(self.network.name),
                    &[0u8; 32],
                )?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    self.strict,
                );
                bip32.from_public_key(public_key)?;
                self.backend = Some(HdBackend::Bip32(bip32));
                Ok(())
            }
            other => Err(Error::new(
                &format!("Conversion from public key is not implemented for the {other} HD type"),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_spend_private_key(&mut self, spend_private_key: &str) -> Result<(), Error> {
        match self.hd.as_str() {
            "Monero" => {
                let mut hd = MoneroHd::new(self.network.name)?;
                hd.from_spend_private_key(spend_private_key)?;
                self.backend = Some(HdBackend::Monero(hd));
                Ok(())
            }
            other => Err(Error::new(
                &format!(
                    "Conversion from spend private key is not implemented for the {other} HD type"
                ),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_watch_only(
        &mut self,
        view_private_key: &str,
        spend_public_key: &str,
    ) -> Result<(), Error> {
        match self.hd.as_str() {
            "Monero" => {
                let mut hd = MoneroHd::new(self.network.name)?;
                hd.from_watch_only(view_private_key, spend_public_key)?;
                self.backend = Some(HdBackend::Monero(hd));
                Ok(())
            }
            other => Err(Error::new(
                &format!("Conversion from watch-only keys is not implemented for the {other} HD type"),
                None,
                None,
                None,
            )),
        }
    }

    pub fn from_derivation(
        &mut self,
        derivation_name: &str,
        derivation: Box<dyn IDerivation>,
    ) -> Result<(), Error> {
        self.derivation_name = Some(derivation_name.to_string());
        self.derivation = Some(derivation);
        self.apply_derivation()?;
        Ok(())
    }

    pub fn update_derivation(
        &mut self,
        derivation_name: &str,
        derivation: Box<dyn IDerivation>,
    ) -> Result<(), Error> {
        self.derivation_name = Some(derivation_name.to_string());
        self.derivation = Some(derivation);
        self.apply_derivation()?;
        Ok(())
    }

    pub(crate) fn derivation_name(&self) -> &str {
        self.derivation_name.as_deref().unwrap_or("Custom")
    }

    pub fn cryptocurrency_descriptor(&self) -> &'static CryptocurrencyDescriptor {
        self.cryptocurrency
    }

    pub fn derivation(&self) -> Option<&dyn IDerivation> {
        self.derivation.as_deref()
    }

    pub fn dump(&self, exclude: &HashSet<String>) -> Result<Value, Error> {
        if exclude.contains("root") {
            let mut derivation = self.build_derivation_dump()?;
            if exclude.contains("at") {
                if let Some(object) = derivation.as_object_mut() {
                    object.remove("at");
                }
            }
            return Ok(apply_value_exclude(derivation, exclude));
        }

        let mut derivation = if exclude.contains("derivation") {
            None
        } else {
            Some(self.build_derivation_dump()?)
        };

        if let Some(derivation) = derivation.as_mut() {
            if exclude.contains("at") {
                if let Some(object) = derivation.as_object_mut() {
                    object.remove("at");
                }
            }
        }

        let mut root = json!({
            "cryptocurrency": self.cryptocurrency.name,
            "symbol": self.cryptocurrency.symbol,
            "network": self.network.name,
            "coin_type": self.cryptocurrency.coin_type,
            "entropy": self.entropy,
            "strength": self.strength,
            "mnemonic": self.mnemonic,
            "passphrase": self.passphrase,
            "language": if self.mnemonic.is_some() {
                json!(self.language)
            } else {
                json!(null)
            },
            "seed": self.seed_hex,
            "ecc": display_ecc_name(self.cryptocurrency.ecc),
            "hd": self.hd,
        });

        if let Some(map) = root.as_object_mut() {
            if self.uses_bip32_hd() || matches!(self.hd.as_str(), "Cardano" | "Algorand") {
                map.insert("semantic".to_string(), json!(self.semantic));
            }
            match self.backend.as_ref() {
                Some(HdBackend::Bip32(bip32)) if self.uses_bip32_hd() => {
                    map.insert(
                        "root_xprivate_key".to_string(),
                        json!(bip32.root_xprivate_key(self.encoded)),
                    );
                    map.insert(
                        "root_xpublic_key".to_string(),
                        json!(bip32.root_xpublic_key(self.encoded)),
                    );
                    map.insert(
                        "root_private_key".to_string(),
                        json!(bip32.root_private_key()),
                    );
                    map.insert("root_wif".to_string(), json!(bip32.root_wif()));
                    map.insert(
                        "root_chain_code".to_string(),
                        json!(bip32.root_chain_code()),
                    );
                    map.insert(
                        "root_public_key".to_string(),
                        json!(bip32.root_public_key()),
                    );
                    map.insert("strict".to_string(), json!(bip32.strict()));
                    map.insert(
                        "public_key_type".to_string(),
                        json!(bip32.public_key_type()),
                    );
                    map.insert("wif_type".to_string(), json!(bip32.wif_type()));
                }
                Some(HdBackend::ElectrumV1(hd)) => {
                    map.insert(
                        "master_private_key".to_string(),
                        json!(hd.master_private_key()),
                    );
                    map.insert("master_wif".to_string(), json!(hd.master_wif()));
                    map.insert(
                        "master_public_key".to_string(),
                        json!(hd.master_public_key()),
                    );
                    map.insert(
                        "public_key_type".to_string(),
                        json!(hd.public_key_type()),
                    );
                    map.insert("wif_type".to_string(), json!(hd.wif_type()));
                }
                Some(HdBackend::ElectrumV2(hd)) => {
                    map.insert(
                        "master_private_key".to_string(),
                        json!(hd.master_private_key()),
                    );
                    map.insert("master_wif".to_string(), json!(hd.master_wif()));
                    map.insert(
                        "master_public_key".to_string(),
                        json!(hd.master_public_key()),
                    );
                    map.insert(
                        "public_key_type".to_string(),
                        json!(hd.public_key_type()),
                    );
                    map.insert("wif_type".to_string(), json!(hd.wif_type()));
                    map.insert("mode".to_string(), json!(hd.mode()));
                    map.insert("mnemonic_type".to_string(), json!(self.mnemonic_type));
                }
                Some(HdBackend::Cardano(hd)) => {
                    map.insert("cardano_type".to_string(), json!(hd.cardano_type()));
                    map.insert(
                        "root_xprivate_key".to_string(),
                        json!(hd.root_xprivate_key(self.encoded)),
                    );
                    map.insert(
                        "root_xpublic_key".to_string(),
                        json!(hd.root_xpublic_key(self.encoded)),
                    );
                    map.insert(
                        "root_private_key".to_string(),
                        json!(hd.root_private_key()),
                    );
                    map.insert(
                        "root_chain_code".to_string(),
                        json!(hd.root_chain_code()),
                    );
                    map.insert(
                        "root_public_key".to_string(),
                        json!(hd.root_public_key()),
                    );
                    map.insert("strict".to_string(), json!(hd.strict()));
                    if hd.cardano_type() == "byron-legacy" {
                        map.insert("path_key".to_string(), json!(hd.path_key()));
                    }
                }
                Some(HdBackend::Algorand(hd)) => {
                    map.insert(
                        "root_xprivate_key".to_string(),
                        json!(hd.root_xprivate_key(self.encoded)),
                    );
                    map.insert(
                        "root_xpublic_key".to_string(),
                        json!(hd.root_xpublic_key(self.encoded)),
                    );
                    map.insert(
                        "root_private_key".to_string(),
                        json!(hd.root_private_key()),
                    );
                    map.insert(
                        "root_chain_code".to_string(),
                        json!(hd.root_chain_code()),
                    );
                    map.insert(
                        "root_public_key".to_string(),
                        json!(hd.root_public_key()),
                    );
                }
                Some(HdBackend::Monero(hd)) => {
                    map.insert("private_key".to_string(), json!(hd.private_key()));
                    map.insert(
                        "spend_private_key".to_string(),
                        json!(hd.spend_private_key()),
                    );
                    map.insert(
                        "view_private_key".to_string(),
                        json!(hd.view_private_key()),
                    );
                    map.insert(
                        "spend_public_key".to_string(),
                        json!(hd.spend_public_key()),
                    );
                    map.insert(
                        "view_public_key".to_string(),
                        json!(hd.view_public_key()),
                    );
                    map.insert(
                        "primary_address".to_string(),
                        json!(MoneroHd::primary_address(hd).ok()),
                    );
                    if hd.private_key().is_none() {
                        if let Some(payment_id) = self.payment_id.as_deref() {
                            if let Ok(bytes) = hex::decode(payment_id.trim()) {
                                map.insert(
                                    "integrated_address".to_string(),
                                    json!(hd.integrated_address(&bytes).ok()),
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(derivation) = derivation {
            if let Some(map) = root.as_object_mut() {
                map.insert("derivation".to_string(), derivation);
            }
        }

        Ok(apply_value_exclude(root, exclude))
    }

    fn build_derivation_dump(&self) -> Result<Value, Error> {
        let backend = self.backend.as_ref().ok_or_else(|| {
            Error::new("HD wallet is not initialized from a master key source", None, None, None)
        })?;
        let derivation_name = self.derivation_name();
        let at = self
            .derivation
            .as_ref()
            .map(|derivation| derivation_at(derivation_name, derivation.as_ref()));

        match backend {
            HdBackend::Bip32(bip32) => {
                let mut dump = json!({
                    "xprivate_key": bip32.xprivate_key(self.encoded),
                    "xpublic_key": bip32.xpublic_key(self.encoded),
                    "private_key": bip32.private_key(),
                    "wif": bip32.wif(),
                    "chain_code": bip32.chain_code(),
                    "public_key": bip32.public_key(),
                    "uncompressed": bip32.uncompressed(),
                    "compressed": bip32.compressed(),
                    "hash": bip32.hash(),
                    "fingerprint": bip32.fingerprint(),
                    "parent_fingerprint": bip32.parent_fingerprint(),
                });
                if let Some(at) = at {
                    if let Some(object) = dump.as_object_mut() {
                        object.insert("at".to_string(), at);
                    }
                }
                if self.hd.as_str() == "BIP32" {
                    if let Some(addresses) =
                        self.build_addresses_dump(bip32.public_key().as_deref())?
                    {
                        if let Some(object) = dump.as_object_mut() {
                            object.insert("addresses".to_string(), addresses);
                        }
                    }
                } else if matches!(
                    self.hd.as_str(),
                    "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141"
                ) {
                    if let Some(object) = dump.as_object_mut() {
                        object.insert(
                            "address".to_string(),
                            json!(self.encode_address(bip32.public_key().as_deref())?),
                        );
                    }
                } else if let Some(addresses) =
                    self.build_addresses_dump(bip32.public_key().as_deref())?
                {
                    if let Some(object) = dump.as_object_mut() {
                        if addresses.as_object().map(|m| !m.is_empty()).unwrap_or(false) {
                            object.insert("addresses".to_string(), addresses);
                        }
                        if !object.contains_key("address") {
                            object.insert(
                                "address".to_string(),
                                json!(self.encode_address(bip32.public_key().as_deref()).ok()),
                            );
                        }
                    }
                } else if let Some(object) = dump.as_object_mut() {
                    object.insert(
                        "address".to_string(),
                        json!(self.encode_address(bip32.public_key().as_deref())?),
                    );
                }
                Ok(dump)
            }
            HdBackend::ElectrumV1(hd) => {
                let address =
                    hd.encode_address(self.network.public_key_address_prefix)?;
                Ok(json!({
                    "at": at,
                    "private_key": hd.private_key(),
                    "wif": hd.wif(),
                    "public_key": hd.public_key(),
                    "uncompressed": hd.uncompressed(),
                    "compressed": hd.compressed(),
                    "address": address,
                }))
            }
            HdBackend::ElectrumV2(hd) => {
                let witness_version = if hd.mode().as_deref() == Some(crate::hdwallet::consts::Modes::SEGWIT) {
                    Some(0)
                } else {
                    self.network
                        .witness_versions
                        .map(|versions| versions.0[0])
                };
                let address = hd.encode_address(
                    self.network.public_key_address_prefix,
                    self.network.hrp,
                    witness_version,
                )?;
                Ok(json!({
                    "at": at,
                    "private_key": hd.private_key(),
                    "wif": hd.wif(),
                    "public_key": hd.public_key(),
                    "uncompressed": hd.uncompressed(),
                    "compressed": hd.compressed(),
                    "address": address,
                }))
            }
            HdBackend::Cardano(hd) => {
                let address = hd.encode_address()?;
                let mut dump = json!({
                    "xprivate_key": hd.xprivate_key(self.encoded),
                    "xpublic_key": hd.xpublic_key(self.encoded),
                    "private_key": hd.private_key(),
                    "chain_code": hd.chain_code(),
                    "public_key": hd.public_key(),
                    "hash": hd.hash(),
                    "fingerprint": hd.fingerprint(),
                    "parent_fingerprint": hd.parent_fingerprint(),
                    "address": address,
                });
                if let Some(at) = at {
                    if let Some(object) = dump.as_object_mut() {
                        object.insert("at".to_string(), at);
                    }
                }
                Ok(dump)
            }
            HdBackend::Algorand(hd) => {
                let address = hd.address().map_err(|error| {
                    Error::new("Address encoding failed", Some(&error.to_string()), None, None)
                })?;
                Ok(json!({
                    "at": at,
                    "xprivate_key": hd.xprivate_key(self.encoded),
                    "xpublic_key": hd.xpublic_key(self.encoded),
                    "private_key": hd.private_key(),
                    "chain_code": hd.chain_code(),
                    "public_key": hd.public_key(),
                    "fingerprint": hd.fingerprint(),
                    "parent_fingerprint": hd.parent_fingerprint(),
                    "address": address,
                }))
            }
            HdBackend::Monero(hd) => Ok(json!({
                "at": at,
                "sub_address": Ihd::sub_address(hd).unwrap_or_default(),
            })),
        }
    }

    fn build_addresses_dump(&self, public_key: Option<&str>) -> Result<Option<Value>, Error> {
        let public_key = match public_key {
            Some(value) => value,
            None => return Ok(None),
        };
        if self.cryptocurrency.addresses.len() <= 1 && self.cryptocurrency.name != "Tezos" {
            return Ok(None);
        }

        let mut addresses = Map::new();
        let public_key_input = EncodePublicKey::Str(public_key);

        if matches!(
            self.cryptocurrency.name,
            "Bitcoin-Cash" | "Bitcoin-Cash-SLP" | "eCash"
        ) {
            let kwargs = self.base_encode_kwargs(None);
            if let Some(address_types) = self.cryptocurrency.address_types {
                for (label, _) in address_types {
                    for address_name in self.cryptocurrency.addresses {
                        let registry =
                            resolve_address_registry_name(self.cryptocurrency.address_types, address_name);
                        let mut address_kwargs = kwargs.clone();
                        address_kwargs.witness_version = self
                            .network
                            .witness_versions
                            .and_then(|versions| versions.get_witness_version(registry));
                        address_kwargs.public_key_address_prefix = legacy_or_std_prefix(
                            self.network.legacy_public_key_address_prefix,
                            self.network.std_public_key_address_prefix,
                            label,
                        );
                        address_kwargs.script_address_prefix = legacy_or_std_prefix(
                            self.network.legacy_script_address_prefix,
                            self.network.std_script_address_prefix,
                            label,
                        );
                        if let Ok(scheme) = Addresses::address(registry) {
                            if let Ok(encoded) = scheme.encode(public_key_input, &address_kwargs) {
                                addresses.insert(
                                    format!(
                                        "{}-{}",
                                        label.to_ascii_lowercase(),
                                        address_name.to_ascii_lowercase()
                                    ),
                                    json!(encoded),
                                );
                            }
                        }
                    }
                }
            }
        } else if self.cryptocurrency.name == "Tezos" {
            let kwargs = self.base_encode_kwargs(None);
            if let Some(prefixes) = self.cryptocurrency.address_prefixes {
                for prefix in prefixes {
                    let mut address_kwargs = kwargs.clone();
                    address_kwargs.address_prefix = Some(prefix);
                    if let Ok(scheme) = Addresses::address("Tezos") {
                        if let Ok(encoded) = scheme.encode(public_key_input, &address_kwargs) {
                            addresses.insert(prefix.to_string(), json!(encoded));
                        }
                    }
                }
            }
        } else {
            for address_name in self.cryptocurrency.addresses {
                let registry =
                    resolve_address_registry_name(self.cryptocurrency.address_types, address_name);
                let key = address_name.to_ascii_lowercase().replace('-', "_");
                let kwargs = self.base_encode_kwargs(Some(registry));
                if let Ok(scheme) = Addresses::address(registry) {
                    if let Ok(encoded) = scheme.encode(public_key_input, &kwargs) {
                        addresses.insert(key, json!(encoded));
                    }
                }
            }
        }

        if addresses.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Value::Object(addresses)))
        }
    }

    fn base_encode_kwargs(&self, address_registry: Option<&str>) -> EncodeKwargs<'_> {
        let compressed = self.public_key_type != PublicKeyTypes::UNCOMPRESSED;
        EncodeKwargs {
            public_key_type: Some(if compressed {
                PublicKeyTypes::COMPRESSED
            } else {
                PublicKeyTypes::UNCOMPRESSED
            }),
            network: Some(self.network.name),
            public_key_address_prefix: self.network.public_key_address_prefix,
            script_address_prefix: self.network.script_address_prefix,
            witness_version: address_registry.and_then(|address| {
                self.network
                    .witness_versions
                    .and_then(|versions| versions.get_witness_version(address))
            }),
            alphabet: self
                .cryptocurrency
                .params
                .and_then(|params| params.alphabet),
            hrp: self.network.hrp,
            ..Default::default()
        }
    }

    fn configure_cardano_hd(&self, hd: &mut CardanoHd) -> Result<(), Error> {
        if let Some(staking_public_key) = self.staking_public_key.as_deref() {
            let bytes = hex::decode(staking_public_key.trim()).map_err(|error| {
                Error::new(
                    "Invalid staking public key hex",
                    Some(&error.to_string()),
                    None,
                    None,
                )
            })?;
            hd.set_staking_public_key(Some(&bytes));
        }
        hd.set_address_type(self.address_type.clone());
        Ok(())
    }

    fn encode_address(&self, public_key: Option<&str>) -> Result<String, Error> {
        let public_key = public_key.ok_or_else(|| {
            Error::new("Public key is not available for address encoding", None, None, None)
        })?;
        let address_registry = match self.hd.as_str() {
            "BIP49" => "P2WPKH-In-P2SH",
            "BIP84" | "BIP141" if self.semantic.as_deref() == Some("p2wpkh") => "P2WPKH",
            "BIP141" if self.semantic.as_deref() == Some("p2wpkh-in-p2sh") => "P2WPKH-In-P2SH",
            "BIP141" if self.semantic.as_deref() == Some("p2wsh") => "P2WSH",
            "BIP141" if self.semantic.as_deref() == Some("p2wsh-in-p2sh") => "P2WSH-In-P2SH",
            "BIP86" => "P2TR",
            "BIP44" => "P2PKH",
            _ => self.cryptocurrency.default_address,
        };
        let kwargs = self.base_encode_kwargs(Some(address_registry));
        let public_key_input = EncodePublicKey::Str(public_key);
        if self.cryptocurrency.addresses.len() > 1
            || matches!(self.hd.as_str(), "BIP32" | "BIP141")
        {
            return match self.hd.as_str() {
                "BIP49" => P2WPKHInP2SHAddress.encode(public_key_input, &kwargs),
                "BIP84" | "BIP141" if self.semantic.as_deref() == Some("p2wpkh") => {
                    P2WPKHAddress.encode(public_key_input, &kwargs)
                }
                "BIP141" if self.semantic.as_deref() == Some("p2wpkh-in-p2sh") => {
                    P2WPKHInP2SHAddress.encode(public_key_input, &kwargs)
                }
                "BIP86" => P2TRAddress.encode(public_key_input, &kwargs),
                "BIP44" => P2PKHAddress.encode(public_key_input, &kwargs),
                _ => Addresses::address(address_registry)
                    .map_err(|error| Error::new(&error.to_string(), None, None, None))?
                    .encode(public_key_input, &kwargs),
            }
            .map_err(|error| {
                Error::new("Address encoding failed", Some(&error.to_string()), None, None)
            });
        }

        Addresses::address(address_registry)
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?
            .encode(public_key_input, &kwargs)
            .map_err(|error| {
                Error::new("Address encoding failed", Some(&error.to_string()), None, None)
            })
    }

    fn uses_bip32_hd(&self) -> bool {
        matches!(
            self.hd.as_str(),
            "BIP32" | "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141"
        )
    }

    fn backend_from_seed(&self, seed_bytes: &[u8]) -> Result<HdBackend, Error> {
        match self.hd.as_str() {
            "BIP32" | "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141" => {
                let mut bip32 =
                    Bip32Hd::from_seed(map_bitcoin_network(self.network.name), seed_bytes)?;
                bip32.configure(
                    self.network.wif_prefix,
                    &self.public_key_type,
                    true,
                );
                bip32.set_xkey_versions(
                    xprivate_version_for_semantic(&self.network, self.semantic.as_deref()),
                    xpublic_version_for_semantic(&self.network, self.semantic.as_deref()),
                );
                Ok(HdBackend::Bip32(bip32))
            }
            "Electrum-V1" => {
                let mut hd = ElectrumV1Hd::new(&self.public_key_type, self.network.wif_prefix)?;
                hd.from_seed_bytes(seed_bytes)?;
                Ok(HdBackend::ElectrumV1(hd))
            }
            "Electrum-V2" => {
                let mut hd = ElectrumV2Hd::new(
                    map_bitcoin_network(self.network.name),
                    &self.mode,
                    &self.public_key_type,
                    self.network.wif_prefix,
                )?;
                hd.from_seed_bytes(seed_bytes)?;
                Ok(HdBackend::ElectrumV2(hd))
            }
            "Cardano" => {
                let mut hd = CardanoHd::new(&self.cardano_type, self.network.name)?;
                self.configure_cardano_hd(&mut hd)?;
                hd.from_seed_bytes(seed_bytes, self.passphrase.as_deref())?;
                Ok(HdBackend::Cardano(hd))
            }
            "Monero" => {
                let mut hd = MoneroHd::new(self.network.name)?;
                hd.from_seed_bytes(seed_bytes)?;
                Ok(HdBackend::Monero(hd))
            }
            "Algorand" => {
                let hd = AlgorandHd::from_seed_bytes(seed_bytes)?;
                Ok(HdBackend::Algorand(hd))
            }
            other => Err(Error::new(
                &format!("Dump from seed is not implemented yet for {other} HD"),
                None,
                None,
                Some(other),
            )),
        }
    }

    fn apply_derivation(&mut self) -> Result<(), Error> {
        let derivation = self
            .derivation
            .as_ref()
            .ok_or_else(|| Error::new("Derivation is not set", None, None, None))?;
        match self.backend.as_mut() {
            Some(HdBackend::Bip32(bip32)) => {
                bip32.from_derivation(derivation.path())?;
            }
            Some(HdBackend::ElectrumV1(hd)) => {
                let (change, address) = electrum_derivation_indices(derivation.as_ref())?;
                hd.from_derivation_indices(change, address)?;
            }
            Some(HdBackend::ElectrumV2(hd)) => {
                let (change, address) = electrum_derivation_indices(derivation.as_ref())?;
                hd.from_derivation_indices(change, address)?;
            }
            Some(HdBackend::Cardano(hd)) => {
                hd.derive_path(derivation.path())?;
            }
            Some(HdBackend::Algorand(hd)) => {
                hd.derive_path(derivation.path())?;
            }
            Some(HdBackend::Monero(hd)) => {
                let minor = derivation.minor().unwrap_or(1);
                let major = derivation.major().unwrap_or(0);
                hd.set_derivation_indices(minor, major);
            }
            None => {
                return Err(Error::new(
                    "HD wallet is not initialized from a master key source",
                    None,
                    None,
                    None,
                ));
            }
        }
        Ok(())
    }
}

fn electrum_derivation_indices(derivation: &dyn IDerivation) -> Result<(u32, u32), Error> {
    let change = match derivation.change() {
        Some(ChangeValue::Index(index)) => index,
        Some(ChangeValue::Label(label)) => label.parse().map_err(|_| {
            Error::new("Invalid Electrum change index", None, None, Some(label))
        })?,
        None => 0,
    };
    let address = derivation.address().unwrap_or(0);
    Ok((change, address))
}

pub fn build_derivation(
    derivation_name: &str,
    cryptocurrency: &CryptocurrencyDescriptor,
    account: &str,
    change: &str,
    address: &str,
    role: &str,
    ecc: &str,
    minor: &str,
    major: &str,
    path: Option<&str>,
    indexes: &[u32],
) -> Result<Box<dyn IDerivation>, Error> {
    if !Derivations::is_derivation(derivation_name) {
        return Err(Error::new(
            "Wrong from derivation name",
            None,
            Some(&Derivations::names().join(", ")),
            Some(derivation_name),
        ));
    }

    let account_input = IndexInput::Str(account);
    let address_input = IndexInput::Str(address);
    let minor_input = IndexInput::Str(minor);
    let major_input = IndexInput::Str(major);

    match derivation_name {
        "BIP44" => Ok(Box::new(
            Bip44Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(account_input),
                Some(ChangeInput::Str(change)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "BIP49" => Ok(Box::new(
            Bip49Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(account_input),
                Some(ChangeInput::Str(change)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "BIP84" => Ok(Box::new(
            Bip84Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(account_input),
                Some(ChangeInput::Str(change)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "BIP86" => Ok(Box::new(
            Bip86Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(account_input),
                Some(ChangeInput::Str(change)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "CIP1852" => Ok(Box::new(
            Cip1852Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(account_input),
                Some(RoleInput::Str(role)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "Custom" => Ok(Box::new(
            CustomDerivation::new(
                path,
                if indexes.is_empty() {
                    None
                } else {
                    Some(indexes)
                },
            )
            .map_err(map_derivation_error)?,
        )),
        "Electrum" => Ok(Box::new(
            ElectrumDerivation::new(
                Some(IndexInput::Str(change)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        "Monero" => Ok(Box::new(
            MoneroDerivation::new(Some(minor_input), Some(major_input))
                .map_err(map_derivation_error)?,
        )),
        "HDW" => Ok(Box::new(
            HdwDerivation::new(
                Some(account_input),
                Some(EccInput::Str(ecc)),
                Some(address_input),
            )
            .map_err(map_derivation_error)?,
        )),
        other => Err(Error::new(
            "Dump for this derivation is not implemented yet",
            None,
            Some(&Derivations::names().join(", ")),
            Some(other),
        )),
    }
}

pub fn default_include_for_hd(hd: &str, cryptocurrency: &CryptocurrencyDescriptor) -> String {
    let mut include = match hd {
        "BIP32" => "at:path,addresses:p2pkh,public_key,wif".to_string(),
        "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141" => {
            "at:path,address,public_key,wif".to_string()
        }
        "Cardano" => "at:path,address,public_key,private_key".to_string(),
        "Algorand" => "at:path,address,public_key,private_key".to_string(),
        "Electrum-V1" | "Electrum-V2" => {
            "at:change,at:address,address,public_key,wif".to_string()
        }
        "Monero" => "at:minor,at:major,sub_address".to_string(),
        _ => "at:path,address,public_key,wif".to_string(),
    };

    if matches!(
        cryptocurrency.name,
        "Bitcoin-Cash" | "Bitcoin-Cash-SLP" | "eCash"
    ) {
        include = "at:path,addresses:legacy-p2pkh,public_key,wif".to_string();
    } else if cryptocurrency.name == "Avalanche" {
        include = "at:path,addresses:p-chain,public_key,wif".to_string();
    } else if cryptocurrency.name == "Binance" {
        include = "at:path,addresses:chain,public_key,wif".to_string();
    } else if cryptocurrency.addresses.len() > 1
        || matches!(
            cryptocurrency.name,
            "Algorand" | "Aptos" | "Avalanche" | "Cosmos" | "EOS" | "Ergo" | "Ethereum"
                | "Filecoin" | "Harmony" | "Icon" | "Injective" | "MultiversX" | "Nano" | "Near"
                | "Neo" | "OKT-Chain" | "Ripple" | "Solana" | "Stellar" | "Sui" | "Tezos"
                | "Tron" | "XinFin" | "Zilliqa"
        )
    {
        include = "at:path,address,public_key,private_key".to_string();
    }

    include
}

pub fn flatten_dump_row(dump: &Value, include: &str) -> Map<String, Value> {
    let mut row = Map::new();
    let derivation = dump.get("derivation");
    for key in include.split(',').filter(|item| !item.is_empty()) {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() == 2 {
            if let Some(value) = dump
                .get(parts[0])
                .and_then(|item| item.get(parts[1]))
                .or_else(|| {
                    derivation
                        .and_then(|item| item.get(parts[0]))
                        .and_then(|item| item.get(parts[1]))
                })
            {
                row.insert(key.to_string(), value.clone());
            }
        } else if let Some(value) = dump.get(parts[0]).or_else(|| {
            derivation.and_then(|item| item.get(parts[0]))
        }) {
            row.insert(parts[0].to_string(), value.clone());
        }
    }
    row
}

fn resolve_address_registry_name(
    address_types: Option<crate::hdwallet::cryptocurrencies::icryptocurrency::AddressTypes>,
    address_key: &str,
) -> &str {
    if let Some(types) = address_types {
        for (key, registry) in types {
            if *key == address_key {
                return registry;
            }
        }
    }
    address_key
}

fn derivation_at(derivation_name: &str, derivation: &dyn IDerivation) -> Value {
    match derivation_name {
        "BIP44" | "BIP49" | "BIP84" | "BIP86" => json!({
            "path": derivation.path(),
            "indexes": derivation.indexes(),
            "depth": derivation.depth(),
            "purpose": derivation.purpose(),
            "coin_type": derivation.coin_type(),
            "account": derivation.account(),
            "change": match derivation.change() {
                Some(ChangeValue::Label(label)) => label,
                Some(ChangeValue::Index(0)) => "external-chain",
                Some(ChangeValue::Index(1)) => "internal-chain",
                Some(ChangeValue::Index(_)) => "external-chain",
                None => "external-chain",
            },
            "address": derivation.address(),
        }),
        "CIP1852" => json!({
            "path": derivation.path(),
            "indexes": derivation.indexes(),
            "depth": derivation.depth(),
            "purpose": derivation.purpose(),
            "coin_type": derivation.coin_type(),
            "account": derivation.account(),
            "role": derivation.role(),
            "address": derivation.address(),
        }),
        "Electrum" => json!({
            "change": derivation.change().map(|value| match value {
                ChangeValue::Label(label) => json!(label),
                ChangeValue::Index(index) => json!(index),
            }),
            "address": derivation.address(),
        }),
        "Monero" => json!({
            "minor": derivation.minor(),
            "major": derivation.major(),
        }),
        _ => {
            let indexes = derivation.indexes();
            let mut at = json!({
                "path": derivation.path(),
                "indexes": indexes,
                "depth": derivation.depth(),
            });
            if let Some(index) = indexes.last() {
                if let Some(object) = at.as_object_mut() {
                    object.insert("index".to_string(), json!(index));
                }
            }
            at
        }
    }
}

fn apply_value_exclude(mut value: Value, exclude: &HashSet<String>) -> Value {
    if exclude.is_empty() {
        return value;
    }
    if let Some(object) = value.as_object_mut() {
        object.retain(|key, nested| {
            if exclude.contains(key) {
                return false;
            }
            if let Value::Object(inner) = nested {
                let mut inner = inner.clone();
                inner.retain(|inner_key, _| !exclude.contains(inner_key));
                *nested = Value::Object(inner);
            }
            true
        });
    }
    value
}

fn resolve_semantic(
    hd: &str,
    semantic: Option<&str>,
    cryptocurrency: &CryptocurrencyDescriptor,
) -> Option<String> {
    if let Some(value) = semantic {
        return Some(value.to_string());
    }
    match hd {
        "BIP32" | "BIP44" | "BIP86" | "Cardano" => {
            cryptocurrency.default_semantic.map(str::to_string)
        }
        "BIP49" => Some("p2wpkh-in-p2sh".to_string()),
        "BIP84" | "BIP141" => Some("p2wpkh".to_string()),
        _ => None,
    }
}

fn semantic_xkey_label(semantic: Option<&str>) -> &'static str {
    match semantic {
        Some("p2wpkh-in-p2sh") => "P2WPKH_IN_P2SH",
        Some("p2wpkh") => "P2WPKH",
        Some("p2wsh") => "P2WSH",
        Some("p2wsh-in-p2sh") => "P2WSH_IN_P2SH",
        Some("p2tr") => "P2TR",
        Some("p2sh") => "P2SH",
        _ => "P2PKH",
    }
}

fn xprivate_version_for_semantic(
    network: &crate::hdwallet::cryptocurrencies::icryptocurrency::NetworkDescriptor,
    semantic: Option<&str>,
) -> u32 {
    let label = semantic_xkey_label(semantic);
    network
        .xprivate_key_versions
        .and_then(|versions| versions.iter().find(|(name, _)| *name == label).map(|(_, v)| *v))
        .unwrap_or(0x0488ade4)
}

fn xpublic_version_for_semantic(
    network: &crate::hdwallet::cryptocurrencies::icryptocurrency::NetworkDescriptor,
    semantic: Option<&str>,
) -> u32 {
    let label = semantic_xkey_label(semantic);
    network
        .xpublic_key_versions
        .and_then(|versions| versions.iter().find(|(name, _)| *name == label).map(|(_, v)| *v))
        .unwrap_or(0x0488b21e)
}

fn display_ecc_name(name: &str) -> String {
    match name {
        "SLIP10Secp256k1ECC" => "SLIP10-Secp256k1".to_string(),
        "SLIP10Ed25519ECC" => "SLIP10-Ed25519".to_string(),
        "SLIP10Ed25519Blake2bECC" => "SLIP10-Ed25519-Blake2b".to_string(),
        "SLIP10Ed25519MoneroECC" => "SLIP10-Ed25519-Monero".to_string(),
        "SLIP10Nist256p1ECC" => "SLIP10-Nist256p1".to_string(),
        "KholawEd25519ECC" => "Kholaw-Ed25519".to_string(),
        other => other.to_string(),
    }
}

fn map_bitcoin_network(name: &str) -> Network {
    match name.to_lowercase().as_str() {
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => Network::Bitcoin,
    }
}

fn map_derivation_error(error: crate::hdwallet::exceptions::DerivationError) -> Error {
    Error::new(&error.to_string(), None, None, None)
}

fn hd_name_matches(registry_name: &str, candidate: &str) -> bool {
    if registry_name == candidate {
        return true;
    }
    normalize_hd_name(registry_name) == normalize_hd_name(candidate)
}

fn normalize_hd_name(name: &str) -> String {
    name.to_ascii_uppercase().replace('-', "_")
}

fn client_name_matches(registry_name: &str, candidate: &str) -> bool {
    if registry_name == candidate {
        return true;
    }
    normalize_client_name(registry_name) == normalize_client_name(candidate)
}

fn normalize_client_name(name: &str) -> String {
    name.to_ascii_uppercase().replace('-', "_")
}

fn legacy_or_std_prefix(legacy: Option<u8>, std: Option<u8>, label: &str) -> Option<u32> {
    let value = if label.eq_ignore_ascii_case("legacy") {
        legacy.or(std)
    } else {
        std.or(legacy)
    };
    value.map(u32::from)
}
