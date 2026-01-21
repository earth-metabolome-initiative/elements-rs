//! Isotopes of the element Praseodymium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Praseodymium
pub enum PraseodymiumIsotope {
    /// Isotope Pr121 of Praseodymium
    Pr121,
    /// Isotope Pr122 of Praseodymium
    Pr122,
    /// Isotope Pr123 of Praseodymium
    Pr123,
    /// Isotope Pr124 of Praseodymium
    Pr124,
    /// Isotope Pr125 of Praseodymium
    Pr125,
    /// Isotope Pr126 of Praseodymium
    Pr126,
    /// Isotope Pr127 of Praseodymium
    Pr127,
    /// Isotope Pr128 of Praseodymium
    Pr128,
    /// Isotope Pr129 of Praseodymium
    Pr129,
    /// Isotope Pr130 of Praseodymium
    Pr130,
    /// Isotope Pr131 of Praseodymium
    Pr131,
    /// Isotope Pr132 of Praseodymium
    Pr132,
    /// Isotope Pr133 of Praseodymium
    Pr133,
    /// Isotope Pr134 of Praseodymium
    Pr134,
    /// Isotope Pr135 of Praseodymium
    Pr135,
    /// Isotope Pr136 of Praseodymium
    Pr136,
    /// Isotope Pr137 of Praseodymium
    Pr137,
    /// Isotope Pr138 of Praseodymium
    Pr138,
    /// Isotope Pr139 of Praseodymium
    Pr139,
    /// Isotope Pr140 of Praseodymium
    Pr140,
    /// Isotope Pr141 of Praseodymium
    Pr141,
    /// Isotope Pr142 of Praseodymium
    Pr142,
    /// Isotope Pr143 of Praseodymium
    Pr143,
    /// Isotope Pr144 of Praseodymium
    Pr144,
    /// Isotope Pr145 of Praseodymium
    Pr145,
    /// Isotope Pr146 of Praseodymium
    Pr146,
    /// Isotope Pr147 of Praseodymium
    Pr147,
    /// Isotope Pr148 of Praseodymium
    Pr148,
    /// Isotope Pr149 of Praseodymium
    Pr149,
    /// Isotope Pr150 of Praseodymium
    Pr150,
    /// Isotope Pr151 of Praseodymium
    Pr151,
    /// Isotope Pr152 of Praseodymium
    Pr152,
    /// Isotope Pr153 of Praseodymium
    Pr153,
    /// Isotope Pr154 of Praseodymium
    Pr154,
    /// Isotope Pr155 of Praseodymium
    Pr155,
    /// Isotope Pr156 of Praseodymium
    Pr156,
    /// Isotope Pr157 of Praseodymium
    Pr157,
    /// Isotope Pr158 of Praseodymium
    Pr158,
    /// Isotope Pr159 of Praseodymium
    Pr159,
}
impl super::RelativeAtomicMass for PraseodymiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pr121 => 120.95532f64,
            Self::Pr122 => 121.95175f64,
            Self::Pr123 => 122.94596f64,
            Self::Pr124 => 123.94294f64,
            Self::Pr125 => 124.9377f64,
            Self::Pr126 => 125.93524f64,
            Self::Pr127 => 126.93071f64,
            Self::Pr128 => 127.928791f64,
            Self::Pr129 => 128.925095f64,
            Self::Pr130 => 129.92359f64,
            Self::Pr131 => 130.920235f64,
            Self::Pr132 => 131.919255f64,
            Self::Pr133 => 132.916331f64,
            Self::Pr134 => 133.915697f64,
            Self::Pr135 => 134.913112f64,
            Self::Pr136 => 135.912677f64,
            Self::Pr137 => 136.9106792f64,
            Self::Pr138 => 137.910754f64,
            Self::Pr139 => 138.9089408f64,
            Self::Pr140 => 139.9090803f64,
            Self::Pr141 => 140.9076576f64,
            Self::Pr142 => 141.9100496f64,
            Self::Pr143 => 142.9108228f64,
            Self::Pr144 => 143.9133109f64,
            Self::Pr145 => 144.9145182f64,
            Self::Pr146 => 145.91768f64,
            Self::Pr147 => 146.919008f64,
            Self::Pr148 => 147.92213f64,
            Self::Pr149 => 148.923736f64,
            Self::Pr150 => 149.9266765f64,
            Self::Pr151 => 150.928309f64,
            Self::Pr152 => 151.931553f64,
            Self::Pr153 => 152.933904f64,
            Self::Pr154 => 153.93753f64,
            Self::Pr155 => 154.940509f64,
            Self::Pr156 => 155.94464f64,
            Self::Pr157 => 156.94789f64,
            Self::Pr158 => 157.95241f64,
            Self::Pr159 => 158.95589f64,
        }
    }
}
impl super::ElementVariant for PraseodymiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Pr
    }
}
impl super::MassNumber for PraseodymiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pr121 => 121u16,
            Self::Pr122 => 122u16,
            Self::Pr123 => 123u16,
            Self::Pr124 => 124u16,
            Self::Pr125 => 125u16,
            Self::Pr126 => 126u16,
            Self::Pr127 => 127u16,
            Self::Pr128 => 128u16,
            Self::Pr129 => 129u16,
            Self::Pr130 => 130u16,
            Self::Pr131 => 131u16,
            Self::Pr132 => 132u16,
            Self::Pr133 => 133u16,
            Self::Pr134 => 134u16,
            Self::Pr135 => 135u16,
            Self::Pr136 => 136u16,
            Self::Pr137 => 137u16,
            Self::Pr138 => 138u16,
            Self::Pr139 => 139u16,
            Self::Pr140 => 140u16,
            Self::Pr141 => 141u16,
            Self::Pr142 => 142u16,
            Self::Pr143 => 143u16,
            Self::Pr144 => 144u16,
            Self::Pr145 => 145u16,
            Self::Pr146 => 146u16,
            Self::Pr147 => 147u16,
            Self::Pr148 => 148u16,
            Self::Pr149 => 149u16,
            Self::Pr150 => 150u16,
            Self::Pr151 => 151u16,
            Self::Pr152 => 152u16,
            Self::Pr153 => 153u16,
            Self::Pr154 => 154u16,
            Self::Pr155 => 155u16,
            Self::Pr156 => 156u16,
            Self::Pr157 => 157u16,
            Self::Pr158 => 158u16,
            Self::Pr159 => 159u16,
        }
    }
}
impl super::IsotopicComposition for PraseodymiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Pr141 => Some(1f64),
            Self::Pr121
            | Self::Pr122
            | Self::Pr123
            | Self::Pr124
            | Self::Pr125
            | Self::Pr126
            | Self::Pr127
            | Self::Pr128
            | Self::Pr129
            | Self::Pr130
            | Self::Pr131
            | Self::Pr132
            | Self::Pr133
            | Self::Pr134
            | Self::Pr135
            | Self::Pr136
            | Self::Pr137
            | Self::Pr138
            | Self::Pr139
            | Self::Pr140
            | Self::Pr142
            | Self::Pr143
            | Self::Pr144
            | Self::Pr145
            | Self::Pr146
            | Self::Pr147
            | Self::Pr148
            | Self::Pr149
            | Self::Pr150
            | Self::Pr151
            | Self::Pr152
            | Self::Pr153
            | Self::Pr154
            | Self::Pr155
            | Self::Pr156
            | Self::Pr157
            | Self::Pr158
            | Self::Pr159 => None,
        }
    }
}
impl super::MostAbundantIsotope for PraseodymiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pr141
    }
}
impl From<PraseodymiumIsotope> for crate::Isotope {
    fn from(isotope: PraseodymiumIsotope) -> Self {
        crate::Isotope::Pr(isotope)
    }
}
impl From<PraseodymiumIsotope> for crate::Element {
    fn from(_isotope: PraseodymiumIsotope) -> Self {
        crate::Element::Pr
    }
}
impl TryFrom<u64> for PraseodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            121u64 => Ok(Self::Pr121),
            122u64 => Ok(Self::Pr122),
            123u64 => Ok(Self::Pr123),
            124u64 => Ok(Self::Pr124),
            125u64 => Ok(Self::Pr125),
            126u64 => Ok(Self::Pr126),
            127u64 => Ok(Self::Pr127),
            128u64 => Ok(Self::Pr128),
            129u64 => Ok(Self::Pr129),
            130u64 => Ok(Self::Pr130),
            131u64 => Ok(Self::Pr131),
            132u64 => Ok(Self::Pr132),
            133u64 => Ok(Self::Pr133),
            134u64 => Ok(Self::Pr134),
            135u64 => Ok(Self::Pr135),
            136u64 => Ok(Self::Pr136),
            137u64 => Ok(Self::Pr137),
            138u64 => Ok(Self::Pr138),
            139u64 => Ok(Self::Pr139),
            140u64 => Ok(Self::Pr140),
            141u64 => Ok(Self::Pr141),
            142u64 => Ok(Self::Pr142),
            143u64 => Ok(Self::Pr143),
            144u64 => Ok(Self::Pr144),
            145u64 => Ok(Self::Pr145),
            146u64 => Ok(Self::Pr146),
            147u64 => Ok(Self::Pr147),
            148u64 => Ok(Self::Pr148),
            149u64 => Ok(Self::Pr149),
            150u64 => Ok(Self::Pr150),
            151u64 => Ok(Self::Pr151),
            152u64 => Ok(Self::Pr152),
            153u64 => Ok(Self::Pr153),
            154u64 => Ok(Self::Pr154),
            155u64 => Ok(Self::Pr155),
            156u64 => Ok(Self::Pr156),
            157u64 => Ok(Self::Pr157),
            158u64 => Ok(Self::Pr158),
            159u64 => Ok(Self::Pr159),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pr, value)),
        }
    }
}
impl TryFrom<u8> for PraseodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for PraseodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for PraseodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl std::fmt::Display for PraseodymiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pr121 => write!(f, "Pr121"),
            Self::Pr122 => write!(f, "Pr122"),
            Self::Pr123 => write!(f, "Pr123"),
            Self::Pr124 => write!(f, "Pr124"),
            Self::Pr125 => write!(f, "Pr125"),
            Self::Pr126 => write!(f, "Pr126"),
            Self::Pr127 => write!(f, "Pr127"),
            Self::Pr128 => write!(f, "Pr128"),
            Self::Pr129 => write!(f, "Pr129"),
            Self::Pr130 => write!(f, "Pr130"),
            Self::Pr131 => write!(f, "Pr131"),
            Self::Pr132 => write!(f, "Pr132"),
            Self::Pr133 => write!(f, "Pr133"),
            Self::Pr134 => write!(f, "Pr134"),
            Self::Pr135 => write!(f, "Pr135"),
            Self::Pr136 => write!(f, "Pr136"),
            Self::Pr137 => write!(f, "Pr137"),
            Self::Pr138 => write!(f, "Pr138"),
            Self::Pr139 => write!(f, "Pr139"),
            Self::Pr140 => write!(f, "Pr140"),
            Self::Pr141 => write!(f, "Pr141"),
            Self::Pr142 => write!(f, "Pr142"),
            Self::Pr143 => write!(f, "Pr143"),
            Self::Pr144 => write!(f, "Pr144"),
            Self::Pr145 => write!(f, "Pr145"),
            Self::Pr146 => write!(f, "Pr146"),
            Self::Pr147 => write!(f, "Pr147"),
            Self::Pr148 => write!(f, "Pr148"),
            Self::Pr149 => write!(f, "Pr149"),
            Self::Pr150 => write!(f, "Pr150"),
            Self::Pr151 => write!(f, "Pr151"),
            Self::Pr152 => write!(f, "Pr152"),
            Self::Pr153 => write!(f, "Pr153"),
            Self::Pr154 => write!(f, "Pr154"),
            Self::Pr155 => write!(f, "Pr155"),
            Self::Pr156 => write!(f, "Pr156"),
            Self::Pr157 => write!(f, "Pr157"),
            Self::Pr158 => write!(f, "Pr158"),
            Self::Pr159 => write!(f, "Pr159"),
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
        for isotope in PraseodymiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in PraseodymiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Pr, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in PraseodymiumIsotope::iter() {
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
        for isotope in PraseodymiumIsotope::iter() {
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
        let most_abundant = PraseodymiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in PraseodymiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Pr(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in PraseodymiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Pr);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in PraseodymiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = PraseodymiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(PraseodymiumIsotope::try_from(0_u16).is_err());
        assert!(PraseodymiumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in PraseodymiumIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
