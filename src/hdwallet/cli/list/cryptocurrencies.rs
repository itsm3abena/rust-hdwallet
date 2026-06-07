use crate::hdwallet::exceptions::Error;

pub fn list_cryptocurrencies() -> Result<String, Error> {
    Err(Error::new(
        "list cryptocurrencies is not implemented yet",
        None,
        None,
        None,
    ))
}
