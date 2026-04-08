pub mod info;
pub mod hdwallet;
pub mod utils;

pub mod exceptions;
// pub mod cryptocurrencies;
// pub mod hds;
// pub mod addresses;
pub mod entropies;
pub mod mnemonics;

#[cfg(test)]
#[path = "tests/test_exceptions.rs"]
mod test_exceptions;
// pub mod mnemonics;
// pub mod seeds;
// pub mod eccs;
// pub mod consts;