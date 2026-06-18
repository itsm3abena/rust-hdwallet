use rust_hdwallet::hdwallet::crypto::{
    blake2b, blake2b_160, blake2b_224, blake2b_256, blake2b_32, blake2b_40, blake2b_512,
    chacha20_poly1305_decrypt, chacha20_poly1305_encrypt, crc32, double_sha256, hash160, hmac_sha256,
    hmac_sha512, kekkak256, pbkdf2_hmac_sha512, ripemd160, sha256, sha3_256, sha512, sha512_256,
    xmodem_crc,
};

const DATA: &[u8] = b"data";
const KEY: &[u8] = b"key";

#[test]
fn test_hmac_sha256() {
    assert_eq!(
        hex::encode(hmac_sha256(KEY, DATA)),
        "5031fe3d989c6d1537a013fa6e739da23463fdaec3b70137d828e36ace221bd0"
    );
    assert_eq!(
        hex::encode(hmac_sha256(b"key", b"data")),
        "5031fe3d989c6d1537a013fa6e739da23463fdaec3b70137d828e36ace221bd0"
    );
}

#[test]
fn test_hmac_sha512() {
    assert_eq!(
        hex::encode(hmac_sha512(KEY, DATA)),
        "3c5953a18f7303ec653ba170ae334fafa08e3846f2efe317b87efce82376253cb52a8c31ddcde5a3a2eee183c2b34cb91f85e64ddbc325f7692b199473579c58"
    );
}

#[test]
fn test_blake2b() {
    assert_eq!(
        hex::encode(blake2b(DATA, 64)),
        "8e009c642a5e5ba8916a637ad745fb9269245d75cdbd92571a852230c6defc156d066607bf9baf9b6ab07faabadbf49b59d4d3f3be39d17ed48711ad1447996b"
    );
}

#[test]
fn test_blake2b_32() {
    assert_eq!(hex::encode(blake2b_32(DATA)), "089d3619");
}

#[test]
fn test_blake2b_40() {
    assert_eq!(hex::encode(blake2b_40(DATA)), "2433533284");
}

#[test]
fn test_blake2b_160() {
    assert_eq!(
        hex::encode(blake2b_160(DATA)),
        "0db95f3f7e5691731ad8642d33de139ebe26b175"
    );
}

#[test]
fn test_blake2b_224() {
    assert_eq!(
        hex::encode(blake2b_224(DATA)),
        "2539ac9d21b73fc56b337db94f78b9af21d711d4dd6d9f194027c0c4"
    );
}

#[test]
fn test_blake2b_256() {
    assert_eq!(
        hex::encode(blake2b_256(DATA)),
        "a035872d6af8639ede962dfe7536b0c150b590f3234a922fb7064cd11971b58e"
    );
}

#[test]
fn test_blake2b_512() {
    assert_eq!(
        hex::encode(blake2b_512(DATA)),
        "8e009c642a5e5ba8916a637ad745fb9269245d75cdbd92571a852230c6defc156d066607bf9baf9b6ab07faabadbf49b59d4d3f3be39d17ed48711ad1447996b"
    );
}

#[test]
fn test_chacha20_poly1305_encrypt_decrypt() {
    let key = [b'0'; 32];
    let nonce = [b'0'; 12];
    let assoc_data = b"header";
    let plain_text = b"plaintext";

    let (cipher_text, tag) = chacha20_poly1305_encrypt(key, nonce, assoc_data, plain_text)
        .expect("encrypt");
    assert_eq!(hex::encode(&cipher_text), "e8c6ee0166bdef2e7c");
    assert_eq!(hex::encode(&tag), "a6e0fdf17951e654c71d88398a0cb5db");

    let decrypted =
        chacha20_poly1305_decrypt(key, nonce, assoc_data, cipher_text, tag).expect("decrypt");
    assert_eq!(decrypted, plain_text);
}

#[test]
fn test_sha256() {
    assert_eq!(
        hex::encode(sha256(DATA)),
        "3a6eb0790f39ac87c94f3856b2dd2c5d110e6811602261a9a923d3bb23adc8b7"
    );
}

#[test]
fn test_double_sha256() {
    assert_eq!(
        hex::encode(double_sha256(DATA)),
        "464472b56079ded3d359b17935624bdb8487b6a64856090725277ddb5fb5576a"
    );
}

#[test]
fn test_hash160() {
    assert_eq!(
        hex::encode(hash160(DATA)),
        "d562657f063655cf06d88e9ab358300f8b4ca6d9"
    );
}

#[test]
fn test_crc32() {
    assert_eq!(hex::encode(crc32(DATA)), "adf3f363");
}

#[test]
fn test_xmodem_crc() {
    assert_eq!(hex::encode(xmodem_crc(DATA)), "8762");
}

#[test]
fn test_pbkdf2_hmac_sha512() {
    assert_eq!(
        hex::encode(pbkdf2_hmac_sha512(b"password", b"salt", 1000, None)),
        "afe6c5530785b6cc6b1c6453384731bd5ee432ee549fd42fb6695779ad8a1c5bf59de69c48f774efc4007d5298f9033c0241d5ab69305e7b64eceeb8d834cfec"
    );
}

#[test]
fn test_kekkak256() {
    assert_eq!(
        hex::encode(kekkak256(DATA)),
        "8f54f1c2d0eb5771cd5bf67a6689fcd6eed9444d91a39e5ef32a9b4ae5ca14ff"
    );
}

#[test]
fn test_ripemd160() {
    assert_eq!(
        hex::encode(ripemd160(DATA)),
        "cd43325b85172ca28e96785d0cb4832fd62cdf43"
    );
}

#[test]
fn test_sha512() {
    assert_eq!(
        hex::encode(sha512(DATA)),
        "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876"
    );
}

#[test]
fn test_sha512_256() {
    assert_eq!(
        hex::encode(sha512_256(DATA)),
        "99902eaf90e92264667843cde66675ed94caa361634bad57874642aa364aa968"
    );
}

#[test]
fn test_sha3_256() {
    assert_eq!(
        hex::encode(sha3_256(DATA)),
        "efda893aa850b0c0e61f33325615b9d93bcf6b42d60d8f5d37ebc720fd4e3daf"
    );
}
