use crate::hdwallet::cryptocurrencies::avalanche::{
    ADDRESS_TYPES, AVALANCHE_PARAMS, Mainnet as AvalancheMainnet,
};
use super::cosmos::CosmosAddress;
use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AvalancheAddress;

impl AvalancheAddress {
    pub const REGISTRY_NAME: &'static str = "Avalanche";

    const DEFAULT_ADDRESS_TYPE: &'static str = "p-chain";

    fn valid_address_type_labels() -> &'static [&'static str] {
        &["p-chain", "x-chain"]
    }

    fn resolve_prefix(address_type: Option<&str>) -> Result<&'static str, AddressSchemeError> {
        let label = address_type.unwrap_or(Self::DEFAULT_ADDRESS_TYPE);
        if !Self::valid_address_type_labels().contains(&label) {
            let expected = Self::valid_address_type_labels().join(", ");
            return Err(AddressSchemeError::Msg(format!(
                "Invalid Avalanche address type (expected: {expected} | got: '{label}')"
            )));
        }
        let prefixes = AVALANCHE_PARAMS
            .chain_prefixes_by_address_type
            .unwrap_or(&[]);
        let registry_key = ADDRESS_TYPES
            .iter()
            .find_map(|(key, addr_label)| (*addr_label == label).then_some(*key))
            .ok_or_else(|| {
                AddressSchemeError::Msg(format!("Invalid Avalanche address type: {label}"))
            })?;
        prefixes
            .iter()
            .find_map(|(key, prefix)| (*key == registry_key).then_some(*prefix))
            .ok_or_else(|| {
                AddressSchemeError::Msg(format!(
                    "no chain prefix configured for Avalanche address type {label}"
                ))
            })
    }

    fn encode_kwargs(kwargs: &EncodeKwargs) -> EncodeKwargs {
        EncodeKwargs {
            hrp: Some(kwargs.hrp.unwrap_or(AvalancheMainnet::HRP)),
            address_type: kwargs.address_type,
            public_key_type: kwargs.public_key_type,
        }
    }

    fn decode_kwargs(kwargs: &DecodeKwargs) -> DecodeKwargs {
        DecodeKwargs {
            hrp: Some(kwargs.hrp.unwrap_or(AvalancheMainnet::HRP)),
            address_type: kwargs.address_type,
        }
    }
}

impl IAddress for AvalancheAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        kwargs: &EncodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let prefix = Self::resolve_prefix(kwargs.address_type)?;
        let cosmos_kwargs = Self::encode_kwargs(kwargs);
        let bech32 = CosmosAddress.encode(public_key, &cosmos_kwargs)?;
        Ok(format!("{prefix}{bech32}"))
    }

    fn decode(
        &self,
        address: &str,
        kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let prefix = Self::resolve_prefix(kwargs.address_type)?;
        let prefix_len = prefix.len();
        if address.len() < prefix_len {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: )"
            )));
        }
        let prefix_got = &address[..prefix_len];
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        CosmosAddress.decode(&address[prefix_len..], &Self::decode_kwargs(kwargs))
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use super::*;
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::eccs::{IPublicKey, Slip10Secp256k1PublicKey};
    use crate::hdwallet::hds::bip44::bip44_path;
    use crate::hdwallet::hds::Bip32Hd;
    use crate::hdwallet::slip44::coin_types;

    fn test_public_key() -> Slip10Secp256k1PublicKey {
        let seed = hex::decode(
            "5eb00bbddcf069084889a8ab9155568165f4c767744d69fbf5d1791c2fe1563254c36a54693640219b24e06f1011116141de699825262eeacc22f8ac8ae26a",
        )
        .expect("seed hex");
        let hd = Bip32Hd::from_seed(Network::Bitcoin, &seed).expect("bip32");
        let path = bip44_path(coin_types::AVALANCHE, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn p_chain_roundtrip_default() {
        let pk = test_public_key();
        let avax = AvalancheAddress;
        let addr =
            encode_with_ipublic_key(&avax, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("P-avax1"));
        let decoded = avax
            .decode(&addr, &DecodeKwargs::default())
            .expect("decode");
        assert_eq!(decoded.len(), 40);
        let hash = super::super::cosmos::CosmosAddress::public_key_hash(&pk.raw_compressed());
        assert_eq!(decoded, hex::encode(hash));
    }

    #[test]
    fn x_chain_roundtrip() {
        let pk = test_public_key();
        let avax = AvalancheAddress;
        let kwargs = EncodeKwargs {
            address_type: Some("x-chain"),
            ..EncodeKwargs::default()
        };
        let addr = encode_with_ipublic_key(&avax, &pk, &kwargs).expect("encode");
        assert!(addr.starts_with("X-avax1"));
        let decode_kwargs = DecodeKwargs {
            address_type: Some("x-chain"),
            ..DecodeKwargs::default()
        };
        avax.decode(&addr, &decode_kwargs).expect("decode");
    }

    #[test]
    fn invalid_address_type_errors() {
        let pk = test_public_key();
        let avax = AvalancheAddress;
        let kwargs = EncodeKwargs {
            address_type: Some("c-chain"),
            ..EncodeKwargs::default()
        };
        assert!(encode_with_ipublic_key(&avax, &pk, &kwargs).is_err());
    }

    #[test]
    fn wrong_prefix_errors() {
        let pk = test_public_key();
        let avax = AvalancheAddress;
        let x_addr = encode_with_ipublic_key(
            &avax,
            &pk,
            &EncodeKwargs {
                address_type: Some("x-chain"),
                ..EncodeKwargs::default()
            },
        )
        .expect("encode");
        assert!(avax.decode(&x_addr, &DecodeKwargs::default()).is_err());
    }
}
