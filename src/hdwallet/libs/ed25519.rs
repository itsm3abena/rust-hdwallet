use std::sync::OnceLock;

use curve25519_dalek::constants::ED25519_BASEPOINT_COMPRESSED;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::scalar::Scalar;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use thiserror::Error;

pub fn q() -> &'static BigUint {
    static Q: OnceLock<BigUint> = OnceLock::new();
    Q.get_or_init(|| (BigUint::from(1u8) << 255) - BigUint::from(19u8))
}

pub fn l() -> &'static BigUint {
    static L: OnceLock<BigUint> = OnceLock::new();
    L.get_or_init(|| {
        (BigUint::from(1u8) << 252)
            + BigUint::parse_bytes(
                b"27742317777372353535851937790883648493",
                10,
            )
            .expect("L constant")
    })
}

fn edwards_d() -> &'static BigUint {
    static D: OnceLock<BigUint> = OnceLock::new();
    D.get_or_init(|| {
        let q = q();
        let inv121666 = mod_inv(&BigUint::from(121666u32), q);
        let t = (BigUint::from(121665u32) * &inv121666) % q;
        (q - t) % q
    })
}

fn sqrt_m1_i() -> &'static BigUint {
    static I: OnceLock<BigUint> = OnceLock::new();
    I.get_or_init(|| {
        let q = q();
        let exp = (q - BigUint::one()) / BigUint::from(4u8);
        BigUint::from(2u8).modpow(&exp, q)
    })
}

fn mod_inv(x: &BigUint, m: &BigUint) -> BigUint {
    x.modpow(&(m - BigUint::from(2u8)), m)
}

fn mod_sub(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    (a + m - (b % m)) % m
}

fn x_recover(y: &BigUint) -> BigUint {
    let qq = q();
    let d = edwards_d();
    let i = sqrt_m1_i();
    let y = y % qq;
    let yy = mod_mul(&y, &y, qq);
    let denom = mod_add(&mod_mul(&mod_mul(d, &y, qq), &y, qq), &BigUint::one(), qq);
    let xx = mod_mul(&mod_sub(&yy, &BigUint::one(), qq), &mod_inv(&denom, qq), qq);
    let exp = (qq + BigUint::from(3u8)) / BigUint::from(8u8);
    let mut x = xx.modpow(&exp, qq);
    if mod_sub(&mod_mul(&x, &x, qq), &xx, qq) != BigUint::zero() {
        x = mod_mul(&x, i, qq);
    }
    if (&x % BigUint::from(2u8)) != BigUint::zero() {
        x = mod_sub(qq, &x, qq);
    }
    x
}

fn mod_mul(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    (a * b) % m
}

fn mod_add(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    (a + b) % m
}

pub const COORD_BYTE_LEN: usize = 32;

pub const G_ENC_BYTES: [u8; COORD_BYTE_LEN] = *ED25519_BASEPOINT_COMPRESSED.as_bytes();

pub const G_DEC_BYTES: [u8; COORD_BYTE_LEN * 2] = [
    0x1a, 0xd5, 0x25, 0x8f, 0x60, 0x2d, 0x56, 0xc9, 0xb2, 0xa7, 0x25, 0x95, 0x60, 0xc7, 0x2c, 0x69,
    0x5c, 0xdc, 0xd6, 0xfd, 0x31, 0xe2, 0xa4, 0xc0, 0xfe, 0x53, 0x6e, 0xcd, 0xd3, 0x36, 0x69, 0x21,
    0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
    0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
];

pub fn int_decode(int_bytes: &[u8; COORD_BYTE_LEN]) -> BigUint {
    BigUint::from_bytes_le(int_bytes.as_ref())
}

pub fn int_encode(val: &BigUint) -> [u8; COORD_BYTE_LEN] {
    bigint_to_le32(&(val % q()))
}

fn bigint_to_le32(v: &BigUint) -> [u8; COORD_BYTE_LEN] {
    let mut out = [0u8; COORD_BYTE_LEN];
    let b = v.to_bytes_le();
    let n = b.len().min(COORD_BYTE_LEN);
    out[..n].copy_from_slice(&b[..n]);
    out
}

pub fn point_is_decoded_bytes(point_bytes: &[u8]) -> bool {
    point_bytes.len() == COORD_BYTE_LEN * 2
}

pub fn point_is_encoded_bytes(point_bytes: &[u8]) -> bool {
    point_bytes.len() == COORD_BYTE_LEN
}

pub fn point_is_valid_bytes(point_bytes: &[u8]) -> bool {
    point_is_decoded_bytes(point_bytes) || point_is_encoded_bytes(point_bytes)
}

#[derive(Debug, Error)]
pub enum Ed25519Error {
    #[error("invalid point encoding length")]
    InvalidPointLength,
    #[error("invalid compressed point")]
    InvalidCompressedPoint,
    #[error("decoded point is not on the curve")]
    NotOnCurve,
    #[error("integer does not fit in {COORD_BYTE_LEN} bytes")]
    IntegerOverflow,
}

pub fn point_bytes_to_coord(point_bytes: &[u8]) -> Result<(BigUint, BigUint), Ed25519Error> {
    if point_bytes.len() == COORD_BYTE_LEN * 2 {
        let (x, y) = point_bytes.split_at(COORD_BYTE_LEN);
        Ok((
            int_decode(x.try_into().unwrap()),
            int_decode(y.try_into().unwrap()),
        ))
    } else if point_bytes.len() == COORD_BYTE_LEN {
        Ok(point_decode_no_check(
            point_bytes.try_into().map_err(|_| Ed25519Error::InvalidPointLength)?,
        ))
    } else {
        Err(Ed25519Error::InvalidPointLength)
    }
}

pub fn point_coord_to_bytes(x: &BigUint, y: &BigUint) -> [u8; COORD_BYTE_LEN * 2] {
    let mut out = [0u8; COORD_BYTE_LEN * 2];
    out[..COORD_BYTE_LEN].copy_from_slice(&int_encode(&(x % q())));
    out[COORD_BYTE_LEN..].copy_from_slice(&int_encode(&(y % q())));
    out
}

pub fn point_decode_no_check(point_bytes: &[u8; COORD_BYTE_LEN]) -> (BigUint, BigUint) {
    let qq = q();
    let point_int = int_decode(point_bytes);
    let clamp = (BigUint::one() << 255) - BigUint::one();
    let y = &point_int & &clamp;
    let mut x = x_recover(&y);
    let x_odd = (&x % BigUint::from(2u8)) != BigUint::zero();
    let enc_high = (&point_int >> 255usize) & BigUint::one() != BigUint::zero();
    if x_odd != enc_high {
        x = mod_sub(qq, &x, qq);
    }
    (x, y)
}

pub fn point_decode(point_bytes: &[u8; COORD_BYTE_LEN]) -> Result<(BigUint, BigUint), Ed25519Error> {
    let (x, y) = point_decode_no_check(point_bytes);
    if !point_is_on_curve_xy(&x, &y) {
        return Err(Ed25519Error::NotOnCurve);
    }
    Ok((x, y))
}

pub fn point_encode(x: &BigUint, y: &BigUint) -> [u8; COORD_BYTE_LEN] {
    let b = point_coord_to_bytes(x, y);
    compressed_y_from_xy_bytes(&b).to_bytes()
}

fn compressed_y_from_xy_bytes(point_bytes: &[u8; COORD_BYTE_LEN * 2]) -> CompressedEdwardsY {
    let mut y_bytes: [u8; COORD_BYTE_LEN] = point_bytes[COORD_BYTE_LEN..].try_into().unwrap();
    if point_bytes[0] & 1 != 0 {
        y_bytes[COORD_BYTE_LEN - 1] |= 0x80;
    }
    CompressedEdwardsY(y_bytes)
}

fn edwards_from_point_bytes(point_bytes: &[u8]) -> Result<EdwardsPoint, Ed25519Error> {
    const DECODED_LEN: usize = COORD_BYTE_LEN * 2;
    match point_bytes.len() {
        COORD_BYTE_LEN => CompressedEdwardsY(
            point_bytes
                .try_into()
                .map_err(|_| Ed25519Error::InvalidPointLength)?,
        )
        .decompress()
        .ok_or(Ed25519Error::InvalidCompressedPoint),
        DECODED_LEN => {
            let b: &[u8; DECODED_LEN] = point_bytes
                .try_into()
                .map_err(|_| Ed25519Error::InvalidPointLength)?;
            compressed_y_from_xy_bytes(b)
                .decompress()
                .ok_or(Ed25519Error::InvalidCompressedPoint)
        }
        _ => Err(Ed25519Error::InvalidPointLength),
    }
}

fn edwards_from_coord(x: &BigUint, y: &BigUint) -> Result<EdwardsPoint, Ed25519Error> {
    let b = point_coord_to_bytes(x, y);
    compressed_y_from_xy_bytes(&b)
        .decompress()
        .ok_or(Ed25519Error::InvalidCompressedPoint)
}

pub enum PointRef<'a> {
    Encoded(&'a [u8; COORD_BYTE_LEN]),
    Decoded(&'a [u8; COORD_BYTE_LEN * 2]),
    Coord { x: &'a BigUint, y: &'a BigUint },
}

impl PointRef<'_> {
    fn to_edwards(&self) -> Result<EdwardsPoint, Ed25519Error> {
        match self {
            PointRef::Encoded(b) => edwards_from_point_bytes(b.as_ref()),
            PointRef::Decoded(b) => edwards_from_point_bytes(b.as_ref()),
            PointRef::Coord { x, y } => edwards_from_coord(x, y),
        }
    }
}

pub fn point_add(p1: PointRef<'_>, p2: PointRef<'_>) -> Result<[u8; COORD_BYTE_LEN], Ed25519Error> {
    let a = p1.to_edwards()?;
    let b = p2.to_edwards()?;
    Ok((&a + &b).compress().to_bytes())
}

pub fn point_scalar_mul(scalar: &[u8; COORD_BYTE_LEN], point: PointRef<'_>) -> Result<[u8; COORD_BYTE_LEN], Ed25519Error> {
    let s = Scalar::from_bytes_mod_order(*scalar);
    let p = point.to_edwards()?;
    Ok((&s * &p).compress().to_bytes())
}

pub fn point_scalar_mul_base(scalar: &[u8; COORD_BYTE_LEN]) -> [u8; COORD_BYTE_LEN] {
    let s = Scalar::from_bytes_mod_order(*scalar);
    EdwardsPoint::mul_base(&s).compress().to_bytes()
}

pub fn scalar_reduce(scalar: &[u8]) -> [u8; COORD_BYTE_LEN] {
    let mut wide = [0u8; COORD_BYTE_LEN * 2];
    let n = scalar.len().min(COORD_BYTE_LEN * 2);
    wide[..n].copy_from_slice(&scalar[..n]);
    Scalar::from_bytes_mod_order_wide(&wide).to_bytes()
}

pub fn scalar_reduce_from_int(scalar: &BigUint) -> [u8; COORD_BYTE_LEN] {
    scalar_reduce(&int_encode(scalar))
}

pub fn scalar_is_valid_le(scalar: &[u8; COORD_BYTE_LEN]) -> bool {
    int_decode(scalar) < *l()
}

pub fn scalar_is_valid(scalar: &BigUint) -> bool {
    scalar < l()
}

pub fn point_is_generator_bytes(point: &[u8]) -> Result<bool, Ed25519Error> {
    if point.len() == COORD_BYTE_LEN {
        Ok(point == G_ENC_BYTES)
    } else if point.len() == COORD_BYTE_LEN * 2 {
        Ok(point == G_DEC_BYTES.as_slice())
    } else {
        Err(Ed25519Error::InvalidPointLength)
    }
}

pub fn point_is_generator_xy(x: &BigUint, y: &BigUint) -> bool {
    let gx = int_decode((&G_DEC_BYTES[..COORD_BYTE_LEN]).try_into().unwrap());
    let gy = int_decode((&G_DEC_BYTES[COORD_BYTE_LEN..]).try_into().unwrap());
    (x % q()) == gx && (y % q()) == gy
}

pub fn point_is_generator_point(p: PointRef<'_>) -> Result<bool, Ed25519Error> {
    Ok(match p {
        PointRef::Encoded(b) => *b == G_ENC_BYTES,
        PointRef::Decoded(b) => *b == G_DEC_BYTES,
        PointRef::Coord { x, y } => point_is_generator_xy(x, y),
    })
}

pub fn point_is_on_curve_xy(x: &BigUint, y: &BigUint) -> bool {
    let q = q();
    let d = edwards_d();
    let x = x % q;
    let y = y % q;
    let lhs = mod_sub(&mod_sub(&mod_mul(&y, &y, q), &mod_mul(&x, &x, q), q), &BigUint::one(), q);
    let rhs = mod_mul(&mod_mul(&mod_mul(&mod_mul(&d, &x, q), &x, q), &y, q), &y, q);
    lhs == rhs
}

pub fn point_is_on_curve_bytes(point_bytes: &[u8]) -> Result<bool, Ed25519Error> {
    if !point_is_valid_bytes(point_bytes) {
        return Err(Ed25519Error::InvalidPointLength);
    }
    let (x, y) = point_bytes_to_coord(point_bytes)?;
    Ok(point_is_on_curve_xy(&x, &y))
}

#[cfg(test)]
mod tests {
    use curve25519_dalek::traits::Identity;

    use super::*;

    #[test]
    fn g_constants_and_generator() {
        assert!(point_is_generator_bytes(G_ENC_BYTES.as_slice()).unwrap());
        assert!(point_is_generator_bytes(G_DEC_BYTES.as_slice()).unwrap());
        let (x, y) = point_bytes_to_coord(G_ENC_BYTES.as_slice()).unwrap();
        assert!(point_is_generator_xy(&x, &y));
        assert_eq!(point_scalar_mul_base(&Scalar::ONE.to_bytes()), G_ENC_BYTES);
    }

    #[test]
    fn scalar_reduce_wide_matches_one() {
        let wide = [0u8; 64];
        let out = scalar_reduce(&wide);
        assert_eq!(out, Scalar::ZERO.to_bytes());
    }

    #[test]
    fn point_add_g_with_neg_g_is_identity_encoding() {
        let g = edwards_from_point_bytes(&G_ENC_BYTES).unwrap();
        let neg_g = -g;
        let sum = (&g + &neg_g).compress().to_bytes();
        assert_eq!(sum, EdwardsPoint::identity().compress().to_bytes());
    }
}
