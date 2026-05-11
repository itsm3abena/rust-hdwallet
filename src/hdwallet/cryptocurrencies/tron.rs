use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const PUBLIC_KEY_ADDRESS_PREFIX: u32 = 0x41;
    pub const SCRIPT_ADDRESS_PREFIX: u32 = 0x05;
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
    public_key_address_prefix: Some(Mainnet::PUBLIC_KEY_ADDRESS_PREFIX),
    script_address_prefix: Some(Mainnet::SCRIPT_ADDRESS_PREFIX),
    hrp: None,
    witness_versions: None,
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

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[("MAINNET", &MAINNET)];

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("TRON", "Tron")];

pub static TRON_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: Some("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: None,
    address_prefix_byte: None,
    address_version_byte: None,
    key_type: None,
    address_type_prefix_bytes: None,
};

pub struct Tron {
    pub network: NetworkName,
}

impl Tron {
    pub const NAME: &'static str = "Tron";
    pub const SYMBOL: &'static str = "TRX";
    pub const COIN_TYPE: u32 = coin_types::TRON;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/195'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "TRON";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["TRON"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh", "p2sh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/tronprotocol/java-tron"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://developers.tron.network/docs"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://trondao.org", "https://tron.network"]),
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
    name: Tron::NAME,
    symbol: Tron::SYMBOL,
    info: Tron::INFO,
    ecc: Tron::ECC,
    coin_type: Tron::COIN_TYPE,
    support_bip38: Tron::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Tron::ENTROPIES,
    mnemonics: Tron::MNEMONICS,
    seeds: Tron::SEEDS,
    hds: Tron::HDS,
    default_hd: Tron::DEFAULT_HD,
    default_path: Tron::DEFAULT_PATH,
    addresses: Tron::ADDRESSES,
    default_address: Tron::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: Some("TRON"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Tron::SEMANTICS,
    default_semantic: Tron::DEFAULT_SEMANTIC,
    params: Some(TRON_PARAMS),
};
