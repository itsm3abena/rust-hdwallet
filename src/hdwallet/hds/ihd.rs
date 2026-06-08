use crate::hdwallet::exceptions::{Error, Result};

#[inline]
fn not_supported(op: &'static str) -> Error {
    Error::new(
        "IHD operation not supported for this HD scheme",
        Some(op),
        None,
        None,
    )
}

pub type IndexList = Vec<u32>;

pub trait Ihd {
    fn registry_name(&self) -> &'static str;

    fn from_seed(&mut self, _seed: &[u8]) -> Result<()> {
        Err(not_supported("from_seed"))
    }

    fn from_xprivate_key(&mut self, _xprivate_key: &str, _encoded: bool, _strict: bool) -> Result<()> {
        Err(not_supported("from_xprivate_key"))
    }

    fn from_xpublic_key(&mut self, _xpublic_key: &str, _encoded: bool, _strict: bool) -> Result<()> {
        Err(not_supported("from_xpublic_key"))
    }

    fn from_wif(&mut self, _wif: &str) -> Result<()> {
        Err(not_supported("from_wif"))
    }

    fn from_private_key(&mut self, _private_key: &str) -> Result<()> {
        Err(not_supported("from_private_key"))
    }

    fn from_spend_private_key(&mut self, _spend_private_key: &str) -> Result<()> {
        Err(not_supported("from_spend_private_key"))
    }

    fn from_public_key(&mut self, _public_key: &str) -> Result<()> {
        Err(not_supported("from_public_key"))
    }

    fn from_watch_only(
        &mut self,
        _view_private_key: &str,
        _spend_public_key: &str,
    ) -> Result<()> {
        Err(not_supported("from_watch_only"))
    }

    fn from_derivation(&mut self, _derivation: &str) -> Result<()> {
        Err(not_supported("from_derivation"))
    }

    fn update_derivation(&mut self, _derivation: &str) -> Result<()> {
        Err(not_supported("update_derivation"))
    }

    fn clean_derivation(&mut self) -> Result<()> {
        Err(not_supported("clean_derivation"))
    }

    fn derivation_display(&self) -> Option<String> {
        None
    }

    fn seed_hex(&self) -> Option<String> {
        None
    }

    fn semantic(&self) -> Option<&str> {
        None
    }

    fn root_xprivate_key(&self, _encoded: bool) -> Option<String> {
        None
    }

    fn root_xpublic_key(&self, _encoded: bool) -> Option<String> {
        None
    }

    fn master_xprivate_key(&self, encoded: bool) -> Option<String> {
        self.root_xprivate_key(encoded)
    }

    fn master_xpublic_key(&self, encoded: bool) -> Option<String> {
        self.root_xpublic_key(encoded)
    }

    fn root_private_key(&self) -> Option<String> {
        None
    }

    fn root_wif(&self) -> Option<String> {
        None
    }

    fn root_chain_code(&self) -> Option<String> {
        None
    }

    fn root_public_key(&self) -> Option<String> {
        None
    }

    fn master_private_key(&self) -> Option<String> {
        self.root_private_key()
    }

    fn master_wif(&self) -> Option<String> {
        self.root_wif()
    }

    fn master_chain_code(&self) -> Option<String> {
        self.root_chain_code()
    }

    fn master_public_key(&self) -> Option<String> {
        self.root_public_key()
    }

    fn xprivate_key(&self, _encoded: bool) -> Option<String> {
        None
    }

    fn xpublic_key(&self, _encoded: bool) -> Option<String> {
        None
    }

    fn private_key(&self) -> Option<String> {
        None
    }

    fn spend_private_key(&self) -> Option<String> {
        None
    }

    fn view_private_key(&self) -> Option<String> {
        None
    }

    fn wif(&self) -> Option<String> {
        None
    }

    fn wif_type(&self) -> Option<String> {
        None
    }

    fn chain_code(&self) -> Option<String> {
        None
    }

    fn public_key(&self) -> Option<String> {
        None
    }

    fn compressed(&self) -> Option<String> {
        None
    }

    fn uncompressed(&self) -> Option<String> {
        None
    }

    fn spend_public_key(&self) -> Option<String> {
        None
    }

    fn view_public_key(&self) -> Option<String> {
        None
    }

    fn public_key_type(&self) -> Option<String> {
        None
    }

    fn mode(&self) -> Option<String> {
        None
    }

    fn hash(&self) -> Option<String> {
        None
    }

    fn fingerprint(&self) -> Option<String> {
        None
    }

    fn parent_fingerprint(&self) -> Option<String> {
        None
    }

    fn depth(&self) -> Option<u32> {
        None
    }

    fn path(&self) -> Option<String> {
        None
    }

    fn path_key(&self) -> Option<String> {
        None
    }

    fn index(&self) -> Option<u32> {
        None
    }

    fn indexes(&self) -> Option<IndexList> {
        None
    }

    fn strict(&self) -> Option<bool> {
        None
    }

    fn integrated_address(&self) -> Option<String> {
        None
    }

    fn primary_address(&self) -> Option<String> {
        None
    }

    fn sub_address(&self) -> Option<String> {
        None
    }

    fn address(&self) -> Option<String> {
        None
    }
}

