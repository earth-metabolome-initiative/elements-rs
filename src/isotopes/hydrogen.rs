//! Isotopes of the element Hydrogen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Hydrogen
pub enum HydrogenIsotope {
    /// Isotope H1 of Hydrogen
    H1,
    /// Isotope D2 of Hydrogen
    D2,
    /// Isotope T3 of Hydrogen
    T3,
    /// Isotope H4 of Hydrogen
    H4,
    /// Isotope H5 of Hydrogen
    H5,
    /// Isotope H6 of Hydrogen
    H6,
    /// Isotope H7 of Hydrogen
    H7,
}
impl super::RelativeAtomicMass for HydrogenIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::H1 => 1.00782503223f64,
            Self::D2 => 2.01410177812f64,
            Self::T3 => 3.0160492779f64,
            Self::H4 => 4.02643f64,
            Self::H5 => 5.035311f64,
            Self::H6 => 6.04496f64,
            Self::H7 => 7.0527f64,
        }
    }
}
impl super::ElementVariant for HydrogenIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::H
    }
}
impl super::MassNumber for HydrogenIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::H1 => 1u16,
            Self::D2 => 2u16,
            Self::T3 => 3u16,
            Self::H4 => 4u16,
            Self::H5 => 5u16,
            Self::H6 => 6u16,
            Self::H7 => 7u16,
        }
    }
}
impl super::IsotopicComposition for HydrogenIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::H1 => Some(0.999885f64),
            Self::D2 => Some(0.000115f64),
            Self::T3 | Self::H4 | Self::H5 | Self::H6 | Self::H7 => None,
        }
    }
}
impl super::MostAbundantIsotope for HydrogenIsotope {
    fn most_abundant_isotope() -> Self {
        Self::H1
    }
}
impl From<HydrogenIsotope> for crate::Isotope {
    fn from(isotope: HydrogenIsotope) -> Self {
        crate::Isotope::H(isotope)
    }
}
impl From<HydrogenIsotope> for crate::Element {
    fn from(_isotope: HydrogenIsotope) -> Self {
        crate::Element::H
    }
}
impl TryFrom<u64> for HydrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1u64 => Ok(Self::H1),
            2u64 => Ok(Self::D2),
            3u64 => Ok(Self::T3),
            4u64 => Ok(Self::H4),
            5u64 => Ok(Self::H5),
            6u64 => Ok(Self::H6),
            7u64 => Ok(Self::H7),
            _ => Err(crate::errors::Error::Isotope(crate::Element::H, value)),
        }
    }
}
impl TryFrom<u8> for HydrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for HydrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for HydrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl std::fmt::Display for HydrogenIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::H1 => write!(f, "H1"),
            Self::D2 => write!(f, "D2"),
            Self::T3 => write!(f, "T3"),
            Self::H4 => write!(f, "H4"),
            Self::H5 => write!(f, "H5"),
            Self::H6 => write!(f, "H6"),
            Self::H7 => write!(f, "H7"),
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
        for isotope in HydrogenIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in HydrogenIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::H, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in HydrogenIsotope::iter() {
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
        for isotope in HydrogenIsotope::iter() {
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
        let most_abundant = HydrogenIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in HydrogenIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::H(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in HydrogenIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::H);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in HydrogenIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = HydrogenIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(HydrogenIsotope::try_from(0_u16).is_err());
        assert!(HydrogenIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in HydrogenIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
