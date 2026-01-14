//! Isotopes of the element Moscovium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Moscovium
pub enum MoscoviumIsotope {
    /// Isotope Mc287 of Moscovium
    Mc287,
    /// Isotope Mc288 of Moscovium
    Mc288,
    /// Isotope Mc289 of Moscovium
    Mc289,
    /// Isotope Mc290 of Moscovium
    Mc290,
    /// Isotope Mc291 of Moscovium
    Mc291,
}
impl super::RelativeAtomicMass for MoscoviumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mc287 => 287.1907f64,
            Self::Mc288 => 288.19274f64,
            Self::Mc289 => 289.19363f64,
            Self::Mc290 => 290.19598f64,
            Self::Mc291 => 291.19707f64,
        }
    }
}
impl super::ElementVariant for MoscoviumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mc
    }
}
impl super::MassNumber for MoscoviumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mc287 => 287u16,
            Self::Mc288 => 288u16,
            Self::Mc289 => 289u16,
            Self::Mc290 => 290u16,
            Self::Mc291 => 291u16,
        }
    }
}
impl super::IsotopicComposition for MoscoviumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for MoscoviumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mc291
    }
}
impl From<MoscoviumIsotope> for crate::Isotope {
    fn from(isotope: MoscoviumIsotope) -> Self {
        crate::Isotope::Mc(isotope)
    }
}
impl From<MoscoviumIsotope> for crate::Element {
    fn from(_isotope: MoscoviumIsotope) -> Self {
        crate::Element::Mc
    }
}
impl TryFrom<u16> for MoscoviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            287u16 => Ok(Self::Mc287),
            288u16 => Ok(Self::Mc288),
            289u16 => Ok(Self::Mc289),
            290u16 => Ok(Self::Mc290),
            291u16 => Ok(Self::Mc291),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Mc, value)),
        }
    }
}
impl std::fmt::Display for MoscoviumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mc287 => write!(f, "Mc287"),
            Self::Mc288 => write!(f, "Mc288"),
            Self::Mc289 => write!(f, "Mc289"),
            Self::Mc290 => write!(f, "Mc290"),
            Self::Mc291 => write!(f, "Mc291"),
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
        for isotope in MoscoviumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in MoscoviumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Mc, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in MoscoviumIsotope::iter() {
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
        for isotope in MoscoviumIsotope::iter() {
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
        let most_abundant = MoscoviumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in MoscoviumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Mc(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in MoscoviumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Mc);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in MoscoviumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = MoscoviumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
    }
    #[test]
    fn test_display() {
        for isotope in MoscoviumIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
