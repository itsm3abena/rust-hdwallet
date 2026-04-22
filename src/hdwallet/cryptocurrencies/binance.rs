use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const HRP: &'static str = "bnb";
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x0488ade4)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x0488b21e)];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: Some(Mainnet::HRP),
    witness_versions: None,
    xprivate_key_versions: Some(Mainnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Mainnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: Some(Mainnet::WIF_PREFIX),
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[("MAINNET", &MAINNET)];

pub static ADDRESS_TYPES: AddressTypes = &[
    ("CHAIN", "chain"),
    ("SMART_CHAIN", "smart-chain"),
];

pub struct Binance {
    pub network: NetworkName,
}

impl Binance {
    pub const NAME: &'static str = "Binance";
    pub const SYMBOL: &'static str = "BNB";
    pub const COIN_TYPE: u32 = coin_types::BINANCE;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/714'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "COSMOS";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["COSMOS", "ETHEREUM"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/bnb-chain/bsc"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://github.com/bnb-chain/whitepaper"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.bnbchain.org"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet | NetworkName::Regtest => &MAINNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Binance::NAME,
    symbol: Binance::SYMBOL,
    info: Binance::INFO,
    ecc: Binance::ECC,
    coin_type: Binance::COIN_TYPE,
    support_bip38: Binance::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Binance::ENTROPIES,
    mnemonics: Binance::MNEMONICS,
    seeds: Binance::SEEDS,
    hds: Binance::HDS,
    default_hd: Binance::DEFAULT_HD,
    default_path: Binance::DEFAULT_PATH,
    addresses: Binance::ADDRESSES,
    default_address: Binance::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPES),
    default_address_type: Some("CHAIN"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Binance::SEMANTICS,
    default_semantic: Binance::DEFAULT_SEMANTIC,
    params: None,
};
