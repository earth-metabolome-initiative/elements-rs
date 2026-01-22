//! Isotopes of the element Yttrium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Yttrium
pub enum YttriumIsotope {
    /// Isotope Y76 of Yttrium
    Y76,
    /// Isotope Y77 of Yttrium
    Y77,
    /// Isotope Y78 of Yttrium
    Y78,
    /// Isotope Y79 of Yttrium
    Y79,
    /// Isotope Y80 of Yttrium
    Y80,
    /// Isotope Y81 of Yttrium
    Y81,
    /// Isotope Y82 of Yttrium
    Y82,
    /// Isotope Y83 of Yttrium
    Y83,
    /// Isotope Y84 of Yttrium
    Y84,
    /// Isotope Y85 of Yttrium
    Y85,
    /// Isotope Y86 of Yttrium
    Y86,
    /// Isotope Y87 of Yttrium
    Y87,
    /// Isotope Y88 of Yttrium
    Y88,
    /// Isotope Y89 of Yttrium
    Y89,
    /// Isotope Y90 of Yttrium
    Y90,
    /// Isotope Y91 of Yttrium
    Y91,
    /// Isotope Y92 of Yttrium
    Y92,
    /// Isotope Y93 of Yttrium
    Y93,
    /// Isotope Y94 of Yttrium
    Y94,
    /// Isotope Y95 of Yttrium
    Y95,
    /// Isotope Y96 of Yttrium
    Y96,
    /// Isotope Y97 of Yttrium
    Y97,
    /// Isotope Y98 of Yttrium
    Y98,
    /// Isotope Y99 of Yttrium
    Y99,
    /// Isotope Y100 of Yttrium
    Y100,
    /// Isotope Y101 of Yttrium
    Y101,
    /// Isotope Y102 of Yttrium
    Y102,
    /// Isotope Y103 of Yttrium
    Y103,
    /// Isotope Y104 of Yttrium
    Y104,
    /// Isotope Y105 of Yttrium
    Y105,
    /// Isotope Y106 of Yttrium
    Y106,
    /// Isotope Y107 of Yttrium
    Y107,
    /// Isotope Y108 of Yttrium
    Y108,
    /// Isotope Y109 of Yttrium
    Y109,
}
impl super::RelativeAtomicMass for YttriumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Y76 => 75.95856f64,
            Self::Y77 => 76.949781f64,
            Self::Y78 => 77.94361f64,
            Self::Y79 => 78.93735f64,
            Self::Y80 => 79.9343561f64,
            Self::Y81 => 80.9294556f64,
            Self::Y82 => 81.9269314f64,
            Self::Y83 => 82.922485f64,
            Self::Y84 => 83.9206721f64,
            Self::Y85 => 84.916433f64,
            Self::Y86 => 85.914886f64,
            Self::Y87 => 86.9108761f64,
            Self::Y88 => 87.9095016f64,
            Self::Y89 => 88.9058403f64,
            Self::Y90 => 89.9071439f64,
            Self::Y91 => 90.9072974f64,
            Self::Y92 => 91.9089451f64,
            Self::Y93 => 92.909578f64,
            Self::Y94 => 93.9115906f64,
            Self::Y95 => 94.9128161f64,
            Self::Y96 => 95.9158968f64,
            Self::Y97 => 96.9182741f64,
            Self::Y98 => 97.9223821f64,
            Self::Y99 => 98.924148f64,
            Self::Y100 => 99.927715f64,
            Self::Y101 => 100.9301477f64,
            Self::Y102 => 101.9343277f64,
            Self::Y103 => 102.937243f64,
            Self::Y104 => 103.94196f64,
            Self::Y105 => 104.94544f64,
            Self::Y106 => 105.95056f64,
            Self::Y107 => 106.95452f64,
            Self::Y108 => 107.95996f64,
            Self::Y109 => 108.96436f64,
        }
    }
}
impl super::ElementVariant for YttriumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Y
    }
}
impl super::MassNumber for YttriumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Y76 => 76u16,
            Self::Y77 => 77u16,
            Self::Y78 => 78u16,
            Self::Y79 => 79u16,
            Self::Y80 => 80u16,
            Self::Y81 => 81u16,
            Self::Y82 => 82u16,
            Self::Y83 => 83u16,
            Self::Y84 => 84u16,
            Self::Y85 => 85u16,
            Self::Y86 => 86u16,
            Self::Y87 => 87u16,
            Self::Y88 => 88u16,
            Self::Y89 => 89u16,
            Self::Y90 => 90u16,
            Self::Y91 => 91u16,
            Self::Y92 => 92u16,
            Self::Y93 => 93u16,
            Self::Y94 => 94u16,
            Self::Y95 => 95u16,
            Self::Y96 => 96u16,
            Self::Y97 => 97u16,
            Self::Y98 => 98u16,
            Self::Y99 => 99u16,
            Self::Y100 => 100u16,
            Self::Y101 => 101u16,
            Self::Y102 => 102u16,
            Self::Y103 => 103u16,
            Self::Y104 => 104u16,
            Self::Y105 => 105u16,
            Self::Y106 => 106u16,
            Self::Y107 => 107u16,
            Self::Y108 => 108u16,
            Self::Y109 => 109u16,
        }
    }
}
impl super::IsotopicComposition for YttriumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Y89 => Some(1f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for YttriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Y89
    }
}
impl From<YttriumIsotope> for crate::Isotope {
    fn from(isotope: YttriumIsotope) -> Self {
        crate::Isotope::Y(isotope)
    }
}
impl From<YttriumIsotope> for crate::Element {
    fn from(_isotope: YttriumIsotope) -> Self {
        crate::Element::Y
    }
}
impl TryFrom<u64> for YttriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            76u64 => Ok(Self::Y76),
            77u64 => Ok(Self::Y77),
            78u64 => Ok(Self::Y78),
            79u64 => Ok(Self::Y79),
            80u64 => Ok(Self::Y80),
            81u64 => Ok(Self::Y81),
            82u64 => Ok(Self::Y82),
            83u64 => Ok(Self::Y83),
            84u64 => Ok(Self::Y84),
            85u64 => Ok(Self::Y85),
            86u64 => Ok(Self::Y86),
            87u64 => Ok(Self::Y87),
            88u64 => Ok(Self::Y88),
            89u64 => Ok(Self::Y89),
            90u64 => Ok(Self::Y90),
            91u64 => Ok(Self::Y91),
            92u64 => Ok(Self::Y92),
            93u64 => Ok(Self::Y93),
            94u64 => Ok(Self::Y94),
            95u64 => Ok(Self::Y95),
            96u64 => Ok(Self::Y96),
            97u64 => Ok(Self::Y97),
            98u64 => Ok(Self::Y98),
            99u64 => Ok(Self::Y99),
            100u64 => Ok(Self::Y100),
            101u64 => Ok(Self::Y101),
            102u64 => Ok(Self::Y102),
            103u64 => Ok(Self::Y103),
            104u64 => Ok(Self::Y104),
            105u64 => Ok(Self::Y105),
            106u64 => Ok(Self::Y106),
            107u64 => Ok(Self::Y107),
            108u64 => Ok(Self::Y108),
            109u64 => Ok(Self::Y109),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Y, value)),
        }
    }
}
impl TryFrom<u8> for YttriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for YttriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for YttriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for YttriumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Y76 => write!(f, "Y76"),
            Self::Y77 => write!(f, "Y77"),
            Self::Y78 => write!(f, "Y78"),
            Self::Y79 => write!(f, "Y79"),
            Self::Y80 => write!(f, "Y80"),
            Self::Y81 => write!(f, "Y81"),
            Self::Y82 => write!(f, "Y82"),
            Self::Y83 => write!(f, "Y83"),
            Self::Y84 => write!(f, "Y84"),
            Self::Y85 => write!(f, "Y85"),
            Self::Y86 => write!(f, "Y86"),
            Self::Y87 => write!(f, "Y87"),
            Self::Y88 => write!(f, "Y88"),
            Self::Y89 => write!(f, "Y89"),
            Self::Y90 => write!(f, "Y90"),
            Self::Y91 => write!(f, "Y91"),
            Self::Y92 => write!(f, "Y92"),
            Self::Y93 => write!(f, "Y93"),
            Self::Y94 => write!(f, "Y94"),
            Self::Y95 => write!(f, "Y95"),
            Self::Y96 => write!(f, "Y96"),
            Self::Y97 => write!(f, "Y97"),
            Self::Y98 => write!(f, "Y98"),
            Self::Y99 => write!(f, "Y99"),
            Self::Y100 => write!(f, "Y100"),
            Self::Y101 => write!(f, "Y101"),
            Self::Y102 => write!(f, "Y102"),
            Self::Y103 => write!(f, "Y103"),
            Self::Y104 => write!(f, "Y104"),
            Self::Y105 => write!(f, "Y105"),
            Self::Y106 => write!(f, "Y106"),
            Self::Y107 => write!(f, "Y107"),
            Self::Y108 => write!(f, "Y108"),
            Self::Y109 => write!(f, "Y109"),
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
        for isotope in YttriumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in YttriumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Y, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in YttriumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in YttriumIsotope::iter() {
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
        let most_abundant = YttriumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in YttriumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Y(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in YttriumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Y);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in YttriumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = YttriumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(YttriumIsotope::try_from(0_u16).is_err());
        assert!(YttriumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in YttriumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
