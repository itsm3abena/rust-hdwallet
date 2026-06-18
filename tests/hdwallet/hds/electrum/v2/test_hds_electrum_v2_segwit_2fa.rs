use super::conftest::{run_electrum_v2_test, TestData};
use rust_hdwallet::hdwallet::consts::{Modes, PublicKeyTypes};

#[test]
fn test_electrum_v2_segwit_2fa_hd() {
    run_electrum_v2_test(
        TestData::load(),
        "segwit-2fa",
        Modes::SEGWIT,
        PublicKeyTypes::UNCOMPRESSED,
    );
}
