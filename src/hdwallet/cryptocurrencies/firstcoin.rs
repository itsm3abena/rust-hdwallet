use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, XPrivateKeyVersions,
    XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x23;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x05;
    pub const MESSAGE_PREFIX: &'static [u8] = b"\x18FirstCoin Signed Message:\n";
    pub const WIF_PREFIX: u8 = 0xa3;
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

pub struct Firstcoin {
    pub network: NetworkName,
}

impl Firstcoin {
    pub const NAME: &'static str = "Firstcoin";
    pub const SYMBOL: &'static str = "FRST";
    pub const COIN_TYPE: u32 = coin_types::FIRSTCOIN;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = true;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/167'/0'/0/0";
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
            key: "WHITEPAPER",
            value: InfoValue::Str("https://first-coin-club.blogspot.com/2017/11/whitepaper.html"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://firstcoinproject.com"]),
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
    name: Firstcoin::NAME,
    symbol: Firstcoin::SYMBOL,
    info: Firstcoin::INFO,
    ecc: Firstcoin::ECC,
    coin_type: Firstcoin::COIN_TYPE,
    support_bip38: Firstcoin::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Firstcoin::ENTROPIES,
    mnemonics: Firstcoin::MNEMONICS,
    seeds: Firstcoin::SEEDS,
    hds: Firstcoin::HDS,
    default_hd: Firstcoin::DEFAULT_HD,
    default_path: Firstcoin::DEFAULT_PATH,
    addresses: Firstcoin::ADDRESSES,
    default_address: Firstcoin::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Firstcoin::SEMANTICS,
    default_semantic: Firstcoin::DEFAULT_SEMANTIC,
    params: None,
};
