//! Isotopes of the element Helium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Helium
pub enum HeliumIsotope {
    /// Isotope He3 of Helium
    He3,
    /// Isotope He4 of Helium
    He4,
    /// Isotope He5 of Helium
    He5,
    /// Isotope He6 of Helium
    He6,
    /// Isotope He7 of Helium
    He7,
    /// Isotope He8 of Helium
    He8,
    /// Isotope He9 of Helium
    He9,
    /// Isotope He10 of Helium
    He10,
}
impl super::RelativeAtomicMass for HeliumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::He3 => 3.0160293201f64,
            Self::He4 => 4.00260325413f64,
            Self::He5 => 5.012057f64,
            Self::He6 => 6.018885891f64,
            Self::He7 => 7.0279907f64,
            Self::He8 => 8.03393439f64,
            Self::He9 => 9.043946f64,
            Self::He10 => 10.05279f64,
        }
    }
}
impl super::ElementVariant for HeliumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::He
    }
}
impl super::MassNumber for HeliumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::He3 => 3u16,
            Self::He4 => 4u16,
            Self::He5 => 5u16,
            Self::He6 => 6u16,
            Self::He7 => 7u16,
            Self::He8 => 8u16,
            Self::He9 => 9u16,
            Self::He10 => 10u16,
        }
    }
}
impl super::IsotopicComposition for HeliumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::He3 => Some(0.00000134f64),
            Self::He4 => Some(0.99999866f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for HeliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::He4
    }
}
impl From<HeliumIsotope> for crate::Isotope {
    fn from(isotope: HeliumIsotope) -> Self {
        crate::Isotope::He(isotope)
    }
}
impl From<HeliumIsotope> for crate::Element {
    fn from(_isotope: HeliumIsotope) -> Self {
        crate::Element::He
    }
}
impl TryFrom<u64> for HeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            3u64 => Ok(Self::He3),
            4u64 => Ok(Self::He4),
            5u64 => Ok(Self::He5),
            6u64 => Ok(Self::He6),
            7u64 => Ok(Self::He7),
            8u64 => Ok(Self::He8),
            9u64 => Ok(Self::He9),
            10u64 => Ok(Self::He10),
            _ => Err(crate::errors::Error::Isotope(crate::Element::He, value)),
        }
    }
}
impl TryFrom<u8> for HeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for HeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for HeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for HeliumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::He3 => write!(f, "He3"),
            Self::He4 => write!(f, "He4"),
            Self::He5 => write!(f, "He5"),
            Self::He6 => write!(f, "He6"),
            Self::He7 => write!(f, "He7"),
            Self::He8 => write!(f, "He8"),
            Self::He9 => write!(f, "He9"),
            Self::He10 => write!(f, "He10"),
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
        for isotope in HeliumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in HeliumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::He, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in HeliumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in HeliumIsotope::iter() {
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
        let most_abundant = HeliumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in HeliumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::He(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in HeliumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::He);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in HeliumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = HeliumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(HeliumIsotope::try_from(0_u16).is_err());
        assert!(HeliumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in HeliumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
