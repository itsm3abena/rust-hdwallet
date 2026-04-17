const ML: [usize; 80] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14,
    11, 8, 3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, 1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7,
    15, 14, 5, 6, 2, 4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
];

const MR: [usize; 80] = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, 6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4,
    9, 1, 2, 15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, 8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2,
    13, 9, 7, 10, 14, 12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
];

const RL: [u32; 80] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, 7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11,
    7, 13, 12, 11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, 11, 12, 14, 15, 14, 15, 9, 8, 9,
    14, 5, 6, 8, 6, 5, 12, 9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
];

const RR: [u32; 80] = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, 9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6,
    15, 13, 11, 9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, 15, 5, 8, 11, 14, 14, 6, 14, 6,
    9, 12, 9, 12, 5, 15, 8, 8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
];

const KL: [u32; 5] = [0, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e];

const KR: [u32; 5] = [0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0];

#[inline]
fn fi(x: u32, y: u32, z: u32, i: usize) -> u32 {
    match i {
        0 => x ^ y ^ z,
        1 => (x & y) | (!x & z),
        2 => (x | !y) ^ z,
        3 => (x & z) | (y & !z),
        4 => x ^ (y | !z),
        _ => unreachable!(),
    }
}

#[inline]
fn rol(x: u32, i: u32) -> u32 {
    x.rotate_left(i)
}

fn compress(h0: u32, h1: u32, h2: u32, h3: u32, h4: u32, block: &[u8; 64]) -> (u32, u32, u32, u32, u32) {
    let mut x = [0u32; 16];
    for i in 0..16 {
        x[i] = u32::from_le_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
    }

    let mut al = h0;
    let mut bl = h1;
    let mut cl = h2;
    let mut dl = h3;
    let mut el = h4;

    let mut ar = h0;
    let mut br = h1;
    let mut cr = h2;
    let mut dr = h3;
    let mut er = h4;

    for j in 0..80 {
        let rnd = j >> 4;

        let oal = al;
        let obl = bl;
        let ocl = cl;
        let odl = dl;
        let oel = el;

        let al_new = rol(
            oal
                .wrapping_add(fi(obl, ocl, odl, rnd))
                .wrapping_add(x[ML[j]])
                .wrapping_add(KL[rnd]),
            RL[j],
        )
        .wrapping_add(oel);
        al = oel;
        bl = al_new;
        cl = obl;
        dl = rol(ocl, 10);
        el = odl;

        let oar = ar;
        let obr = br;
        let ocr = cr;
        let odr = dr;
        let oer = er;

        let ar_new = rol(
            oar
                .wrapping_add(fi(obr, ocr, odr, 4 - rnd))
                .wrapping_add(x[MR[j]])
                .wrapping_add(KR[rnd]),
            RR[j],
        )
        .wrapping_add(oer);
        ar = oer;
        br = ar_new;
        cr = obr;
        dr = rol(ocr, 10);
        er = odr;
    }

    (
        h1.wrapping_add(cl).wrapping_add(dr),
        h2.wrapping_add(dl).wrapping_add(er),
        h3.wrapping_add(el).wrapping_add(ar),
        h4.wrapping_add(al).wrapping_add(br),
        h0.wrapping_add(bl).wrapping_add(cr),
    )
}

pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let mut state = (0x6745_2301u32, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476, 0xc3d2_e1f0);

    let full_blocks = data.len() / 64;
    for b in 0..full_blocks {
        let chunk: &[u8; 64] = data[b * 64..b * 64 + 64].try_into().unwrap();
        let (a, b_, c, d, e) = state;
        state = compress(a, b_, c, d, e, chunk);
    }

    let pad_len = (119usize.wrapping_sub(data.len())) & 63;
    let mut fin = Vec::with_capacity(data.len() - full_blocks * 64 + 1 + pad_len + 8);
    fin.extend_from_slice(&data[full_blocks * 64..]);
    fin.push(0x80);
    fin.resize(fin.len() + pad_len, 0);
    fin.extend_from_slice(&(data.len() as u64).wrapping_mul(8).to_le_bytes());

    for b in 0..fin.len() / 64 {
        let chunk: &[u8; 64] = fin[b * 64..b * 64 + 64].try_into().unwrap();
        let (a, b_, c, d, e) = state;
        state = compress(a, b_, c, d, e, chunk);
    }

    let (a, b, c, d, e) = state;
    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&a.to_le_bytes());
    out[4..8].copy_from_slice(&b.to_le_bytes());
    out[8..12].copy_from_slice(&c.to_le_bytes());
    out[12..16].copy_from_slice(&d.to_le_bytes());
    out[16..20].copy_from_slice(&e.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const MSG_8X_1234567890: [u8; 80] = {
        let p = *b"1234567890";
        let mut a = [0u8; 80];
        let mut i = 0;
        while i < 8 {
            let mut j = 0;
            while j < 10 {
                a[i * 10 + j] = p[j];
                j += 1;
            }
            i += 1;
        }
        a
    };

    #[test]
    fn test_vectors_kuleuven() {
        let cases: &[(&[u8], &str)] = &[
            (b"", "9c1185a5c5e9fc54612808977ee8f548b2258d31"),
            (b"a", "0bdc9d2d256b3ee9daae347be6f4dc835a467ffe"),
            (b"abc", "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc"),
            (b"message digest", "5d0689ef49d2fae572b881b123a85ffa21595f36"),
            (
                b"abcdefghijklmnopqrstuvwxyz",
                "f71c27109c692c1b56bbdceb5b9d2865b3708dbc",
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "12a053384a9c0c88e405a06c27dcf49ada62eb2b",
            ),
            (
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                "b0e20b6e3116640286ed3a87a5713079b21f5189",
            ),
            (&MSG_8X_1234567890[..], "9b752e45573d4b39f4dbd3323cab82bf63326bfb"),
        ];

        for (msg, hexout) in cases {
            let got = ripemd160(msg);
            assert_eq!(hex::encode(got), *hexout, "msg len {}", msg.len());
        }
    }

    #[test]
    fn million_a_vector() {
        let msg = vec![b'a'; 1_000_000];
        let got = ripemd160(&msg);
        assert_eq!(
            hex::encode(got),
            "52783243c1697bdbe16d37f97f68f08325dc1528"
        );
    }
}
