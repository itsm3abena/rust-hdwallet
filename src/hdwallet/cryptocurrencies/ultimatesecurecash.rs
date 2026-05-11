use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, XPrivateKeyVersions,
    XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x44;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x7d;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18UltimateSecureCash Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xbf;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0xee8031e8),
        ("P2SH", 0xee8031e8),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0xee80286a),
        ("P2SH", 0xee80286a),
    ];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: Some(Mainnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Mainnet::SCRIPT_ADDRESS_PREFIX),
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(Mainnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Mainnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: Some(Mainnet::MESSAGE_PREFIX),
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

pub struct UltimateSecureCash {
    pub network: NetworkName,
}

impl UltimateSecureCash {
    pub const NAME: &'static str = "Ultimate-Secure-Cash";
    pub const SYMBOL: &'static str = "USC";
    pub const COIN_TYPE: u32 = coin_types::ULTIMATE_SECURE_CASH;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/112'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "P2PKH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["P2PKH", "P2SH"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh", "p2sh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/SilentTrader/UltimateSecureCash"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://ultimatesecurecash.info/#spec"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["http://ultimatesecurecash.info"]),
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
    name: UltimateSecureCash::NAME,
    symbol: UltimateSecureCash::SYMBOL,
    info: UltimateSecureCash::INFO,
    ecc: UltimateSecureCash::ECC,
    coin_type: UltimateSecureCash::COIN_TYPE,
    support_bip38: UltimateSecureCash::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: UltimateSecureCash::ENTROPIES,
    mnemonics: UltimateSecureCash::MNEMONICS,
    seeds: UltimateSecureCash::SEEDS,
    hds: UltimateSecureCash::HDS,
    default_hd: UltimateSecureCash::DEFAULT_HD,
    default_path: UltimateSecureCash::DEFAULT_PATH,
    addresses: UltimateSecureCash::ADDRESSES,
    default_address: UltimateSecureCash::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: UltimateSecureCash::SEMANTICS,
    default_semantic: UltimateSecureCash::DEFAULT_SEMANTIC,
    params: None,
};
