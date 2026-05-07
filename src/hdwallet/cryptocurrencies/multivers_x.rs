use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const HRP: &'static str = "erd";
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("MULTIVERSX", "MultiversX")];

pub struct MultiversX {
    pub network: NetworkName,
}

impl MultiversX {
    pub const NAME: &'static str = "MultiversX";
    pub const SYMBOL: &'static str = "EGLD";
    pub const COIN_TYPE: u32 = coin_types::MULTIVERSX;
    pub const ECC: &'static str = "SLIP10Ed25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/508'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "MULTIVERSX";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["MULTIVERSX"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/multiversx/mx-chain-go"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://files.multiversx.com/multiversx-whitepaper.pdf"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&[
                "https://multiversx.com",
                "https://multiversx.com/ecosystem",
            ]),
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
    name: MultiversX::NAME,
    symbol: MultiversX::SYMBOL,
    info: MultiversX::INFO,
    ecc: MultiversX::ECC,
    coin_type: MultiversX::COIN_TYPE,
    support_bip38: MultiversX::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: MultiversX::ENTROPIES,
    mnemonics: MultiversX::MNEMONICS,
    seeds: MultiversX::SEEDS,
    hds: MultiversX::HDS,
    default_hd: MultiversX::DEFAULT_HD,
    default_path: MultiversX::DEFAULT_PATH,
    addresses: MultiversX::ADDRESSES,
    default_address: MultiversX::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: MultiversX::SEMANTICS,
    default_semantic: MultiversX::DEFAULT_SEMANTIC,
    params: None,
};
