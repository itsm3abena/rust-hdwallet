use std::rc::Rc;
use std::collections::HashMap;

// use crate::cryptocurrencies::icryptocurrency::{ICryptocurrency, INetwork};
// use crate::hds::{IHD, HDS};
// use crate::addresses::{IAddress, ADDRESSES};
// use crate::entropies::{IEntropy, ENTROPIES};
// use crate::mnemonics::{IMnemonic, ElectrumV2Mnemonic, MoneroMnemonic, MNEMONICS};
// use crate::seeds::{ISeed, BIP39Seed, CardanoSeed, ElectrumV2Seed, SEEDS};
// use crate::eccs::IEllipticCurveCryptography;
// use crate::exceptions::{Error, NetworkError, AddressError, CryptocurrencyError};
// use crate::consts::{PUBLIC_KEY_TYPES, SEMANTICS, MODES};

pub struct HDWallet {
    // cryptocurrency: Rc<dyn ICryptocurrency>,
    // network: Rc<dyn INetwork>,
    // address: Rc<dyn IAddress>,
    // address_type: Option<String>,
    // address_prefix: Option<String>,
    // ecc: Rc<dyn IEllipticCurveCryptography>,

    entropy: Option<Rc<dyn IEntropy>>,
    // language: Option<String>,
    // passphrase: Option<String>,
    // mnemonic: Option<Rc<dyn IMnemonic>>,
    // seed: Option<Rc<dyn ISeed>>,
    // derivation: Option<String>,

    // semantic: Option<String>,

    // // Electrum-V2
    // mode: Option<String>,
    // mnemonic_type: Option<String>,

    // public_key_type: Option<String>,
    // cardano_type: Option<String>,
    // use_default_path: bool,
    // checksum: bool,
    kwargs: HashMap<String, Option<String>>,

    // hd: Rc<dyn IHD>,
}

impl HDWallet {
    pub fn new(
        cryptocurrency: Rc<dyn ICryptocurrency>,
        hd_type: Option<Rc<dyn IHD>>,
        network_name: &str,
        address_name: Option<&str>,
        kwargs: HashMap<String, Option<String>>,
    ) -> Result<Self, Error> {

        let ecc = cryptocurrency.get_ecc();

        let hd: Rc<dyn IHD> = match hd_type {
            Some(hd_val) => hd_val,
            None => HDS::hd(cryptocurrency.default_hd())?,
        };

        if !cryptocurrency.networks().is_network(network_name) {
            return Err(Error::NetworkError(NetworkError {
                expected: cryptocurrency.networks().get_networks(),
                got: network_name.to_string(),
            }));
        }
        let network = cryptocurrency.networks().get_network(network_name)?;

        let address_name = address_name.unwrap_or(cryptocurrency.default_address());
        if !cryptocurrency.addresses().get_addresses().contains(&address_name.to_string()) {
            return Err(Error::AddressError(AddressError {
                expected: cryptocurrency.addresses().get_addresses(),
                got: address_name.to_string(),
            }));
        }
        let address = ADDRESSES::address(address_name)?;

        let mode = kwargs.get("mode").cloned().unwrap_or(Some(MODES::STANDARD.to_string()));
        let mnemonic_type = kwargs
            .get("mnemonic_type")
            .cloned()
            .unwrap_or(Some("standard".to_string()));
        let checksum = kwargs.get("checksum").and_then(|c| {
            c.as_ref().map(|s| s == "true")
        }).unwrap_or(true);
        let cardano_type = kwargs.get("cardano_type").cloned().unwrap_or(None);

        Ok(Self {
            cryptocurrency,
            network,
            address,
            address_type: None,
            address_prefix: None,
            ecc,
            entropy: None,
            language: kwargs.get("language").cloned().unwrap_or(Some("english".to_string())),
            passphrase: kwargs.get("passphrase").cloned().unwrap_or(None),
            mnemonic: None,
            seed: None,
            derivation: None,
            semantic: None,
            mode,
            mnemonic_type,
            public_key_type: kwargs.get("public_key_type").cloned(),
            cardano_type,
            use_default_path: kwargs.get("use_default_path").is_some(),
            checksum,
            kwargs,
            hd,
        })
    }

    pub fn from_entropy(&mut self, entropy: Rc<dyn IEntropy>) -> Result<Rc<dyn IMnemonic>, Error> {
        if !self.cryptocurrency.entropies().get_entropies().contains(&entropy.name()) {
            return Err(Error::new(format!(
                "Invalid entropy class for {}",
                self.cryptocurrency.name()
            )));
        }
        self.entropy = Some(entropy.clone());

        let mnemonic: Rc<dyn IMnemonic> = match entropy.name().as_str() {
            "Electrum-V2" => {
                let mnemonic_str = ElectrumV2Mnemonic::from_entropy(
                    entropy.entropy(),
                    self.language.clone(),
                    self.mnemonic_type.clone(),
                )?;
                Rc::new(ElectrumV2Mnemonic::new(mnemonic_str, self.mnemonic_type.clone()))
            }
            "Monero" => {
                let mnemonic_str =
                    MoneroMnemonic::from_entropy(entropy.entropy(), self.language.clone(), self.checksum)?;
                Rc::new(MoneroMnemonic::new(mnemonic_str, self.checksum))
            }
            _ => {
                let mnemonic_str =
                    MNEMONICS::mnemonic(entropy.name())?.from_entropy(entropy.entropy(), self.language.clone())?;
                MNEMONICS::mnemonic(entropy.name())?.call(mnemonic_str)?
            }
        };

        self.from_mnemonic(mnemonic.clone())?;
        Ok(mnemonic)
    }

    pub fn from_mnemonic(&mut self, mnemonic: Rc<dyn IMnemonic>) -> Result<(), Error> {
        if !self.cryptocurrency.mnemonics().get_mnemonics().contains(&mnemonic.name()) {
            return Err(Error::new(format!(
                "Invalid mnemonic class for {}",
                self.cryptocurrency.name()
            )));
        }
        self.mnemonic = Some(mnemonic.clone());

        let entropy = ENTROPIES::entropy(mnemonic.name())?.call(
            &mnemonic.decode(self.mnemonic_type.clone())?
        )?;
        self.entropy = Some(entropy);

        let seed: Rc<dyn ISeed> = match mnemonic.name().as_str() {
            "BIP39" if self.hd.name() == "Cardano" => Rc::new(CardanoSeed::from_mnemonic(
                mnemonic.mnemonic(),
                self.passphrase.clone(),
                self.cardano_type.clone(),
            )?),
            "BIP39" => Rc::new(BIP39Seed::from_mnemonic(mnemonic.mnemonic(), self.passphrase.clone())?),
            "Electrum-V2" => Rc::new(ElectrumV2Seed::from_mnemonic(
                mnemonic.mnemonic(),
                self.passphrase.clone(),
                self.mnemonic_type.clone(),
            )?),
            _ => SEEDS::seed(mnemonic.name())?.from_mnemonic(mnemonic.mnemonic())?,
        };

        self.seed = Some(seed);
        Ok(())
    }

    pub fn from_seed(&mut self, seed: Rc<dyn ISeed>) -> Result<(), Error> {
        if !self.cryptocurrency.seeds().get_seeds().contains(&seed.name()) {
            return Err(Error::new(format!(
                "Invalid seed class for {}",
                self.cryptocurrency.name()
            )));
        }
        self.seed = Some(seed.clone());
        self.hd.from_seed(seed.seed(), self.passphrase.clone())?;
        self.derivation = Some(self.hd.derivation());
        Ok(())
    }

    pub fn from_xprivate_key(
        &mut self,
        xprivate_key: &str,
        encoded: bool,
        strict: bool,
    ) -> Result<(), Error> {
        if self.hd.name() == "Electrum-V1" || self.hd.name() == "Monero" {
            return Err(Error::new(format!(
                "Conversion from xprivate key is not implemented for {} HD type",
                self.hd.name()
            )));
        }

        if !crate::keys::is_valid_key(xprivate_key, encoded) {
            return Err(Error::XPrivateKeyError(
                "Invalid xprivate key data".to_string(),
            ));
        }

        let (version, _depth, _parent_fingerprint, _index, _chain_code, _key) =
            crate::keys::deserialize(xprivate_key, encoded)?;

        if !self.network.xprivate_key_versions().is_version(version)
            || (if encoded {
                crate::libs::base58::check_decode(xprivate_key)?.len()
            } else {
                xprivate_key.len()
            } != 78 && xprivate_key.len() != 110)
        {
            return Err(Error::new(format!(
                "Invalid {} extended(x) private key",
                self.cryptocurrency.name()
            )));
        }

        self.hd.from_xprivate_key(xprivate_key, encoded, strict)?;
        Ok(())
    }

    pub fn from_xpublic_key(
        &mut self,
        xpublic_key: &str,
        encoded: bool,
        strict: bool,
    ) -> Result<(), Error> {
        if self.hd.name() == "Electrum-V1" || self.hd.name() == "Monero" {
            return Err(Error::new(format!(
                "Conversion from xpublic key is not implemented for {} HD type",
                self.hd.name()
            )));
        } else if self.hd.name() == "Cardano" && self.cardano_type.as_deref() == Some("byron-legacy") {
            return Err(Error::new(format!(
                "Conversion from xpublic key is not implemented for {} HD {} type",
                self.hd.name(),
                self.cardano_type.clone().unwrap_or_default()
            )));
        }

        if !crate::keys::is_valid_key(xpublic_key, encoded) {
            return Err(Error::XPrivateKeyError(
                "Invalid xpublic key data".to_string(),
            ));
        }

        let (version, _depth, _parent_fingerprint, _index, _chain_code, _key) =
            crate::keys::deserialize(xpublic_key, encoded)?;

        if !self.network.xpublic_key_versions().is_version(version)
            || (if encoded {
                crate::libs::base58::check_decode(xpublic_key)?.len()
            } else {
                xpublic_key.len()
            } != 78 && xpublic_key.len() != 110)
        {
            return Err(Error::new(format!(
                "Invalid {} extended(x) public key",
                self.cryptocurrency.name()
            )));
        }

        self.hd.from_xpublic_key(xpublic_key, encoded, strict)?;
        Ok(())
    }

    pub fn from_derivation(&mut self, derivation: Rc<dyn IDerivation>) -> Result<(), Error> {
        self.hd.from_derivation(derivation.clone())?;
        self.derivation = Some(derivation);
        Ok(())
    }

    pub fn update_derivation(&mut self, derivation: Rc<dyn IDerivation>) -> Result<(), Error> {
        self.hd.update_derivation(derivation.clone())?;
        self.derivation = Some(derivation);
        Ok(())
    }

    pub fn clean_derivation(&mut self) -> Result<(), Error> {
        self.hd.clean_derivation()?;
        if let Some(derivation) = &self.derivation {
            derivation.clean()?;
        }
        Ok(())
    }

    pub fn from_private_key(&mut self, private_key: &str) -> Result<(), Error> {
        self.hd.from_private_key(private_key)?;
        Ok(())
    }

    pub fn from_wif(&mut self, wif: &str) -> Result<(), Error> {
        if ["Algorand", "Cardano", "Monero"].contains(&self.hd.name().as_str()) {
            return Err(Error::new(format!(
                "WIF isn't supported by {} HD",
                self.hd.name()
            )));
        }

        if self.network.wif_prefix().is_none() {
            return Err(Error::new(format!(
                "WIF isn't supported by {} cryptocurrency",
                self.cryptocurrency.name()
            )));
        }

        self.hd.from_wif(wif)?;
        Ok(())
    }

    pub fn from_public_key(&mut self, public_key: &str) -> Result<(), Error> {
        if self.hd.name() == "Monero" {
            return Err(Error::new(format!(
                "From public key isn't implemented for the {} HD type",
                self.hd.name()
            )));
        }

        self.hd.from_public_key(public_key)?;
        Ok(())
    }

    pub fn from_spend_private_key(
        &mut self,
        spend_private_key: &dyn IPrivateKeyLike,
    ) -> Result<(), Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new("From spend private key only supported by Monero HD".into()));
        }

        self.hd
            .from_spend_private_key(spend_private_key)
            .map_err(|_| Error::PrivateKeyError("Invalid spend private key data".into()))?;
        Ok(())
    }

    pub fn from_watch_only(
        &mut self,
        view_private_key: &dyn IPrivateKeyLike,
        spend_public_key: &dyn IPublicKeyLike,
    ) -> Result<(), Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new(
                "From spend watch only supported by Monero HD wallet".into(),
            ));
        }

        self.hd
            .from_watch_only(view_private_key, spend_public_key)?;
        Ok(())
    }

    pub fn cryptocurrency(&self) -> &str {
        &self.cryptocurrency.name
    }

    pub fn symbol(&self) -> &str {
        &self.cryptocurrency.symbol
    }

    pub fn coin_type(&self) -> u32 {
        self.cryptocurrency.coin_type
    }

    pub fn network(&self) -> &str {
        &self.network.name
    }

    pub fn entropy(&self) -> Option<&str> {
        self.entropy.as_ref().map(|e| e.entropy())
    }

    pub fn strength(&self) -> Option<&str> {
        self.entropy.as_ref().map(|e| e.strength())
    }

    pub fn mnemonic(&self) -> Option<&str> {
        self.mnemonic.as_ref().map(|m| m.mnemonic())
    }

    pub fn mnemonic_type(&self) -> Option<&str> {
        self.mnemonic_type.as_deref()
    }

    pub fn language(&self) -> Option<&str> {
        self.mnemonic.as_ref().map(|m| m.language())
    }

    pub fn words(&self) -> Option<usize> {
        self.mnemonic.as_ref().map(|m| m.words())
    }

    pub fn passphrase(&self) -> Option<&str> {
        self.passphrase.as_deref()
    }

    pub fn seed(&self) -> Option<&str> {
        self.hd.seed()
    }

    pub fn ecc(&self) -> &str {
        &self.hd.ecc.name
    }

    pub fn hd(&self) -> &str {
        self.hd.name()
    }

    pub fn semantic(&self) -> Option<&str> {
        self.semantic.as_deref()
    }

    pub fn cardano_type(&self) -> Option<&str> {
        if self.hd.name() == "Cardano" {
            self.cardano_type.as_deref()
        } else {
            None
        }
    }

    pub fn mode(&self) -> Result<&str, Error> {
        if self.hd.name() != "Electrum-V2" {
            return Err(Error::new(format!(
                "Get mode is only for {} HD type",
                self.hd.name()
            )));
        }
        Ok(self.hd.mode())
    }

    pub fn path_key(&self) -> Option<&str> {
        self.hd.path_key()
    }

    pub fn root_xprivate_key(
        &self,
        semantic: Option<&str>,
        encoded: bool,
    ) -> Option<String> {
        let sem = semantic.unwrap_or_else(|| self.semantic.as_deref().unwrap_or(""));
        if ["Electrum-V1", "Monero"].contains(&self.hd.name()) {
            return None;
        }
        self.hd.root_xprivate_key(
            self.network.xprivate_key_versions.get_version(sem),
            encoded,
        )
    }

    pub fn root_xpublic_key(
        &self,
        semantic: Option<&str>,
        encoded: bool,
    ) -> Option<String> {
        let sem = semantic.unwrap_or_else(|| self.semantic.as_deref().unwrap_or(""));
        if ["Electrum-V1", "Monero"].contains(&self.hd.name()) {
            return None;
        }
        self.hd.root_xpublic_key(
            self.network.xpublic_key_versions.get_version(sem),
            encoded,
        )
    }

    pub fn master_xprivate_key(&self, semantic: Option<&str>, encoded: bool) -> Option<String> {
        self.root_xprivate_key(semantic, encoded)
    }

    pub fn master_xpublic_key(&self, semantic: Option<&str>, encoded: bool) -> Option<String> {
        self.root_xpublic_key(semantic, encoded)
    }

    pub fn root_private_key(&self) -> Option<String> {
        match self.hd.name() {
            "Electrum-V1" | "Electrum-V2" => self.hd.master_private_key(),
            _ => self.hd.root_private_key(),
        }
    }

    pub fn root_wif(&self, wif_type: Option<&str>) -> Option<String> {
        match self.hd.name() {
            "Algorand" | "Cardano" | "Monero" => None,
            "Electrum-V1" | "Electrum-V2" => self.hd.master_wif(wif_type),
            _ => self.hd.root_wif(wif_type),
        }
    }

    pub fn root_chain_code(&self) -> Option<String> {
        self.hd.root_chain_code()
    }

    pub fn root_public_key(&self, public_key_type: Option<&str>) -> Option<String> {
        match self.hd.name() {
            "Electrum-V1" | "Electrum-V2" => self.hd.master_public_key(public_key_type),
            _ => self.hd.root_public_key(public_key_type),
        }
    }

    pub fn master_private_key(&self) -> Option<String> {
        match self.hd.name() {
            "Electrum-V1" | "Electrum-V2" => self.hd.master_private_key(),
            _ => self.hd.root_private_key(),
        }
    }

    pub fn master_wif(&self, wif_type: Option<&str>) -> Option<String> {
        match self.hd.name() {
            "Algorand" | "Cardano" | "Monero" => None,
            "Electrum-V1" | "Electrum-V2" => self.hd.master_wif(wif_type),
            _ => self.hd.root_wif(wif_type),
        }
    }

    pub fn master_chain_code(&self) -> Option<String> {
        self.hd.root_chain_code()
    }

    pub fn master_public_key(&self, public_key_type: Option<&str>) -> Option<String> {
        match self.hd.name() {
            "Electrum-V1" | "Electrum-V2" => self.hd.master_public_key(public_key_type),
            _ => self.hd.root_public_key(public_key_type),
        }
    }

    pub fn xprivate_key(&self, semantic: Option<&str>, encoded: bool) -> Option<String> {
        let sem = semantic.unwrap_or_else(|| self.semantic.as_deref().unwrap_or(""));
        if ["Electrum-V1", "Monero"].contains(&self.hd.name()) {
            return None;
        }
        self.hd.xprivate_key(self.network.xprivate_key_versions.get_version(sem), encoded)
    }

    pub fn xpublic_key(&self, semantic: Option<&str>, encoded: bool) -> Option<String> {
        let sem = semantic.unwrap_or_else(|| self.semantic.as_deref().unwrap_or(""));
        if ["Electrum-V1", "Monero"].contains(&self.hd.name()) {
            return None;
        }
        self.hd.xpublic_key(self.network.xpublic_key_versions.get_version(sem), encoded)
    }

    pub fn private_key(&self) -> Option<String> {
        self.hd.private_key()
    }

    pub fn spend_private_key(&self) -> Result<String, Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new("Spend private key only supported by Monero HD wallet"));
        }
        Ok(self.hd.spend_private_key())
    }

    pub fn view_private_key(&self) -> Result<String, Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new("View private key only supported by Monero HD wallet"));
        }
        Ok(self.hd.view_private_key())
    }

    pub fn wif(&self, wif_type: Option<&str>) -> Option<String> {
        match self.hd.name() {
            "Algorand" | "Cardano" | "Monero" => None,
            _ => self.hd.wif(wif_type),
        }
    }

    pub fn wif_type(&self) -> Option<String> {
        self.wif(None).and_then(|_| self.hd.wif_type())
    }

    pub fn chain_code(&self) -> Option<String> {
        self.hd.chain_code()
    }

    pub fn public_key(&self, public_key_type: Option<&str>) -> String {
        self.hd.public_key(public_key_type)
    }

    pub fn public_key_type(&self) -> String {
        self.hd.public_key_type()
    }

    pub fn uncompressed(&self) -> String {
        self.hd.uncompressed()
    }

    pub fn compressed(&self) -> String {
        self.hd.compressed()
    }

    pub fn spend_public_key(&self) -> Result<String, Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new("Spend public key only supported by Monero HD wallet"));
        }
        Ok(self.hd.spend_public_key())
    }

    pub fn view_public_key(&self) -> Result<String, Error> {
        if self.hd.name() != "Monero" {
            return Err(Error::new("View public key only supported by Monero HD wallet"));
        }
        Ok(self.hd.view_public_key())
    }

    pub fn hash(&self) -> String {
        self.hd.hash()
    }

    pub fn depth(&self) -> u32 {
        self.hd.depth()
    }

    pub fn fingerprint(&self) -> String {
        self.hd.fingerprint()
    }

    pub fn parent_fingerprint(&self) -> String {
        self.hd.parent_fingerprint()
    }

    pub fn path(&self) -> String {
        self.hd.path()
    }

    pub fn index(&self) -> u32 {
        self.hd.index()
    }

    pub fn indexes(&self) -> Vec<u32> {
        self.hd.indexes()
    }

    pub fn strict(&self) -> Option<bool> {
        match self.hd.name() {
            "Electrum-V1" | "Monero" => None,
            _ => self.hd.strict(),
        }
    }

    // Monero-specific addresses
    pub fn primary_address(&self) -> Option<String> {
        if self.hd.name() == "Monero" {
            Some(self.hd.primary_address())
        } else {
            None
        }
    }

    pub fn integrated_address(&self, payment_id: Option<&[u8]>) -> Option<String> {
        if self.hd.name() == "Monero" {
            let pid = payment_id.unwrap_or_else(|| self.kwargs.get("payment_id").unwrap_or(&[]));
            Some(self.hd.integrated_address(pid))
        } else {
            None
        }
    }

    pub fn sub_address(&self, minor: Option<u32>, major: Option<u32>) -> Option<String> {
        if self.hd.name() == "Monero" {
            Some(self.hd.sub_address(minor, major))
        } else {
            None
        }
    }

    pub fn address(&self, address: Option<&str>, kwargs: &HashMap<String, String>) -> Result<String, AddressError> {
        let addr_name = match address {
            Some(name) => {
                if self.cryptocurrency.ADDRESSES.get_addresses().contains(&name.to_string()) {
                    name.to_string()
                } else {
                    return Err(AddressError::new(
                        &format!("Wrong {} address", self.cryptocurrency.NAME),
                        self.cryptocurrency.ADDRESSES.get_addresses(),
                        name.to_string(),
                    ));
                }
            }
            None => self.address.name().to_string(),
        };

        match self.hd.name() {
            "Cardano" => Ok(self.hd.address(
                Some(&self.network.NAME),
                kwargs.get("address_type").unwrap_or(&self.address_type),
                kwargs.get("staking_public_key").or(self.kwargs.get("staking_public_key")),
            )),
            "Electrum-V1" => Ok(self.hd.address(Some(&self.network.PUBLIC_KEY_ADDRESS_PREFIX), None, None)),
            "Electrum-V2" => Ok(self.hd.address(
                Some(&self.network.PUBLIC_KEY_ADDRESS_PREFIX),
                Some(&self.network.HRP),
                Some(self.network.WITNESS_VERSIONS.get_witness_version("P2WPKH")),
            )),
            "Monero" => {
                match kwargs.get("version_type").map(|s| s.as_str()) {
                    Some("standard") => Ok(self.primary_address().unwrap_or_default()),
                    Some("integrated") => Ok(self.integrated_address(
                        kwargs.get("payment_id").map(|pid| pid.as_bytes())
                    ).unwrap_or_default()),
                    Some("sub-address") => Ok(self.sub_address(
                        kwargs.get("minor").and_then(|v| v.parse::<u32>().ok()),
                        kwargs.get("major").and_then(|v| v.parse::<u32>().ok())
                    ).unwrap_or_default()),
                    _ => Ok(self.primary_address().unwrap_or_default()),
                }
            }
            _ => {
                // Generic address encoding
                let addr = ADDRESSES.address(&addr_name);
                Ok(addr.encode(
                    &self.public_key(),
                    &self.network.PUBLIC_KEY_ADDRESS_PREFIX,
                    &self.network.SCRIPT_ADDRESS_PREFIX,
                    &self.network.NAME,
                    &self.public_key_type(),
                    &self.network.HRP,
                    kwargs.get("address_type").unwrap_or(&self.address_type),
                    kwargs.get("address_prefix").unwrap_or(&self.address_prefix),
                ))
            }
        }
    }

    pub fn dump(&self, exclude: Option<HashSet<&str>>) -> HashMap<String, serde_json::Value> {
        let exclude = exclude.unwrap_or_default();
        let mut derivation: HashMap<String, serde_json::Value> = HashMap::new();

        // --- Derivation details ---
        if let Some(derivation_obj) = &self.derivation {
            let at: HashMap<String, serde_json::Value> = match derivation_obj.name().as_str() {
                "BIP44" | "BIP49" | "BIP84" | "BIP86" => {
                    hashmap! {
                        "path".to_string() => serde_json::Value::String(derivation_obj.path()),
                        "indexes".to_string() => serde_json::to_value(derivation_obj.indexes()).unwrap(),
                        "depth".to_string() => serde_json::Value::Number(self.depth().into()),
                        "purpose".to_string() => serde_json::Value::Number(derivation_obj.purpose().into()),
                        "coin_type".to_string() => serde_json::Value::Number(derivation_obj.coin_type().into()),
                        "account".to_string() => serde_json::Value::Number(derivation_obj.account().into()),
                        "change".to_string() => serde_json::Value::Number(derivation_obj.change().into()),
                        "address".to_string() => serde_json::Value::String(derivation_obj.address()),
                    }
                }
                "CIP1852" => {
                    hashmap! {
                        "path".to_string() => serde_json::Value::String(derivation_obj.path()),
                        "indexes".to_string() => serde_json::to_value(derivation_obj.indexes()).unwrap(),
                        "depth".to_string() => serde_json::Value::Number(self.depth().into()),
                        "purpose".to_string() => serde_json::Value::Number(derivation_obj.purpose().into()),
                        "coin_type".to_string() => serde_json::Value::Number(derivation_obj.coin_type().into()),
                        "account".to_string() => serde_json::Value::Number(derivation_obj.account().into()),
                        "role".to_string() => serde_json::Value::Number(derivation_obj.role().into()),
                        "address".to_string() => serde_json::Value::String(derivation_obj.address()),
                    }
                }
                "Electrum" => {
                    hashmap! {
                        "change".to_string() => serde_json::Value::Number(derivation_obj.change().into()),
                        "address".to_string() => serde_json::Value::String(derivation_obj.address()),
                    }
                }
                "Monero" => {
                    hashmap! {
                        "minor".to_string() => serde_json::Value::Number(derivation_obj.minor().into()),
                        "major".to_string() => serde_json::Value::Number(derivation_obj.major().into()),
                    }
                }
                _ => {
                    hashmap! {
                        "path".to_string() => serde_json::Value::String(derivation_obj.path()),
                        "indexes".to_string() => serde_json::to_value(derivation_obj.indexes()).unwrap(),
                        "depth".to_string() => serde_json::Value::Number(self.depth().into()),
                        "index".to_string() => serde_json::Value::Number(self.index().into()),
                    }
                }
            };
            derivation.insert("at".to_string(), serde_json::Value::Object(at));
        }

        // --- HD-specific keys ---
        match self.hd.name().as_str() {
            "Algorand" | "BIP32" | "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141" | "Cardano" => {
                derivation.insert("xprivate_key".to_string(), serde_json::Value::String(self.xprivate_key().unwrap_or_default()));
                derivation.insert("xpublic_key".to_string(), serde_json::Value::String(self.xpublic_key().unwrap_or_default()));
                derivation.insert("private_key".to_string(), serde_json::Value::String(self.private_key().unwrap_or_default()));
                derivation.insert("wif".to_string(), serde_json::Value::String(self.wif().unwrap_or_default()));
                derivation.insert("chain_code".to_string(), serde_json::Value::String(self.chain_code().unwrap_or_default()));
                derivation.insert("public_key".to_string(), serde_json::Value::String(self.public_key()));
                derivation.insert("uncompressed".to_string(), serde_json::Value::String(self.uncompressed()));
                derivation.insert("compressed".to_string(), serde_json::Value::String(self.compressed()));
                derivation.insert("hash".to_string(), serde_json::Value::String(self.hash()));
                derivation.insert("fingerprint".to_string(), serde_json::Value::String(self.fingerprint()));
                derivation.insert("parent_fingerprint".to_string(), serde_json::Value::String(self.parent_fingerprint()));

                if self.hd.name() == "Algorand" || self.hd.name() == "Cardano" {
                    derivation.remove("wif");
                    derivation.remove("uncompressed");
                    derivation.remove("compressed");
                }
            }
            "Electrum-V1" | "Electrum-V2" => {
                derivation.insert("private_key".to_string(), serde_json::Value::String(self.private_key().unwrap_or_default()));
                derivation.insert("wif".to_string(), serde_json::Value::String(self.wif().unwrap_or_default()));
                derivation.insert("public_key".to_string(), serde_json::Value::String(self.public_key()));
                derivation.insert("uncompressed".to_string(), serde_json::Value::String(self.uncompressed()));
                derivation.insert("compressed".to_string(), serde_json::Value::String(self.compressed()));
                derivation.insert("address".to_string(), serde_json::Value::String(self.address(None, &HashMap::new()).unwrap_or_default()));
            }
            "Monero" => {
                derivation.insert("sub_address".to_string(), serde_json::Value::String(self.sub_address(None, None).unwrap_or_default()));
            }
            _ => {}
        }

        if exclude.contains("at") {
            derivation.remove("at");
        }

        // --- Root-level info ---
        let mut root: HashMap<String, serde_json::Value> = hashmap! {
            "cryptocurrency".to_string() => serde_json::Value::String(self.cryptocurrency()),
            "symbol".to_string() => serde_json::Value::String(self.symbol()),
            "network".to_string() => serde_json::Value::String(self.network()),
            "coin_type".to_string() => serde_json::Value::Number(self.coin_type().into()),
            "entropy".to_string() => serde_json::Value::String(self.entropy()),
            "strength".to_string() => serde_json::Value::Number(self.strength().into()),
            "mnemonic".to_string() => serde_json::Value::String(self.mnemonic()),
            "passphrase".to_string() => serde_json::Value::String(self.passphrase()),
            "language".to_string() => serde_json::Value::String(self.language()),
            "seed".to_string() => serde_json::Value::String(self.seed()),
            "ecc".to_string() => serde_json::Value::String(self.ecc()),
            "hd".to_string() => serde_json::Value::String(self.hd.name()),
        };

        if !exclude.contains("derivation") {
            root.insert("derivation".to_string(), serde_json::Value::Object(derivation));
        }

        // Apply exclude keys
        root.retain(|k, _| !exclude.contains(k.as_str()));

        root
    }

    pub fn dumps(
        &mut self,
        exclude: Option<HashSet<&str>>,
    ) -> Option<serde_json::Value> {
        let exclude = exclude.unwrap_or_default();
        let mut derivations: Vec<HashMap<String, serde_json::Value>> = vec![];

        fn drive_helper(
            wallet: &mut HDWallet,
            derivations_list: &[Vec<(i32, bool)>],
            current_derivation: &mut Vec<(i32, bool)>,
            exclude: &HashSet<&str>,
            output: &mut Vec<HashMap<String, serde_json::Value>>,
        ) -> Vec<String> {
            if derivations_list.is_empty() {
                // Build new derivation based on wallet type
                let derivation_name = wallet.derivation.as_ref().unwrap().name();
                let new_derivation = match derivation_name.as_str() {
                    "BIP44" | "BIP49" | "BIP84" | "BIP86" => {
                        DERIVATIONS
                            .derivation(&derivation_name)
                            .call_bip44(
                                current_derivation[1].0,
                                current_derivation[2].0,
                                current_derivation[3].0,
                                current_derivation[4].0,
                            )
                    }
                    "CIP1852" => {
                        DERIVATIONS
                            .derivation(&derivation_name)
                            .call_cip1852(
                                current_derivation[1].0,
                                current_derivation[2].0,
                                current_derivation[3].0,
                                current_derivation[4].0,
                            )
                    }
                    "Electrum" => {
                        DERIVATIONS
                            .derivation(&derivation_name)
                            .call_electrum(current_derivation[0].0, current_derivation[1].0)
                    }
                    "Monero" => {
                        DERIVATIONS
                            .derivation(&derivation_name)
                            .call_monero(current_derivation[0].0, current_derivation[1].0)
                    }
                    "HDW" => {
                        DERIVATIONS
                            .derivation(&derivation_name)
                            .call_hdw(current_derivation[0].0, current_derivation[1].0, current_derivation[2].0)
                    }
                    _ => {
                        let path_str: String = current_derivation
                            .iter()
                            .map(|(v, hardened)| if *hardened { format!("{}'", v) } else { v.to_string() })
                            .collect::<Vec<String>>()
                            .join("/");
                        DERIVATIONS.derivation(&derivation_name).call_path(&format!("m/{}", path_str))
                    }
                };

                wallet.update_derivation(new_derivation.clone());
                output.push(wallet.dump(Some(exclude.clone())));
                vec![new_derivation.path()]
            } else {
                let mut paths = vec![];
                let first = &derivations_list[0];
                if first.len() == 3 {
                    // Range derivation
                    for value in first[0].0..=first[1].0 {
                        let mut new_current = current_derivation.clone();
                        new_current.push((value, first[2].1));
                        paths.extend(drive_helper(wallet, &derivations_list[1..], &mut new_current, exclude, output));
                    }
                } else {
                    let mut new_current = current_derivation.clone();
                    new_current.extend_from_slice(first);
                    paths.extend(drive_helper(wallet, &derivations_list[1..], &mut new_current, exclude, output));
                }
                paths
            }
        }

        if self.derivation.is_none() {
            return None;
        }

        let mut current = vec![];
        drive_helper(
            self,
            &self.derivation.as_ref().unwrap().derivations(),
            &mut current,
            &exclude,
            &mut derivations,
        );

        if exclude.contains("root") {
            return Some(serde_json::Value::Array(
                derivations.into_iter().map(serde_json::Value::Object).collect(),
            ));
        }

        let mut root = self.dump(Some(exclude.union(&["derivation"].iter().cloned().collect()).cloned().collect()));

        if !exclude.contains("derivations") {
            root.insert(
                "derivations".to_string(),
                serde_json::Value::Array(
                    derivations.into_iter().map(serde_json::Value::Object).collect(),
                ),
            );
        }

        Some(root)
    }
}