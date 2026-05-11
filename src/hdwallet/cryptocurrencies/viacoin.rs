use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    WitnessVersions, XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub const MAINNET_XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
    ("P2PKH", 0x0488ade4),
    ("P2SH", 0x0488ade4),
    ("P2WPKH", 0x0488ade4),
    ("P2WPKH_IN_P2SH", 0x0488ade4),
];

pub const MAINNET_XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
    ("P2PKH", 0x0488b21e),
    ("P2SH", 0x0488b21e),
    ("P2WPKH", 0x0488b21e),
    ("P2WPKH_IN_P2SH", 0x0488b21e),
];

pub const TESTNET_XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
    ("P2PKH", 0x0435_8394),
    ("P2SH", 0x0435_8394),
    ("P2WPKH", 0x045f18bc),
    ("P2WPKH_IN_P2SH", 0x044a4e28),
];

pub const TESTNET_XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
    ("P2PKH", 0x0435_87cf),
    ("P2SH", 0x0435_87cf),
    ("P2WPKH", 0x045f1cf6),
    ("P2WPKH_IN_P2SH", 0x044a5262),
];

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x47;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x21;
    pub const HRP: &'static str = "via";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Viacoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xc7;
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: Some(Mainnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Mainnet::SCRIPT_ADDRESS_PREFIX),
    hrp: Some(Mainnet::HRP),
    witness_versions: Some(Mainnet::WITNESS_VERSIONS),
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
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x7f;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0xc4;
    pub const HRP: &'static str = "tvia";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Viacoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xff;
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: Some(Testnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Testnet::SCRIPT_ADDRESS_PREFIX),
    hrp: Some(Testnet::HRP),
    witness_versions: Some(Testnet::WITNESS_VERSIONS),
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("P2WPKH_IN_P2SH", "P2WPKH-In-P2SH")];

pub struct Viacoin {
    pub network: NetworkName,
}

impl Viacoin {
    pub const NAME: &'static str = "Viacoin";
    pub const SYMBOL: &'static str = "VIA";
    pub const COIN_TYPE: u32 = coin_types::VIACOIN;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/14'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "P2PKH";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["P2PKH", "P2SH", "P2WPKH", "P2WPKH_IN_P2SH"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh", "p2sh", "p2wpkh", "p2wpkh-in-p2sh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/viacoin/viacoin"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://github.com/viacoin/documents/blob/master/whitepapers/Viacoin_fullcolor_whitepaper.pdf",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["http://viacoin.org"]),
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
    name: Viacoin::NAME,
    symbol: Viacoin::SYMBOL,
    info: Viacoin::INFO,
    ecc: Viacoin::ECC,
    coin_type: Viacoin::COIN_TYPE,
    support_bip38: Viacoin::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Viacoin::ENTROPIES,
    mnemonics: Viacoin::MNEMONICS,
    seeds: Viacoin::SEEDS,
    hds: Viacoin::HDS,
    default_hd: Viacoin::DEFAULT_HD,
    default_path: Viacoin::DEFAULT_PATH,
    addresses: Viacoin::ADDRESSES,
    default_address: Viacoin::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Viacoin::SEMANTICS,
    default_semantic: Viacoin::DEFAULT_SEMANTIC,
    params: None,
};
