use super::conftest::{run_electrum_v2_test, TestData};
use hdwallet_rust::hdwallet::consts::{Modes, PublicKeyTypes};

#[test]
fn test_electrum_v2_standard_hd() {
    run_electrum_v2_test(
        TestData::load(),
        "standard",
        Modes::STANDARD,
        PublicKeyTypes::UNCOMPRESSED,
    );
}
