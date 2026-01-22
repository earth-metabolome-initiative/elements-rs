//! Isotopes of the element Oganesson
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Oganesson
pub enum OganessonIsotope {
    /// Isotope Og293 of Oganesson
    Og293,
    /// Isotope Og294 of Oganesson
    Og294,
    /// Isotope Og295 of Oganesson
    Og295,
}
impl super::RelativeAtomicMass for OganessonIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Og293 => 293.21356f64,
            Self::Og294 => 294.21392f64,
            Self::Og295 => 295.21624f64,
        }
    }
}
impl super::ElementVariant for OganessonIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Og
    }
}
impl super::MassNumber for OganessonIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Og293 => 293u16,
            Self::Og294 => 294u16,
            Self::Og295 => 295u16,
        }
    }
}
impl super::IsotopicComposition for OganessonIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for OganessonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Og295
    }
}
impl From<OganessonIsotope> for crate::Isotope {
    fn from(isotope: OganessonIsotope) -> Self {
        crate::Isotope::Og(isotope)
    }
}
impl From<OganessonIsotope> for crate::Element {
    fn from(_isotope: OganessonIsotope) -> Self {
        crate::Element::Og
    }
}
impl TryFrom<u64> for OganessonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            293u64 => Ok(Self::Og293),
            294u64 => Ok(Self::Og294),
            295u64 => Ok(Self::Og295),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Og, value)),
        }
    }
}
impl TryFrom<u8> for OganessonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for OganessonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for OganessonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for OganessonIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Og293 => write!(f, "Og293"),
            Self::Og294 => write!(f, "Og294"),
            Self::Og295 => write!(f, "Og295"),
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
        for isotope in OganessonIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in OganessonIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Og, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in OganessonIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in OganessonIsotope::iter() {
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
        let most_abundant = OganessonIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in OganessonIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Og(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in OganessonIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Og);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in OganessonIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = OganessonIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(OganessonIsotope::try_from(0_u16).is_err());
        assert!(OganessonIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in OganessonIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
