use std::ops::{Add, Div, Mul, Sub};
use std::sync::OnceLock;

use bitcoin::hashes::{hash160 as hash160_hash, sha256, Hash};
use hmac::{Hmac, KeyInit, Mac};
use num_bigint::BigUint;
use num_traits::{Num, One, Zero};
use sha2::Sha256;
use thiserror::Error;

use super::base58;

pub const A: u64 = 0;
pub const B: u64 = 7;

pub const BASE58_ALPHABET: &str = base58::DEFAULT_ALPHABET;

static SECP256K1_P: OnceLock<BigUint> = OnceLock::new();
static SECP256K1_N: OnceLock<BigUint> = OnceLock::new();

pub fn p() -> &'static BigUint {
    SECP256K1_P.get_or_init(|| {
        BigUint::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .expect("P")
    })
}

pub fn n() -> &'static BigUint {
    SECP256K1_N.get_or_init(|| {
        BigUint::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .expect("N")
    })
}

pub fn encode_base58(data: &[u8]) -> String {
    base58::encode(data, Some(BASE58_ALPHABET))
}

pub fn encode_base58_checksum(data: &[u8]) -> String {
    base58::check_encode(data, Some(BASE58_ALPHABET))
}

pub fn hash160(data: &[u8]) -> [u8; 20] {
    hash160_hash::Hash::hash(data).to_byte_array()
}

pub fn hash256(data: &[u8]) -> [u8; 32] {
    let first = sha256::Hash::hash(data);
    sha256::Hash::hash(first.as_byte_array()).to_byte_array()
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    sha256::Hash::hash(data).to_byte_array()
}

#[derive(Debug, Error)]
pub enum EccError {
    #[error("num not in field range 0..prime-1")]
    NotInField,
    #[error("({0}, {1}) is not on the curve")]
    NotOnCurve(String, String),
    #[error("cannot combine values in different fields")]
    DifferentFields,
    #[error("points are not on the same curve")]
    DifferentCurves,
    #[error("bad signature: {0}")]
    BadSignature(&'static str),
    #[error("invalid SEC public key")]
    InvalidSecKey,
}

#[derive(Clone, Debug)]
pub struct FieldElement {
    num: BigUint,
    prime: BigUint,
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Eq for FieldElement {}

impl FieldElement {
    pub fn new(num: impl Into<BigUint>, prime: impl Into<BigUint>) -> Result<Self, EccError> {
        let prime = prime.into();
        let num = num.into();
        if num >= prime {
            return Err(EccError::NotInField);
        }
        Ok(Self { num, prime })
    }

    pub fn num(&self) -> &BigUint {
        &self.num
    }

    pub fn prime(&self) -> &BigUint {
        &self.prime
    }

    fn same_prime(&self, other: &Self) -> Result<(), EccError> {
        if self.prime != other.prime {
            return Err(EccError::DifferentFields);
        }
        Ok(())
    }

    pub fn pow(&self, exponent: impl Into<BigUint>) -> Self {
        let exp = exponent.into() % (&self.prime - BigUint::one());
        let num = self.num.modpow(&exp, &self.prime);
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> FieldElement {
        self.same_prime(rhs).expect("prime");
        FieldElement {
            num: (&self.num + &rhs.num) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> FieldElement {
        self.same_prime(rhs).expect("prime");
        let num = if self.num >= rhs.num {
            &self.num - &rhs.num
        } else {
            &self.prime + &self.num - &rhs.num
        };
        FieldElement {
            num: num % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> FieldElement {
        self.same_prime(rhs).expect("prime");
        FieldElement {
            num: (&self.num * &rhs.num) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Mul<&FieldElement> for u64 {
    type Output = FieldElement;

    fn mul(self, rhs: &FieldElement) -> FieldElement {
        FieldElement {
            num: (BigUint::from(self) * &rhs.num) % &rhs.prime,
            prime: rhs.prime.clone(),
        }
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &FieldElement) -> FieldElement {
        &self * rhs
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> FieldElement {
        self.same_prime(rhs).expect("prime");
        let exp = &rhs.prime - BigUint::from(2u8);
        let inv = rhs.num.modpow(&exp, &rhs.prime);
        FieldElement {
            num: (&self.num * inv) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S256Field(FieldElement);

impl S256Field {
    pub fn new(num: impl Into<BigUint>) -> Result<Self, EccError> {
        Ok(Self(FieldElement::new(num, p().clone())?))
    }

    pub fn inner(&self) -> &FieldElement {
        &self.0
    }

    pub fn sqrt(&self) -> Self {
        let exp = (p() + BigUint::one()) / BigUint::from(4u8);
        Self(self.0.pow(exp))
    }
}

impl From<FieldElement> for S256Field {
    fn from(f: FieldElement) -> Self {
        Self(f)
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl Eq for Point {}

impl Point {
    pub fn infinity(a: FieldElement, b: FieldElement) -> Self {
        Self {
            x: None,
            y: None,
            a,
            b,
        }
    }

    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, EccError> {
        if x.is_none() && y.is_none() {
            return Ok(Self { x, y, a, b });
        }
        let (Some(x), Some(y)) = (&x, &y) else {
            return Err(EccError::NotOnCurve("partial".into(), "none".into()));
        };
        x.same_prime(y)?;
        x.same_prime(&a)?;
        x.same_prime(&b)?;
        let lhs = y.pow(BigUint::from(2u8));
        let ax = a.clone() * x;
        let rhs = &(&x.pow(BigUint::from(3u8)) + &ax) + &b;
        if lhs != rhs {
            return Err(EccError::NotOnCurve(format!("{:x}", x.num), format!("{:x}", y.num)));
        }
        Ok(Self { x: Some(x.clone()), y: Some(y.clone()), a, b })
    }

    pub fn add(&self, other: &Point) -> Result<Point, EccError> {
        if self.a != other.a || self.b != other.b {
            return Err(EccError::DifferentCurves);
        }
        if self.x.is_none() {
            return Ok(other.clone());
        }
        if other.x.is_none() {
            return Ok(self.clone());
        }
        let sx = self.x.as_ref().unwrap();
        let sy = self.y.as_ref().unwrap();
        let ox = other.x.as_ref().unwrap();
        let oy = other.y.as_ref().unwrap();

        if sx == ox && sy != oy {
            return Ok(Point::infinity(self.a.clone(), self.b.clone()));
        }

        if sx != ox {
            let s = &(oy - sy) / &(ox - sx);
            let x = &(&s.pow(BigUint::from(2u8)) - sx) - ox;
            let y = &(s * &(sx - &x)) - sy;
            return Point::new(Some(x), Some(y), self.a.clone(), self.b.clone());
        }

        if sy.num().is_zero() {
            return Ok(Point::infinity(self.a.clone(), self.b.clone()));
        }

        let x_sq = sx * sx;
        let three_x_sq = &(&x_sq + &x_sq) + &x_sq;
        let num = &(&three_x_sq + &self.a);
        let den = &(sy + sy);
        let s = &(num / den);
        let x = &s.pow(BigUint::from(2u8)) - &(sx + sx);
        let y = &(s * &(sx - &x)) - sy;
        Point::new(Some(x), Some(y), self.a.clone(), self.b.clone())
    }

    pub fn mul_scalar(&self, coefficient: &BigUint) -> Point {
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        let mut result = Point::infinity(self.a.clone(), self.b.clone());
        while !coef.is_zero() {
            if &coef % BigUint::from(2u8) == BigUint::one() {
                result = result.add(&current).expect("same curve");
            }
            current = current.add(&current).expect("dbl");
            coef >>= 1;
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct S256Point(Point);

impl S256Point {
    fn curve_a_b() -> (FieldElement, FieldElement) {
        (
            FieldElement::new(A, p().clone()).expect("a"),
            FieldElement::new(B, p().clone()).expect("b"),
        )
    }

    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> Result<Self, EccError> {
        let (a, b) = Self::curve_a_b();
        let x = x.map(|f| f.0);
        let y = y.map(|f| f.0);
        Ok(Self(Point::new(x, y, a, b)?))
    }

    pub fn from_xy(x: impl Into<BigUint>, y: impl Into<BigUint>) -> Result<Self, EccError> {
        Self::new(Some(S256Field::new(x)?), Some(S256Field::new(y)?))
    }

    pub fn generator() -> &'static Self {
        static G: OnceLock<S256Point> = OnceLock::new();
        G.get_or_init(|| {
            S256Point::from_xy(
                BigUint::from_str_radix(
                    "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                    16,
                )
                .unwrap(),
                BigUint::from_str_radix(
                    "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                    16,
                )
                .unwrap(),
            )
            .expect("G")
        })
    }

    pub fn inner(&self) -> &Point {
        &self.0
    }

    pub fn mul_scalar(&self, coefficient: &BigUint) -> Self {
        let k = coefficient % n();
        Self(self.0.mul_scalar(&k))
    }

    pub fn add(&self, other: &S256Point) -> Result<S256Point, EccError> {
        Ok(S256Point(self.0.add(&other.0)?))
    }

    pub fn verify(&self, z: &BigUint, sig: &Signature) -> bool {
        let s_inv = sig.s.modpow(&(n() - BigUint::from(2u8)), n());
        let u = (z * &s_inv) % n();
        let v = (&sig.r * &s_inv) % n();
        let total = S256Point::generator()
            .mul_scalar(&u)
            .add(&self.mul_scalar(&v))
            .expect("same curve");
        match total.0.x {
            Some(ref x) => x.num == sig.r,
            None => false,
        }
    }

    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        let x = self.0.x.as_ref().expect("finite");
        let y = self.0.y.as_ref().expect("finite");
        let xb = to_fixed_32(x.num());
        if compressed {
            let prefix = if y.num() % 2u8 == BigUint::zero() {
                0x02u8
            } else {
                0x03u8
            };
            std::iter::once(prefix).chain(xb).collect()
        } else {
            std::iter::once(0x04u8)
                .chain(xb)
                .chain(to_fixed_32(y.num()))
                .collect()
        }
    }

    pub fn hash160(&self, compressed: bool) -> [u8; 20] {
        hash160(&self.sec(compressed))
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let mut payload = Vec::with_capacity(1 + 20);
        payload.push(if testnet { 0x6f } else { 0x00 });
        payload.extend_from_slice(&h160);
        encode_base58_checksum(&payload)
    }

    pub fn parse(sec_bin: &[u8]) -> Result<Self, EccError> {
        if sec_bin.is_empty() {
            return Err(EccError::InvalidSecKey);
        }
        let (a, b) = Self::curve_a_b();
        match sec_bin[0] {
            4 if sec_bin.len() == 65 => {
                let x = BigUint::from_bytes_be(&sec_bin[1..33]);
                let y = BigUint::from_bytes_be(&sec_bin[33..65]);
                Ok(Self(Point::new(
                    Some(FieldElement::new(x, p().clone())?),
                    Some(FieldElement::new(y, p().clone())?),
                    a,
                    b,
                )?))
            }
            2 | 3 if sec_bin.len() == 33 => {
                let is_even = sec_bin[0] == 2;
                let x = FieldElement::new(BigUint::from_bytes_be(&sec_bin[1..]), p().clone())?;
                let alpha = &(&x.pow(BigUint::from(3u8)) + &FieldElement::new(B, p().clone()).unwrap());
                let beta = S256Field(alpha.clone()).sqrt();
                let (even_beta, odd_beta) = if beta.0.num() % 2u8 == BigUint::zero() {
                    (beta.clone(), S256Field::new(p() - beta.0.num()).unwrap())
                } else {
                    (S256Field::new(p() - beta.0.num()).unwrap(), beta)
                };
                let y = if is_even { even_beta } else { odd_beta };
                Ok(Self(Point::new(Some(x), Some(y.0), a, b)?))
            }
            _ => Err(EccError::InvalidSecKey),
        }
    }
}

fn to_fixed_32(n: &BigUint) -> [u8; 32] {
    let mut out = [0u8; 32];
    let b = n.to_bytes_be();
    let start = 32usize.saturating_sub(b.len());
    out[start..].copy_from_slice(&b);
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub r: BigUint,
    pub s: BigUint,
}

impl Signature {
    pub fn new(r: impl Into<BigUint>, s: impl Into<BigUint>) -> Self {
        Self {
            r: r.into(),
            s: s.into(),
        }
    }

    pub fn der(&self) -> Vec<u8> {
        let mut rbin = to_fixed_32(&self.r).to_vec();
        trim_leading_zeros(&mut rbin);
        if rbin[0] & 0x80 != 0 {
            rbin.insert(0, 0);
        }
        let mut body = vec![2u8, rbin.len() as u8];
        body.extend_from_slice(&rbin);

        let mut sbin = to_fixed_32(&self.s).to_vec();
        trim_leading_zeros(&mut sbin);
        if sbin[0] & 0x80 != 0 {
            sbin.insert(0, 0);
        }
        body.push(2);
        body.push(sbin.len() as u8);
        body.extend_from_slice(&sbin);

        let mut out = vec![0x30u8, body.len() as u8];
        out.extend_from_slice(&body);
        out
    }

    pub fn parse(signature_bin: &[u8]) -> Result<Self, EccError> {
        if signature_bin.len() < 2 || signature_bin[0] != 0x30 {
            return Err(EccError::BadSignature("compound"));
        }
        let length = signature_bin[1] as usize;
        if length + 2 != signature_bin.len() {
            return Err(EccError::BadSignature("length"));
        }
        let mut i = 2;
        if signature_bin.get(i) != Some(&0x02) {
            return Err(EccError::BadSignature("r marker"));
        }
        i += 1;
        let rlength = signature_bin[i] as usize;
        i += 1;
        let r = BigUint::from_bytes_be(&signature_bin[i..i + rlength]);
        i += rlength;
        if signature_bin.get(i) != Some(&0x02) {
            return Err(EccError::BadSignature("s marker"));
        }
        i += 1;
        let slength = signature_bin[i] as usize;
        i += 1;
        let s = BigUint::from_bytes_be(&signature_bin[i..i + slength]);
        i += slength;
        if i != signature_bin.len() {
            return Err(EccError::BadSignature("trailing"));
        }
        Ok(Self { r, s })
    }
}

fn trim_leading_zeros(v: &mut Vec<u8>) {
    while v.len() > 1 && v[0] == 0 {
        v.remove(0);
    }
}

#[derive(Clone, Debug)]
pub struct PrivateKey {
    pub secret: BigUint,
    pub point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: impl Into<BigUint>) -> Result<Self, EccError> {
        let secret = secret.into();
        if secret.is_zero() || secret >= *n() {
            return Err(EccError::NotInField);
        }
        let point = S256Point::generator().mul_scalar(&secret);
        Ok(Self { secret, point })
    }

    pub fn hex(&self) -> String {
        format!("{:064x}", self.secret)
    }

    pub fn sign(&self, z: &BigUint) -> Signature {
        let k = self.deterministic_k(z);
        let r = S256Point::generator()
            .mul_scalar(&k)
            .0
            .x
            .as_ref()
            .expect("r")
            .num
            .clone();
        let k_inv = k.modpow(&(n() - BigUint::from(2u8)), n());
        let mut s = (z + &(&r * &self.secret)) % n();
        s = (&s * &k_inv) % n();
        let half = n() / BigUint::from(2u8);
        if s > half {
            s = n() - s;
        }
        Signature { r, s }
    }

    fn deterministic_k(&self, z: &BigUint) -> BigUint {
        type HmacSha256 = Hmac<Sha256>;
        let mut z = z.clone();
        if z > *n() {
            z -= n();
        }
        let z_bytes = to_fixed_32(&z);
        let secret_bytes = to_fixed_32(&self.secret);

        let mut k = [0u8; 32];
        let mut v = [1u8; 32];

        fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
            let mut mac = HmacSha256::new_from_slice(key).expect("hmac key");
            mac.update(data);
            mac.finalize().into_bytes().into()
        }

        let mut buf = Vec::with_capacity(32 + 1 + 32 + 32);
        buf.extend_from_slice(&v);
        buf.push(0);
        buf.extend_from_slice(&secret_bytes);
        buf.extend_from_slice(&z_bytes);
        k = hmac_sha256(&k, &buf);

        v = hmac_sha256(&k, &v);

        buf.clear();
        buf.extend_from_slice(&v);
        buf.push(1);
        buf.extend_from_slice(&secret_bytes);
        buf.extend_from_slice(&z_bytes);
        k = hmac_sha256(&k, &buf);

        v = hmac_sha256(&k, &v);

        loop {
            v = hmac_sha256(&k, &v);
            let candidate = BigUint::from_bytes_be(&v);
            if candidate >= BigUint::one() && candidate < *n() {
                return candidate;
            }
            let mut inner = Vec::with_capacity(v.len() + 1);
            inner.extend_from_slice(&v);
            inner.push(0);
            k = hmac_sha256(&k, &inner);
            v = hmac_sha256(&k, &v);
        }
    }

    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let secret_bytes = to_fixed_32(&self.secret);
        let mut payload = Vec::with_capacity(1 + 32 + 1);
        payload.push(if testnet { 0xef } else { 0x80 });
        payload.extend_from_slice(&secret_bytes);
        if compressed {
            payload.push(0x01);
        }
        encode_base58_checksum(&payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_on_curve() {
        let _ = S256Point::generator();
    }

    #[test]
    fn base58_checksum_matches_helper() {
        let h = b"hello";
        assert_eq!(encode_base58_checksum(h), base58::check_encode(h, Some(BASE58_ALPHABET)));
    }

    #[test]
    fn sign_verify_roundtrip() {
        let z = BigUint::from_bytes_be(b"Programming Bitcoin!");
        let pk = PrivateKey::new(BigUint::from(2u64).pow(256) / BigUint::from(10u64)).unwrap();
        let sig = pk.sign(&z);
        assert!(pk.point.verify(&z, &sig));
    }

    #[test]
    fn der_roundtrip() {
        let sig = Signature::new(0xdeadbeefu64, 0xcafeu64);
        let der = sig.der();
        let parsed = Signature::parse(&der).unwrap();
        assert_eq!(parsed.r, sig.r);
        assert_eq!(parsed.s, sig.s);
    }

    #[test]
    fn sec_parse_compressed_roundtrip() {
        let g = S256Point::generator().clone();
        let sec = g.sec(true);
        let p2 = S256Point::parse(&sec).unwrap();
        assert_eq!(p2, *S256Point::generator());
    }
}
