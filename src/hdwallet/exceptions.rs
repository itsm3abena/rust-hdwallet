use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
    detail: Option<String>,
    expected: Option<String>,
    got: Option<String>,
}

impl Error {
    pub fn new(
        message: &str,
        detail: Option<&str>,
        expected: Option<&str>,
        got: Option<&str>,
    ) -> Self {
        Self {
            message: message.to_string(),
            detail: detail.map(|s| s.to_string()),
            expected: expected.map(|s| s.to_string()),
            got: got.map(|s| format!("'{}'", s)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.expected, &self.got, &self.detail) {
            (Some(exp), Some(g), Some(d)) => {
                write!(f, "{}, (expected: {} | got: {}) {}", self.message, exp, g, d)
            }
            (Some(exp), Some(g), None) => {
                write!(f, "{}, (expected: {} | got: {})", self.message, exp, g)
            }
            (_, _, Some(d)) => {
                write!(f, "{} {}", self.message, d)
            }
            _ => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! define_error {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name(pub Error);

        impl $name {
            pub fn new(message: &str, detail: Option<&str>, expected: Option<&str>, got: Option<&str>) -> Self {
                Self(Error::new(message, detail, expected, got))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::error::Error for $name {}
    };
}

define_error!(EntropyError);
define_error!(ChecksumError);
define_error!(MnemonicError);

impl MnemonicError {
    pub fn simple(message: &str) -> Self {
        Self(Error::new(message, None, None, None))
    }
}
define_error!(SeedError);
define_error!(HDError);
define_error!(DerivationError);
define_error!(SemanticError);
define_error!(ECCError);
define_error!(CryptocurrencyError);
define_error!(ExtendedKeyError);
define_error!(AddressError);
define_error!(WIFError);
define_error!(SymbolError);
define_error!(NetworkError);
define_error!(PublicKeyError);
define_error!(PrivateKeyError);
define_error!(XPublicKeyError);
define_error!(XPrivateKeyError);