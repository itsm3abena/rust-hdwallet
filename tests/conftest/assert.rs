use std::panic;

use serde_json::Value;

pub fn assert_json_eq(actual: &Value, expected: &Value) {
    if actual != expected {
        let actual_pretty = serde_json::to_string_pretty(actual).unwrap_or_else(|_| actual.to_string());
        let expected_pretty =
            serde_json::to_string_pretty(expected).unwrap_or_else(|_| expected.to_string());
        panic!(
            "JSON values differ.\nexpected:\n{expected_pretty}\n\ngot:\n{actual_pretty}"
        );
    }
}

pub fn assert_error_contains<E: std::fmt::Display, T>(result: Result<T, E>, expected: &str) {
    match result {
        Ok(_) => panic!("Expected error containing '{expected}', but operation succeeded"),
        Err(error) => {
            let message = error.to_string();
            assert!(
                message.contains(expected),
                "Error message '{message}' does not contain '{expected}'"
            );
        }
    }
}

pub fn assert_panics<F: FnOnce() + panic::UnwindSafe>(function: F, expected: &str) {
    let result = panic::catch_unwind(function);
    assert!(result.is_err(), "Expected panic, but function did not panic");
    let payload = result.unwrap_err();
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap_or("");
    assert!(
        message.contains(expected),
        "Panic message '{message}' does not contain '{expected}'"
    );
}

pub fn json_str(value: &Value) -> String {
    value.as_str().unwrap_or("").to_string()
}

pub fn json_nullable_str(value: &Value) -> Option<String> {
    if value.is_null() {
        None
    } else {
        Some(json_str(value))
    }
}

pub fn json_usize(value: &Value) -> usize {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
        .unwrap_or_else(|| panic!("expected numeric JSON value, got {value}")) as usize
}
