use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    WitnessVersions, XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x26;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x17;
    pub const HRP: &'static str = "btg";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x1DBitcoin Gold Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0488ade4),
        ("P2SH", 0x0488ade4),
        ("P2WPKH", 0x04b2430c),
        ("P2WPKH_IN_P2SH", 0x049d7878),
        ("P2WSH", 0x02aa7a99),
        ("P2WSH_IN_P2SH", 0x0295b005),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x0488b21e),
        ("P2SH", 0x0488b21e),
        ("P2WPKH", 0x04b24746),
        ("P2WPKH_IN_P2SH", 0x049d7cb2),
        ("P2WSH", 0x02aa7ed3),
        ("P2WSH_IN_P2SH", 0x0295b43f),
    ];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: Some(Mainnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Mainnet::SCRIPT_ADDRESS_PREFIX),
    hrp: Some(Mainnet::HRP),
    witness_versions: Some(Mainnet::WITNESS_VERSIONS),
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[
    ("P2WPKH_IN_P2SH", "P2WPKH-In-P2SH"),
    ("P2WSH_IN_P2SH", "P2WSH-In-P2SH"),
];

pub struct BitcoinGold {
    pub network: NetworkName,
}

impl BitcoinGold {
    pub const NAME: &'static str = "Bitcoin-Gold";
    pub const SYMBOL: &'static str = "BTG";
    pub const COIN_TYPE: u32 = coin_types::BITCOIN_GOLD;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/156'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "P2PKH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &[
        "P2PKH",
        "P2SH",
        "P2WPKH",
        "P2WPKH_IN_P2SH",
        "P2WSH",
        "P2WSH_IN_P2SH",
    ];
    pub const SEMANTICS: &'static [&'static str] = &[
        "p2pkh",
        "p2sh",
        "p2wpkh",
        "p2wpkh-in-p2sh",
        "p2wsh",
        "p2wsh-in-p2sh",
    ];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/BTCGPU/BTCGPU"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://github.com/BTCGPU/BTCGPU/wiki/Technical-Spec"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://bitcoingold.org"]),
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
    name: BitcoinGold::NAME,
    symbol: BitcoinGold::SYMBOL,
    info: BitcoinGold::INFO,
    ecc: BitcoinGold::ECC,
    coin_type: BitcoinGold::COIN_TYPE,
    support_bip38: BitcoinGold::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: BitcoinGold::ENTROPIES,
    mnemonics: BitcoinGold::MNEMONICS,
    seeds: BitcoinGold::SEEDS,
    hds: BitcoinGold::HDS,
    default_hd: BitcoinGold::DEFAULT_HD,
    default_path: BitcoinGold::DEFAULT_PATH,
    addresses: BitcoinGold::ADDRESSES,
    default_address: BitcoinGold::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: BitcoinGold::SEMANTICS,
    default_semantic: BitcoinGold::DEFAULT_SEMANTIC,
    params: None,
};
