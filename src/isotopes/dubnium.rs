//! Isotopes of the element Dubnium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Dubnium
pub enum DubniumIsotope {
    /// Isotope Db255 of Dubnium
    Db255,
    /// Isotope Db256 of Dubnium
    Db256,
    /// Isotope Db257 of Dubnium
    Db257,
    /// Isotope Db258 of Dubnium
    Db258,
    /// Isotope Db259 of Dubnium
    Db259,
    /// Isotope Db260 of Dubnium
    Db260,
    /// Isotope Db261 of Dubnium
    Db261,
    /// Isotope Db262 of Dubnium
    Db262,
    /// Isotope Db263 of Dubnium
    Db263,
    /// Isotope Db264 of Dubnium
    Db264,
    /// Isotope Db265 of Dubnium
    Db265,
    /// Isotope Db266 of Dubnium
    Db266,
    /// Isotope Db267 of Dubnium
    Db267,
    /// Isotope Db268 of Dubnium
    Db268,
    /// Isotope Db269 of Dubnium
    Db269,
    /// Isotope Db270 of Dubnium
    Db270,
}
impl super::RelativeAtomicMass for DubniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Db255 => 255.10707f64,
            Self::Db256 => 256.10789f64,
            Self::Db257 => 257.10758f64,
            Self::Db258 => 258.10928f64,
            Self::Db259 => 259.109492f64,
            Self::Db260 => 260.1113f64,
            Self::Db261 => 261.11192f64,
            Self::Db262 => 262.11407f64,
            Self::Db263 => 263.11499f64,
            Self::Db264 => 264.11741f64,
            Self::Db265 => 265.11861f64,
            Self::Db266 => 266.12103f64,
            Self::Db267 => 267.12247f64,
            Self::Db268 => 268.12567f64,
            Self::Db269 => 269.12791f64,
            Self::Db270 => 270.13136f64,
        }
    }
}
impl super::ElementVariant for DubniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Db
    }
}
impl super::MassNumber for DubniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Db255 => 255u16,
            Self::Db256 => 256u16,
            Self::Db257 => 257u16,
            Self::Db258 => 258u16,
            Self::Db259 => 259u16,
            Self::Db260 => 260u16,
            Self::Db261 => 261u16,
            Self::Db262 => 262u16,
            Self::Db263 => 263u16,
            Self::Db264 => 264u16,
            Self::Db265 => 265u16,
            Self::Db266 => 266u16,
            Self::Db267 => 267u16,
            Self::Db268 => 268u16,
            Self::Db269 => 269u16,
            Self::Db270 => 270u16,
        }
    }
}
impl super::IsotopicComposition for DubniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for DubniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Db270
    }
}
impl From<DubniumIsotope> for crate::Isotope {
    fn from(isotope: DubniumIsotope) -> Self {
        crate::Isotope::Db(isotope)
    }
}
impl From<DubniumIsotope> for crate::Element {
    fn from(_isotope: DubniumIsotope) -> Self {
        crate::Element::Db
    }
}
impl TryFrom<u64> for DubniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            255u64 => Ok(Self::Db255),
            256u64 => Ok(Self::Db256),
            257u64 => Ok(Self::Db257),
            258u64 => Ok(Self::Db258),
            259u64 => Ok(Self::Db259),
            260u64 => Ok(Self::Db260),
            261u64 => Ok(Self::Db261),
            262u64 => Ok(Self::Db262),
            263u64 => Ok(Self::Db263),
            264u64 => Ok(Self::Db264),
            265u64 => Ok(Self::Db265),
            266u64 => Ok(Self::Db266),
            267u64 => Ok(Self::Db267),
            268u64 => Ok(Self::Db268),
            269u64 => Ok(Self::Db269),
            270u64 => Ok(Self::Db270),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Db, value)),
        }
    }
}
impl TryFrom<u8> for DubniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for DubniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for DubniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for DubniumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Db255 => write!(f, "Db255"),
            Self::Db256 => write!(f, "Db256"),
            Self::Db257 => write!(f, "Db257"),
            Self::Db258 => write!(f, "Db258"),
            Self::Db259 => write!(f, "Db259"),
            Self::Db260 => write!(f, "Db260"),
            Self::Db261 => write!(f, "Db261"),
            Self::Db262 => write!(f, "Db262"),
            Self::Db263 => write!(f, "Db263"),
            Self::Db264 => write!(f, "Db264"),
            Self::Db265 => write!(f, "Db265"),
            Self::Db266 => write!(f, "Db266"),
            Self::Db267 => write!(f, "Db267"),
            Self::Db268 => write!(f, "Db268"),
            Self::Db269 => write!(f, "Db269"),
            Self::Db270 => write!(f, "Db270"),
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
        for isotope in DubniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in DubniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Db, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in DubniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in DubniumIsotope::iter() {
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
        let most_abundant = DubniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in DubniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Db(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in DubniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Db);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in DubniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = DubniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = DubniumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = DubniumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(DubniumIsotope::try_from(0_u16).is_err());
        assert!(DubniumIsotope::try_from(1000_u16).is_err());
        assert!(DubniumIsotope::try_from(0_u32).is_err());
        assert!(DubniumIsotope::try_from(1000_u32).is_err());
        assert!(DubniumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in DubniumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
