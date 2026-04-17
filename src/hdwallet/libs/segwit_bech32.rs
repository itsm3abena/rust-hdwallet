pub const CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

const GENERATOR: [u32; 5] = [
    0x3b6a57b2,
    0x26508e6d,
    0x1ea119fa,
    0x3d4233dd,
    0x2a1462b3,
];

pub fn bech32_polymod(values: &[u8]) -> u32 {
    let mut chk = 1u32;
    for &value in values {
        let top = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ u32::from(value);
        for i in 0..5 {
            if (top >> i) & 1 != 0 {
                chk ^= GENERATOR[i];
            }
        }
    }
    chk
}

pub fn bech32_hrp_expand(hrp: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(hrp.len() * 2 + 1);
    for b in hrp.bytes() {
        out.push(b >> 5);
    }
    out.push(0);
    for b in hrp.bytes() {
        out.push(b & 31);
    }
    out
}

pub fn bech32_verify_checksum(hrp: &str, data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }
    let encoding = if data[0] == 0 { 1 } else { 0x2bc830a3 };
    let mut v = bech32_hrp_expand(hrp);
    v.extend_from_slice(data);
    bech32_polymod(&v) == encoding
}

pub fn bech32_create_checksum(hrp: &str, data: &[u8]) -> Option<[u8; 6]> {
    if data.is_empty() {
        return None;
    }
    let encoding = if data[0] == 0 { 1 } else { 0x2bc830a3 };
    let mut values = bech32_hrp_expand(hrp);
    values.extend_from_slice(data);
    values.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    let polymod = bech32_polymod(&values) ^ encoding;
    Some(std::array::from_fn(|i| ((polymod >> (5 * (5 - i))) & 31) as u8))
}

pub fn base_bech32_encode(hrp: &str, data: &[u8]) -> Option<String> {
    if !data.iter().all(|&d| d < 32) {
        return None;
    }
    let chk = bech32_create_checksum(hrp, data)?;
    let mut combined = Vec::with_capacity(data.len() + 6);
    combined.extend_from_slice(data);
    combined.extend_from_slice(&chk);
    let mut s = String::with_capacity(hrp.len() + 1 + combined.len());
    s.push_str(hrp);
    s.push('1');
    for &d in &combined {
        s.push(CHARSET.as_bytes()[d as usize] as char);
    }
    Some(s)
}

fn has_mixed_case(bech: &str) -> bool {
    let lower = bech.to_ascii_lowercase();
    let upper = bech.to_ascii_uppercase();
    bech != lower.as_str() && bech != upper.as_str()
}

pub fn base_bech32_decode(bech: &str) -> Option<(String, Vec<u8>)> {
    if bech.chars().any(|c| (c as u32) < 33 || (c as u32) > 126) || has_mixed_case(bech) {
        return None;
    }
    let bech = bech.to_ascii_lowercase();
    let pos = bech.rfind('1')?;
    if pos < 1 || pos + 7 > bech.len() || bech.len() > 90 {
        return None;
    }
    let tail = &bech[pos + 1..];
    if !tail.chars().all(|c| CHARSET.contains(c)) {
        return None;
    }
    let hrp = bech[..pos].to_string();
    let mut data = Vec::with_capacity(tail.len());
    for c in tail.chars() {
        let idx = CHARSET.find(c)? as u8;
        data.push(idx);
    }
    if !bech32_verify_checksum(&hrp, &data) {
        return None;
    }
    let data = data[..data.len().saturating_sub(6)].to_vec();
    Some((hrp, data))
}

pub fn convertbits(data: &[u8], frombits: u32, tobits: u32, pad: bool) -> Option<Vec<u8>> {
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut ret = Vec::new();
    let maxv = (1u32 << tobits) - 1;
    let max_acc = (1u32 << (frombits + tobits - 1)) - 1;
    for &value in data {
        if u32::from(value) >> frombits != 0 {
            return None;
        }
        acc = ((acc << frombits) | u32::from(value)) & max_acc;
        bits += frombits;
        while bits >= tobits {
            bits -= tobits;
            ret.push(((acc >> bits) & maxv) as u8);
        }
    }
    if pad {
        if bits != 0 {
            ret.push(((acc << (tobits - bits)) & maxv) as u8);
        }
    } else if bits >= frombits || ((acc << (tobits - bits)) & maxv) != 0 {
        return None;
    }
    Some(ret)
}

pub fn bech32_decode(hrp: &str, addr: &str) -> Option<(u8, Vec<u8>)> {
    let (hrpgot, data) = base_bech32_decode(addr)?;
    if hrpgot != hrp {
        return None;
    }
    if data.is_empty() {
        return None;
    }
    let head = data[0];
    let rest = convertbits(&data, 5, 8, false)?;
    Some((head, rest))
}

pub fn bech32_encode(hrp: &str, witprog: &[u8]) -> Option<String> {
    let converted = convertbits(witprog, 8, 5, true)?;
    let ret = base_bech32_encode(hrp, &converted)?;
    if base_bech32_decode(&ret).is_none() {
        return None;
    }
    Some(ret)
}

pub fn segwit_decode(hrp: &str, addr: &str) -> Option<(u8, Vec<u8>)> {
    let (hrpgot, data) = base_bech32_decode(addr)?;
    if hrpgot != hrp {
        return None;
    }
    if data.is_empty() {
        return None;
    }
    let decoded = convertbits(&data[1..], 5, 8, false)?;
    if decoded.len() < 2 || decoded.len() > 40 {
        return None;
    }
    if data[0] > 16 {
        return None;
    }
    if data[0] == 0 && decoded.len() != 20 && decoded.len() != 32 {
        return None;
    }
    Some((data[0], decoded))
}

pub fn segwit_encode(hrp: &str, witver: u8, witprog: &[u8]) -> Option<String> {
    let payload: Vec<u8> = std::iter::once(witver)
        .chain(convertbits(witprog, 8, 5, true)?.into_iter())
        .collect();
    let ret = base_bech32_encode(hrp, &payload)?;
    if segwit_decode(hrp, &ret).is_none() {
        return None;
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segwit_reference_vectors() {
        let addr = "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4";
        let (v, prog) = segwit_decode("bc", addr).expect("decode");
        assert_eq!(v, 0u8);
        assert_eq!(
            hex::encode(prog),
            "751e76e8199196d454941c45d1b3a323f1433bd6"
        );
        let enc = segwit_encode(
            "bc",
            v,
            &hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap(),
        )
        .expect("encode");
        assert_eq!(enc.to_ascii_lowercase(), addr.to_ascii_lowercase());
    }

    #[test]
    fn taproot_bech32m_roundtrip() {
        let program = hex::decode(
            "a000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        let enc = segwit_encode("bc", 1, &program).expect("encode");
        assert!(enc.starts_with("bc1p"));
        let (v, p) = segwit_decode("bc", &enc).expect("decode");
        assert_eq!(v, 1);
        assert_eq!(p, program);
    }

    #[test]
    fn mixed_case_invalid() {
        assert!(base_bech32_decode("bc1QW508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4").is_none());
    }
}
