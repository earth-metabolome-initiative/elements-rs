//! Isotopes of the element Copernicium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
/// Isotopes of the element Copernicium
pub enum CoperniciumIsotope {
    /// Isotope Cn276 of Copernicium
    Cn276,
    /// Isotope Cn277 of Copernicium
    Cn277,
    /// Isotope Cn278 of Copernicium
    Cn278,
    /// Isotope Cn279 of Copernicium
    Cn279,
    /// Isotope Cn280 of Copernicium
    Cn280,
    /// Isotope Cn281 of Copernicium
    Cn281,
    /// Isotope Cn282 of Copernicium
    Cn282,
    /// Isotope Cn283 of Copernicium
    Cn283,
    /// Isotope Cn284 of Copernicium
    Cn284,
    /// Isotope Cn285 of Copernicium
    Cn285,
}
impl super::RelativeAtomicMass for CoperniciumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cn276 => 276.16141f64,
            Self::Cn277 => 277.16364f64,
            Self::Cn278 => 278.16416f64,
            Self::Cn279 => 279.16654f64,
            Self::Cn280 => 280.16715f64,
            Self::Cn281 => 281.16975f64,
            Self::Cn282 => 282.1705f64,
            Self::Cn283 => 283.17327f64,
            Self::Cn284 => 284.17416f64,
            Self::Cn285 => 285.17712f64,
        }
    }
}
impl super::ElementVariant for CoperniciumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Cn
    }
}
impl super::MassNumber for CoperniciumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cn276 => 276u16,
            Self::Cn277 => 277u16,
            Self::Cn278 => 278u16,
            Self::Cn279 => 279u16,
            Self::Cn280 => 280u16,
            Self::Cn281 => 281u16,
            Self::Cn282 => 282u16,
            Self::Cn283 => 283u16,
            Self::Cn284 => 284u16,
            Self::Cn285 => 285u16,
        }
    }
}
impl super::IsotopicComposition for CoperniciumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for CoperniciumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cn285
    }
}
impl From<CoperniciumIsotope> for crate::Isotope {
    fn from(isotope: CoperniciumIsotope) -> Self {
        crate::Isotope::Cn(isotope)
    }
}
impl From<CoperniciumIsotope> for crate::Element {
    fn from(_isotope: CoperniciumIsotope) -> Self {
        crate::Element::Cn
    }
}
impl TryFrom<u64> for CoperniciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            276u64 => Ok(Self::Cn276),
            277u64 => Ok(Self::Cn277),
            278u64 => Ok(Self::Cn278),
            279u64 => Ok(Self::Cn279),
            280u64 => Ok(Self::Cn280),
            281u64 => Ok(Self::Cn281),
            282u64 => Ok(Self::Cn282),
            283u64 => Ok(Self::Cn283),
            284u64 => Ok(Self::Cn284),
            285u64 => Ok(Self::Cn285),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cn, value)),
        }
    }
}
impl TryFrom<u8> for CoperniciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for CoperniciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for CoperniciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for CoperniciumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Cn276 => write!(f, "Cn276"),
            Self::Cn277 => write!(f, "Cn277"),
            Self::Cn278 => write!(f, "Cn278"),
            Self::Cn279 => write!(f, "Cn279"),
            Self::Cn280 => write!(f, "Cn280"),
            Self::Cn281 => write!(f, "Cn281"),
            Self::Cn282 => write!(f, "Cn282"),
            Self::Cn283 => write!(f, "Cn283"),
            Self::Cn284 => write!(f, "Cn284"),
            Self::Cn285 => write!(f, "Cn285"),
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
        for isotope in CoperniciumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in CoperniciumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Cn, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in CoperniciumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in CoperniciumIsotope::iter() {
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
        let most_abundant = CoperniciumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in CoperniciumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Cn(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in CoperniciumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Cn);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in CoperniciumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = CoperniciumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = CoperniciumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = CoperniciumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(CoperniciumIsotope::try_from(0_u16).is_err());
        assert!(CoperniciumIsotope::try_from(1000_u16).is_err());
        assert!(CoperniciumIsotope::try_from(0_u32).is_err());
        assert!(CoperniciumIsotope::try_from(1000_u32).is_err());
        assert!(CoperniciumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in CoperniciumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
