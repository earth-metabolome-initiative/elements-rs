//! Isotopes of the element Livermorium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
/// Isotopes of the element Livermorium
pub enum LivermoriumIsotope {
    /// Isotope Lv289 of Livermorium
    Lv289,
    /// Isotope Lv290 of Livermorium
    Lv290,
    /// Isotope Lv291 of Livermorium
    Lv291,
    /// Isotope Lv292 of Livermorium
    Lv292,
    /// Isotope Lv293 of Livermorium
    Lv293,
}
impl super::RelativeAtomicMass for LivermoriumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Lv289 => 289.19816f64,
            Self::Lv290 => 290.19864f64,
            Self::Lv291 => 291.20108f64,
            Self::Lv292 => 292.20174f64,
            Self::Lv293 => 293.20449f64,
        }
    }
}
impl super::ElementVariant for LivermoriumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Lv
    }
}
impl super::MassNumber for LivermoriumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Lv289 => 289u16,
            Self::Lv290 => 290u16,
            Self::Lv291 => 291u16,
            Self::Lv292 => 292u16,
            Self::Lv293 => 293u16,
        }
    }
}
impl super::IsotopicComposition for LivermoriumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for LivermoriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Lv293
    }
}
impl From<LivermoriumIsotope> for crate::Isotope {
    fn from(isotope: LivermoriumIsotope) -> Self {
        crate::Isotope::Lv(isotope)
    }
}
impl From<LivermoriumIsotope> for crate::Element {
    fn from(_isotope: LivermoriumIsotope) -> Self {
        crate::Element::Lv
    }
}
impl TryFrom<u64> for LivermoriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            289u64 => Ok(Self::Lv289),
            290u64 => Ok(Self::Lv290),
            291u64 => Ok(Self::Lv291),
            292u64 => Ok(Self::Lv292),
            293u64 => Ok(Self::Lv293),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Lv, value)),
        }
    }
}
impl TryFrom<u8> for LivermoriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for LivermoriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for LivermoriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for LivermoriumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lv289 => write!(f, "Lv289"),
            Self::Lv290 => write!(f, "Lv290"),
            Self::Lv291 => write!(f, "Lv291"),
            Self::Lv292 => write!(f, "Lv292"),
            Self::Lv293 => write!(f, "Lv293"),
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
        for isotope in LivermoriumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in LivermoriumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Lv, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in LivermoriumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in LivermoriumIsotope::iter() {
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
        let most_abundant = LivermoriumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in LivermoriumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Lv(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in LivermoriumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Lv);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in LivermoriumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = LivermoriumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = LivermoriumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = LivermoriumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(LivermoriumIsotope::try_from(0_u16).is_err());
        assert!(LivermoriumIsotope::try_from(1000_u16).is_err());
        assert!(LivermoriumIsotope::try_from(0_u32).is_err());
        assert!(LivermoriumIsotope::try_from(1000_u32).is_err());
        assert!(LivermoriumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in LivermoriumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
