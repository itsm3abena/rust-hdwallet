mod conftest;

use crate::conftest::{assert_error_contains, TestError};

#[test]
fn test_exception() {
    assert_error_contains(
        Result::<(), TestError>::Err(TestError::new("Error")),
        "Error",
    );
    assert_error_contains(
        Result::<(), TestError>::Err(TestError::with_detail("Error", "more_detail")),
        "more_detail",
    );
    assert_error_contains(
        Result::<(), TestError>::Err(TestError::with_expected("Error", "value_expect")),
        "value_expect",
    );
    assert_error_contains(
        Result::<(), TestError>::Err(TestError::with_expected_got(
            "Error",
            "value_expect",
            "value_got",
        )),
        "value_got",
    );
}
