use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const HRP: &'static str = "avax";
    pub const WIF_PREFIX: u8 = 0x80;
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x0488ade4)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x0488b21e)];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: Some(Mainnet::HRP),
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

pub static ADDRESS_TYPES: AddressTypes = &[
    ("C_CHAIN", "c-chain"),
    ("P_CHAIN", "p-chain"),
    ("X_CHAIN", "x-chain"),
];

pub static AVALANCHE_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: Some(&[("P_CHAIN", "P-"), ("X_CHAIN", "X-")]),
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: None,
};

pub struct Avalanche {
    pub network: NetworkName,
}

impl Avalanche {
    pub const NAME: &'static str = "Avalanche";
    pub const SYMBOL: &'static str = "AVAX";
    pub const COIN_TYPE: u32 = coin_types::AVALANCHE;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/9000'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "AVALANCHE";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["AVALANCHE", "ETHEREUM"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/ava-labs/avalanchego"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://www.avalabs.org/whitepapers"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://avax.network", "https://www.avalabs.org"]),
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
    name: Avalanche::NAME,
    symbol: Avalanche::SYMBOL,
    info: Avalanche::INFO,
    ecc: Avalanche::ECC,
    coin_type: Avalanche::COIN_TYPE,
    support_bip38: Avalanche::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Avalanche::ENTROPIES,
    mnemonics: Avalanche::MNEMONICS,
    seeds: Avalanche::SEEDS,
    hds: Avalanche::HDS,
    default_hd: Avalanche::DEFAULT_HD,
    default_path: Avalanche::DEFAULT_PATH,
    addresses: Avalanche::ADDRESSES,
    default_address: Avalanche::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPES),
    default_address_type: Some("P_CHAIN"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Avalanche::SEMANTICS,
    default_semantic: Avalanche::DEFAULT_SEMANTIC,
    params: Some(AVALANCHE_PARAMS),
};
