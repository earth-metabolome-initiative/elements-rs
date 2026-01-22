//! Isotopes of the element Beryllium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Beryllium
pub enum BerylliumIsotope {
    /// Isotope Be5 of Beryllium
    Be5,
    /// Isotope Be6 of Beryllium
    Be6,
    /// Isotope Be7 of Beryllium
    Be7,
    /// Isotope Be8 of Beryllium
    Be8,
    /// Isotope Be9 of Beryllium
    Be9,
    /// Isotope Be10 of Beryllium
    Be10,
    /// Isotope Be11 of Beryllium
    Be11,
    /// Isotope Be12 of Beryllium
    Be12,
    /// Isotope Be13 of Beryllium
    Be13,
    /// Isotope Be14 of Beryllium
    Be14,
    /// Isotope Be15 of Beryllium
    Be15,
    /// Isotope Be16 of Beryllium
    Be16,
}
impl super::RelativeAtomicMass for BerylliumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Be5 => 5.0399f64,
            Self::Be6 => 6.0197264f64,
            Self::Be7 => 7.016928717f64,
            Self::Be8 => 8.005305102f64,
            Self::Be9 => 9.012183065f64,
            Self::Be10 => 10.013534695f64,
            Self::Be11 => 11.02166108f64,
            Self::Be12 => 12.0269221f64,
            Self::Be13 => 13.036135f64,
            Self::Be14 => 14.04289f64,
            Self::Be15 => 15.05342f64,
            Self::Be16 => 16.06167f64,
        }
    }
}
impl super::ElementVariant for BerylliumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Be
    }
}
impl super::MassNumber for BerylliumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Be5 => 5u16,
            Self::Be6 => 6u16,
            Self::Be7 => 7u16,
            Self::Be8 => 8u16,
            Self::Be9 => 9u16,
            Self::Be10 => 10u16,
            Self::Be11 => 11u16,
            Self::Be12 => 12u16,
            Self::Be13 => 13u16,
            Self::Be14 => 14u16,
            Self::Be15 => 15u16,
            Self::Be16 => 16u16,
        }
    }
}
impl super::IsotopicComposition for BerylliumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Be9 => Some(1f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for BerylliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Be9
    }
}
impl From<BerylliumIsotope> for crate::Isotope {
    fn from(isotope: BerylliumIsotope) -> Self {
        crate::Isotope::Be(isotope)
    }
}
impl From<BerylliumIsotope> for crate::Element {
    fn from(_isotope: BerylliumIsotope) -> Self {
        crate::Element::Be
    }
}
impl TryFrom<u64> for BerylliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            5u64 => Ok(Self::Be5),
            6u64 => Ok(Self::Be6),
            7u64 => Ok(Self::Be7),
            8u64 => Ok(Self::Be8),
            9u64 => Ok(Self::Be9),
            10u64 => Ok(Self::Be10),
            11u64 => Ok(Self::Be11),
            12u64 => Ok(Self::Be12),
            13u64 => Ok(Self::Be13),
            14u64 => Ok(Self::Be14),
            15u64 => Ok(Self::Be15),
            16u64 => Ok(Self::Be16),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Be, value)),
        }
    }
}
impl TryFrom<u8> for BerylliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for BerylliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for BerylliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for BerylliumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Be5 => write!(f, "Be5"),
            Self::Be6 => write!(f, "Be6"),
            Self::Be7 => write!(f, "Be7"),
            Self::Be8 => write!(f, "Be8"),
            Self::Be9 => write!(f, "Be9"),
            Self::Be10 => write!(f, "Be10"),
            Self::Be11 => write!(f, "Be11"),
            Self::Be12 => write!(f, "Be12"),
            Self::Be13 => write!(f, "Be13"),
            Self::Be14 => write!(f, "Be14"),
            Self::Be15 => write!(f, "Be15"),
            Self::Be16 => write!(f, "Be16"),
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
        for isotope in BerylliumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in BerylliumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Be, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in BerylliumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in BerylliumIsotope::iter() {
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
        let most_abundant = BerylliumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in BerylliumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Be(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in BerylliumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Be);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in BerylliumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = BerylliumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = BerylliumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = BerylliumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(BerylliumIsotope::try_from(0_u16).is_err());
        assert!(BerylliumIsotope::try_from(1000_u16).is_err());
        assert!(BerylliumIsotope::try_from(0_u32).is_err());
        assert!(BerylliumIsotope::try_from(1000_u32).is_err());
        assert!(BerylliumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in BerylliumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
