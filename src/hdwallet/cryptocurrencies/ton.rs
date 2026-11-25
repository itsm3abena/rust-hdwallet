use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x0488ade4)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x0488b21e)];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(Mainnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Mainnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: None,
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("TON", "Ton")];

pub struct Ton {
    pub network: NetworkName,
}

impl Ton {
    pub const NAME: &'static str = "Ton";
    pub const SYMBOL: &'static str = "TON";
    pub const COIN_TYPE: u32 = coin_types::TON;
    pub const ECC: &'static str = "SLIP10Ed25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/607'/0'";
    pub const DEFAULT_ADDRESS: &'static str = "TON";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["TON"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const BOUNCEABLE_TAG: u8 = 0x11;
    pub const NON_BOUNCEABLE_TAG: u8 = 0x51;
    pub const TEST_FLAG: u8 = 0x80;
    pub const DEFAULT_BOUNCEABLE: bool = false;
    pub const WORKCHAIN: i8 = 0;
    pub const WALLET_ID: i64 = 698_983_191;
    pub const DEFAULT_WALLET_VERSION: &'static str = "v5r1";
    pub const MAINNET_NETWORK_GLOBAL_ID: i64 = -239;
    pub const TESTNET_NETWORK_GLOBAL_ID: i64 = -3;
    pub const SUBWALLET_NUMBER: u32 = 0;

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/ton-blockchain"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://docs.ton.org/learn/docs"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://ton.org"]),
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
    name: Ton::NAME,
    symbol: Ton::SYMBOL,
    info: Ton::INFO,
    ecc: Ton::ECC,
    coin_type: Ton::COIN_TYPE,
    support_bip38: Ton::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Ton::ENTROPIES,
    mnemonics: Ton::MNEMONICS,
    seeds: Ton::SEEDS,
    hds: Ton::HDS,
    default_hd: Ton::DEFAULT_HD,
    default_path: Ton::DEFAULT_PATH,
    addresses: Ton::ADDRESSES,
    default_address: Ton::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Ton::SEMANTICS,
    default_semantic: Ton::DEFAULT_SEMANTIC,
    params: None,
};
