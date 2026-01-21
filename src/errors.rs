//! Error types for element and isotope parsing.

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
