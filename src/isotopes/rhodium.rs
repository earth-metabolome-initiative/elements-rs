//! Isotopes of the element Rhodium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Rhodium
pub enum RhodiumIsotope {
    /// Isotope Rh89 of Rhodium
    Rh89,
    /// Isotope Rh90 of Rhodium
    Rh90,
    /// Isotope Rh91 of Rhodium
    Rh91,
    /// Isotope Rh92 of Rhodium
    Rh92,
    /// Isotope Rh93 of Rhodium
    Rh93,
    /// Isotope Rh94 of Rhodium
    Rh94,
    /// Isotope Rh95 of Rhodium
    Rh95,
    /// Isotope Rh96 of Rhodium
    Rh96,
    /// Isotope Rh97 of Rhodium
    Rh97,
    /// Isotope Rh98 of Rhodium
    Rh98,
    /// Isotope Rh99 of Rhodium
    Rh99,
    /// Isotope Rh100 of Rhodium
    Rh100,
    /// Isotope Rh101 of Rhodium
    Rh101,
    /// Isotope Rh102 of Rhodium
    Rh102,
    /// Isotope Rh103 of Rhodium
    Rh103,
    /// Isotope Rh104 of Rhodium
    Rh104,
    /// Isotope Rh105 of Rhodium
    Rh105,
    /// Isotope Rh106 of Rhodium
    Rh106,
    /// Isotope Rh107 of Rhodium
    Rh107,
    /// Isotope Rh108 of Rhodium
    Rh108,
    /// Isotope Rh109 of Rhodium
    Rh109,
    /// Isotope Rh110 of Rhodium
    Rh110,
    /// Isotope Rh111 of Rhodium
    Rh111,
    /// Isotope Rh112 of Rhodium
    Rh112,
    /// Isotope Rh113 of Rhodium
    Rh113,
    /// Isotope Rh114 of Rhodium
    Rh114,
    /// Isotope Rh115 of Rhodium
    Rh115,
    /// Isotope Rh116 of Rhodium
    Rh116,
    /// Isotope Rh117 of Rhodium
    Rh117,
    /// Isotope Rh118 of Rhodium
    Rh118,
    /// Isotope Rh119 of Rhodium
    Rh119,
    /// Isotope Rh120 of Rhodium
    Rh120,
    /// Isotope Rh121 of Rhodium
    Rh121,
    /// Isotope Rh122 of Rhodium
    Rh122,
    /// Isotope Rh123 of Rhodium
    Rh123,
    /// Isotope Rh124 of Rhodium
    Rh124,
    /// Isotope Rh125 of Rhodium
    Rh125,
    /// Isotope Rh126 of Rhodium
    Rh126,
}
impl super::RelativeAtomicMass for RhodiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rh89 => 88.95058f64,
            Self::Rh90 => 89.94422f64,
            Self::Rh91 => 90.93688f64,
            Self::Rh92 => 91.9323677f64,
            Self::Rh93 => 92.9259128f64,
            Self::Rh94 => 93.9217305f64,
            Self::Rh95 => 94.9158979f64,
            Self::Rh96 => 95.914453f64,
            Self::Rh97 => 96.911329f64,
            Self::Rh98 => 97.910708f64,
            Self::Rh99 => 98.9081282f64,
            Self::Rh100 => 99.908117f64,
            Self::Rh101 => 100.9061606f64,
            Self::Rh102 => 101.9068374f64,
            Self::Rh103 => 102.905498f64,
            Self::Rh104 => 103.9066492f64,
            Self::Rh105 => 104.9056885f64,
            Self::Rh106 => 105.9072868f64,
            Self::Rh107 => 106.906748f64,
            Self::Rh108 => 107.908714f64,
            Self::Rh109 => 108.9087488f64,
            Self::Rh110 => 109.911079f64,
            Self::Rh111 => 110.9116423f64,
            Self::Rh112 => 111.914403f64,
            Self::Rh113 => 112.9154393f64,
            Self::Rh114 => 113.918718f64,
            Self::Rh115 => 114.9203116f64,
            Self::Rh116 => 115.924059f64,
            Self::Rh117 => 116.9260354f64,
            Self::Rh118 => 117.93034f64,
            Self::Rh119 => 118.932557f64,
            Self::Rh120 => 119.93686f64,
            Self::Rh121 => 120.93942f64,
            Self::Rh122 => 121.94399f64,
            Self::Rh123 => 122.94685f64,
            Self::Rh124 => 123.95151f64,
            Self::Rh125 => 124.95469f64,
            Self::Rh126 => 125.95946f64,
        }
    }
}
impl super::ElementVariant for RhodiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Rh
    }
}
impl super::MassNumber for RhodiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rh89 => 89u16,
            Self::Rh90 => 90u16,
            Self::Rh91 => 91u16,
            Self::Rh92 => 92u16,
            Self::Rh93 => 93u16,
            Self::Rh94 => 94u16,
            Self::Rh95 => 95u16,
            Self::Rh96 => 96u16,
            Self::Rh97 => 97u16,
            Self::Rh98 => 98u16,
            Self::Rh99 => 99u16,
            Self::Rh100 => 100u16,
            Self::Rh101 => 101u16,
            Self::Rh102 => 102u16,
            Self::Rh103 => 103u16,
            Self::Rh104 => 104u16,
            Self::Rh105 => 105u16,
            Self::Rh106 => 106u16,
            Self::Rh107 => 107u16,
            Self::Rh108 => 108u16,
            Self::Rh109 => 109u16,
            Self::Rh110 => 110u16,
            Self::Rh111 => 111u16,
            Self::Rh112 => 112u16,
            Self::Rh113 => 113u16,
            Self::Rh114 => 114u16,
            Self::Rh115 => 115u16,
            Self::Rh116 => 116u16,
            Self::Rh117 => 117u16,
            Self::Rh118 => 118u16,
            Self::Rh119 => 119u16,
            Self::Rh120 => 120u16,
            Self::Rh121 => 121u16,
            Self::Rh122 => 122u16,
            Self::Rh123 => 123u16,
            Self::Rh124 => 124u16,
            Self::Rh125 => 125u16,
            Self::Rh126 => 126u16,
        }
    }
}
impl super::IsotopicComposition for RhodiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Rh103 => Some(1f64),
            Self::Rh89
            | Self::Rh90
            | Self::Rh91
            | Self::Rh92
            | Self::Rh93
            | Self::Rh94
            | Self::Rh95
            | Self::Rh96
            | Self::Rh97
            | Self::Rh98
            | Self::Rh99
            | Self::Rh100
            | Self::Rh101
            | Self::Rh102
            | Self::Rh104
            | Self::Rh105
            | Self::Rh106
            | Self::Rh107
            | Self::Rh108
            | Self::Rh109
            | Self::Rh110
            | Self::Rh111
            | Self::Rh112
            | Self::Rh113
            | Self::Rh114
            | Self::Rh115
            | Self::Rh116
            | Self::Rh117
            | Self::Rh118
            | Self::Rh119
            | Self::Rh120
            | Self::Rh121
            | Self::Rh122
            | Self::Rh123
            | Self::Rh124
            | Self::Rh125
            | Self::Rh126 => None,
        }
    }
}
impl super::MostAbundantIsotope for RhodiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rh103
    }
}
impl From<RhodiumIsotope> for crate::Isotope {
    fn from(isotope: RhodiumIsotope) -> Self {
        crate::Isotope::Rh(isotope)
    }
}
impl From<RhodiumIsotope> for crate::Element {
    fn from(_isotope: RhodiumIsotope) -> Self {
        crate::Element::Rh
    }
}
impl TryFrom<u64> for RhodiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            89u64 => Ok(Self::Rh89),
            90u64 => Ok(Self::Rh90),
            91u64 => Ok(Self::Rh91),
            92u64 => Ok(Self::Rh92),
            93u64 => Ok(Self::Rh93),
            94u64 => Ok(Self::Rh94),
            95u64 => Ok(Self::Rh95),
            96u64 => Ok(Self::Rh96),
            97u64 => Ok(Self::Rh97),
            98u64 => Ok(Self::Rh98),
            99u64 => Ok(Self::Rh99),
            100u64 => Ok(Self::Rh100),
            101u64 => Ok(Self::Rh101),
            102u64 => Ok(Self::Rh102),
            103u64 => Ok(Self::Rh103),
            104u64 => Ok(Self::Rh104),
            105u64 => Ok(Self::Rh105),
            106u64 => Ok(Self::Rh106),
            107u64 => Ok(Self::Rh107),
            108u64 => Ok(Self::Rh108),
            109u64 => Ok(Self::Rh109),
            110u64 => Ok(Self::Rh110),
            111u64 => Ok(Self::Rh111),
            112u64 => Ok(Self::Rh112),
            113u64 => Ok(Self::Rh113),
            114u64 => Ok(Self::Rh114),
            115u64 => Ok(Self::Rh115),
            116u64 => Ok(Self::Rh116),
            117u64 => Ok(Self::Rh117),
            118u64 => Ok(Self::Rh118),
            119u64 => Ok(Self::Rh119),
            120u64 => Ok(Self::Rh120),
            121u64 => Ok(Self::Rh121),
            122u64 => Ok(Self::Rh122),
            123u64 => Ok(Self::Rh123),
            124u64 => Ok(Self::Rh124),
            125u64 => Ok(Self::Rh125),
            126u64 => Ok(Self::Rh126),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rh, value)),
        }
    }
}
impl TryFrom<u8> for RhodiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for RhodiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for RhodiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for RhodiumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rh89 => write!(f, "Rh89"),
            Self::Rh90 => write!(f, "Rh90"),
            Self::Rh91 => write!(f, "Rh91"),
            Self::Rh92 => write!(f, "Rh92"),
            Self::Rh93 => write!(f, "Rh93"),
            Self::Rh94 => write!(f, "Rh94"),
            Self::Rh95 => write!(f, "Rh95"),
            Self::Rh96 => write!(f, "Rh96"),
            Self::Rh97 => write!(f, "Rh97"),
            Self::Rh98 => write!(f, "Rh98"),
            Self::Rh99 => write!(f, "Rh99"),
            Self::Rh100 => write!(f, "Rh100"),
            Self::Rh101 => write!(f, "Rh101"),
            Self::Rh102 => write!(f, "Rh102"),
            Self::Rh103 => write!(f, "Rh103"),
            Self::Rh104 => write!(f, "Rh104"),
            Self::Rh105 => write!(f, "Rh105"),
            Self::Rh106 => write!(f, "Rh106"),
            Self::Rh107 => write!(f, "Rh107"),
            Self::Rh108 => write!(f, "Rh108"),
            Self::Rh109 => write!(f, "Rh109"),
            Self::Rh110 => write!(f, "Rh110"),
            Self::Rh111 => write!(f, "Rh111"),
            Self::Rh112 => write!(f, "Rh112"),
            Self::Rh113 => write!(f, "Rh113"),
            Self::Rh114 => write!(f, "Rh114"),
            Self::Rh115 => write!(f, "Rh115"),
            Self::Rh116 => write!(f, "Rh116"),
            Self::Rh117 => write!(f, "Rh117"),
            Self::Rh118 => write!(f, "Rh118"),
            Self::Rh119 => write!(f, "Rh119"),
            Self::Rh120 => write!(f, "Rh120"),
            Self::Rh121 => write!(f, "Rh121"),
            Self::Rh122 => write!(f, "Rh122"),
            Self::Rh123 => write!(f, "Rh123"),
            Self::Rh124 => write!(f, "Rh124"),
            Self::Rh125 => write!(f, "Rh125"),
            Self::Rh126 => write!(f, "Rh126"),
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
        for isotope in RhodiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in RhodiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Rh, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in RhodiumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in RhodiumIsotope::iter() {
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
        let most_abundant = RhodiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in RhodiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Rh(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in RhodiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Rh);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in RhodiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = RhodiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(RhodiumIsotope::try_from(0_u16).is_err());
        assert!(RhodiumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in RhodiumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
