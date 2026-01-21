//! Isotopes of the element Nihonium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Nihonium
pub enum NihoniumIsotope {
    /// Isotope Nh278 of Nihonium
    Nh278,
    /// Isotope Nh279 of Nihonium
    Nh279,
    /// Isotope Nh280 of Nihonium
    Nh280,
    /// Isotope Nh281 of Nihonium
    Nh281,
    /// Isotope Nh282 of Nihonium
    Nh282,
    /// Isotope Nh283 of Nihonium
    Nh283,
    /// Isotope Nh284 of Nihonium
    Nh284,
    /// Isotope Nh285 of Nihonium
    Nh285,
    /// Isotope Nh286 of Nihonium
    Nh286,
    /// Isotope Nh287 of Nihonium
    Nh287,
}
impl super::RelativeAtomicMass for NihoniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Nh278 => 278.17058f64,
            Self::Nh279 => 279.17095f64,
            Self::Nh280 => 280.17293f64,
            Self::Nh281 => 281.17348f64,
            Self::Nh282 => 282.17567f64,
            Self::Nh283 => 283.17657f64,
            Self::Nh284 => 284.17873f64,
            Self::Nh285 => 285.17973f64,
            Self::Nh286 => 286.18221f64,
            Self::Nh287 => 287.18339f64,
        }
    }
}
impl super::ElementVariant for NihoniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Nh
    }
}
impl super::MassNumber for NihoniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Nh278 => 278u16,
            Self::Nh279 => 279u16,
            Self::Nh280 => 280u16,
            Self::Nh281 => 281u16,
            Self::Nh282 => 282u16,
            Self::Nh283 => 283u16,
            Self::Nh284 => 284u16,
            Self::Nh285 => 285u16,
            Self::Nh286 => 286u16,
            Self::Nh287 => 287u16,
        }
    }
}
impl super::IsotopicComposition for NihoniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for NihoniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Nh287
    }
}
impl From<NihoniumIsotope> for crate::Isotope {
    fn from(isotope: NihoniumIsotope) -> Self {
        crate::Isotope::Nh(isotope)
    }
}
impl From<NihoniumIsotope> for crate::Element {
    fn from(_isotope: NihoniumIsotope) -> Self {
        crate::Element::Nh
    }
}
impl TryFrom<u64> for NihoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            278u64 => Ok(Self::Nh278),
            279u64 => Ok(Self::Nh279),
            280u64 => Ok(Self::Nh280),
            281u64 => Ok(Self::Nh281),
            282u64 => Ok(Self::Nh282),
            283u64 => Ok(Self::Nh283),
            284u64 => Ok(Self::Nh284),
            285u64 => Ok(Self::Nh285),
            286u64 => Ok(Self::Nh286),
            287u64 => Ok(Self::Nh287),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Nh, value)),
        }
    }
}
impl TryFrom<u8> for NihoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for NihoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for NihoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl std::fmt::Display for NihoniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nh278 => write!(f, "Nh278"),
            Self::Nh279 => write!(f, "Nh279"),
            Self::Nh280 => write!(f, "Nh280"),
            Self::Nh281 => write!(f, "Nh281"),
            Self::Nh282 => write!(f, "Nh282"),
            Self::Nh283 => write!(f, "Nh283"),
            Self::Nh284 => write!(f, "Nh284"),
            Self::Nh285 => write!(f, "Nh285"),
            Self::Nh286 => write!(f, "Nh286"),
            Self::Nh287 => write!(f, "Nh287"),
        }
    }
}
#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;
    use crate::isotopes::{
        ElementVariant, IsotopicComposition, MassNumber, MostAbundantIsotope, RelativeAtomicMass,
    };
    #[test]
    fn test_relative_atomic_mass() {
        for isotope in NihoniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in NihoniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Nh, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in NihoniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {:?}",
                isotope
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in NihoniumIsotope::iter() {
            let comp = isotope.isotopic_composition();
            if let Some(c) = comp {
                assert!(
                    (0.0..=1.0).contains(&c),
                    "Composition should be between 0 and 1 for {:?}",
                    isotope
                );
            }
        }
    }
    #[test]
    fn test_most_abundant() {
        let most_abundant = NihoniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in NihoniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Nh(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in NihoniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Nh);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in NihoniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = NihoniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(NihoniumIsotope::try_from(0_u16).is_err());
        assert!(NihoniumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in NihoniumIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
