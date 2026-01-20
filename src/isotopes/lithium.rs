//! Isotopes of the element Lithium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Lithium
pub enum LithiumIsotope {
    /// Isotope Li3 of Lithium
    Li3,
    /// Isotope Li4 of Lithium
    Li4,
    /// Isotope Li5 of Lithium
    Li5,
    /// Isotope Li6 of Lithium
    Li6,
    /// Isotope Li7 of Lithium
    Li7,
    /// Isotope Li8 of Lithium
    Li8,
    /// Isotope Li9 of Lithium
    Li9,
    /// Isotope Li10 of Lithium
    Li10,
    /// Isotope Li11 of Lithium
    Li11,
    /// Isotope Li12 of Lithium
    Li12,
    /// Isotope Li13 of Lithium
    Li13,
}
impl super::RelativeAtomicMass for LithiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Li3 => 3.0308f64,
            Self::Li4 => 4.02719f64,
            Self::Li5 => 5.012538f64,
            Self::Li6 => 6.0151228874f64,
            Self::Li7 => 7.0160034366f64,
            Self::Li8 => 8.022486246f64,
            Self::Li9 => 9.02679019f64,
            Self::Li10 => 10.035483f64,
            Self::Li11 => 11.04372358f64,
            Self::Li12 => 12.052517f64,
            Self::Li13 => 13.06263f64,
        }
    }
}
impl super::ElementVariant for LithiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Li
    }
}
impl super::MassNumber for LithiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Li3 => 3u16,
            Self::Li4 => 4u16,
            Self::Li5 => 5u16,
            Self::Li6 => 6u16,
            Self::Li7 => 7u16,
            Self::Li8 => 8u16,
            Self::Li9 => 9u16,
            Self::Li10 => 10u16,
            Self::Li11 => 11u16,
            Self::Li12 => 12u16,
            Self::Li13 => 13u16,
        }
    }
}
impl super::IsotopicComposition for LithiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Li6 => Some(0.0759f64),
            Self::Li7 => Some(0.9241f64),
            Self::Li3
            | Self::Li4
            | Self::Li5
            | Self::Li8
            | Self::Li9
            | Self::Li10
            | Self::Li11
            | Self::Li12
            | Self::Li13 => None,
        }
    }
}
impl super::MostAbundantIsotope for LithiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Li7
    }
}
impl From<LithiumIsotope> for crate::Isotope {
    fn from(isotope: LithiumIsotope) -> Self {
        crate::Isotope::Li(isotope)
    }
}
impl From<LithiumIsotope> for crate::Element {
    fn from(_isotope: LithiumIsotope) -> Self {
        crate::Element::Li
    }
}
impl TryFrom<u16> for LithiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            3u16 => Ok(Self::Li3),
            4u16 => Ok(Self::Li4),
            5u16 => Ok(Self::Li5),
            6u16 => Ok(Self::Li6),
            7u16 => Ok(Self::Li7),
            8u16 => Ok(Self::Li8),
            9u16 => Ok(Self::Li9),
            10u16 => Ok(Self::Li10),
            11u16 => Ok(Self::Li11),
            12u16 => Ok(Self::Li12),
            13u16 => Ok(Self::Li13),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Li, value)),
        }
    }
}
impl std::fmt::Display for LithiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Li3 => write!(f, "Li3"),
            Self::Li4 => write!(f, "Li4"),
            Self::Li5 => write!(f, "Li5"),
            Self::Li6 => write!(f, "Li6"),
            Self::Li7 => write!(f, "Li7"),
            Self::Li8 => write!(f, "Li8"),
            Self::Li9 => write!(f, "Li9"),
            Self::Li10 => write!(f, "Li10"),
            Self::Li11 => write!(f, "Li11"),
            Self::Li12 => write!(f, "Li12"),
            Self::Li13 => write!(f, "Li13"),
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
        for isotope in LithiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in LithiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Li, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in LithiumIsotope::iter() {
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
        for isotope in LithiumIsotope::iter() {
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
        let most_abundant = LithiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in LithiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Li(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in LithiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Li);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in LithiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = LithiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(LithiumIsotope::try_from(0).is_err());
        assert!(LithiumIsotope::try_from(1000).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in LithiumIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
