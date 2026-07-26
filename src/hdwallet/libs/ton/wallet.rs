//! TON wallet V4R2 / V5R1 address hashing (1:1 port of python-hdwallet `libs/ton/wallet.py`).

use super::cell::{BitString, Cell};

const WALLET_V4R2_CODE_BOC_HEX: &str = concat!(
    "B5EE9C72410214010002D4000114FF00F4A413F4BCF2C80B010201200203020148040504F8F28308D71820",
    "D31FD31FD31F02F823BBF264ED44D0D31FD31FD3FFF404D15143BAF2A15151BAF2A205F901541064F910F2A3F800",
    "24A4C8CB1F5240CB1F5230CBFF5210F400C9ED54F80F01D30721C0009F6C519320D74A96D307D402FB00E830E021",
    "C001E30021C002E30001C0039130E30D03A4C8CB1F12CB1FCBFF1011121302E6D001D0D3032171B0925F04E022",
    "D749C120925F04E002D31F218210706C7567BD22821064737472BDB0925F05E003FA403020FA4401C8CA07CBFFC9",
    "D0ED44D0810140D721F404305C810108F40A6FA131B3925F07E005D33FC8258210706C7567BA923830E30D038",
    "21064737472BA925F06E30D06070201200809007801FA00F40430F8276F2230500AA121BEF2E0508210706C7567",
    "831EB17080185004CB0526CF1658FA0219F400CB6917CB1F5260CB3F20C98040FB0006008A5004810108F45930ED",
    "44D0810140D720C801CF16F400C9ED540172B08E23821064737472831EB17080185005CB055003CF1623FA0213C",
    "B6ACB1FCB3FC98040FB00925F03E20201200A0B0059BD242B6F6A2684080A06B90FA0218470D4080847A4937D",
    "29910CE6903E9FF9837812801B7810148987159F31840201580C0D0011B8C97ED44D0D70B1F8003DB29DFB51342",
    "0405035C87D010C00B23281F2FFF274006040423D029BE84C600201200E0F0019ADCE76A26840206B90EB85FFC00",
    "019AF1DF6A26840106B90EB858FC0006ED207FA00D4D422F90005C8CA0715CBFFC9D077748018C8CB05CB0222CF",
    "165005FA0214CB6B12CCCCC973FB00C84014810108F451F2A7020070810108D718FA00D33FC8542047810108F45",
    "1F2A782106E6F746570748018C8CB05CB025006CF165004FA0214CB6A12CB1FCB3FC973FB0002006C810108D718",
    "FA00D33F305224810108F459F2A782106473747270748018C8CB05CB025005CF165003FA0213CB6ACB1F12CB3FC9",
    "73FB00000AF400C9ED54696225E5"
);

const WALLET_V5R1_CODE_BOC_HEX: &str = concat!(
    "b5ee9c7241021401000281000114ff00f4a413f4bcf2c80b01020120020d020148030402dcd020d749c120915b8f6320",
    "d70b1f2082106578746ebd21821073696e74bdb0925f03e082106578746eba8eb48020d72101d074d721fa4030fa44f828",
    "fa443058bd915be0ed44d0810141d721f4058307f40e6fa1319130e18040d721707fdb3ce03120d749810280b99130e070",
    "e2100f020120050c020120060902016e07080019adce76a2684020eb90eb85ffc00019af1df6a2684010eb90eb858fc002",
    "01480a0b0017b325fb51341c75c875c2c7e00011b262fb513435c280200019be5f0f6a2684080a0eb90fa02c0102f20e",
    "011e20d70b1f82107369676ebaf2e08a7f0f01e68ef0eda2edfb218308d722028308d723208020d721d31fd31fd31",
    "fed44d0d200d31f20d31fd3ffd70a000af90140ccf9109a28945f0adb31e1f2c087df02b35007b0f2d0845125baf2",
    "e0855036baf2e086f823bbf2d0882292f800de01a47fc8ca00cb1f01cf16c9ed542092f80fde70db3cd81003f6eda2",
    "edfb02f404216e926c218e4c0221d73930709421c700b38e2d01d72820761e436c20d749c008f2e09320d74ac002",
    "f2e09320d71d06c712c2005230b0f2d089d74cd7393001a4e86c128407bbf2e093d74ac000f2e093ed55e2d20001",
    "c000915be0ebd72c08142091709601d72c081c12e25210b1e30f20d74a111213009601fa4001fa44f828fa443058",
    "baf2e091ed44d0810141d718f405049d7fc8ca0040048307f453f2e08b8e14038307f45bf2e08c22d70a00216e",
    "01b3b0f2d090e2c85003cf1612f400c9ed54007230d72c08248e2d21f2e092d200ed44d0d2005113baf2d08f545",
    "03091319c01810140d721d70a00f2e08ee2c8ca0058cf16c9ed5493f2c08de20010935bdb31e1d74cd0b4d6c35e"
);

fn wallet_v4r2_code_boc() -> Vec<u8> {
    hex::decode(WALLET_V4R2_CODE_BOC_HEX).expect("valid v4r2 BoC hex")
}

fn wallet_v5r1_code_boc() -> Vec<u8> {
    hex::decode(WALLET_V5R1_CODE_BOC_HEX).expect("valid v5r1 BoC hex")
}

pub const DEFAULT_WALLET_ID: i64 = 698_983_191;
pub const MAINNET_NETWORK_GLOBAL_ID: i64 = -239;
pub const TESTNET_NETWORK_GLOBAL_ID: i64 = -3;

fn create_wallet_v4r2_data_cell(public_key: &[u8], wallet_id: i64) -> Result<Cell, String> {
    let mut cell = Cell::new();
    cell.bits.write_uint(0, 32)?;
    cell.bits.write_uint(wallet_id as u64, 32)?;
    cell.bits.write_bytes(public_key)?;
    cell.bits.write_uint(0, 1)?;
    Ok(cell)
}

fn create_state_init(code: Cell, data: Cell) -> Cell {
    let mut state_init = Cell::new();
    state_init.bits.write_bit_array("00110");
    state_init.refs.push(code);
    state_init.refs.push(data);
    state_init
}

fn write_int(bits: &mut BitString, mut number: i64, bit_length: usize) -> Result<(), String> {
    if number < 0 {
        number += 1i64 << bit_length;
    }
    bits.write_uint(number as u64, bit_length)
}

pub fn wallet_id_v5r1(network_global_id: i64, workchain: i32, subwallet_number: u32) -> i64 {
    let mut context: i64 =
        (1i64 << 31) | (((workchain as i64) & 0xFF) << 23) | ((subwallet_number as i64) & 0x7FFF);
    if context >= (1i64 << 31) {
        context -= 1i64 << 32;
    }
    let mut wallet_id = network_global_id ^ context;
    if wallet_id >= (1i64 << 31) {
        wallet_id -= 1i64 << 32;
    }
    wallet_id
}

fn create_wallet_v5r1_data_cell(public_key: &[u8], wallet_id: i64) -> Result<Cell, String> {
    let mut cell = Cell::new();
    cell.bits.write_uint(1, 1)?;
    cell.bits.write_uint(0, 32)?;
    write_int(&mut cell.bits, wallet_id, 32)?;
    cell.bits.write_bytes(public_key)?;
    cell.bits.write_uint(0, 1)?;
    Ok(cell)
}

/// Derive the account hash for a TON Wallet V4R2 contract from an Ed25519 public key.
pub fn wallet_v4r2_address_hash(
    public_key: &[u8],
    _workchain: i32,
    wallet_id: i64,
) -> Result<Vec<u8>, String> {
    if public_key.len() != 32 {
        return Err(format!(
            "Invalid public key length (expected: 32, got: {})",
            public_key.len()
        ));
    }

    let code = Cell::one_from_boc(&wallet_v4r2_code_boc())?;
    let data = create_wallet_v4r2_data_cell(public_key, wallet_id)?;
    let state_init = create_state_init(code, data);
    Ok(state_init.bytes_hash())
}

/// Derive the account hash for a TON Wallet V5R1 contract from an Ed25519 public key.
pub fn wallet_v5r1_address_hash(
    public_key: &[u8],
    workchain: i32,
    network_global_id: i64,
    subwallet_number: u32,
    wallet_id: Option<i64>,
) -> Result<Vec<u8>, String> {
    if public_key.len() != 32 {
        return Err(format!(
            "Invalid public key length (expected: 32, got: {})",
            public_key.len()
        ));
    }

    let wallet_id =
        wallet_id.unwrap_or_else(|| wallet_id_v5r1(network_global_id, workchain, subwallet_number));

    let code = Cell::one_from_boc(&wallet_v5r1_code_boc())?;
    let data = create_wallet_v5r1_data_cell(public_key, wallet_id)?;
    let state_init = create_state_init(code, data);
    Ok(state_init.bytes_hash())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN_PUBKEY: &str =
        "d14696583ee9144878635b557d515a502b04366818dfe7765737746b4f57978d";
    const GOLDEN_V5R1_HASH: &str =
        "9983839cb46bfbe4fb6e97d4976d23a935379f0fe1abfcad3cf7de9846fbb4da";

    #[test]
    fn v5r1_address_hash_matches_golden() {
        let pubkey = hex::decode(GOLDEN_PUBKEY).unwrap();
        let hash =
            wallet_v5r1_address_hash(&pubkey, 0, MAINNET_NETWORK_GLOBAL_ID, 0, None).unwrap();
        assert_eq!(hex::encode(hash), GOLDEN_V5R1_HASH);
    }

    #[test]
    fn v4r2_address_hash_is_deterministic() {
        let pubkey = hex::decode(GOLDEN_PUBKEY).unwrap();
        let hash = wallet_v4r2_address_hash(&pubkey, 0, DEFAULT_WALLET_ID).unwrap();
        assert_eq!(hash.len(), 32);
        let again = wallet_v4r2_address_hash(&pubkey, 0, DEFAULT_WALLET_ID).unwrap();
        assert_eq!(hash, again);
    }
}
