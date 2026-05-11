use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    WitnessVersions, XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x3a;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x32;
    pub const HRP: &'static str = "qc1";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x01, 0x00, 0x00]);
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0488ade4),
        ("P2SH", 0x0488ade4),
        ("P2WPKH", 0x045f18bc),
        ("P2WPKH_IN_P2SH", 0x049d7878),
        ("P2WSH", 0x02aa7a99),
        ("P2WSH_IN_P2SH", 0x0295b005),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x0488b21e),
        ("P2SH", 0x0488b21e),
        ("P2WPKH", 0x045f1cf6),
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

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x78;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x6e;
    pub const HRP: &'static str = "tq1";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x01, 0x00, 0x00]);
    pub const WIF_PREFIX: u8 = 0xef;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x04358394),
        ("P2SH", 0x04358394),
        ("P2WPKH", 0x045f18bc),
        ("P2WPKH_IN_P2SH", 0x044a4e28),
        ("P2WSH", 0x02575048),
        ("P2WSH_IN_P2SH", 0x024285b5),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x043587cf),
        ("P2SH", 0x043587cf),
        ("P2WPKH", 0x045f1cf6),
        ("P2WPKH_IN_P2SH", 0x044a5262),
        ("P2WSH", 0x02575483),
        ("P2WSH_IN_P2SH", 0x024289ef),
    ];
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: Some(Testnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Testnet::SCRIPT_ADDRESS_PREFIX),
    hrp: Some(Testnet::HRP),
    witness_versions: Some(Testnet::WITNESS_VERSIONS),
    xprivate_key_versions: Some(Testnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Testnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[
    ("P2WPKH_IN_P2SH", "P2WPKH-In-P2SH"),
    ("P2WSH_IN_P2SH", "P2WSH-In-P2SH"),
];

pub struct Qtum {
    pub network: NetworkName,
}

impl Qtum {
    pub const NAME: &'static str = "Qtum";
    pub const SYMBOL: &'static str = "QTUM";
    pub const COIN_TYPE: u32 = coin_types::QTUM;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/2301'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "P2PKH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &[
        "BIP32",
        "BIP44",
        "BIP49",
        "BIP84",
        "BIP86",
        "BIP141",
    ];
    pub const ADDRESSES: &'static [&'static str] = &[
        "P2PKH",
        "P2SH",
        "P2TR",
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
            value: InfoValue::Str("https://github.com/qtumproject/qtum"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://qtumorg.s3.ap-northeast-2.amazonaws.com/Qtum_New_Whitepaper_en.pdf",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://qtum.org", "https://qtum.info"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet => &TESTNET,
            NetworkName::Regtest => &TESTNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Qtum::NAME,
    symbol: Qtum::SYMBOL,
    info: Qtum::INFO,
    ecc: Qtum::ECC,
    coin_type: Qtum::COIN_TYPE,
    support_bip38: Qtum::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Qtum::ENTROPIES,
    mnemonics: Qtum::MNEMONICS,
    seeds: Qtum::SEEDS,
    hds: Qtum::HDS,
    default_hd: Qtum::DEFAULT_HD,
    default_path: Qtum::DEFAULT_PATH,
    addresses: Qtum::ADDRESSES,
    default_address: Qtum::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Qtum::SEMANTICS,
    default_semantic: Qtum::DEFAULT_SEMANTIC,
    params: None,
};
