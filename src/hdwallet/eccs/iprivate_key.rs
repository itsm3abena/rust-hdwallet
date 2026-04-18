use std::borrow::Cow;

use super::EccsError;

pub trait IPrivateKey: Sized + Clone {
    fn ecc_name() -> &'static str;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError>;
    fn raw_bytes<'a>(&'a self) -> Cow<'a, [u8]>;
    fn byte_len() -> usize;
    fn is_valid_bytes(bytes: &[u8]) -> bool {
        Self::from_bytes(bytes).is_ok()
    }
}
