use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, XPrivateKeyVersions,
    XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x66;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x05;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18Bitsend Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xcc;
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

pub struct BitSend {
    pub network: NetworkName,
}

impl BitSend {
    pub const NAME: &'static str = "Bit-Send";
    pub const SYMBOL: &'static str = "BSD";
    pub const COIN_TYPE: u32 = coin_types::BITSEND;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/91'/0'/0/0";
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
            value: InfoValue::Str("https://github.com/LIMXTEC/BitSend"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://github.com/LIMXTEC/BitSend/releases/download/v0.14.0.5/Z.bitsend_whitePaper.Dec_17.pdf",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://bitsend.cc", "http://www.mybitsend.com"]),
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
    name: BitSend::NAME,
    symbol: BitSend::SYMBOL,
    info: BitSend::INFO,
    ecc: BitSend::ECC,
    coin_type: BitSend::COIN_TYPE,
    support_bip38: BitSend::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: BitSend::ENTROPIES,
    mnemonics: BitSend::MNEMONICS,
    seeds: BitSend::SEEDS,
    hds: BitSend::HDS,
    default_hd: BitSend::DEFAULT_HD,
    default_path: BitSend::DEFAULT_PATH,
    addresses: BitSend::ADDRESSES,
    default_address: BitSend::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: BitSend::SEMANTICS,
    default_semantic: BitSend::DEFAULT_SEMANTIC,
    params: None,
};
