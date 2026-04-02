//! Bitcoin (mirrors `hdwallet/cryptocurrencies/bitcoin.py`).

use bitcoin::Network;

use super::NetworkName;

#[derive(Debug, Clone, Copy)]
pub struct Bitcoin {
    pub network: NetworkName,
}

impl Bitcoin {
    #[must_use]
    pub fn network(self) -> Network {
        match self.network {
            NetworkName::Mainnet => Network::Bitcoin,
            NetworkName::Testnet => Network::Testnet,
            NetworkName::Regtest => Network::Regtest,
        }
    }
}
