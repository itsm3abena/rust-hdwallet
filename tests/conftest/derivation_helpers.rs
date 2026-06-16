use hdwallet_rust::hdwallet::derivations::{
    Bip44Derivation, Bip49Derivation, Bip84Derivation, Bip86Derivation, Cip1852Derivation,
    ChangeInput, ChangeValue, CustomDerivation, ElectrumDerivation, HdwDerivation, IDerivation,
    MoneroDerivation, RoleInput,
};
use hdwallet_rust::hdwallet::exceptions::DerivationError;
use serde_json::Value;

use super::json_helpers::json_u32;
use super::{assert_error_contains, json_str, TestData};
use hdwallet_rust::hdwallet::derivations::CHANGES;

fn change_value_from_json(value: &str) -> ChangeValue {
    match value {
        "external-chain" => ChangeValue::Label(CHANGES::EXTERNAL_CHAIN),
        "internal-chain" => ChangeValue::Label(CHANGES::INTERNAL_CHAIN),
        other => panic!("unsupported change label {other}"),
    }
}



pub fn assert_purpose_derivation_defaults<D>(
    data: &TestData,
    key: &str,
    purpose: fn() -> Result<D, DerivationError>,
    accessors: fn(&D, &Value),
) where
    D: IDerivation,
{
    let default_fixture = &data.derivations[key]["default"];
    let derivation = purpose().expect("default derivation");
    assert_eq!(D::name(), json_str(&default_fixture["name"]));
    accessors(&derivation, default_fixture);
}

pub fn assert_bip44(data: &TestData) {
    use hdwallet_rust::hdwallet::derivations::CHANGES;

    assert_eq!(CHANGES::EXTERNAL_CHAIN, "external-chain");
    assert_eq!(CHANGES::INTERNAL_CHAIN, "internal-chain");

    let default_fixture = &data.derivations["BIP44"]["default"];
    let from_fixture = &data.derivations["BIP44"]["from"];

    let derivation = Bip44Derivation::new(None, None, None, None).expect("Bip44Derivation::new");
    assert_eq!(Bip44Derivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.purpose(), Some(json_u32(&default_fixture["purpose"])));
    assert_eq!(derivation.coin_type(), Some(json_u32(&default_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
    assert_eq!(
        derivation.change(),
        Some(change_value_from_json(&json_str(&default_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&default_fixture["path"]));

    let mut derivation = Bip44Derivation::new(
        Some(json_u32(&from_fixture["coin_type"]).into()),
        Some(json_u32(&from_fixture["account"]).into()),
        Some(ChangeInput::Str(json_str(&from_fixture["change"]).as_str())),
        Some(json_u32(&from_fixture["address"]).into()),
    )
    .expect("Bip44Derivation::new");
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    assert_eq!(
        derivation.change(),
        Some(change_value_from_json(&json_str(&from_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    derivation.clean().expect("clean");
    assert_eq!(Bip44Derivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.purpose(), Some(json_u32(&default_fixture["purpose"])));
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
    assert_eq!(
        derivation.change(),
        Some(change_value_from_json(&json_str(&default_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

    let mut derivation = Bip44Derivation::new(None, None, None, None).expect("Bip44Derivation::new");
    derivation
        .from_coin_type(json_u32(&from_fixture["coin_type"]))
        .expect("from_coin_type")
        .from_account(json_u32(&from_fixture["account"]))
        .expect("from_account")
        .from_change(ChangeInput::Str(json_str(&from_fixture["change"]).as_str()))
        .expect("from_change")
        .from_address(json_u32(&from_fixture["address"]))
        .expect("from_address");
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    assert_eq!(
        derivation.change(),
        Some(change_value_from_json(&json_str(&from_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    assert_error_contains(
        Bip44Derivation::new(None, None, Some(ChangeInput::Str("invalid-change")), None),
        "Bad BIP44 change index",
    );
    let mut derivation = Bip44Derivation::new(None, None, None, None).expect("Bip44Derivation::new");
    assert_error_contains(
        derivation.from_change(ChangeInput::Str("invalid-change")),
        "Bad BIP44 change index",
    );
}

macro_rules! purpose_derivation_tests {
    ($name:ident, $ty:ty, $key:literal) => {
        pub fn $name(data: &TestData) {
            let default_fixture = &data.derivations[$key]["default"];
            let from_fixture = &data.derivations[$key]["from"];

            let derivation = <$ty>::new(None, None, None, None).expect("new");
            assert_eq!(<$ty>::name(), json_str(&default_fixture["name"]));
            assert_eq!(derivation.purpose(), Some(json_u32(&default_fixture["purpose"])));
            assert_eq!(derivation.coin_type(), Some(json_u32(&default_fixture["coin_type"])));
            assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
            assert_eq!(
                derivation.change(),
                Some(change_value_from_json(&json_str(&default_fixture["change"])))
            );
            assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));
            assert_eq!(derivation.path(), json_str(&default_fixture["path"]));

            let mut derivation = <$ty>::new(
                Some(json_u32(&from_fixture["coin_type"]).into()),
                Some(json_u32(&from_fixture["account"]).into()),
                Some(ChangeInput::Str(json_str(&from_fixture["change"]).as_str())),
                Some(json_u32(&from_fixture["address"]).into()),
            )
            .expect("new");
            assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
            assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
            assert_eq!(
                derivation.change(),
                Some(change_value_from_json(&json_str(&from_fixture["change"])))
            );
            assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
            assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

            derivation.clean().expect("clean");
            assert_eq!(<$ty>::name(), json_str(&default_fixture["name"]));
            assert_eq!(derivation.purpose(), Some(json_u32(&default_fixture["purpose"])));
            assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
            assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
            assert_eq!(
                derivation.change(),
                Some(change_value_from_json(&json_str(&default_fixture["change"])))
            );
            assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

            let mut derivation = <$ty>::new(None, None, None, None).expect("new");
            derivation
                .from_coin_type(json_u32(&from_fixture["coin_type"]))
                .expect("from_coin_type")
                .from_account(json_u32(&from_fixture["account"]))
                .expect("from_account")
                .from_change(ChangeInput::Str(json_str(&from_fixture["change"]).as_str()))
                .expect("from_change")
                .from_address(json_u32(&from_fixture["address"]))
                .expect("from_address");
            assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
            assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
            assert_eq!(
                derivation.change(),
                Some(change_value_from_json(&json_str(&from_fixture["change"])))
            );
            assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
            assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

            assert_error_contains(
                <$ty>::new(None, None, Some(ChangeInput::Str("invalid-change")), None),
                "Bad BIP44 change index",
            );
            let mut derivation = <$ty>::new(None, None, None, None).expect("new");
            assert_error_contains(
                derivation.from_change(ChangeInput::Str("invalid-change")),
                "Bad BIP44 change index",
            );
        }
    };
}

purpose_derivation_tests!(assert_bip49, Bip49Derivation, "BIP49");
purpose_derivation_tests!(assert_bip84, Bip84Derivation, "BIP84");
purpose_derivation_tests!(assert_bip86, Bip86Derivation, "BIP86");

pub fn assert_custom_derivation(data: &TestData) {
    let fixture = &data.derivations["Custom"];
    assert_eq!(CustomDerivation::name(), json_str(&fixture["name"]));
    assert_eq!(
        CustomDerivation::new(None, None).expect("new").path(),
        json_str(&fixture["default-path"])
    );

    let mut derivation = CustomDerivation::new(None, None).expect("new");
    derivation
        .from_path(json_str(&fixture["from-path"]["path"]).as_str())
        .expect("from_path");
    assert_eq!(derivation.path(), json_str(&fixture["from-path"]["path"]));
    derivation.clean().expect("clean");
    assert_eq!(derivation.path(), json_str(&fixture["default-path"]));

    let indexes: Vec<u32> = fixture["from-path"]["indexes"]
        .as_array()
        .expect("indexes")
        .iter()
        .map(json_u32)
        .collect();
    let mut derivation = CustomDerivation::new(None, None).expect("new");
    derivation
        .from_indexes(&indexes)
        .expect("from_indexes");
    assert_eq!(derivation.path(), json_str(&fixture["from-path"]["path"]));

    let mut derivation = CustomDerivation::new(None, None).expect("new");
    derivation
        .from_index(
            json_u32(&fixture["from-index"]["index"]),
            fixture["from-index"]["hardened"].as_bool().unwrap_or(false),
        )
        .expect("from_index");
    assert_eq!(derivation.path(), json_str(&fixture["from-index"]["path"]));

    let mut derivation = CustomDerivation::new(None, None).expect("new");
    assert_error_contains(derivation.from_path("n/15/0/0/0/0"), "Bad path format");
}

pub fn assert_electrum_derivation(data: &TestData) {
    let default_fixture = &data.derivations["Electrum"]["default"];
    let from_fixture = &data.derivations["Electrum"]["from"];

    let derivation = ElectrumDerivation::new(None, None).expect("new");
    assert_eq!(ElectrumDerivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

    let mut derivation = ElectrumDerivation::new(
        Some(json_u32(&from_fixture["change"]).into()),
        Some(json_u32(&from_fixture["address"]).into()),
    )
    .expect("new");
    assert_eq!(
        derivation.change(),
        Some(ChangeValue::Index(json_u32(&from_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));

    derivation.clean().expect("clean");
    assert_eq!(
        derivation.change(),
        Some(ChangeValue::Index(json_u32(&default_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

    let mut derivation = ElectrumDerivation::new(None, None).expect("new");
    derivation
        .from_change(json_u32(&from_fixture["change"]))
        .expect("from_change")
        .from_address(json_u32(&from_fixture["address"]))
        .expect("from_address");
    assert_eq!(
        derivation.change(),
        Some(ChangeValue::Index(json_u32(&from_fixture["change"])))
    );
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
}

pub fn assert_hdw_derivation(data: &TestData) {
    use hdwallet_rust::hdwallet::derivations::EccValue;

    let default_fixture = &data.derivations["HDW"]["default"];
    let from_fixture = &data.derivations["HDW"]["from"];

    let derivation = HdwDerivation::new(None, None, None).expect("new");
    assert_eq!(HdwDerivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
    match derivation.ecc_value(true).expect("ecc_value") {
        EccValue::Name(name) => assert_eq!(name, json_str(&default_fixture["ecc"]).as_str()),
        EccValue::Index(index) => panic!("expected ecc name, got index {index}"),
    }
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&default_fixture["path"]));

    let mut derivation = HdwDerivation::new(
        Some(json_u32(&from_fixture["account"]).into()),
        Some(hdwallet_rust::hdwallet::derivations::EccInput::Str(
            json_str(&from_fixture["ecc"]).as_str(),
        )),
        Some(json_u32(&from_fixture["address"]).into()),
    )
    .expect("new");
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    match derivation.ecc_value(true).expect("ecc_value") {
        EccValue::Name(name) => assert_eq!(name, json_str(&from_fixture["ecc"]).as_str()),
        EccValue::Index(index) => panic!("expected ecc name, got index {index}"),
    }
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    derivation.clean().expect("clean");
    assert_eq!(HdwDerivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
    match derivation.ecc_value(true).expect("ecc_value") {
        EccValue::Name(name) => assert_eq!(name, json_str(&from_fixture["ecc"]).as_str()),
        EccValue::Index(index) => panic!("expected ecc name, got index {index}"),
    }
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

    let mut derivation = HdwDerivation::new(None, None, None).expect("new");
    derivation
        .from_account(json_u32(&from_fixture["account"]))
        .expect("from_account")
        .from_ecc(hdwallet_rust::hdwallet::derivations::EccInput::Str(
            json_str(&from_fixture["ecc"]).as_str(),
        ))
        .expect("from_ecc")
        .from_address(json_u32(&from_fixture["address"]))
        .expect("from_address");
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    match derivation.ecc_value(true).expect("ecc_value") {
        EccValue::Name(name) => assert_eq!(name, json_str(&from_fixture["ecc"]).as_str()),
        EccValue::Index(index) => panic!("expected ecc name, got index {index}"),
    }
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    assert_error_contains(
        HdwDerivation::new(
            None,
            Some(hdwallet_rust::hdwallet::derivations::EccInput::Str("invalid-ecc")),
            None,
        ),
        "Bad HDW ECC index",
    );
    let mut derivation = HdwDerivation::new(None, None, None).expect("new");
    assert_error_contains(
        derivation.from_ecc(hdwallet_rust::hdwallet::derivations::EccInput::Str(
            "invalid-ecc",
        )),
        "Bad HDW ECC index",
    );
}

pub fn assert_cip1852_derivation(data: &TestData) {
    use hdwallet_rust::hdwallet::derivations::ROLES;

    assert_eq!(ROLES::EXTERNAL_CHAIN, "external-chain");
    assert_eq!(ROLES::INTERNAL_CHAIN, "internal-chain");

    let default_fixture = &data.derivations["CIP1852"]["default"];
    let from_fixture = &data.derivations["CIP1852"]["from"];

    let derivation = Cip1852Derivation::new(None, None, None, None).expect("new");
    assert_eq!(Cip1852Derivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&default_fixture["path"]));

    let mut derivation = Cip1852Derivation::new(
        Some(json_u32(&from_fixture["coin_type"]).into()),
        Some(json_u32(&from_fixture["account"]).into()),
        Some(RoleInput::Str(json_str(&from_fixture["role"]).as_str())),
        Some(json_u32(&from_fixture["address"]).into()),
    )
    .expect("new");
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    assert_eq!(derivation.role(), Some(json_str(&from_fixture["role"]).as_str()));
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    derivation.clean().expect("clean");
    assert_eq!(Cip1852Derivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.purpose(), Some(json_u32(&default_fixture["purpose"])));
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&default_fixture["account"])));
    assert_eq!(derivation.role(), Some(json_str(&default_fixture["role"]).as_str()));
    assert_eq!(derivation.address(), Some(json_u32(&default_fixture["address"])));

    let mut derivation = Cip1852Derivation::new(None, None, None, None).expect("new");
    derivation
        .from_coin_type(json_u32(&from_fixture["coin_type"]))
        .expect("from_coin_type")
        .from_account(json_u32(&from_fixture["account"]))
        .expect("from_account")
        .from_role(RoleInput::Str(json_str(&from_fixture["role"]).as_str()))
        .expect("from_role")
        .from_address(json_u32(&from_fixture["address"]))
        .expect("from_address");
    assert_eq!(derivation.coin_type(), Some(json_u32(&from_fixture["coin_type"])));
    assert_eq!(derivation.account(), Some(json_u32(&from_fixture["account"])));
    assert_eq!(derivation.role(), Some(json_str(&from_fixture["role"]).as_str()));
    assert_eq!(derivation.address(), Some(json_u32(&from_fixture["address"])));
    assert_eq!(derivation.path(), json_str(&from_fixture["path"]));

    assert_error_contains(
        Cip1852Derivation::new(None, None, Some(RoleInput::Str("invalid-role")), None),
        "Bad CIP1852 role",
    );
    let mut derivation = Cip1852Derivation::new(None, None, None, None).expect("new");
    assert_error_contains(
        derivation.from_role(RoleInput::Str("invalid-role")),
        "Bad CIP1852 role",
    );
}

pub fn assert_monero_derivation(data: &TestData) {
    let default_fixture = &data.derivations["Monero"]["default"];
    let from_fixture = &data.derivations["Monero"]["from"];

    let derivation = MoneroDerivation::new(None, None).expect("new");
    assert_eq!(MoneroDerivation::name(), json_str(&default_fixture["name"]));
    assert_eq!(derivation.minor(), Some(json_u32(&default_fixture["minor"])));
    assert_eq!(derivation.major(), Some(json_u32(&default_fixture["major"])));

    let mut derivation = MoneroDerivation::new(
        Some(json_u32(&from_fixture["minor"]).into()),
        Some(json_u32(&from_fixture["major"]).into()),
    )
    .expect("new");
    assert_eq!(derivation.minor(), Some(json_u32(&from_fixture["minor"])));
    assert_eq!(derivation.major(), Some(json_u32(&from_fixture["major"])));

    derivation.clean().expect("clean");
    assert_eq!(derivation.minor(), Some(json_u32(&default_fixture["minor"])));
    assert_eq!(derivation.major(), Some(json_u32(&default_fixture["major"])));

    let mut derivation = MoneroDerivation::new(None, None).expect("new");
    derivation
        .from_minor(json_u32(&from_fixture["minor"]))
        .expect("from_minor")
        .from_major(json_u32(&from_fixture["major"]))
        .expect("from_major");
    assert_eq!(derivation.minor(), Some(json_u32(&from_fixture["minor"])));
    assert_eq!(derivation.major(), Some(json_u32(&from_fixture["major"])));
}
