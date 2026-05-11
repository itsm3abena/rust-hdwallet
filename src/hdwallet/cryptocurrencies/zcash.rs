use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, XPrivateKeyVersions,
    XPublicKeyVersions,
};
use super::NetworkName;

pub const MAINNET_XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
    ("P2PKH", 0x0488ade4),
    ("P2SH", 0x0488ade4),
];

pub const MAINNET_XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
    ("P2PKH", 0x0488b21e),
    ("P2SH", 0x0488b21e),
];

pub const TESTNET_XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
    ("P2PKH", 0x0435_8394),
    ("P2SH", 0x0435_8394),
];

pub const TESTNET_XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
    ("P2PKH", 0x0435_87cf),
    ("P2SH", 0x0435_87cf),
];

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x1cb8;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x1cbd;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Zcash Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0x80;
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: Some(Mainnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Mainnet::SCRIPT_ADDRESS_PREFIX),
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(MAINNET_XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(MAINNET_XPUBLIC_KEY_VERSIONS),
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
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x1d25;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x1cba;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Zcash Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xef;
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: Some(Testnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Testnet::SCRIPT_ADDRESS_PREFIX),
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(TESTNET_XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(TESTNET_XPUBLIC_KEY_VERSIONS),
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

pub struct Zcash {
    pub network: NetworkName,
}

impl Zcash {
    pub const NAME: &'static str = "Zcash";
    pub const SYMBOL: &'static str = "ZEC";
    pub const COIN_TYPE: u32 = coin_types::ZCASH;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/133'/0'/0/0";
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
            value: InfoValue::Str("https://github.com/zcash/zcash"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://github.com/zcash/zips/blob/master/protocol/protocol.pdf",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://z.cash"]),
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
    name: Zcash::NAME,
    symbol: Zcash::SYMBOL,
    info: Zcash::INFO,
    ecc: Zcash::ECC,
    coin_type: Zcash::COIN_TYPE,
    support_bip38: Zcash::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Zcash::ENTROPIES,
    mnemonics: Zcash::MNEMONICS,
    seeds: Zcash::SEEDS,
    hds: Zcash::HDS,
    default_hd: Zcash::DEFAULT_HD,
    default_path: Zcash::DEFAULT_PATH,
    addresses: Zcash::ADDRESSES,
    default_address: Zcash::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Zcash::SEMANTICS,
    default_semantic: Zcash::DEFAULT_SEMANTIC,
    params: None,
};
