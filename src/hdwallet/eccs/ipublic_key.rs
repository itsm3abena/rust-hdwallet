use super::EccsError;
use super::ipoint::IPoint;

pub trait IPublicKey: Sized + Clone {
    type Point: IPoint;

    fn ecc_name() -> &'static str;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError>;
    fn from_point(point: &Self::Point) -> Result<Self, EccsError>;
    fn raw_compressed(&self) -> Vec<u8>;
    fn raw_uncompressed(&self) -> Vec<u8>;
    fn point(&self) -> Self::Point;
    fn compressed_len() -> usize;
    fn uncompressed_len() -> usize;
    fn is_valid_bytes(bytes: &[u8]) -> bool {
        Self::from_bytes(bytes).is_ok()
    }
}
