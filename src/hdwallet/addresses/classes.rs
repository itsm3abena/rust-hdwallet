pub use super::algorand::AlgorandAddress;
pub use super::aptos::AptosAddress;

use super::iaddress::IAddress;

macro_rules! address_class {
    ($(#[$meta:meta])* $ty:ident, $name:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $ty;

        impl $ty {
            pub const REGISTRY_NAME: &'static str = $name;
        }

        impl IAddress for $ty {
            fn name(&self) -> &'static str {
                $name
            }
        }
    };
}

address_class!(AvalancheAddress, "Avalanche");
address_class!(BitcoinCashAddress, "BitcoinCash");
address_class!(CardanoAddress, "Cardano");
address_class!(CosmosAddress, "Cosmos");
address_class!(EOSAddress, "EOS");
address_class!(ErgoAddress, "Ergo");
address_class!(EthereumAddress, "Ethereum");
address_class!(FilecoinAddress, "Filecoin");
address_class!(HarmonyAddress, "Harmony");
address_class!(IconAddress, "Icon");
address_class!(InjectiveAddress, "Injective");
address_class!(MoneroAddress, "Monero");
address_class!(MultiversXAddress, "MultiversX");
address_class!(NanoAddress, "Nano");
address_class!(NearAddress, "Near");
address_class!(NeoAddress, "Neo");
address_class!(OKTChainAddress, "OKT-Chain");
address_class!(P2PKHAddress, "P2PKH");
address_class!(P2SHAddress, "P2SH");
address_class!(P2TRAddress, "P2TR");
address_class!(P2WPKHAddress, "P2WPKH");
address_class!(P2WPKHInP2SHAddress, "P2WPKH-In-P2SH");
address_class!(P2WSHAddress, "P2WSH");
address_class!(P2WSHInP2SHAddress, "P2WSH-In-P2SH");
address_class!(RippleAddress, "Ripple");
address_class!(SolanaAddress, "Solana");
address_class!(StellarAddress, "Stellar");
address_class!(SuiAddress, "Sui");
address_class!(TezosAddress, "Tezos");
address_class!(TronAddress, "Tron");
address_class!(XinFinAddress, "XinFin");
address_class!(ZilliqaAddress, "Zilliqa");

pub static ALGORAND: AlgorandAddress = AlgorandAddress;
pub static APTOS: AptosAddress = AptosAddress;
pub static AVALANCHE: AvalancheAddress = AvalancheAddress;
pub static BITCOIN_CASH: BitcoinCashAddress = BitcoinCashAddress;
pub static CARDANO: CardanoAddress = CardanoAddress;
pub static COSMOS: CosmosAddress = CosmosAddress;
pub static EOS: EOSAddress = EOSAddress;
pub static ERGO: ErgoAddress = ErgoAddress;
pub static ETHEREUM: EthereumAddress = EthereumAddress;
pub static FILECOIN: FilecoinAddress = FilecoinAddress;
pub static HARMONY: HarmonyAddress = HarmonyAddress;
pub static ICON: IconAddress = IconAddress;
pub static INJECTIVE: InjectiveAddress = InjectiveAddress;
pub static MONERO: MoneroAddress = MoneroAddress;
pub static MULTIVERS_X: MultiversXAddress = MultiversXAddress;
pub static NANO: NanoAddress = NanoAddress;
pub static NEAR: NearAddress = NearAddress;
pub static NEO: NeoAddress = NeoAddress;
pub static OKT_CHAIN: OKTChainAddress = OKTChainAddress;
pub static P2PKH: P2PKHAddress = P2PKHAddress;
pub static P2SH: P2SHAddress = P2SHAddress;
pub static P2TR: P2TRAddress = P2TRAddress;
pub static P2WPKH: P2WPKHAddress = P2WPKHAddress;
pub static P2WPKH_IN_P2SH: P2WPKHInP2SHAddress = P2WPKHInP2SHAddress;
pub static P2WSH: P2WSHAddress = P2WSHAddress;
pub static P2WSH_IN_P2SH: P2WSHInP2SHAddress = P2WSHInP2SHAddress;
pub static RIPPLE: RippleAddress = RippleAddress;
pub static SOLANA: SolanaAddress = SolanaAddress;
pub static STELLAR: StellarAddress = StellarAddress;
pub static SUI: SuiAddress = SuiAddress;
pub static TEZOS: TezosAddress = TezosAddress;
pub static TRON: TronAddress = TronAddress;
pub static XIN_FIN: XinFinAddress = XinFinAddress;
pub static ZILLIQA: ZilliqaAddress = ZilliqaAddress;

pub static CLASS_ORDER: &[&dyn IAddress] = &[
    &ALGORAND as &dyn IAddress,
    &APTOS as &dyn IAddress,
    &AVALANCHE as &dyn IAddress,
    &BITCOIN_CASH as &dyn IAddress,
    &CARDANO as &dyn IAddress,
    &COSMOS as &dyn IAddress,
    &EOS as &dyn IAddress,
    &ERGO as &dyn IAddress,
    &ETHEREUM as &dyn IAddress,
    &FILECOIN as &dyn IAddress,
    &HARMONY as &dyn IAddress,
    &ICON as &dyn IAddress,
    &INJECTIVE as &dyn IAddress,
    &MONERO as &dyn IAddress,
    &MULTIVERS_X as &dyn IAddress,
    &NANO as &dyn IAddress,
    &NEAR as &dyn IAddress,
    &NEO as &dyn IAddress,
    &OKT_CHAIN as &dyn IAddress,
    &P2PKH as &dyn IAddress,
    &P2SH as &dyn IAddress,
    &P2TR as &dyn IAddress,
    &P2WPKH as &dyn IAddress,
    &P2WPKH_IN_P2SH as &dyn IAddress,
    &P2WSH as &dyn IAddress,
    &P2WSH_IN_P2SH as &dyn IAddress,
    &RIPPLE as &dyn IAddress,
    &SOLANA as &dyn IAddress,
    &STELLAR as &dyn IAddress,
    &SUI as &dyn IAddress,
    &TEZOS as &dyn IAddress,
    &TRON as &dyn IAddress,
    &XIN_FIN as &dyn IAddress,
    &ZILLIQA as &dyn IAddress,
];
