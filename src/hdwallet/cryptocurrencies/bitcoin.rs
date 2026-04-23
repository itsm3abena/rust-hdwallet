use bitcoin::Network;

use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    WitnessVersions, XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

#[derive(Debug, Clone, Copy)]
pub struct Bitcoin {
    pub network: NetworkName,
}

impl Bitcoin {
    #[must_use]
    pub fn network(self) -> Network {
        match self.network {
            NetworkName::Mainnet => Network::Bitcoin,
            NetworkName::Testnet => Network::Testnet,
            NetworkName::Regtest => Network::Regtest,
        }
    }

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet => &TESTNET,
            NetworkName::Regtest => &REGTEST,
        }
    }

    pub const NAME: &'static str = "Bitcoin";
    pub const SYMBOL: &'static str = "BTC";
    pub const COIN_TYPE: u32 = coin_types::BITCOIN;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/0'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "P2PKH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39", "ELECTRUM_V1", "ELECTRUM_V2"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39", "ELECTRUM_V1", "ELECTRUM_V2"];
    pub const SEEDS: &'static [&'static str] = &["BIP39", "ELECTRUM_V1", "ELECTRUM_V2"];
    pub const HDS: &'static [&'static str] = &[
        "BIP32",
        "BIP44",
        "BIP49",
        "BIP84",
        "BIP86",
        "BIP141",
        "ELECTRUM_V1",
        "ELECTRUM_V2",
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
        "p2tr",
        "p2wpkh",
        "p2wpkh-in-p2sh",
        "p2wsh",
        "p2wsh-in-p2sh",
    ];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/bitcoin/bitcoin"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://bitcoin.org/bitcoin.pdf"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://bitcoin.org"]),
        },
    ]);
}

// --- Mainnet

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x00;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x05;
    pub const HRP: &'static str = "bc";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x01, 0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Bitcoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0488ade4),
        ("P2SH", 0x0488ade4),
        ("P2TR", 0x0488ade4),
        ("P2WPKH", 0x04b2430c),
        ("P2WPKH_IN_P2SH", 0x049d7878),
        ("P2WSH", 0x02aa7a99),
        ("P2WSH_IN_P2SH", 0x0295b005),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x0488b21e),
        ("P2SH", 0x0488b21e),
        ("P2TR", 0x0488b21e),
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

// --- Testnet

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x6f;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0xc4;
    pub const HRP: &'static str = "tb";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x01, 0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Bitcoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xef;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x04358394),
        ("P2SH", 0x04358394),
        ("P2TR", 0x04358394),
        ("P2WPKH", 0x045f18bc),
        ("P2WPKH_IN_P2SH", 0x044a4e28),
        ("P2WSH", 0x02575048),
        ("P2WSH_IN_P2SH", 0x024285b5),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x043587cf),
        ("P2SH", 0x043587cf),
        ("P2TR", 0x043587cf),
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

// --- Regtest (same as testnet except name and HRP)

pub struct Regtest;

impl Regtest {
    pub const NAME: &'static str = "regtest";
    pub const HRP: &'static str = "bcrt";
}

pub static REGTEST: NetworkDescriptor = NetworkDescriptor {
    name: Regtest::NAME,
    public_key_address_prefix: Some(Testnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Testnet::SCRIPT_ADDRESS_PREFIX),
    hrp: Some(Regtest::HRP),
    witness_versions: Some(Testnet::WITNESS_VERSIONS),
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

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[
    ("MAINNET", &MAINNET),
    ("TESTNET", &TESTNET),
    ("REGTEST", &REGTEST),
];

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[
    ("P2WPKH_IN_P2SH", "P2WPKH-In-P2SH"),
    ("P2WSH_IN_P2SH", "P2WSH-In-P2SH"),
];

pub static BITCOIN_PARAMS: Params = Params {
    checksum_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: Some("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
    field_size_hex: Some(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
    ),
    tap_tweak_sha256: Some("e80fe1639c9ca050e3af1b39c143c63e429cbceb15d940fbb5c5a1f4af57c5e9"),
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
};

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Bitcoin::NAME,
    symbol: Bitcoin::SYMBOL,
    info: Bitcoin::INFO,
    ecc: Bitcoin::ECC,
    coin_type: Bitcoin::COIN_TYPE,
    support_bip38: Bitcoin::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Bitcoin::ENTROPIES,
    mnemonics: Bitcoin::MNEMONICS,
    seeds: Bitcoin::SEEDS,
    hds: Bitcoin::HDS,
    default_hd: Bitcoin::DEFAULT_HD,
    default_path: Bitcoin::DEFAULT_PATH,
    addresses: Bitcoin::ADDRESSES,
    default_address: Bitcoin::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Bitcoin::SEMANTICS,
    default_semantic: Bitcoin::DEFAULT_SEMANTIC,
    params: Some(BITCOIN_PARAMS),
};
