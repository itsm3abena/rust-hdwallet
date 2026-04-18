pub trait Ed25519Variant: Copy + 'static {
    const NAME: &'static str;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slip10Ed25519Id;

impl Ed25519Variant for Slip10Ed25519Id {
    const NAME: &'static str = "SLIP10-Ed25519";
}
