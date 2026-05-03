use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    WitnessVersions, XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x1c;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x0a;
    pub const HRP: &'static str = "cp";
    pub const WITNESS_VERSIONS: WitnessVersions = WitnessVersions(&[0x00, 0x00]);
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Bitcoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0x7b;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[
        ("P2PKH", 0x0488ade4),
        ("P2SH", 0x0488ade4),
        ("P2WPKH", 0x04b2430c),
        ("P2WPKH_IN_P2SH", 0x049d7878),
    ];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[
        ("P2PKH", 0x0488b21e),
        ("P2SH", 0x0488b21e),
        ("P2WPKH", 0x04b24746),
        ("P2WPKH_IN_P2SH", 0x049d7cb2),
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("P2WPKH_IN_P2SH", "P2WPKH-In-P2SH")];

pub struct CranePay {
    pub network: NetworkName,
}

impl CranePay {
    pub const NAME: &'static str = "Crane-Pay";
    pub const SYMBOL: &'static str = "CRP";
    pub const COIN_TYPE: u32 = coin_types::CRANEPAY;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/2304'/0'/0/0";
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
            value: InfoValue::Str("https://github.com/cranepay/cranepay-core"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://cranepay.io"]),
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
    name: CranePay::NAME,
    symbol: CranePay::SYMBOL,
    info: CranePay::INFO,
    ecc: CranePay::ECC,
    coin_type: CranePay::COIN_TYPE,
    support_bip38: CranePay::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: CranePay::ENTROPIES,
    mnemonics: CranePay::MNEMONICS,
    seeds: CranePay::SEEDS,
    hds: CranePay::HDS,
    default_hd: CranePay::DEFAULT_HD,
    default_path: CranePay::DEFAULT_PATH,
    addresses: CranePay::ADDRESSES,
    default_address: CranePay::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: CranePay::SEMANTICS,
    default_semantic: CranePay::DEFAULT_SEMANTIC,
    params: None,
};
