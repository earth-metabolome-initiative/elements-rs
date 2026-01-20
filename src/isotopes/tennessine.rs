//! Isotopes of the element Tennessine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Tennessine
pub enum TennessineIsotope {
    /// Isotope Ts291 of Tennessine
    Ts291,
    /// Isotope Ts292 of Tennessine
    Ts292,
    /// Isotope Ts293 of Tennessine
    Ts293,
    /// Isotope Ts294 of Tennessine
    Ts294,
}
impl super::RelativeAtomicMass for TennessineIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ts291 => 291.20553f64,
            Self::Ts292 => 292.20746f64,
            Self::Ts293 => 293.20824f64,
            Self::Ts294 => 294.21046f64,
        }
    }
}
impl super::ElementVariant for TennessineIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Ts
    }
}
impl super::MassNumber for TennessineIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ts291 => 291u16,
            Self::Ts292 => 292u16,
            Self::Ts293 => 293u16,
            Self::Ts294 => 294u16,
        }
    }
}
impl super::IsotopicComposition for TennessineIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for TennessineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ts294
    }
}
impl From<TennessineIsotope> for crate::Isotope {
    fn from(isotope: TennessineIsotope) -> Self {
        crate::Isotope::Ts(isotope)
    }
}
impl From<TennessineIsotope> for crate::Element {
    fn from(_isotope: TennessineIsotope) -> Self {
        crate::Element::Ts
    }
}
impl TryFrom<u16> for TennessineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            291u16 => Ok(Self::Ts291),
            292u16 => Ok(Self::Ts292),
            293u16 => Ok(Self::Ts293),
            294u16 => Ok(Self::Ts294),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ts, value)),
        }
    }
}
impl std::fmt::Display for TennessineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ts291 => write!(f, "Ts291"),
            Self::Ts292 => write!(f, "Ts292"),
            Self::Ts293 => write!(f, "Ts293"),
            Self::Ts294 => write!(f, "Ts294"),
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
        for isotope in TennessineIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in TennessineIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Ts, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in TennessineIsotope::iter() {
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
        for isotope in TennessineIsotope::iter() {
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
        let most_abundant = TennessineIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in TennessineIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Ts(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in TennessineIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Ts);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in TennessineIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = TennessineIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(TennessineIsotope::try_from(0).is_err());
        assert!(TennessineIsotope::try_from(1000).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in TennessineIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
