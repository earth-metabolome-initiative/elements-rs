//! Isotopes of the element Caesium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Caesium
pub enum CaesiumIsotope {
    /// Isotope Cs112 of Caesium
    Cs112,
    /// Isotope Cs113 of Caesium
    Cs113,
    /// Isotope Cs114 of Caesium
    Cs114,
    /// Isotope Cs115 of Caesium
    Cs115,
    /// Isotope Cs116 of Caesium
    Cs116,
    /// Isotope Cs117 of Caesium
    Cs117,
    /// Isotope Cs118 of Caesium
    Cs118,
    /// Isotope Cs119 of Caesium
    Cs119,
    /// Isotope Cs120 of Caesium
    Cs120,
    /// Isotope Cs121 of Caesium
    Cs121,
    /// Isotope Cs122 of Caesium
    Cs122,
    /// Isotope Cs123 of Caesium
    Cs123,
    /// Isotope Cs124 of Caesium
    Cs124,
    /// Isotope Cs125 of Caesium
    Cs125,
    /// Isotope Cs126 of Caesium
    Cs126,
    /// Isotope Cs127 of Caesium
    Cs127,
    /// Isotope Cs128 of Caesium
    Cs128,
    /// Isotope Cs129 of Caesium
    Cs129,
    /// Isotope Cs130 of Caesium
    Cs130,
    /// Isotope Cs131 of Caesium
    Cs131,
    /// Isotope Cs132 of Caesium
    Cs132,
    /// Isotope Cs133 of Caesium
    Cs133,
    /// Isotope Cs134 of Caesium
    Cs134,
    /// Isotope Cs135 of Caesium
    Cs135,
    /// Isotope Cs136 of Caesium
    Cs136,
    /// Isotope Cs137 of Caesium
    Cs137,
    /// Isotope Cs138 of Caesium
    Cs138,
    /// Isotope Cs139 of Caesium
    Cs139,
    /// Isotope Cs140 of Caesium
    Cs140,
    /// Isotope Cs141 of Caesium
    Cs141,
    /// Isotope Cs142 of Caesium
    Cs142,
    /// Isotope Cs143 of Caesium
    Cs143,
    /// Isotope Cs144 of Caesium
    Cs144,
    /// Isotope Cs145 of Caesium
    Cs145,
    /// Isotope Cs146 of Caesium
    Cs146,
    /// Isotope Cs147 of Caesium
    Cs147,
    /// Isotope Cs148 of Caesium
    Cs148,
    /// Isotope Cs149 of Caesium
    Cs149,
    /// Isotope Cs150 of Caesium
    Cs150,
    /// Isotope Cs151 of Caesium
    Cs151,
}
impl super::RelativeAtomicMass for CaesiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cs112 => 111.950309f64,
            Self::Cs113 => 112.9444291f64,
            Self::Cs114 => 113.941296f64,
            Self::Cs115 => 114.93591f64,
            Self::Cs116 => 115.93337f64,
            Self::Cs117 => 116.928617f64,
            Self::Cs118 => 117.92656f64,
            Self::Cs119 => 118.922377f64,
            Self::Cs120 => 119.920677f64,
            Self::Cs121 => 120.917227f64,
            Self::Cs122 => 121.916108f64,
            Self::Cs123 => 122.912996f64,
            Self::Cs124 => 123.9122578f64,
            Self::Cs125 => 124.909728f64,
            Self::Cs126 => 125.909446f64,
            Self::Cs127 => 126.9074174f64,
            Self::Cs128 => 127.9077487f64,
            Self::Cs129 => 128.9060657f64,
            Self::Cs130 => 129.9067093f64,
            Self::Cs131 => 130.9054649f64,
            Self::Cs132 => 131.9064339f64,
            Self::Cs133 => 132.905451961f64,
            Self::Cs134 => 133.906718503f64,
            Self::Cs135 => 134.905977f64,
            Self::Cs136 => 135.9073114f64,
            Self::Cs137 => 136.90708923f64,
            Self::Cs138 => 137.9110171f64,
            Self::Cs139 => 138.9133638f64,
            Self::Cs140 => 139.9172831f64,
            Self::Cs141 => 140.9200455f64,
            Self::Cs142 => 141.924296f64,
            Self::Cs143 => 142.927349f64,
            Self::Cs144 => 143.932076f64,
            Self::Cs145 => 144.935527f64,
            Self::Cs146 => 145.940344f64,
            Self::Cs147 => 146.944156f64,
            Self::Cs148 => 147.94923f64,
            Self::Cs149 => 148.95302f64,
            Self::Cs150 => 149.95833f64,
            Self::Cs151 => 150.96258f64,
        }
    }
}
impl super::ElementVariant for CaesiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Cs
    }
}
impl super::MassNumber for CaesiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cs112 => 112u16,
            Self::Cs113 => 113u16,
            Self::Cs114 => 114u16,
            Self::Cs115 => 115u16,
            Self::Cs116 => 116u16,
            Self::Cs117 => 117u16,
            Self::Cs118 => 118u16,
            Self::Cs119 => 119u16,
            Self::Cs120 => 120u16,
            Self::Cs121 => 121u16,
            Self::Cs122 => 122u16,
            Self::Cs123 => 123u16,
            Self::Cs124 => 124u16,
            Self::Cs125 => 125u16,
            Self::Cs126 => 126u16,
            Self::Cs127 => 127u16,
            Self::Cs128 => 128u16,
            Self::Cs129 => 129u16,
            Self::Cs130 => 130u16,
            Self::Cs131 => 131u16,
            Self::Cs132 => 132u16,
            Self::Cs133 => 133u16,
            Self::Cs134 => 134u16,
            Self::Cs135 => 135u16,
            Self::Cs136 => 136u16,
            Self::Cs137 => 137u16,
            Self::Cs138 => 138u16,
            Self::Cs139 => 139u16,
            Self::Cs140 => 140u16,
            Self::Cs141 => 141u16,
            Self::Cs142 => 142u16,
            Self::Cs143 => 143u16,
            Self::Cs144 => 144u16,
            Self::Cs145 => 145u16,
            Self::Cs146 => 146u16,
            Self::Cs147 => 147u16,
            Self::Cs148 => 148u16,
            Self::Cs149 => 149u16,
            Self::Cs150 => 150u16,
            Self::Cs151 => 151u16,
        }
    }
}
impl super::IsotopicComposition for CaesiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cs133 => Some(1f64),
            Self::Cs112
            | Self::Cs113
            | Self::Cs114
            | Self::Cs115
            | Self::Cs116
            | Self::Cs117
            | Self::Cs118
            | Self::Cs119
            | Self::Cs120
            | Self::Cs121
            | Self::Cs122
            | Self::Cs123
            | Self::Cs124
            | Self::Cs125
            | Self::Cs126
            | Self::Cs127
            | Self::Cs128
            | Self::Cs129
            | Self::Cs130
            | Self::Cs131
            | Self::Cs132
            | Self::Cs134
            | Self::Cs135
            | Self::Cs136
            | Self::Cs137
            | Self::Cs138
            | Self::Cs139
            | Self::Cs140
            | Self::Cs141
            | Self::Cs142
            | Self::Cs143
            | Self::Cs144
            | Self::Cs145
            | Self::Cs146
            | Self::Cs147
            | Self::Cs148
            | Self::Cs149
            | Self::Cs150
            | Self::Cs151 => None,
        }
    }
}
impl super::MostAbundantIsotope for CaesiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cs133
    }
}
impl From<CaesiumIsotope> for crate::Isotope {
    fn from(isotope: CaesiumIsotope) -> Self {
        crate::Isotope::Cs(isotope)
    }
}
impl From<CaesiumIsotope> for crate::Element {
    fn from(_isotope: CaesiumIsotope) -> Self {
        crate::Element::Cs
    }
}
impl TryFrom<u64> for CaesiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            112u64 => Ok(Self::Cs112),
            113u64 => Ok(Self::Cs113),
            114u64 => Ok(Self::Cs114),
            115u64 => Ok(Self::Cs115),
            116u64 => Ok(Self::Cs116),
            117u64 => Ok(Self::Cs117),
            118u64 => Ok(Self::Cs118),
            119u64 => Ok(Self::Cs119),
            120u64 => Ok(Self::Cs120),
            121u64 => Ok(Self::Cs121),
            122u64 => Ok(Self::Cs122),
            123u64 => Ok(Self::Cs123),
            124u64 => Ok(Self::Cs124),
            125u64 => Ok(Self::Cs125),
            126u64 => Ok(Self::Cs126),
            127u64 => Ok(Self::Cs127),
            128u64 => Ok(Self::Cs128),
            129u64 => Ok(Self::Cs129),
            130u64 => Ok(Self::Cs130),
            131u64 => Ok(Self::Cs131),
            132u64 => Ok(Self::Cs132),
            133u64 => Ok(Self::Cs133),
            134u64 => Ok(Self::Cs134),
            135u64 => Ok(Self::Cs135),
            136u64 => Ok(Self::Cs136),
            137u64 => Ok(Self::Cs137),
            138u64 => Ok(Self::Cs138),
            139u64 => Ok(Self::Cs139),
            140u64 => Ok(Self::Cs140),
            141u64 => Ok(Self::Cs141),
            142u64 => Ok(Self::Cs142),
            143u64 => Ok(Self::Cs143),
            144u64 => Ok(Self::Cs144),
            145u64 => Ok(Self::Cs145),
            146u64 => Ok(Self::Cs146),
            147u64 => Ok(Self::Cs147),
            148u64 => Ok(Self::Cs148),
            149u64 => Ok(Self::Cs149),
            150u64 => Ok(Self::Cs150),
            151u64 => Ok(Self::Cs151),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cs, value)),
        }
    }
}
impl TryFrom<u8> for CaesiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for CaesiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for CaesiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl std::fmt::Display for CaesiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cs112 => write!(f, "Cs112"),
            Self::Cs113 => write!(f, "Cs113"),
            Self::Cs114 => write!(f, "Cs114"),
            Self::Cs115 => write!(f, "Cs115"),
            Self::Cs116 => write!(f, "Cs116"),
            Self::Cs117 => write!(f, "Cs117"),
            Self::Cs118 => write!(f, "Cs118"),
            Self::Cs119 => write!(f, "Cs119"),
            Self::Cs120 => write!(f, "Cs120"),
            Self::Cs121 => write!(f, "Cs121"),
            Self::Cs122 => write!(f, "Cs122"),
            Self::Cs123 => write!(f, "Cs123"),
            Self::Cs124 => write!(f, "Cs124"),
            Self::Cs125 => write!(f, "Cs125"),
            Self::Cs126 => write!(f, "Cs126"),
            Self::Cs127 => write!(f, "Cs127"),
            Self::Cs128 => write!(f, "Cs128"),
            Self::Cs129 => write!(f, "Cs129"),
            Self::Cs130 => write!(f, "Cs130"),
            Self::Cs131 => write!(f, "Cs131"),
            Self::Cs132 => write!(f, "Cs132"),
            Self::Cs133 => write!(f, "Cs133"),
            Self::Cs134 => write!(f, "Cs134"),
            Self::Cs135 => write!(f, "Cs135"),
            Self::Cs136 => write!(f, "Cs136"),
            Self::Cs137 => write!(f, "Cs137"),
            Self::Cs138 => write!(f, "Cs138"),
            Self::Cs139 => write!(f, "Cs139"),
            Self::Cs140 => write!(f, "Cs140"),
            Self::Cs141 => write!(f, "Cs141"),
            Self::Cs142 => write!(f, "Cs142"),
            Self::Cs143 => write!(f, "Cs143"),
            Self::Cs144 => write!(f, "Cs144"),
            Self::Cs145 => write!(f, "Cs145"),
            Self::Cs146 => write!(f, "Cs146"),
            Self::Cs147 => write!(f, "Cs147"),
            Self::Cs148 => write!(f, "Cs148"),
            Self::Cs149 => write!(f, "Cs149"),
            Self::Cs150 => write!(f, "Cs150"),
            Self::Cs151 => write!(f, "Cs151"),
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
        for isotope in CaesiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in CaesiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Cs, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in CaesiumIsotope::iter() {
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
        for isotope in CaesiumIsotope::iter() {
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
        let most_abundant = CaesiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in CaesiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Cs(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in CaesiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Cs);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in CaesiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = CaesiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(CaesiumIsotope::try_from(0_u16).is_err());
        assert!(CaesiumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in CaesiumIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
