use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("ETHEREUM", "Ethereum")];

pub struct VeChain {
    pub network: NetworkName,
}

impl VeChain {
    pub const NAME: &'static str = "VeChain";
    pub const SYMBOL: &'static str = "VET";
    pub const COIN_TYPE: u32 = coin_types::VECHAIN;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/818'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "ETHEREUM";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["ETHEREUM"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/vechain"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://www.vechain.org/whitepaper/#bit_65sv8"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.vechain.org", "https://vebetterdao.org"]),
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
    name: VeChain::NAME,
    symbol: VeChain::SYMBOL,
    info: VeChain::INFO,
    ecc: VeChain::ECC,
    coin_type: VeChain::COIN_TYPE,
    support_bip38: VeChain::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: VeChain::ENTROPIES,
    mnemonics: VeChain::MNEMONICS,
    seeds: VeChain::SEEDS,
    hds: VeChain::HDS,
    default_hd: VeChain::DEFAULT_HD,
    default_path: VeChain::DEFAULT_PATH,
    addresses: VeChain::ADDRESSES,
    default_address: VeChain::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: Some("ETHEREUM"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: VeChain::SEMANTICS,
    default_semantic: VeChain::DEFAULT_SEMANTIC,
    params: None,
};
