//! Isotopes of the element Vanadium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
/// Isotopes of the element Vanadium
pub enum VanadiumIsotope {
    /// Isotope V40 of Vanadium
    V40,
    /// Isotope V41 of Vanadium
    V41,
    /// Isotope V42 of Vanadium
    V42,
    /// Isotope V43 of Vanadium
    V43,
    /// Isotope V44 of Vanadium
    V44,
    /// Isotope V45 of Vanadium
    V45,
    /// Isotope V46 of Vanadium
    V46,
    /// Isotope V47 of Vanadium
    V47,
    /// Isotope V48 of Vanadium
    V48,
    /// Isotope V49 of Vanadium
    V49,
    /// Isotope V50 of Vanadium
    V50,
    /// Isotope V51 of Vanadium
    V51,
    /// Isotope V52 of Vanadium
    V52,
    /// Isotope V53 of Vanadium
    V53,
    /// Isotope V54 of Vanadium
    V54,
    /// Isotope V55 of Vanadium
    V55,
    /// Isotope V56 of Vanadium
    V56,
    /// Isotope V57 of Vanadium
    V57,
    /// Isotope V58 of Vanadium
    V58,
    /// Isotope V59 of Vanadium
    V59,
    /// Isotope V60 of Vanadium
    V60,
    /// Isotope V61 of Vanadium
    V61,
    /// Isotope V62 of Vanadium
    V62,
    /// Isotope V63 of Vanadium
    V63,
    /// Isotope V64 of Vanadium
    V64,
    /// Isotope V65 of Vanadium
    V65,
    /// Isotope V66 of Vanadium
    V66,
}
impl super::RelativeAtomicMass for VanadiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::V40 => 40.01276f64,
            Self::V41 => 41.00021f64,
            Self::V42 => 41.99182f64,
            Self::V43 => 42.980766f64,
            Self::V44 => 43.97411f64,
            Self::V45 => 44.9657748f64,
            Self::V46 => 45.96019878f64,
            Self::V47 => 46.95490491f64,
            Self::V48 => 47.9522522f64,
            Self::V49 => 48.9485118f64,
            Self::V50 => 49.94715601f64,
            Self::V51 => 50.94395704f64,
            Self::V52 => 51.94477301f64,
            Self::V53 => 52.9443367f64,
            Self::V54 => 53.946439f64,
            Self::V55 => 54.94724f64,
            Self::V56 => 55.95048f64,
            Self::V57 => 56.95252f64,
            Self::V58 => 57.95672f64,
            Self::V59 => 58.95939f64,
            Self::V60 => 59.96431f64,
            Self::V61 => 60.96725f64,
            Self::V62 => 61.97265f64,
            Self::V63 => 62.97639f64,
            Self::V64 => 63.98264f64,
            Self::V65 => 64.9875f64,
            Self::V66 => 65.99398f64,
        }
    }
}
impl super::ElementVariant for VanadiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::V
    }
}
impl super::MassNumber for VanadiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::V40 => 40u16,
            Self::V41 => 41u16,
            Self::V42 => 42u16,
            Self::V43 => 43u16,
            Self::V44 => 44u16,
            Self::V45 => 45u16,
            Self::V46 => 46u16,
            Self::V47 => 47u16,
            Self::V48 => 48u16,
            Self::V49 => 49u16,
            Self::V50 => 50u16,
            Self::V51 => 51u16,
            Self::V52 => 52u16,
            Self::V53 => 53u16,
            Self::V54 => 54u16,
            Self::V55 => 55u16,
            Self::V56 => 56u16,
            Self::V57 => 57u16,
            Self::V58 => 58u16,
            Self::V59 => 59u16,
            Self::V60 => 60u16,
            Self::V61 => 61u16,
            Self::V62 => 62u16,
            Self::V63 => 63u16,
            Self::V64 => 64u16,
            Self::V65 => 65u16,
            Self::V66 => 66u16,
        }
    }
}
impl super::IsotopicComposition for VanadiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::V50 => Some(0.0025f64),
            Self::V51 => Some(0.9975f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for VanadiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::V51
    }
}
impl From<VanadiumIsotope> for crate::Isotope {
    fn from(isotope: VanadiumIsotope) -> Self {
        crate::Isotope::V(isotope)
    }
}
impl From<VanadiumIsotope> for crate::Element {
    fn from(_isotope: VanadiumIsotope) -> Self {
        crate::Element::V
    }
}
impl TryFrom<u64> for VanadiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            40u64 => Ok(Self::V40),
            41u64 => Ok(Self::V41),
            42u64 => Ok(Self::V42),
            43u64 => Ok(Self::V43),
            44u64 => Ok(Self::V44),
            45u64 => Ok(Self::V45),
            46u64 => Ok(Self::V46),
            47u64 => Ok(Self::V47),
            48u64 => Ok(Self::V48),
            49u64 => Ok(Self::V49),
            50u64 => Ok(Self::V50),
            51u64 => Ok(Self::V51),
            52u64 => Ok(Self::V52),
            53u64 => Ok(Self::V53),
            54u64 => Ok(Self::V54),
            55u64 => Ok(Self::V55),
            56u64 => Ok(Self::V56),
            57u64 => Ok(Self::V57),
            58u64 => Ok(Self::V58),
            59u64 => Ok(Self::V59),
            60u64 => Ok(Self::V60),
            61u64 => Ok(Self::V61),
            62u64 => Ok(Self::V62),
            63u64 => Ok(Self::V63),
            64u64 => Ok(Self::V64),
            65u64 => Ok(Self::V65),
            66u64 => Ok(Self::V66),
            _ => Err(crate::errors::Error::Isotope(crate::Element::V, value)),
        }
    }
}
impl TryFrom<u8> for VanadiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for VanadiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for VanadiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for VanadiumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::V40 => write!(f, "V40"),
            Self::V41 => write!(f, "V41"),
            Self::V42 => write!(f, "V42"),
            Self::V43 => write!(f, "V43"),
            Self::V44 => write!(f, "V44"),
            Self::V45 => write!(f, "V45"),
            Self::V46 => write!(f, "V46"),
            Self::V47 => write!(f, "V47"),
            Self::V48 => write!(f, "V48"),
            Self::V49 => write!(f, "V49"),
            Self::V50 => write!(f, "V50"),
            Self::V51 => write!(f, "V51"),
            Self::V52 => write!(f, "V52"),
            Self::V53 => write!(f, "V53"),
            Self::V54 => write!(f, "V54"),
            Self::V55 => write!(f, "V55"),
            Self::V56 => write!(f, "V56"),
            Self::V57 => write!(f, "V57"),
            Self::V58 => write!(f, "V58"),
            Self::V59 => write!(f, "V59"),
            Self::V60 => write!(f, "V60"),
            Self::V61 => write!(f, "V61"),
            Self::V62 => write!(f, "V62"),
            Self::V63 => write!(f, "V63"),
            Self::V64 => write!(f, "V64"),
            Self::V65 => write!(f, "V65"),
            Self::V66 => write!(f, "V66"),
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
        for isotope in VanadiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in VanadiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::V, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in VanadiumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in VanadiumIsotope::iter() {
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
        let most_abundant = VanadiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in VanadiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::V(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in VanadiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::V);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in VanadiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = VanadiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = VanadiumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = VanadiumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(VanadiumIsotope::try_from(0_u16).is_err());
        assert!(VanadiumIsotope::try_from(1000_u16).is_err());
        assert!(VanadiumIsotope::try_from(0_u32).is_err());
        assert!(VanadiumIsotope::try_from(1000_u32).is_err());
        assert!(VanadiumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in VanadiumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
