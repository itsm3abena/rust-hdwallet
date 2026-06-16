use std::path::PathBuf;
use std::sync::LazyLock;

use serde_json::Value;

pub struct TestData {
    pub addresses: Value,
    pub derivations: Value,
    pub eccs: Value,
    pub entropies: Value,
    pub hds: Value,
    pub hdwallet: Value,
    pub mnemonics: Value,
    pub seeds: Value,
}

static DATA: LazyLock<TestData> = LazyLock::new(TestData::load_from_disk);

impl TestData {
    pub fn load() -> &'static TestData {
        &DATA
    }

    fn data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data")
    }

    fn load_json(name: &str) -> Value {
        let path = Self::data_dir().join("json").join(format!("{name}.json"));
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        serde_json::from_str(&content)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
    }

    fn load_from_disk() -> Self {
        Self {
            addresses: Self::load_json("addresses"),
            derivations: Self::load_json("derivations"),
            eccs: Self::load_json("eccs"),
            entropies: Self::load_json("entropies"),
            hds: Self::load_json("hds"),
            hdwallet: Self::load_json("hdwallet"),
            mnemonics: Self::load_json("mnemonics"),
            seeds: Self::load_json("seeds"),
        }
    }

    pub fn raw(name: &str) -> String {
        let path = Self::data_dir().join("raw").join(format!("{name}.txt"));
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
    }
}
