use std::fmt;

#[derive(Debug)]
pub struct TestError {
    pub message: String,
    pub detail: Option<String>,
    pub expected: Option<String>,
    pub got: Option<String>,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let detail_str = self
            .detail
            .as_ref()
            .map(|detail| format!(": {detail}"))
            .unwrap_or_default();
        let expected_str = self
            .expected
            .as_ref()
            .map(|expected| format!(" (expected: {expected})"))
            .unwrap_or_default();
        let got_str = self
            .got
            .as_ref()
            .map(|got| format!(" (got: {got})"))
            .unwrap_or_default();
        write!(f, "{}{}{}{}", self.message, detail_str, expected_str, got_str)
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
}
