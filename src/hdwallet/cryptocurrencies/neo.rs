use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("NEO", "Neo")];

pub static NEO_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: Some(0xAC),
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
    address_prefix_byte: Some(0x21),
    address_version_byte: Some(0x17),
};

pub struct Neo {
    pub network: NetworkName,
}

impl Neo {
    pub const NAME: &'static str = "Neo";
    pub const SYMBOL: &'static str = "NEO";
    pub const COIN_TYPE: u32 = coin_types::NEO;
    pub const ECC: &'static str = "SLIP10Nist256p1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/888'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "NEO";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["NEO"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/neo-project/neo"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://docs.neo.org/docs/en-us/index.html"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://neo.org"]),
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
    name: Neo::NAME,
    symbol: Neo::SYMBOL,
    info: Neo::INFO,
    ecc: Neo::ECC,
    coin_type: Neo::COIN_TYPE,
    support_bip38: Neo::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Neo::ENTROPIES,
    mnemonics: Neo::MNEMONICS,
    seeds: Neo::SEEDS,
    hds: Neo::HDS,
    default_hd: Neo::DEFAULT_HD,
    default_path: Neo::DEFAULT_PATH,
    addresses: Neo::ADDRESSES,
    default_address: Neo::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Neo::SEMANTICS,
    default_semantic: Neo::DEFAULT_SEMANTIC,
    params: Some(NEO_PARAMS),
};
