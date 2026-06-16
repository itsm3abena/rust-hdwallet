use bitcoin::Network;
use hdwallet_rust::hdwallet::cryptocurrencies::bitcoin::Mainnet as BitcoinMainnet;
use hdwallet_rust::hdwallet::cryptocurrencies::Bitcoin;
use hdwallet_rust::hdwallet::derivations::{
    Bip44Derivation, Bip49Derivation, Bip84Derivation, Bip86Derivation, ChangeInput,
};
use hdwallet_rust::hdwallet::extended_keys::{deserialize_extended, serialize_extended};
use hdwallet_rust::hdwallet::hds::bip32::Bip32Hd;
use hdwallet_rust::hdwallet::hds::ihd::Ihd;
use hdwallet_rust::hdwallet::hds::{CardanoHd, ElectrumV1Hd, ElectrumV2Hd, MoneroHd};
use serde_json::Value;

use super::json_helpers::{json_indexes, json_kebab_str, json_u32};
use super::{get_bytes, json_str, TestData};

pub fn bitcoin_bip32_hd() -> Bip32Hd {
    let mut hd = Bip32Hd::from_seed(Network::Bitcoin, &[]).expect("Bip32Hd::from_seed");
    hd.configure(Some(BitcoinMainnet::WIF_PREFIX), "compressed", false);
    hd
}

pub fn xkey_version(semantic: &str) -> (u32, u32) {
    let xpriv = BitcoinMainnet::XPRIVATE_KEY_VERSIONS
        .iter()
        .find(|(name, _)| *name == semantic)
        .map(|(_, v)| *v)
        .expect("xprivate version");
    let xpub = BitcoinMainnet::XPUBLIC_KEY_VERSIONS
        .iter()
        .find(|(name, _)| *name == semantic)
        .map(|(_, v)| *v)
        .expect("xpublic version");
    (xpriv, xpub)
}

pub fn bip32_xprivate_key(hd: &Bip32Hd, semantic: &str) -> String {
    let xpriv = hd.master_xpriv();
    let (version, _) = xkey_version(semantic);
    serialize_extended(
        version,
        xpriv.depth,
        &xpriv.parent_fingerprint.to_bytes(),
        u32::from(xpriv.child_number),
        xpriv.chain_code.as_ref(),
        &[0x00]
            .into_iter()
            .chain(xpriv.private_key.secret_bytes())
            .collect::<Vec<_>>(),
        true,
    )
    .expect("serialize_extended")
}

pub fn bip32_xpublic_key(hd: &Bip32Hd, semantic: &str) -> String {
    use bitcoin::bip32::Xpub;
    use bitcoin::secp256k1::Secp256k1;
    let xpriv = hd.master_xpriv();
    let secp = Secp256k1::new();
    let xpub = Xpub::from_priv(&secp, xpriv);
    let (_, version) = xkey_version(semantic);
    serialize_extended(
        version,
        xpub.depth,
        &xpub.parent_fingerprint.to_bytes(),
        u32::from(xpub.child_number),
        xpub.chain_code.as_ref(),
        &xpub.public_key.serialize(),
        true,
    )
    .expect("serialize_extended")
}

pub fn derived_xprivate_key(hd: &Bip32Hd, semantic: &str) -> String {
    let key = hd.xprivate_key(true).expect("xprivate_key");
    reencode_extended_key(&key, semantic, true)
}

pub fn derived_xpublic_key(hd: &Bip32Hd, semantic: &str) -> String {
    let key = hd.xpublic_key(true).expect("xpublic_key");
    reencode_extended_key(&key, semantic, false)
}

fn reencode_extended_key(key: &str, semantic: &str, private: bool) -> String {
    let parts = deserialize_extended(key, true).expect("deserialize_extended");
    let (xpriv_version, xpub_version) = xkey_version(semantic);
    let version = if private { xpriv_version } else { xpub_version };
    serialize_extended(
        version,
        parts.depth,
        &parts.parent_fingerprint,
        parts.index,
        &parts.chain_code,
        &parts.key,
        true,
    )
    .expect("serialize_extended")
}

pub fn assert_bip32_root(hd: &Bip32Hd, fixture: &Value, semantic: &str) {
    assert_eq!(hd.registry_name(), json_str(&fixture["name"]));
    assert_eq!(hd.seed_hex().as_deref(), Some(json_str(&fixture["seed"]).as_str()));
    assert_eq!(bip32_xprivate_key(hd, semantic), json_kebab_str(fixture, "root-xprivate-key"));
    assert_eq!(bip32_xpublic_key(hd, semantic), json_kebab_str(fixture, "root-xpublic-key"));
    assert_eq!(hd.root_private_key().as_deref(), Some(json_kebab_str(fixture, "root-private-key").as_str()));
    assert_eq!(hd.root_public_key().as_deref(), Some(json_kebab_str(fixture, "root-public-key").as_str()));
    assert_eq!(hd.root_chain_code().as_deref(), Some(json_kebab_str(fixture, "root-chain-code").as_str()));
}

pub fn assert_bip32_derivation(hd: &Bip32Hd, fixture: &Value, semantic: &str) {
    let derivation = &fixture["derivation"];
    assert_eq!(derived_xprivate_key(hd, semantic), json_kebab_str(derivation, "xprivate-key"));
    assert_eq!(derived_xpublic_key(hd, semantic), json_kebab_str(derivation, "xpublic-key"));
    assert_eq!(hd.private_key().as_deref(), Some(json_kebab_str(derivation, "private-key").as_str()));
    assert_eq!(hd.wif().as_deref(), Some(json_kebab_str(derivation, "wif").as_str()));
    assert_eq!(hd.chain_code().as_deref(), Some(json_kebab_str(derivation, "chain-code").as_str()));
    assert_eq!(hd.public_key().as_deref(), Some(json_kebab_str(derivation, "public-key").as_str()));
    assert_eq!(hd.uncompressed().as_deref(), Some(json_kebab_str(derivation, "uncompressed").as_str()));
    assert_eq!(hd.compressed().as_deref(), Some(json_kebab_str(derivation, "compressed").as_str()));
    assert_eq!(hd.hash().as_deref(), Some(json_kebab_str(derivation, "hash").as_str()));
    assert_eq!(hd.depth(), Some(json_u32(&derivation["depth"])));
    assert_eq!(hd.path().as_deref(), Some(json_kebab_str(derivation, "path").as_str()));
    assert_eq!(hd.index(), Some(json_u32(&derivation["index"])));
    assert_eq!(hd.indexes(), Some(json_indexes(&derivation["indexes"])));
    assert_eq!(hd.fingerprint().as_deref(), Some(json_kebab_str(derivation, "fingerprint").as_str()));
    assert_eq!(
        hd.parent_fingerprint().as_deref(),
        Some(json_kebab_str(derivation, "parent-fingerprint").as_str())
    );
}

pub fn bip44_derivation_internal() -> Bip44Derivation {
    let mut derivation = Bip44Derivation::new(Some(Bitcoin::COIN_TYPE.into()), None, None, None).expect("Bip44Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("internal-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn bip49_derivation_internal() -> Bip49Derivation {
    let mut derivation = Bip49Derivation::new(Some(Bitcoin::COIN_TYPE.into()), None, None, None).expect("Bip49Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("internal-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn bip84_derivation_internal() -> Bip84Derivation {
    let mut derivation = Bip84Derivation::new(Some(Bitcoin::COIN_TYPE.into()), None, None, None).expect("Bip84Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("internal-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn bip86_derivation_internal() -> Bip86Derivation {
    let mut derivation = Bip86Derivation::new(Some(Bitcoin::COIN_TYPE.into()), None, None, None).expect("Bip86Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("internal-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn bip44_derivation_external() -> Bip44Derivation {
    let mut derivation = Bip44Derivation::new(Some(Bitcoin::COIN_TYPE.into()), None, None, None).expect("Bip44Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("external-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn load_bip32_from_seed(data: &TestData, hd_name: &str, _semantic: &str) -> Bip32Hd {
    let fixture = &data.hds[hd_name];
    let seed = get_bytes(json_str(&fixture["seed"]).as_str());
    let mut hd = bitcoin_bip32_hd();
    hd.from_seed(&seed).expect("from_seed");
    if hd_name == "BIP32" {
        let path = json_kebab_str(&fixture["derivation"], "path");
        hd.from_derivation(&path).expect("from_derivation");
    }
    hd
}

pub fn cardano_bip44_external(coin_type: u32) -> Bip44Derivation {
    let mut derivation = Bip44Derivation::new(Some(coin_type.into()), None, None, None).expect("Bip44Derivation");
    derivation
        .from_account(0)
        .expect("from_account")
        .from_change(ChangeInput::Str("external-chain"))
        .expect("from_change")
        .from_address(0)
        .expect("from_address");
    derivation
}

pub fn assert_cardano_hd(hd: &CardanoHd, fixture: &Value) {
    assert_eq!(hd.registry_name(), json_str(&fixture["name"]));
    assert_eq!(hd.seed_hex().as_deref(), Some(json_str(&fixture["seed"]).as_str()));
    assert_eq!(hd.root_xprivate_key(true).as_deref(), Some(json_kebab_str(fixture, "root-xprivate-key").as_str()));
    assert_eq!(hd.root_xpublic_key(true).as_deref(), Some(json_kebab_str(fixture, "root-xpublic-key").as_str()));
    assert_eq!(hd.root_private_key().as_deref(), Some(json_kebab_str(fixture, "root-private-key").as_str()));
    assert_eq!(hd.root_public_key().as_deref(), Some(json_kebab_str(fixture, "root-public-key").as_str()));
    assert_eq!(hd.root_chain_code().as_deref(), Some(json_kebab_str(fixture, "root-chain-code").as_str()));

    let derivation = &fixture["derivation"];
    assert_eq!(hd.xprivate_key(true).as_deref(), Some(json_kebab_str(derivation, "xprivate-key").as_str()));
    assert_eq!(hd.xpublic_key(true).as_deref(), Some(json_kebab_str(derivation, "xpublic-key").as_str()));
    assert_eq!(hd.private_key().as_deref(), Some(json_kebab_str(derivation, "private-key").as_str()));
    assert_eq!(hd.chain_code().as_deref(), Some(json_kebab_str(derivation, "chain-code").as_str()));
    assert_eq!(hd.public_key().as_deref(), Some(json_kebab_str(derivation, "public-key").as_str()));
    assert_eq!(hd.depth(), Some(json_u32(&derivation["depth"])));
    assert_eq!(hd.path().as_deref(), Some(json_kebab_str(derivation, "path").as_str()));
    assert_eq!(hd.index(), Some(json_u32(&derivation["index"])));
    assert_eq!(hd.indexes(), Some(json_indexes(&derivation["indexes"])));
    assert_eq!(hd.fingerprint().as_deref(), Some(json_kebab_str(derivation, "fingerprint").as_str()));
    assert_eq!(
        hd.parent_fingerprint().as_deref(),
        Some(json_kebab_str(derivation, "parent-fingerprint").as_str())
    );
    assert_eq!(hd.address().as_deref(), Some(json_kebab_str(derivation, "address").as_str()));
}

pub fn assert_electrum_v1_master(hd: &ElectrumV1Hd, fixture: &Value) {
    assert_eq!(hd.registry_name(), json_str(&fixture["name"]));
    assert_eq!(hd.seed_hex().as_deref(), Some(json_str(&fixture["seed"]).as_str()));
    assert_eq!(hd.master_private_key().as_deref(), Some(json_kebab_str(fixture, "master-private-key").as_str()));
    assert_eq!(hd.master_wif().as_deref(), Some(json_kebab_str(fixture, "master-wif").as_str()));
    assert_eq!(hd.master_public_key().as_deref(), Some(json_kebab_str(fixture, "master-public-key").as_str()));
    assert_eq!(hd.public_key_type().as_deref(), Some(json_kebab_str(fixture, "public-key-type").as_str()));
    assert_eq!(hd.wif_type().as_deref(), Some(json_kebab_str(fixture, "wif-type").as_str()));
}

pub fn assert_electrum_v1_derivation(hd: &ElectrumV1Hd, fixture: &Value) {
    let derivation = &fixture["derivation"];
    assert_eq!(hd.private_key().as_deref(), Some(json_kebab_str(derivation, "private-key").as_str()));
    assert_eq!(hd.public_key().as_deref(), Some(json_kebab_str(derivation, "public-key").as_str()));
    assert_eq!(hd.wif().as_deref(), Some(json_kebab_str(derivation, "wif").as_str()));
    assert_eq!(hd.uncompressed().as_deref(), Some(json_kebab_str(derivation, "uncompressed").as_str()));
    assert_eq!(hd.compressed().as_deref(), Some(json_kebab_str(derivation, "compressed").as_str()));
    assert_eq!(
        hd.encode_address(Some(BitcoinMainnet::PUBLIC_KEY_ADDRESS_PREFIX))
            .expect("encode_address"),
        json_kebab_str(derivation, "address")
    );
}

pub fn assert_electrum_v2_master(hd: &ElectrumV2Hd, fixture: &Value) {
    assert_eq!(hd.registry_name(), json_str(&fixture["name"]));
    assert_eq!(hd.seed_hex().as_deref(), Some(json_str(&fixture["seed"]).as_str()));
    assert_eq!(hd.mode().as_deref(), Some(json_kebab_str(fixture, "mode").as_str()));
    assert_eq!(hd.master_private_key().as_deref(), Some(json_kebab_str(fixture, "master-private-key").as_str()));
    assert_eq!(hd.master_wif().as_deref(), Some(json_kebab_str(fixture, "master-wif").as_str()));
    assert_eq!(hd.master_public_key().as_deref(), Some(json_kebab_str(fixture, "master-public-key").as_str()));
    assert_eq!(hd.public_key_type().as_deref(), Some(json_kebab_str(fixture, "public-key-type").as_str()));
    assert_eq!(hd.wif_type().as_deref(), Some(json_kebab_str(fixture, "wif-type").as_str()));
}

pub fn assert_electrum_v2_derivation(hd: &ElectrumV2Hd, fixture: &Value) {
    let derivation = &fixture["derivation"];
    assert_eq!(hd.private_key().as_deref(), Some(json_kebab_str(derivation, "private-key").as_str()));
    assert_eq!(hd.public_key().as_deref(), Some(json_kebab_str(derivation, "public-key").as_str()));
    assert_eq!(hd.wif().as_deref(), Some(json_kebab_str(derivation, "wif").as_str()));
    assert_eq!(hd.uncompressed().as_deref(), Some(json_kebab_str(derivation, "uncompressed").as_str()));
    assert_eq!(hd.compressed().as_deref(), Some(json_kebab_str(derivation, "compressed").as_str()));
    assert_eq!(
        hd.encode_address(
            Some(BitcoinMainnet::PUBLIC_KEY_ADDRESS_PREFIX),
            Some(BitcoinMainnet::HRP),
            Some(BitcoinMainnet::WITNESS_VERSIONS.0[0]),
        )
        .expect("encode_address"),
        json_kebab_str(derivation, "address")
    );
}

pub fn assert_monero_hd(hd: &MoneroHd, fixture: &Value) {
    assert_eq!(hd.registry_name(), json_str(&fixture["name"]));
    assert_eq!(hd.seed_hex().as_deref(), Some(json_str(&fixture["seed"]).as_str()));
    assert_eq!(hd.spend_private_key().as_deref(), Some(json_kebab_str(fixture, "spend-private-key").as_str()));
    assert_eq!(hd.view_private_key().as_deref(), Some(json_kebab_str(fixture, "view-private-key").as_str()));
    assert_eq!(hd.spend_public_key().as_deref(), Some(json_kebab_str(fixture, "spend-public-key").as_str()));
    assert_eq!(hd.view_public_key().as_deref(), Some(json_kebab_str(fixture, "view-public-key").as_str()));
    assert_eq!(
        hd.primary_address().ok().as_deref(),
        Some(json_kebab_str(fixture, "primary-address").as_str())
    );
    let payment_id = get_bytes(json_kebab_str(fixture, "payment-id").as_str());
    assert_eq!(
        hd.integrated_address(&payment_id).expect("integrated_address"),
        json_kebab_str(fixture, "integrated-address")
    );
    for address in fixture["sub-addresses"].as_array().expect("sub-addresses") {
        assert_eq!(
            hd.sub_address(json_u32(&address["minor"]), json_u32(&address["major"]))
                .expect("sub_address"),
            json_str(&address["address"])
        );
    }
}
