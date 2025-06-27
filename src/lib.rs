use std::u32;

use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    decode(hex_str).map_err(|e| e.to_string())
    // TODO: Decode hex string into Vec<u8>, return error string on failure
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut be = bytes.to_vec();
    be.reverse();
    be

    // TODO: Reverse the byte order of input slice and return as Vec<u8>
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    encode(bytes)
    // TODO: Implement conversion of bytes slice to hex string
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    decode(hex)
    // TODO: Implement conversion of hex string to bytes vector
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
    // TODO: Implement little-endian byte swap for u32
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
    // TODO: Parse input string to u64, return error string if invalid
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.len() >= 3 && script[0] == 0x76 && script[1] == 0xa9 {
        return ScriptType::P2PKH;
    }

    if script.len() >= 2 && script[0] == 0x00 {
        return ScriptType::P2WPKH;
    }

    ScriptType::Unknown
}
// TODO: Match script pattern and return corresponding ScriptType

pub struct Outpoint(pub String, pub u32);

// TODO: complete Outpoint tuple struct

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    &script[2..]
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
        // TODO: Return the wallet's confirmed balance
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
    // TODO: Subtract fee from mutable balance reference
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
    // TODO: Return formatted string including the txid for display or logging
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
        // TODO: Implement mapping from byte to Opcode variant
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
    // TODO: Implement UTXO consumption logic (if any)
}
