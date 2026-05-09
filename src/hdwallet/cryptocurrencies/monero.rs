use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneroNetwork {
    Mainnet,
    Stagenet,
    Testnet,
}

impl MoneroNetwork {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mainnet" => Some(Self::Mainnet),
            "stagenet" => Some(Self::Stagenet),
            "testnet" => Some(Self::Testnet),
            _ => None,
        }
    }
}

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const STANDARD: u32 = 0x12;
    pub const INTEGRATED: u32 = 0x13;
    pub const SUB_ADDRESS: u32 = 0x2a;
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: None,
    xpublic_key_versions: None,
    message_prefix: None,
    wif_prefix: None,
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: Some(Mainnet::STANDARD),
    integrated: Some(Mainnet::INTEGRATED),
    sub_address: Some(Mainnet::SUB_ADDRESS),
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub struct Stagenet;

impl Stagenet {
    pub const NAME: &'static str = "stagenet";
    pub const STANDARD: u32 = 0x18;
    pub const INTEGRATED: u32 = 0x19;
    pub const SUB_ADDRESS: u32 = 0x24;
}

pub static STAGENET: NetworkDescriptor = NetworkDescriptor {
    name: Stagenet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: None,
    xpublic_key_versions: None,
    message_prefix: None,
    wif_prefix: None,
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: Some(Stagenet::STANDARD),
    integrated: Some(Stagenet::INTEGRATED),
    sub_address: Some(Stagenet::SUB_ADDRESS),
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const STANDARD: u32 = 0x35;
    pub const INTEGRATED: u32 = 0x36;
    pub const SUB_ADDRESS: u32 = 0x3f;
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: None,
    xpublic_key_versions: None,
    message_prefix: None,
    wif_prefix: None,
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: Some(Testnet::STANDARD),
    integrated: Some(Testnet::INTEGRATED),
    sub_address: Some(Testnet::SUB_ADDRESS),
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[
    ("MAINNET", &MAINNET),
    ("STAGENET", &STAGENET),
    ("TESTNET", &TESTNET),
];

pub static MONERO_ADDRESS_TYPES: AddressTypes = &[
    ("MONERO", "Monero"),
    ("STANDARD", "standard"),
    ("INTEGRATED", "integrated"),
    ("SUB_ADDRESS", "sub-address"),
];

pub static MONERO_PARAMS: Params = Params {
    checksum_length: Some(4),
    payment_id_length: Some(8),
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: None,
    address_prefix_byte: None,
    address_version_byte: None,
};

pub struct Monero {
    pub network: MoneroNetwork,
}

impl Monero {
    pub const NAME: &'static str = "Monero";
    pub const SYMBOL: &'static str = "XMR";
    pub const COIN_TYPE: u32 = coin_types::MONERO;
    pub const ECC: &'static str = "SLIP10Ed25519MoneroECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "MONERO";
    pub const DEFAULT_PATH: &'static str = "m/44'/128'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "MONERO";
    pub const ENTROPIES: &'static [&'static str] = &["MONERO", "BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["MONERO", "BIP39"];
    pub const SEEDS: &'static [&'static str] = &["MONERO", "BIP39"];
    pub const HDS: &'static [&'static str] = &["MONERO"];
    pub const ADDRESSES: &'static [&'static str] = &["MONERO"];
    pub const SEMANTICS: &'static [&'static str] = &[];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = None;

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/monero-project/monero"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://github.com/monero-project/research-lab/blob/master/whitepaper/whitepaper.pdf",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.getmonero.org"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            MoneroNetwork::Mainnet => &MAINNET,
            MoneroNetwork::Stagenet => &STAGENET,
            MoneroNetwork::Testnet => &TESTNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Monero::NAME,
    symbol: Monero::SYMBOL,
    info: Monero::INFO,
    ecc: Monero::ECC,
    coin_type: Monero::COIN_TYPE,
    support_bip38: Monero::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Monero::ENTROPIES,
    mnemonics: Monero::MNEMONICS,
    seeds: Monero::SEEDS,
    hds: Monero::HDS,
    default_hd: Monero::DEFAULT_HD,
    default_path: Monero::DEFAULT_PATH,
    addresses: Monero::ADDRESSES,
    default_address: Monero::DEFAULT_ADDRESS,
    address_types: Some(MONERO_ADDRESS_TYPES),
    default_address_type: Some("standard"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Monero::SEMANTICS,
    default_semantic: Monero::DEFAULT_SEMANTIC,
    params: Some(MONERO_PARAMS),
};
