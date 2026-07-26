//! TON Cell / BoC primitives (1:1 port of python-hdwallet `libs/ton/cell.py`).

use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct BitString {
    pub array: Vec<u8>,
    pub cursor: usize,
    pub length: usize,
}

impl BitString {
    pub fn new(length: usize) -> Self {
        let bytes = length.div_ceil(8);
        Self {
            array: vec![0u8; bytes],
            cursor: 0,
            length,
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        u8::from((self.array[index / 8] & (1 << (7 - (index % 8)))) > 0)
    }

    pub fn write_bit(&mut self, value: u32) {
        if value != 0 {
            self.array[self.cursor / 8] |= 1 << (7 - (self.cursor % 8));
        } else {
            self.array[self.cursor / 8] &= !(1 << (7 - (self.cursor % 8)));
        }
        self.cursor += 1;
    }

    pub fn write_bit_array(&mut self, bits: &str) {
        for bit in bits.chars() {
            self.write_bit(bit.to_digit(10).unwrap_or(0));
        }
    }

    pub fn write_uint(&mut self, mut number: u64, bit_length: usize) -> Result<(), String> {
        if bit_length == 0 {
            return Ok(());
        }
        if number != 0 {
            let needed = 64 - number.leading_zeros() as usize;
            if needed > bit_length {
                return Err(format!(
                    "bitLength is too small for number, got number={number}, bitLength={bit_length}"
                ));
            }
        }
        for index in (1..=bit_length).rev() {
            let mask = 1u64 << (index - 1);
            self.write_bit(u32::from(number >= mask));
            if number >= mask {
                number -= mask;
            }
        }
        Ok(())
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        for byte in data {
            self.write_uint(u64::from(*byte), 8)?;
        }
        Ok(())
    }

    /// Returns a top-upped copy of the bit array without mutating `self`.
    pub fn get_top_upped_array(&self) -> Vec<u8> {
        let mut bits = self.clone();
        let mut padding = bits.cursor.div_ceil(8) * 8 - bits.cursor;
        if padding > 0 {
            padding -= 1;
            bits.write_bit(1);
            while padding > 0 {
                padding -= 1;
                bits.write_bit(0);
            }
        }
        let end = bits.cursor.div_ceil(8);
        bits.array[..end].to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub bits: BitString,
    pub refs: Vec<Cell>,
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Cell {
    pub const REACH_BOC_MAGIC_PREFIX: [u8; 4] = [0xB5, 0xEE, 0x9C, 0x72];

    pub fn new() -> Self {
        Self {
            bits: BitString::new(1023),
            refs: Vec::new(),
        }
    }

    pub fn write_cell(&mut self, other: &Cell) {
        for bit_index in 0..other.bits.cursor {
            self.bits.write_bit(u32::from(other.bits.get(bit_index)));
        }
        self.refs.extend(other.refs.iter().cloned());
    }

    pub fn bytes_hash(&self) -> Vec<u8> {
        Sha256::digest(self.bytes_repr()).to_vec()
    }

    pub fn bytes_repr(&self) -> Vec<u8> {
        let mut payload = self.get_data_with_descriptors();
        for reference in &self.refs {
            payload.extend_from_slice(&reference.get_max_depth_as_array());
        }
        for reference in &self.refs {
            payload.extend_from_slice(&reference.bytes_hash());
        }
        payload
    }

    pub fn get_data_with_descriptors(&self) -> Vec<u8> {
        let mut out = self.get_refs_descriptor();
        out.extend_from_slice(&self.get_bits_descriptor());
        out.extend_from_slice(&self.bits.get_top_upped_array());
        out
    }

    pub fn get_bits_descriptor(&self) -> Vec<u8> {
        let cursor = self.bits.cursor;
        vec![(cursor.div_ceil(8) + (cursor / 8)) as u8]
    }

    pub fn get_refs_descriptor(&self) -> Vec<u8> {
        vec![self.refs.len() as u8]
    }

    pub fn get_max_depth(&self) -> usize {
        if self.refs.is_empty() {
            return 0;
        }
        self.refs
            .iter()
            .map(Cell::get_max_depth)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn get_max_depth_as_array(&self) -> Vec<u8> {
        let max_depth = self.get_max_depth();
        vec![(max_depth / 256) as u8, (max_depth % 256) as u8]
    }

    pub fn one_from_boc(serialized_boc: &[u8]) -> Result<Cell, String> {
        let cells = deserialize_boc(serialized_boc)?;
        if cells.len() != 1 {
            return Err("Expected 1 root cell".into());
        }
        Ok(cells.into_iter().next().unwrap())
    }
}

struct RawCell {
    bits: BitString,
    ref_indices: Vec<usize>,
}

fn read_n_bytes_uint_from_array(size_bytes: usize, data: &[u8]) -> usize {
    let mut value = 0usize;
    for byte in data.iter().take(size_bytes) {
        value = (value << 8) + usize::from(*byte);
    }
    value
}

fn crc32c(data: &[u8]) -> [u8; 4] {
    let poly: u32 = 0x82F6_3B78;
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if (crc & 1) != 0 {
                (crc >> 1) ^ poly
            } else {
                crc >> 1
            };
        }
    }
    crc ^= 0xFFFF_FFFF;
    crc.to_le_bytes()
}

pub fn deserialize_boc(serialized_boc: &[u8]) -> Result<Vec<Cell>, String> {
    let input_data = serialized_boc;
    if serialized_boc.len() < 5 {
        return Err("Not enough bytes for BoC header".into());
    }

    if serialized_boc[..4] != Cell::REACH_BOC_MAGIC_PREFIX {
        return Err("Unsupported BoC magic prefix".into());
    }

    let flags_byte = serialized_boc[4];
    let has_idx = (flags_byte & 128) != 0;
    let hash_crc32 = (flags_byte & 64) != 0;
    let size_bytes = usize::from(flags_byte % 8);
    let mut serialized_boc = &serialized_boc[5..];

    if serialized_boc.len() < 1 + 5 * size_bytes {
        return Err("Not enough bytes for encoding cells counters".into());
    }

    let offset_bytes = usize::from(serialized_boc[0]);
    serialized_boc = &serialized_boc[1..];
    let cells_num = read_n_bytes_uint_from_array(size_bytes, serialized_boc);
    serialized_boc = &serialized_boc[size_bytes..];
    let roots_num = read_n_bytes_uint_from_array(size_bytes, serialized_boc);
    serialized_boc = &serialized_boc[size_bytes..];
    let _ = read_n_bytes_uint_from_array(size_bytes, serialized_boc);
    serialized_boc = &serialized_boc[size_bytes..];
    let tot_cells_size = read_n_bytes_uint_from_array(offset_bytes, serialized_boc);
    serialized_boc = &serialized_boc[offset_bytes..];

    let mut root_list = Vec::with_capacity(roots_num);
    for _ in 0..roots_num {
        root_list.push(read_n_bytes_uint_from_array(size_bytes, serialized_boc));
        serialized_boc = &serialized_boc[size_bytes..];
    }

    if has_idx {
        for _ in 0..cells_num {
            let _ = read_n_bytes_uint_from_array(offset_bytes, serialized_boc);
            serialized_boc = &serialized_boc[offset_bytes..];
        }
    }

    if serialized_boc.len() < tot_cells_size {
        return Err("Not enough bytes for cell data".into());
    }
    let mut cells_data = &serialized_boc[..tot_cells_size];
    serialized_boc = &serialized_boc[tot_cells_size..];

    if hash_crc32 {
        if serialized_boc.len() < 4 {
            return Err("Not enough bytes for crc32c hashsum".into());
        }
        if crc32c(&input_data[..input_data.len() - 4]) != serialized_boc[..4] {
            return Err("Crc32c hashsum mismatch".into());
        }
    }

    let mut raw_cells = Vec::with_capacity(cells_num);
    for _ in 0..cells_num {
        let (raw, rest) = deserialize_cell_data(cells_data, size_bytes)?;
        cells_data = rest;
        raw_cells.push(raw);
    }

    let resolved = resolve_raw_cells(raw_cells)?;
    Ok(root_list
        .into_iter()
        .map(|root_index| resolved[root_index].clone())
        .collect())
}

fn resolve_raw_cells(raw_cells: Vec<RawCell>) -> Result<Vec<Cell>, String> {
    let n = raw_cells.len();
    let mut resolved: Vec<Option<Cell>> = vec![None; n];
    for cell_index in (0..n).rev() {
        let raw = &raw_cells[cell_index];
        let mut refs = Vec::with_capacity(raw.ref_indices.len());
        for &reference in &raw.ref_indices {
            if reference < cell_index {
                return Err("Topological order is broken".into());
            }
            let child = resolved[reference]
                .as_ref()
                .ok_or_else(|| "Topological order is broken".to_string())?
                .clone();
            refs.push(child);
        }
        resolved[cell_index] = Some(Cell {
            bits: raw.bits.clone(),
            refs,
        });
    }
    Ok(resolved.into_iter().map(|c| c.unwrap()).collect())
}

fn deserialize_cell_data(
    mut cell_data: &[u8],
    reference_index_size: usize,
) -> Result<(RawCell, &[u8]), String> {
    if cell_data.len() < 2 {
        return Err("Not enough bytes to encode cell descriptors".into());
    }

    let d1 = cell_data[0];
    let d2 = cell_data[1];
    cell_data = &cell_data[2..];
    let ref_num = usize::from(d1 % 8);
    let data_bytes_size = usize::from(d2.div_ceil(2));
    let fullfilled_bytes = d2 % 2 == 0;

    if cell_data.len() < data_bytes_size + reference_index_size * ref_num {
        return Err("Not enough bytes to encode cell data".into());
    }

    let mut bits = BitString::new(data_bytes_size * 8);
    bits.cursor = data_bytes_size * 8;
    bits.array = cell_data[..data_bytes_size].to_vec();

    if !fullfilled_bytes && bits.cursor > 0 {
        let mut found_end_bit = false;
        for _ in 0..7 {
            bits.cursor -= 1;
            if bits.get(bits.cursor) != 0 {
                found_end_bit = true;
                bits.array[bits.cursor / 8] &= !(1 << (7 - (bits.cursor % 8)));
                break;
            }
        }
        if !found_end_bit {
            return Err("Incorrect top-upped array".into());
        }
    }

    cell_data = &cell_data[data_bytes_size..];
    let mut ref_indices = Vec::with_capacity(ref_num);
    for _ in 0..ref_num {
        ref_indices.push(read_n_bytes_uint_from_array(reference_index_size, cell_data));
        cell_data = &cell_data[reference_index_size..];
    }

    Ok((
        RawCell {
            bits,
            ref_indices,
        },
        cell_data,
    ))
}
