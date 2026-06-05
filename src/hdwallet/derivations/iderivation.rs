use std::fmt;

use crate::hdwallet::exceptions::DerivationError;
use crate::hdwallet::utils::indexes_to_path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivationStep {
    Index { index: u32, hardened: bool },
    Range { from: u32, to: u32, hardened: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationData {
    path: String,
    indexes: Vec<u32>,
    derivations: Vec<DerivationStep>,
}

impl Default for DerivationData {
    fn default() -> Self {
        Self {
            path: "m/".to_string(),
            indexes: Vec::new(),
            derivations: Vec::new(),
        }
    }
}

impl DerivationData {
    pub fn new(path: Option<&str>, indexes: Option<&[u32]>) -> Result<Self, DerivationError> {
        let (path, indexes, derivations) = normalize_derivation(path, indexes)?;
        Ok(Self {
            path,
            indexes,
            derivations,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn indexes(&self) -> &[u32] {
        &self.indexes
    }

    pub fn derivations(&self) -> &[DerivationStep] {
        &self.derivations
    }

    pub fn depth(&self) -> usize {
        self.derivations.len()
    }

    pub fn clean(&mut self) -> Result<(), DerivationError> {
        *self = Self::new(None, None)?;
        Ok(())
    }
}

impl fmt::Display for DerivationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)
    }
}

/// Common derivation interface (mirrors Python `IDerivation`).
pub trait IDerivation {
    fn name() -> &'static str
    where
        Self: Sized;

    fn data(&self) -> &DerivationData;

    fn data_mut(&mut self) -> &mut DerivationData;

    fn path(&self) -> &str {
        self.data().path()
    }

    fn indexes(&self) -> &[u32] {
        self.data().indexes()
    }

    fn derivations(&self) -> &[DerivationStep] {
        self.data().derivations()
    }

    fn depth(&self) -> usize {
        self.data().depth()
    }

    fn clean(&mut self) -> Result<(), DerivationError> {
        self.data_mut().clean()
    }

    fn purpose(&self) -> Option<u32> {
        None
    }

    fn coin_type(&self) -> Option<u32> {
        None
    }

    fn account(&self) -> Option<u32> {
        None
    }

    fn change(&self) -> Option<ChangeValue> {
        None
    }

    fn role(&self) -> Option<&str> {
        None
    }

    fn address(&self) -> Option<u32> {
        None
    }

    fn minor(&self) -> Option<u32> {
        None
    }

    fn major(&self) -> Option<u32> {
        None
    }
}

impl fmt::Display for dyn IDerivation + '_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.path())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeValue {
    Index(u32),
    Label(&'static str),
}

pub fn normalize_derivation(
    path: Option<&str>,
    indexes: Option<&[u32]>,
) -> Result<(String, Vec<u32>, Vec<DerivationStep>), DerivationError> {
    let mut out_path = String::from("m");
    let mut out_indexes = Vec::new();
    let mut out_derivations = Vec::new();

    let path = if let Some(indexes) = indexes {
        Some(indexes_to_path(indexes))
    } else {
        path.map(str::to_string)
    };

    let path = match path {
        None => return Ok(("m/".to_string(), out_indexes, out_derivations)),
        Some(p) if p == "m" || p == "m/" => return Ok(("m/".to_string(), out_indexes, out_derivations)),
        Some(p) if !p.starts_with("m/") => {
            return Err(DerivationError::new(
                "Bad path format",
                None,
                Some("like this type of path \"m/0'/0\""),
                Some(&p),
            ));
        }
        Some(p) => p,
    };

    for (depth, segment) in path.trim_start_matches("m/").split('/').enumerate() {
        if segment.is_empty() {
            continue;
        }

        if segment.contains('\'') {
            parse_hardened_segment(segment, depth, &mut out_path, &mut out_indexes, &mut out_derivations)?;
        } else {
            parse_normal_segment(segment, depth, &mut out_path, &mut out_indexes, &mut out_derivations)?;
        }
    }

    Ok((out_path, out_indexes, out_derivations))
}

fn parse_hardened_segment(
    segment: &str,
    depth: usize,
    out_path: &mut String,
    out_indexes: &mut Vec<u32>,
    out_derivations: &mut Vec<DerivationStep>,
) -> Result<(), DerivationError> {
    if segment.contains('-') {
        let body = segment.trim_end_matches('\'');
        let (from_str, to_str) = body.split_once('-').ok_or_else(|| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        let from_index: u32 = from_str.parse().map_err(|_| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        let to_index: u32 = to_str.parse().map_err(|_| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        if from_index >= to_index {
            return Err(DerivationError::new(
                &format!(
                    "On {depth} depth, the starting {from_index} must be less than the ending {to_index} index"
                ),
                None,
                None,
                None,
            ));
        }
        out_derivations.push(DerivationStep::Range {
            from: from_index,
            to: to_index,
            hardened: true,
        });
        out_indexes.push(to_index + 0x8000_0000);
        out_path.push_str(&format!("/{to_index}'"));
    } else {
        let index: u32 = segment
            .trim_end_matches('\'')
            .parse()
            .map_err(|_| DerivationError::new("Bad path format", None, None, Some(segment)))?;
        out_derivations.push(DerivationStep::Index {
            index,
            hardened: true,
        });
        out_indexes.push(index + 0x8000_0000);
        out_path.push_str(&format!("/{index}'"));
    }
    Ok(())
}

fn parse_normal_segment(
    segment: &str,
    depth: usize,
    out_path: &mut String,
    out_indexes: &mut Vec<u32>,
    out_derivations: &mut Vec<DerivationStep>,
) -> Result<(), DerivationError> {
    if segment.contains('-') {
        let (from_str, to_str) = segment.split_once('-').ok_or_else(|| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        let from_index: u32 = from_str.parse().map_err(|_| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        let to_index: u32 = to_str.parse().map_err(|_| {
            DerivationError::new("Bad path format", None, None, Some(segment))
        })?;
        if from_index >= to_index {
            return Err(DerivationError::new(
                &format!(
                    "On {depth} depth, the starting {from_index} must be less than the ending {to_index} index"
                ),
                None,
                None,
                None,
            ));
        }
        out_derivations.push(DerivationStep::Range {
            from: from_index,
            to: to_index,
            hardened: false,
        });
        out_indexes.push(to_index);
        out_path.push_str(&format!("/{to_index}"));
    } else {
        let index: u32 = segment
            .parse()
            .map_err(|_| DerivationError::new("Bad path format", None, None, Some(segment)))?;
        out_derivations.push(DerivationStep::Index {
            index,
            hardened: false,
        });
        out_indexes.push(index);
        out_path.push_str(&format!("/{index}"));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizedIndex {
    Single { index: u32, hardened: bool },
    Range { from: u32, to: u32, hardened: bool },
}

#[derive(Debug, Clone, Copy)]
pub enum IndexInput<'a> {
    Int(u32),
    Str(&'a str),
    Range(u32, u32),
}

impl From<u32> for IndexInput<'_> {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}

impl<'a> From<&'a str> for IndexInput<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<(u32, u32)> for IndexInput<'_> {
    fn from(value: (u32, u32)) -> Self {
        Self::Range(value.0, value.1)
    }
}

/// Change argument for BIP-44-style derivations.
#[derive(Debug, Clone, Copy)]
pub enum ChangeInput<'a> {
    Str(&'a str),
    Int(u32),
}

impl<'a> From<&'a str> for ChangeInput<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<u32> for ChangeInput<'_> {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}

impl NormalizedIndex {
    pub fn primary(&self) -> u32 {
        match self {
            Self::Single { index, .. } => *index,
            Self::Range { to, .. } => *to,
        }
    }

    pub fn display_value(&self) -> u32 {
        match self {
            Self::Single { index, .. } => *index,
            Self::Range { to, .. } => *to,
        }
    }
}

pub fn normalize_index(
    index: IndexInput<'_>,
    hardened: bool,
) -> Result<NormalizedIndex, DerivationError> {
    match index {
        IndexInput::Range(from, to) => normalize_index_range(from, to, hardened),
        IndexInput::Int(value) => normalize_index_int(value, hardened),
        IndexInput::Str(value) => normalize_index_str(value, hardened),
    }
}

fn normalize_index_int(index: u32, hardened: bool) -> Result<NormalizedIndex, DerivationError> {
    Ok(NormalizedIndex::Single { index, hardened })
}

fn normalize_index_range(
    from: u32,
    to: u32,
    hardened: bool,
) -> Result<NormalizedIndex, DerivationError> {
    if from > to {
        return Err(DerivationError::new(
            &format!("Bad index, from {from} index should be less than to {to} index"),
            None,
            None,
            None,
        ));
    }
    Ok(NormalizedIndex::Range { from, to, hardened })
}

fn normalize_index_str(index: &str, hardened: bool) -> Result<NormalizedIndex, DerivationError> {
    if let Some((from_str, to_str)) = index.split_once('-') {
        let from_index: u32 = from_str.parse().map_err(|_| {
            DerivationError::new(
                "Bad index format",
                None,
                Some("{non-negative-number} | {number}-{number}"),
                Some(index),
            )
        })?;
        let to_index: u32 = to_str.parse().map_err(|_| {
            DerivationError::new(
                "Bad index format",
                None,
                Some("{non-negative-number} | {number}-{number}"),
                Some(index),
            )
        })?;
        if from_index > to_index {
            return Err(DerivationError::new(
                &format!(
                    "Bad index, from {from_index} index should be less than to {to_index} index"
                ),
                None,
                None,
                None,
            ));
        }
        return Ok(NormalizedIndex::Range {
            from: from_index,
            to: to_index,
            hardened,
        });
    }

    let value: u32 = index.parse().map_err(|_| {
        DerivationError::new(
            "Bad index format",
            None,
            Some("{non-negative-number} | {number}-{number}"),
            Some(index),
        )
    })?;
    Ok(NormalizedIndex::Single {
        index: value,
        hardened,
    })
}

pub fn index_tuple_to_string(index: NormalizedIndex) -> String {
    match index {
        NormalizedIndex::Single { index, hardened } => {
            if hardened {
                format!("{index}'")
            } else {
                index.to_string()
            }
        }
        NormalizedIndex::Range { from, to, hardened } => {
            if hardened {
                format!("{from}-{to}'")
            } else {
                format!("{from}-{to}")
            }
        }
    }
}
