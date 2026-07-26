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

    fn encode_kwargs<'a>(kwargs: &'a EncodeKwargs<'a>) -> EncodeKwargs<'a> {
        EncodeKwargs {
            hrp: Some(kwargs.hrp.unwrap_or(AvalancheMainnet::HRP)),
            address_prefix: kwargs.address_prefix,
            address_type: kwargs.address_type,
            public_key_type: kwargs.public_key_type,
            encode_type: kwargs.encode_type,
            decode_type: kwargs.decode_type,
            network: kwargs.network,
            path: kwargs.path,
            chain_code: kwargs.chain_code,
            path_key: kwargs.path_key,
            staking_public_key: kwargs.staking_public_key,
            skip_checksum_encode: kwargs.skip_checksum_encode,
            public_key_address_prefix: kwargs.public_key_address_prefix,
            script_address_prefix: kwargs.script_address_prefix,
            witness_version: kwargs.witness_version,
            alphabet: kwargs.alphabet,
            bounceable: kwargs.bounceable,
            ton_testnet: kwargs.ton_testnet,
            workchain: kwargs.workchain,
            wallet_version: kwargs.wallet_version,
            wallet_id: kwargs.wallet_id,
            network_global_id: kwargs.network_global_id,
            subwallet_number: kwargs.subwallet_number,
        }
    }

    fn decode_kwargs(kwargs: &DecodeKwargs) -> DecodeKwargs {
        DecodeKwargs {
            hrp: Some(kwargs.hrp.unwrap_or(AvalancheMainnet::HRP)),
            address_prefix: kwargs.address_prefix,
            address_type: kwargs.address_type,
            decode_type: kwargs.decode_type,
            network: kwargs.network,
            skip_checksum_encode: kwargs.skip_checksum_encode,
            public_key_address_prefix: kwargs.public_key_address_prefix,
            script_address_prefix: kwargs.script_address_prefix,
            witness_version: kwargs.witness_version,
            alphabet: kwargs.alphabet,
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
        kwargs: &EncodeKwargs<'_>,
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
