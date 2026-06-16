const CHARSET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

const GENERATOR: [u32; 5] = [
    0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3,
];

fn polymod(values: &[u8]) -> u32 {
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

fn hrp_expand(hrp: &str) -> Vec<u8> {
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

fn verify_checksum(hrp: &str, data: &[u8]) -> bool {
    let mut v = hrp_expand(hrp);
    v.extend_from_slice(data);
    polymod(&v) == 1
}

fn create_checksum(hrp: &str, data: &[u8]) -> [u8; 6] {
    let mut values = hrp_expand(hrp);
    values.extend_from_slice(data);
    values.extend_from_slice(&[0u8; 6]);
    let polym = polymod(&values) ^ 1;
    std::array::from_fn(|i| ((polym >> (5 * (5 - i))) & 31) as u8)
}

fn charset_rev(c: u8) -> Option<u8> {
    CHARSET.iter().position(|&x| x == c).map(|i| i as u8)
}

pub fn base_bech32_encode(hrp: &str, data: &[u8]) -> Option<String> {
    if !data.iter().all(|&d| d < 32) {
        return None;
    }
    let chk = create_checksum(hrp, data);
    let mut s = String::with_capacity(hrp.len() + 1 + data.len() + 6);
    s.push_str(hrp);
    s.push('1');
    for &d in data.iter().chain(chk.iter()) {
        s.push(CHARSET[d as usize] as char);
    }
    Some(s)
}

fn has_mixed_case(bech: &str) -> bool {
    let has_lower = bech.bytes().any(|b| b.is_ascii_lowercase());
    let has_upper = bech.bytes().any(|b| b.is_ascii_uppercase());
    has_lower && has_upper
}

pub fn base_bech32_decode(bech: &str) -> Option<(String, Vec<u8>)> {
    if bech.bytes().any(|b| b < 33 || b > 126) || has_mixed_case(bech) {
        return None;
    }
    let bech_lower = bech.to_ascii_lowercase();
    let pos = bech_lower.rfind('1')?;
    if pos < 1 || pos + 7 > bech_lower.len() {
        return None;
    }
    let hrp = bech_lower[..pos].to_string();
    let payload = &bech_lower.as_bytes()[pos + 1..];
    let mut data = Vec::with_capacity(payload.len());
    for &c in payload {
        data.push(charset_rev(c)?);
    }
    if !verify_checksum(&hrp, &data) {
        return None;
    }
    data.truncate(data.len().saturating_sub(6));
    Some((hrp, data))
}

pub fn convert_bits(data: &[u8], frombits: u32, tobits: u32, pad: bool) -> Option<Vec<u8>> {
    if frombits == 0 || tobits == 0 {
        return None;
    }
    let maxv: u32 = (1 << tobits) - 1;
    let max_acc: u32 = (1 << (frombits + tobits - 1)) - 1;
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut ret = Vec::new();
    for &value in data {
        if u32::from(value) >= (1 << frombits) {
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
        if bits > 0 {
            ret.push(((acc << (tobits - bits)) & maxv) as u8);
        }
    } else if bits >= frombits || ((acc << (tobits - bits)) & maxv) != 0 {
        return None;
    }
    Some(ret)
}

fn witness_program_valid(version: u8, program: &[u8]) -> bool {
    if version > 16 {
        return false;
    }
    let len = program.len();
    if version == 0 {
        len == 20 || len == 32
    } else {
        (2..=40).contains(&len)
    }
}

pub fn bech32_decode(hrp: &str, addr: &str) -> Option<(u8, Vec<u8>)> {
    let (hrp_got, data) = base_bech32_decode(addr)?;
    if hrp_got != hrp {
        return None;
    }
    if data.is_empty() {
        return None;
    }
    let witver = data[0];
    let program = convert_bits(&data[1..], 5, 8, false)?;
    if !witness_program_valid(witver, &program) {
        return None;
    }
    Some((witver, program))
}

pub fn bech32_encode(hrp: &str, witness_version: u8, program: &[u8]) -> Option<String> {
    if witness_version > 16 || !witness_program_valid(witness_version, program) {
        return None;
    }
    let mut data = Vec::with_capacity(1 + program.len() * 2);
    data.push(witness_version);
    data.extend_from_slice(&convert_bits(program, 8, 5, true)?);
    let ret = base_bech32_encode(hrp, &data)?;
    if base_bech32_decode(&ret).is_none() {
        return None;
    }
    Some(ret)
}

/// Cardano Shelley-style bech32 encoding without a witness version prefix.
pub fn bech32_encode_raw(hrp: &str, program: &[u8]) -> Option<String> {
    let converted = convert_bits(program, 8, 5, true)?;
    let ret = base_bech32_encode(hrp, &converted)?;
    if base_bech32_decode(&ret).is_none() {
        return None;
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bip173_example_p2wpkh() {
        let addr = "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4";
        let (v, prog) = bech32_decode("bc", addr).expect("decode");
        assert_eq!(v, 0);
        assert_eq!(
            hex::encode(prog),
            "751e76e8199196d454941c45d1b3a323f1433bd6"
        );
        let enc = bech32_encode("bc", v, &hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap())
            .expect("encode");
        assert_eq!(enc.to_ascii_lowercase(), addr.to_ascii_lowercase());
    }

    #[test]
    fn bip173_example_p2wsh() {
        let addr = "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7";
        let (v, prog) = bech32_decode("tb", addr).expect("decode");
        assert_eq!(v, 0);
        assert_eq!(prog.len(), 32);
        let enc = bech32_encode("tb", v, &prog).expect("encode");
        assert_eq!(enc.to_ascii_lowercase(), addr.to_ascii_lowercase());
    }

    #[test]
    fn invalid_mixed_case() {
        assert!(base_bech32_decode("bc1QW508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4").is_none());
    }
}
