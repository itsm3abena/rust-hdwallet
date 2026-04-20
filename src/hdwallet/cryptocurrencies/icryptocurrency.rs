#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkName {
    Mainnet,
    Testnet,
    Regtest,
}

impl NetworkName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mainnet" => Some(Self::Mainnet),
            "testnet" => Some(Self::Testnet),
            "regtest" => Some(Self::Regtest),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptocurrencyKind {
    Bitcoin,
    Ethereum,
    Litecoin,
    Dogecoin,
    Adcoin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WitnessVersions(pub &'static [u8]);

pub type XPrivateKeyVersions = &'static [(&'static str, u32)];

pub type XPublicKeyVersions = &'static [(&'static str, u32)];

pub type AddressTypes = &'static [&'static str];

pub type AddressPrefixes = &'static [&'static str];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Params {
    pub checksum_length: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoValue<'a> {
    Str(&'a str),
    List(&'a [&'a str]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoItem<'a> {
    pub key: &'static str,
    pub value: InfoValue<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Info<'a>(pub &'a [InfoItem<'a>]);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkDescriptor {
    pub name: &'static str,
    pub public_key_address_prefix: Option<u32>,
    pub script_address_prefix: Option<u32>,
    pub hrp: Option<&'static str>,
    pub witness_versions: Option<WitnessVersions>,
    pub xprivate_key_versions: Option<XPrivateKeyVersions>,
    pub xpublic_key_versions: Option<XPublicKeyVersions>,
    pub message_prefix: Option<&'static [u8]>,
    pub wif_prefix: Option<u8>,
    pub legacy_public_key_address_prefix: Option<u8>,
    pub std_public_key_address_prefix: Option<u8>,
    pub legacy_script_address_prefix: Option<u8>,
    pub std_script_address_prefix: Option<u8>,
    pub standard: Option<u32>,
    pub integrated: Option<u32>,
    pub sub_address: Option<u32>,
    pub network_type: Option<u32>,
    pub payment_address_hrp: Option<&'static str>,
    pub reward_address_hrp: Option<&'static str>,
}

pub trait INetwork {
    fn name(&self) -> &'static str;

    fn public_key_address_prefix(&self) -> Option<u32> {
        None
    }
    fn script_address_prefix(&self) -> Option<u32> {
        None
    }
    fn hrp(&self) -> Option<&'static str> {
        None
    }
    fn witness_versions(&self) -> Option<WitnessVersions> {
        None
    }
    fn xprivate_key_versions(&self) -> Option<XPrivateKeyVersions> {
        None
    }
    fn xpublic_key_versions(&self) -> Option<XPublicKeyVersions> {
        None
    }
    fn message_prefix(&self) -> Option<&'static [u8]> {
        None
    }
    fn wif_prefix(&self) -> Option<u8> {
        None
    }
    fn legacy_public_key_address_prefix(&self) -> Option<u8> {
        None
    }
    fn std_public_key_address_prefix(&self) -> Option<u8> {
        None
    }
    fn legacy_script_address_prefix(&self) -> Option<u8> {
        None
    }
    fn std_script_address_prefix(&self) -> Option<u8> {
        None
    }
    fn standard(&self) -> Option<u32> {
        None
    }
    fn integrated(&self) -> Option<u32> {
        None
    }
    fn sub_address(&self) -> Option<u32> {
        None
    }
    fn network_type(&self) -> Option<u32> {
        None
    }
    fn payment_address_hrp(&self) -> Option<&'static str> {
        None
    }
    fn reward_address_hrp(&self) -> Option<&'static str> {
        None
    }
}

impl INetwork for NetworkDescriptor {
    fn name(&self) -> &'static str {
        self.name
    }

    fn public_key_address_prefix(&self) -> Option<u32> {
        self.public_key_address_prefix
    }

    fn script_address_prefix(&self) -> Option<u32> {
        self.script_address_prefix
    }

    fn hrp(&self) -> Option<&'static str> {
        self.hrp
    }

    fn witness_versions(&self) -> Option<WitnessVersions> {
        self.witness_versions
    }

    fn xprivate_key_versions(&self) -> Option<XPrivateKeyVersions> {
        self.xprivate_key_versions
    }

    fn xpublic_key_versions(&self) -> Option<XPublicKeyVersions> {
        self.xpublic_key_versions
    }

    fn message_prefix(&self) -> Option<&'static [u8]> {
        self.message_prefix
    }

    fn wif_prefix(&self) -> Option<u8> {
        self.wif_prefix
    }

    fn legacy_public_key_address_prefix(&self) -> Option<u8> {
        self.legacy_public_key_address_prefix
    }

    fn std_public_key_address_prefix(&self) -> Option<u8> {
        self.std_public_key_address_prefix
    }

    fn legacy_script_address_prefix(&self) -> Option<u8> {
        self.legacy_script_address_prefix
    }

    fn std_script_address_prefix(&self) -> Option<u8> {
        self.std_script_address_prefix
    }

    fn standard(&self) -> Option<u32> {
        self.standard
    }

    fn integrated(&self) -> Option<u32> {
        self.integrated
    }

    fn sub_address(&self) -> Option<u32> {
        self.sub_address
    }

    fn network_type(&self) -> Option<u32> {
        self.network_type
    }

    fn payment_address_hrp(&self) -> Option<&'static str> {
        self.payment_address_hrp
    }

    fn reward_address_hrp(&self) -> Option<&'static str> {
        self.reward_address_hrp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CryptocurrencyDescriptor {
    pub name: &'static str,
    pub symbol: &'static str,
    pub info: Info<'static>,
    pub ecc: &'static str,
    pub coin_type: u32,
    pub support_bip38: bool,
    pub networks: &'static [(&'static str, &'static NetworkDescriptor)],
    pub default_network: &'static NetworkDescriptor,
    pub entropies: &'static [&'static str],
    pub mnemonics: &'static [&'static str],
    pub seeds: &'static [&'static str],
    pub hds: &'static [&'static str],
    pub default_hd: &'static str,
    pub default_path: &'static str,
    pub addresses: &'static [&'static str],
    pub default_address: &'static str,
    pub address_types: Option<AddressTypes>,
    pub default_address_type: Option<&'static str>,
    pub address_prefixes: Option<AddressPrefixes>,
    pub default_address_prefix: Option<&'static str>,
    pub semantics: &'static [&'static str],
    pub default_semantic: Option<&'static str>,
    pub params: Option<Params>,
}

pub trait ICryptocurrency {
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
    fn info(&self) -> Info<'static>;
    fn ecc_name(&self) -> &'static str;
    fn coin_type(&self) -> u32;
    fn support_bip38(&self) -> bool;
    fn networks(&self) -> &'static [(&'static str, &'static NetworkDescriptor)];
    fn default_network(&self) -> &'static NetworkDescriptor;
    fn entropies(&self) -> &'static [&'static str];
    fn mnemonics(&self) -> &'static [&'static str];
    fn seeds(&self) -> &'static [&'static str];
    fn hds(&self) -> &'static [&'static str];
    fn default_hd(&self) -> &'static str;
    fn default_path(&self) -> &'static str;
    fn addresses(&self) -> &'static [&'static str];
    fn default_address(&self) -> &'static str;

    fn address_types(&self) -> Option<AddressTypes> {
        None
    }
    fn default_address_type(&self) -> Option<&'static str> {
        None
    }
    fn address_prefixes(&self) -> Option<AddressPrefixes> {
        None
    }
    fn default_address_prefix(&self) -> Option<&'static str> {
        None
    }
    fn semantics(&self) -> &'static [&'static str];
    fn default_semantic(&self) -> Option<&'static str>;
    fn params(&self) -> Option<Params> {
        None
    }
}

impl ICryptocurrency for CryptocurrencyDescriptor {
    fn name(&self) -> &'static str {
        self.name
    }

    fn symbol(&self) -> &'static str {
        self.symbol
    }

    fn info(&self) -> Info<'static> {
        self.info
    }

    fn ecc_name(&self) -> &'static str {
        self.ecc
    }

    fn coin_type(&self) -> u32 {
        self.coin_type
    }

    fn support_bip38(&self) -> bool {
        self.support_bip38
    }

    fn networks(&self) -> &'static [(&'static str, &'static NetworkDescriptor)] {
        self.networks
    }

    fn default_network(&self) -> &'static NetworkDescriptor {
        self.default_network
    }

    fn entropies(&self) -> &'static [&'static str] {
        self.entropies
    }

    fn mnemonics(&self) -> &'static [&'static str] {
        self.mnemonics
    }

    fn seeds(&self) -> &'static [&'static str] {
        self.seeds
    }

    fn hds(&self) -> &'static [&'static str] {
        self.hds
    }

    fn default_hd(&self) -> &'static str {
        self.default_hd
    }

    fn default_path(&self) -> &'static str {
        self.default_path
    }

    fn addresses(&self) -> &'static [&'static str] {
        self.addresses
    }

    fn default_address(&self) -> &'static str {
        self.default_address
    }

    fn address_types(&self) -> Option<AddressTypes> {
        self.address_types
    }

    fn default_address_type(&self) -> Option<&'static str> {
        self.default_address_type
    }

    fn address_prefixes(&self) -> Option<AddressPrefixes> {
        self.address_prefixes
    }

    fn default_address_prefix(&self) -> Option<&'static str> {
        self.default_address_prefix
    }

    fn semantics(&self) -> &'static [&'static str] {
        self.semantics
    }

    fn default_semantic(&self) -> Option<&'static str> {
        self.default_semantic
    }

    fn params(&self) -> Option<Params> {
        self.params
    }
}
