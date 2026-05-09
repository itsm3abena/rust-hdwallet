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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("NANO", "Nano")];

pub static NANO_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: None,
    address_prefix: Some("nano_"),
    chain_prefixes_by_address_type: None,
    alphabet: Some("13456789abcdefghijkmnopqrstuwxyz"),
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
};

pub struct Nano {
    pub network: NetworkName,
}

impl Nano {
    pub const NAME: &'static str = "Nano";
    pub const SYMBOL: &'static str = "XNO";
    pub const COIN_TYPE: u32 = coin_types::NANO;
    pub const ECC: &'static str = "SLIP10Ed25519Blake2bECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/165'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "NANO";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["NANO"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/nanocurrency/nano-node"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://nano.org/en/whitepaper"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["http://nano.org/en"]),
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
    name: Nano::NAME,
    symbol: Nano::SYMBOL,
    info: Nano::INFO,
    ecc: Nano::ECC,
    coin_type: Nano::COIN_TYPE,
    support_bip38: Nano::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Nano::ENTROPIES,
    mnemonics: Nano::MNEMONICS,
    seeds: Nano::SEEDS,
    hds: Nano::HDS,
    default_hd: Nano::DEFAULT_HD,
    default_path: Nano::DEFAULT_PATH,
    addresses: Nano::ADDRESSES,
    default_address: Nano::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Nano::SEMANTICS,
    default_semantic: Nano::DEFAULT_SEMANTIC,
    params: Some(NANO_PARAMS),
};
