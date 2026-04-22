use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const LEGACY_PUBLIC_KEY_ADDRESS_PREFIX: u8 = 0x00;
    pub const LEGACY_SCRIPT_ADDRESS_PREFIX: u8 = 0x05;
    pub const STD_PUBLIC_KEY_ADDRESS_PREFIX: u8 = 0x00;
    pub const STD_SCRIPT_ADDRESS_PREFIX: u8 = 0x08;
    pub const HRP: &'static str = "bitcoincash";
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0488ade4),
        ("P2SH", 0x0488ade4),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x0488b21e),
        ("P2SH", 0x0488b21e),
    ];
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
    legacy_public_key_address_prefix: Some(Mainnet::LEGACY_PUBLIC_KEY_ADDRESS_PREFIX),
    std_public_key_address_prefix: Some(Mainnet::STD_PUBLIC_KEY_ADDRESS_PREFIX),
    legacy_script_address_prefix: Some(Mainnet::LEGACY_SCRIPT_ADDRESS_PREFIX),
    std_script_address_prefix: Some(Mainnet::STD_SCRIPT_ADDRESS_PREFIX),
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
    pub const LEGACY_PUBLIC_KEY_ADDRESS_PREFIX: u8 = 0x6f;
    pub const LEGACY_SCRIPT_ADDRESS_PREFIX: u8 = 0xc4;
    pub const STD_PUBLIC_KEY_ADDRESS_PREFIX: u8 = 0x00;
    pub const STD_SCRIPT_ADDRESS_PREFIX: u8 = 0x08;
    pub const HRP: &'static str = "bchtest";
    pub const WIF_PREFIX: u8 = 0xef;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x04358394),
        ("P2SH", 0x04358394),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x043587cf),
        ("P2SH", 0x043587cf),
    ];
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: Some(Testnet::HRP),
    witness_versions: None,
    xprivate_key_versions: Some(Testnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Testnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: Some(Testnet::WIF_PREFIX),
    legacy_public_key_address_prefix: Some(Testnet::LEGACY_PUBLIC_KEY_ADDRESS_PREFIX),
    std_public_key_address_prefix: Some(Testnet::STD_PUBLIC_KEY_ADDRESS_PREFIX),
    legacy_script_address_prefix: Some(Testnet::LEGACY_SCRIPT_ADDRESS_PREFIX),
    std_script_address_prefix: Some(Testnet::STD_SCRIPT_ADDRESS_PREFIX),
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub struct Regtest;

impl Regtest {
    pub const NAME: &'static str = "regtest";
    pub const HRP: &'static str = "bchreg";
}

pub static REGTEST: NetworkDescriptor = NetworkDescriptor {
    name: Regtest::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: Some(Regtest::HRP),
    witness_versions: None,
    xprivate_key_versions: Some(Testnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Testnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: Some(Testnet::WIF_PREFIX),
    legacy_public_key_address_prefix: Some(Testnet::LEGACY_PUBLIC_KEY_ADDRESS_PREFIX),
    std_public_key_address_prefix: Some(Testnet::STD_PUBLIC_KEY_ADDRESS_PREFIX),
    legacy_script_address_prefix: Some(Testnet::LEGACY_SCRIPT_ADDRESS_PREFIX),
    std_script_address_prefix: Some(Testnet::STD_SCRIPT_ADDRESS_PREFIX),
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[
    ("MAINNET", &MAINNET),
    ("TESTNET", &TESTNET),
    ("REGTEST", &REGTEST),
];

pub static ADDRESS_TYPES: AddressTypes = &[
    ("BITCOINCASH", "BitcoinCash"),
    ("STD", "std"),
    ("LEGACY", "legacy"),
];

pub struct BitcoinCash {
    pub network: NetworkName,
}

impl BitcoinCash {
    pub const NAME: &'static str = "Bitcoin-Cash";
    pub const SYMBOL: &'static str = "BCH";
    pub const COIN_TYPE: u32 = coin_types::BITCOIN_CASH;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/145'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "BITCOINCASH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["BITCOINCASH", "P2PKH", "P2SH"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh", "p2sh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/bitcoincashorg/bitcoincash.org"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["http://bch.info"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet => &TESTNET,
            NetworkName::Regtest => &REGTEST,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: BitcoinCash::NAME,
    symbol: BitcoinCash::SYMBOL,
    info: BitcoinCash::INFO,
    ecc: BitcoinCash::ECC,
    coin_type: BitcoinCash::COIN_TYPE,
    support_bip38: BitcoinCash::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: BitcoinCash::ENTROPIES,
    mnemonics: BitcoinCash::MNEMONICS,
    seeds: BitcoinCash::SEEDS,
    hds: BitcoinCash::HDS,
    default_hd: BitcoinCash::DEFAULT_HD,
    default_path: BitcoinCash::DEFAULT_PATH,
    addresses: BitcoinCash::ADDRESSES,
    default_address: BitcoinCash::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPES),
    default_address_type: Some("STD"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: BitcoinCash::SEMANTICS,
    default_semantic: BitcoinCash::DEFAULT_SEMANTIC,
    params: None,
};
