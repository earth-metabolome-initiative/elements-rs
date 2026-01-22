//! Isotopes of the element Carbon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Carbon
pub enum CarbonIsotope {
    /// Isotope C8 of Carbon
    C8,
    /// Isotope C9 of Carbon
    C9,
    /// Isotope C10 of Carbon
    C10,
    /// Isotope C11 of Carbon
    C11,
    /// Isotope C12 of Carbon
    C12,
    /// Isotope C13 of Carbon
    C13,
    /// Isotope C14 of Carbon
    C14,
    /// Isotope C15 of Carbon
    C15,
    /// Isotope C16 of Carbon
    C16,
    /// Isotope C17 of Carbon
    C17,
    /// Isotope C18 of Carbon
    C18,
    /// Isotope C19 of Carbon
    C19,
    /// Isotope C20 of Carbon
    C20,
    /// Isotope C21 of Carbon
    C21,
    /// Isotope C22 of Carbon
    C22,
    /// Isotope C23 of Carbon
    C23,
}
impl super::RelativeAtomicMass for CarbonIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::C8 => 8.037643f64,
            Self::C9 => 9.0310372f64,
            Self::C10 => 10.01685331f64,
            Self::C11 => 11.0114336f64,
            Self::C12 => 12f64,
            Self::C13 => 13.00335483507f64,
            Self::C14 => 14.0032419884f64,
            Self::C15 => 15.01059926f64,
            Self::C16 => 16.0147013f64,
            Self::C17 => 17.022577f64,
            Self::C18 => 18.026751f64,
            Self::C19 => 19.0348f64,
            Self::C20 => 20.04032f64,
            Self::C21 => 21.049f64,
            Self::C22 => 22.05753f64,
            Self::C23 => 23.0689f64,
        }
    }
}
impl super::ElementVariant for CarbonIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::C
    }
}
impl super::MassNumber for CarbonIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::C8 => 8u16,
            Self::C9 => 9u16,
            Self::C10 => 10u16,
            Self::C11 => 11u16,
            Self::C12 => 12u16,
            Self::C13 => 13u16,
            Self::C14 => 14u16,
            Self::C15 => 15u16,
            Self::C16 => 16u16,
            Self::C17 => 17u16,
            Self::C18 => 18u16,
            Self::C19 => 19u16,
            Self::C20 => 20u16,
            Self::C21 => 21u16,
            Self::C22 => 22u16,
            Self::C23 => 23u16,
        }
    }
}
impl super::IsotopicComposition for CarbonIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::C12 => Some(0.9893f64),
            Self::C13 => Some(0.0107f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for CarbonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::C12
    }
}
impl From<CarbonIsotope> for crate::Isotope {
    fn from(isotope: CarbonIsotope) -> Self {
        crate::Isotope::C(isotope)
    }
}
impl From<CarbonIsotope> for crate::Element {
    fn from(_isotope: CarbonIsotope) -> Self {
        crate::Element::C
    }
}
impl TryFrom<u64> for CarbonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            8u64 => Ok(Self::C8),
            9u64 => Ok(Self::C9),
            10u64 => Ok(Self::C10),
            11u64 => Ok(Self::C11),
            12u64 => Ok(Self::C12),
            13u64 => Ok(Self::C13),
            14u64 => Ok(Self::C14),
            15u64 => Ok(Self::C15),
            16u64 => Ok(Self::C16),
            17u64 => Ok(Self::C17),
            18u64 => Ok(Self::C18),
            19u64 => Ok(Self::C19),
            20u64 => Ok(Self::C20),
            21u64 => Ok(Self::C21),
            22u64 => Ok(Self::C22),
            23u64 => Ok(Self::C23),
            _ => Err(crate::errors::Error::Isotope(crate::Element::C, value)),
        }
    }
}
impl TryFrom<u8> for CarbonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for CarbonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for CarbonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for CarbonIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::C8 => write!(f, "C8"),
            Self::C9 => write!(f, "C9"),
            Self::C10 => write!(f, "C10"),
            Self::C11 => write!(f, "C11"),
            Self::C12 => write!(f, "C12"),
            Self::C13 => write!(f, "C13"),
            Self::C14 => write!(f, "C14"),
            Self::C15 => write!(f, "C15"),
            Self::C16 => write!(f, "C16"),
            Self::C17 => write!(f, "C17"),
            Self::C18 => write!(f, "C18"),
            Self::C19 => write!(f, "C19"),
            Self::C20 => write!(f, "C20"),
            Self::C21 => write!(f, "C21"),
            Self::C22 => write!(f, "C22"),
            Self::C23 => write!(f, "C23"),
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
        for isotope in CarbonIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in CarbonIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::C, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in CarbonIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in CarbonIsotope::iter() {
            let comp = isotope.isotopic_composition();
            if let Some(c) = comp {
                assert!(
                    (0.0..=1.0).contains(&c),
                    "Composition should be between 0 and 1 for {isotope:?}"
                );
            }
        }
    }
    #[test]
    fn test_most_abundant() {
        let most_abundant = CarbonIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in CarbonIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::C(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in CarbonIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::C);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in CarbonIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = CarbonIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(CarbonIsotope::try_from(0_u16).is_err());
        assert!(CarbonIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in CarbonIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
