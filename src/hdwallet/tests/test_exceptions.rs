use std::fmt;
use std::panic;

#[derive(Debug)]
pub struct TestError {
    pub message: String,
    pub detail: Option<String>,
    pub expected: Option<String>,
    pub got: Option<String>,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let detail_str = self.detail.as_ref().map(|d| format!(": {}", d)).unwrap_or_default();
        let expected_str = self
            .expected
            .as_ref()
            .map(|e| format!(" (expected: {})", e))
            .unwrap_or_default();
        let got_str = self
            .got
            .as_ref()
            .map(|g| format!(" (got: {})", g))
            .unwrap_or_default();
        write!(
            f,
            "{}{}{}{}",
            self.message, detail_str, expected_str, got_str
        )
    }
}

impl std::error::Error for TestError {}

impl TestError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            detail: None,
            expected: None,
            got: None,
        }
    }

    pub fn with_detail(message: &str, detail: &str) -> Self {
        Self {
            message: message.to_string(),
            detail: Some(detail.to_string()),
            expected: None,
            got: None,
        }
    }

    pub fn with_expected(message: &str, expected: &str) -> Self {
        Self {
            message: message.to_string(),
            detail: None,
            expected: Some(expected.to_string()),
            got: None,
        }
    }

    pub fn with_expected_got(message: &str, expected: &str, got: &str) -> Self {
        Self {
            message: message.to_string(),
            detail: None,
            expected: Some(expected.to_string()),
            got: Some(got.to_string()),
        }
    }

    pub fn raise(&self) {
        panic!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_panics<F: FnOnce() + panic::UnwindSafe>(f: F, expected: &str) {
        let result = panic::catch_unwind(f);
        assert!(result.is_err(), "Expected panic, but function did not panic");
        let payload = result.unwrap_err();
        let msg = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            msg.contains(expected),
            "Panic message '{}' does not contain '{}'",
            msg,
            expected
        );
    }

    #[test]
    fn test_error_basic() {
        assert_panics(
            || {
                TestError::new("Error").raise();
            },
            "Error",
        );
    }

    #[test]
    fn test_error_detail() {
        assert_panics(
            || {
                TestError::with_detail("Error", "more_detail").raise();
            },
            "more_detail",
        );
    }

    #[test]
    fn test_error_expected() {
        assert_panics(
            || {
                TestError::with_expected("Error", "value_expect").raise();
            },
            "value_expect",
        );
    }

    #[test]
    fn test_error_expected_got() {
        assert_panics(
            || {
                TestError::with_expected_got("Error", "value_expect", "value_got").raise();
            },
            "value_expect",
        );
        assert_panics(
            || {
                TestError::with_expected_got("Error", "value_expect", "value_got").raise();
            },
            "value_got",
        );
    }
}
