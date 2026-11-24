//! Error types for element and isotope parsing.

use core::num::ParseIntError;

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "mem_size", derive(mem_dbg::MemSize))]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_size", mem_size(flat))]
/// Errors that can occur when parsing elements or isotopes.
pub enum Error {
    #[error("Unknown element symbol: {0:?}")]
    /// The provided element is unknown.
    Element([char; 2]),
    #[error("Unknown atomic number: {0}")]
    /// The provided atomic number is of an unknown element.
    AtomicNumber(u8),
    #[error("Unknown character isotope: {0}")]
    /// The provided character isotope is unknown.
    CharacterIsotope(char),
    #[error("Unknown isotope for element {0:?} with atomic mass {1}")]
    /// The provided combination of Element and atomic mass is unknown.
    Isotope(Element, u64),
}

/// Errors returned when parsing an isotope symbol.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseIsotopeError {
    /// The symbol has no mass number.
    #[error("missing isotope mass number")]
    MissingMassNumber,
    /// The mass number is invalid or out of range.
    #[error("invalid isotope mass number")]
    InvalidMassNumber(#[from] ParseIntError),
    /// The element symbol or the isotope is unknown.
    #[error(transparent)]
    Unknown(#[from] Error),
}
