use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
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
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[("MAINNET", &MAINNET)];

pub static ADDRESS_TYPES: AddressTypes = &[
    ("SECP256K1", "secp256k1"),
    ("BLS", "bls"),
];

pub static FILECOIN_PARAMS: Params = Params {
    checksum_length: None,
    suffix: None,
    address_prefix: Some("f"),
    chain_prefixes_by_address_type: None,
    alphabet: Some("abcdefghijklmnopqrstuvwxyz234567"),
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: Some(&[("SECP256K1", 1), ("BLS", 3)]),
};

pub struct Filecoin {
    pub network: NetworkName,
}

impl Filecoin {
    pub const NAME: &'static str = "Filecoin";
    pub const SYMBOL: &'static str = "FIL";
    pub const COIN_TYPE: u32 = coin_types::FILECOIN;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/461'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "FILECOIN";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["FILECOIN"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/filecoin-project"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://docs.filecoin.io"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://filecoin.io"]),
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
    name: Filecoin::NAME,
    symbol: Filecoin::SYMBOL,
    info: Filecoin::INFO,
    ecc: Filecoin::ECC,
    coin_type: Filecoin::COIN_TYPE,
    support_bip38: Filecoin::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Filecoin::ENTROPIES,
    mnemonics: Filecoin::MNEMONICS,
    seeds: Filecoin::SEEDS,
    hds: Filecoin::HDS,
    default_hd: Filecoin::DEFAULT_HD,
    default_path: Filecoin::DEFAULT_PATH,
    addresses: Filecoin::ADDRESSES,
    default_address: Filecoin::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPES),
    default_address_type: Some("SECP256K1"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Filecoin::SEMANTICS,
    default_semantic: Filecoin::DEFAULT_SEMANTIC,
    params: Some(FILECOIN_PARAMS),
};
