use super::segwit_bech32::{convertbits, CHARSET};

const GENERATOR: [u64; 5] = [
    0x98f2bc8e61,
    0x79b76d99e2,
    0xf33e5fb3c4,
    0xae2eabe2a8,
    0x1e4f43e470,
];

const CHECKSUM_SIZE: usize = 8;

fn charset_index(c: char) -> Option<u8> {
    CHARSET.find(c).map(|i| i as u8)
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    hrp.bytes().map(|b| b & 0x1f).chain(std::iter::once(0)).collect()
}

fn polymod(values: &[u8]) -> u64 {
    let mut chk: u64 = 1;
    for &value in values {
        let top = chk >> 35;
        chk = ((chk & 0x07ffffffff) << 5) ^ u64::from(value);
        for i in 0..5 {
            if (top >> i) & 1 != 0 {
                chk ^= GENERATOR[i];
            }
        }
    }
    chk ^ 1
}

fn encode_payload(hrp: &str, data: &[u8]) -> Option<String> {
    if !data.iter().all(|&d| d < 32) {
        return None;
    }
    let mut values = hrp_expand(hrp);
    values.extend_from_slice(data);
    values.extend([0u8; CHECKSUM_SIZE]);
    let polymod_val = polymod(&values);
    let checksum: [u8; CHECKSUM_SIZE] =
        std::array::from_fn(|i| ((polymod_val >> (5 * (7 - i))) & 0x1f) as u8);
    let mut s = String::with_capacity(hrp.len() + 1 + data.len() + CHECKSUM_SIZE);
    s.push_str(hrp);
    s.push(':');
    for &d in data.iter().chain(checksum.iter()) {
        s.push(CHARSET.as_bytes()[d as usize] as char);
    }
    Some(s)
}

pub fn cashaddr_encode(hrp: &str, payload: &[u8]) -> Option<String> {
    let data = convertbits(payload, 8, 5, true)?;
    encode_payload(hrp, &data)
}

pub fn cashaddr_decode(hrp: &str, address: &str) -> Option<Vec<u8>> {
    let hrp_expected = hrp.to_ascii_lowercase();
    let (hrp_in_addr, payload_part) = match address.split_once(':') {
        Some((h, rest)) => (Some(h.to_ascii_lowercase()), rest),
        None => (None, address),
    };
    if let Some(ref h) = hrp_in_addr {
        if h != &hrp_expected {
            return None;
        }
    }
    let payload_lower = payload_part.to_ascii_lowercase();
    if !payload_lower.chars().all(|c| CHARSET.contains(c)) {
        return None;
    }
    let mut data = Vec::with_capacity(payload_lower.len());
    for c in payload_lower.chars() {
        data.push(charset_index(c)?);
    }
    if hrp_in_addr.is_some() {
        let mut values = hrp_expand(&hrp_expected);
        values.extend_from_slice(&data);
        if polymod(&values) != 0 {
            return None;
        }
    }
    data.truncate(data.len().saturating_sub(CHECKSUM_SIZE));
    convertbits(&data, 5, 8, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2pkh_roundtrip() {
        let pk_hash = hex::decode("62e907b15cbf27d5425399ebf6f0fb50ebb88f18").unwrap();
        let mut payload = vec![0u8];
        payload.extend_from_slice(&pk_hash);
        let enc = cashaddr_encode("bitcoincash", &payload).expect("encode");
        assert!(enc.starts_with("bitcoincash:q"));
        let dec = cashaddr_decode("bitcoincash", &enc).expect("decode");
        assert_eq!(dec, payload);
    }
}
