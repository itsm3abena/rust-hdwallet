use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const TYPE: u32 = 0x00;
    pub const WIF_PREFIX: u8 = 0x80;
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
    wif_prefix: Some(Mainnet::WIF_PREFIX),
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: Some(Mainnet::TYPE),
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const TYPE: u32 = 0x10;
    pub const WIF_PREFIX: u8 = 0xef;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x04358394)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x043587cf)];
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
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
    network_type: Some(Testnet::TYPE),
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] =
    &[("MAINNET", &MAINNET), ("TESTNET", &TESTNET)];

pub static ADDRESS_TYPES: AddressTypes = &[("P2PKH", "p2pkh"), ("P2SH", "p2sh")];

pub static ERGO_PARAMS: Params = Params {
    checksum_length: Some(4),
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: Some(&[("P2PKH", 0x01), ("P2SH", 0x02)]),
};

pub struct Ergo {
    pub network: NetworkName,
}

impl Ergo {
    pub const NAME: &'static str = "Ergo";
    pub const SYMBOL: &'static str = "ERG";
    pub const COIN_TYPE: u32 = coin_types::ERGO;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/429'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "ERGO";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["ERGO"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/ergoplatform/ergo"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://ergoplatform.org/en/documents"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://ergoplatform.org"]),
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
    name: Ergo::NAME,
    symbol: Ergo::SYMBOL,
    info: Ergo::INFO,
    ecc: Ergo::ECC,
    coin_type: Ergo::COIN_TYPE,
    support_bip38: Ergo::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Ergo::ENTROPIES,
    mnemonics: Ergo::MNEMONICS,
    seeds: Ergo::SEEDS,
    hds: Ergo::HDS,
    default_hd: Ergo::DEFAULT_HD,
    default_path: Ergo::DEFAULT_PATH,
    addresses: Ergo::ADDRESSES,
    default_address: Ergo::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPES),
    default_address_type: Some("P2PKH"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Ergo::SEMANTICS,
    default_semantic: Ergo::DEFAULT_SEMANTIC,
    params: Some(ERGO_PARAMS),
};
