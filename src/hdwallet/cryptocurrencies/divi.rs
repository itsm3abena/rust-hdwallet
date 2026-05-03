use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, XPrivateKeyVersions,
    XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x1e;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x0d;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x19Divi Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xd4;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0221312b),
        ("P2SH", 0x0221312b),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x022d2533),
        ("P2SH", 0x022d2533),
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

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x1e;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x0d;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x19Divi Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xd4;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x3a805837),
        ("P2SH", 0x3a805837),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x3a8061a0),
        ("P2SH", 0x3a8061a0),
    ];
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: Some(Testnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Testnet::SCRIPT_ADDRESS_PREFIX),
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(Testnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Testnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: Some(Testnet::MESSAGE_PREFIX),
    wif_prefix: Some(Testnet::WIF_PREFIX),
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

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] =
    &[("MAINNET", &MAINNET), ("TESTNET", &TESTNET)];

pub struct Divi {
    pub network: NetworkName,
}

impl Divi {
    pub const NAME: &'static str = "Divi";
    pub const SYMBOL: &'static str = "DIVI";
    pub const COIN_TYPE: u32 = coin_types::DIVI;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/301'/0'/0/0";
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
            value: InfoValue::Str("https://github.com/Divicoin/Divi"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://wiki.diviproject.org/#whitepaper"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.diviproject.org", "https://diviwallet.com"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet | NetworkName::Regtest => &TESTNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Divi::NAME,
    symbol: Divi::SYMBOL,
    info: Divi::INFO,
    ecc: Divi::ECC,
    coin_type: Divi::COIN_TYPE,
    support_bip38: Divi::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Divi::ENTROPIES,
    mnemonics: Divi::MNEMONICS,
    seeds: Divi::SEEDS,
    hds: Divi::HDS,
    default_hd: Divi::DEFAULT_HD,
    default_path: Divi::DEFAULT_PATH,
    addresses: Divi::ADDRESSES,
    default_address: Divi::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Divi::SEMANTICS,
    default_semantic: Divi::DEFAULT_SEMANTIC,
    params: None,
};
