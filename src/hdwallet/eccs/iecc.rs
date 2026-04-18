use num_bigint::BigUint;

pub trait IEllipticCurveCryptography: Copy + 'static {
    const NAME: &'static str;
    fn order() -> BigUint;
}
