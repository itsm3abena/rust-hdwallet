use blake2::Digest;
use rust_hdwallet::hdwallet::libs::base58::{
    check_decode, check_encode, checksum_encode, decode, encode,
};
use rust_hdwallet::hdwallet::libs::ripemd160::ripemd160;
use sha2::{Digest as Sha2Digest, Sha256};
use sha3::Sha3_256;

const RAW: &str = "0488ade405d5bc481680000000b5b40efbdef2e0a2e8ee6fa9d371f951f93e3b1e3c1b50fb31f9ea36cfc7c88100b9d3f3\
12b38951361597219c8979e177d99349fd0213bac55508c8469d02064f";

const ENCODED_RAW: &str = "xpub6GzA69ujvGDmXQDwJnCnyDBr5FxBFri57tFBzD1Q3HTUkRRS35eZND12MPL1d7iXT15tLwbKmvxhJDFtExNc7SU\
efgYGNE7xCQan6Cmhp8f";

#[test]
fn test_base58() {
    let raw = hex::decode(RAW).expect("hex raw");
    assert_eq!(
        check_encode(&raw, None),
        "xprvA3zogeNr5tfUJv9UCkfnc5F7XE7grPzDkfKbBpbnUwvVsd6HVYLJp\
QgYWCBjXwZw3GdtSVUQ7BDEgmi6MHt4MatV9gDtWuJuGYhYL7nj6aa"
    );

    let decoded = check_decode(ENCODED_RAW.as_bytes(), None).expect("check_decode");
    assert_eq!(
        hex::encode(decoded),
        concat!(
            "0488b21e05d5bc481680000000b5b40efbdef2e0a2e8ee6fa9d371f951f93e3b1e3c1b50fb31f9ea36cfc7c88100b9d3",
            "f312b38951361597219c8979e177d99349fd0213bac55508c8469d02064f"
        )
    );

    let roundtrip = decode("111233QC4".as_bytes(), None).expect("decode");
    assert_eq!(roundtrip, [0u8, 0, 0, 40, 127, 180, 205]);
    assert_eq!(encode(&roundtrip, None), "111233QC4");

    let eth = "0xfc2077CA7F403cBECA41B1B0F62D91B5EA631B5E";
    let eth_check = checksum_encode(&eth.to_lowercase(), "eth").expect("checksum_encode");
    assert_eq!(eth_check, eth);
}

#[test]
fn test_keccak() {
    let data = b"hello";

    assert_eq!(
        hex::encode(Sha256::digest(data)),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert_eq!(
        hex::encode(Sha3_256::digest(data)),
        "3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392"
    );
    assert_eq!(
        hex::encode(blake2::Blake2s256::digest(data)),
        "19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb00be456f140ca25"
    );
    assert_eq!(
        hex::encode(ripemd160(data)),
        "108f07b8382412612c048d07d13f814118445acd"
    );

    let keccak_hash = {
        use sha3::{Digest as KeccakDigest, Keccak256};
        Keccak256::digest(data)
    };
    assert_eq!(
        hex::encode(keccak_hash),
        "1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8"
    );
}
