//! Isotopes of the element Neodymium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Neodymium
pub enum NeodymiumIsotope {
    /// Isotope Nd124 of Neodymium
    Nd124,
    /// Isotope Nd125 of Neodymium
    Nd125,
    /// Isotope Nd126 of Neodymium
    Nd126,
    /// Isotope Nd127 of Neodymium
    Nd127,
    /// Isotope Nd128 of Neodymium
    Nd128,
    /// Isotope Nd129 of Neodymium
    Nd129,
    /// Isotope Nd130 of Neodymium
    Nd130,
    /// Isotope Nd131 of Neodymium
    Nd131,
    /// Isotope Nd132 of Neodymium
    Nd132,
    /// Isotope Nd133 of Neodymium
    Nd133,
    /// Isotope Nd134 of Neodymium
    Nd134,
    /// Isotope Nd135 of Neodymium
    Nd135,
    /// Isotope Nd136 of Neodymium
    Nd136,
    /// Isotope Nd137 of Neodymium
    Nd137,
    /// Isotope Nd138 of Neodymium
    Nd138,
    /// Isotope Nd139 of Neodymium
    Nd139,
    /// Isotope Nd140 of Neodymium
    Nd140,
    /// Isotope Nd141 of Neodymium
    Nd141,
    /// Isotope Nd142 of Neodymium
    Nd142,
    /// Isotope Nd143 of Neodymium
    Nd143,
    /// Isotope Nd144 of Neodymium
    Nd144,
    /// Isotope Nd145 of Neodymium
    Nd145,
    /// Isotope Nd146 of Neodymium
    Nd146,
    /// Isotope Nd147 of Neodymium
    Nd147,
    /// Isotope Nd148 of Neodymium
    Nd148,
    /// Isotope Nd149 of Neodymium
    Nd149,
    /// Isotope Nd150 of Neodymium
    Nd150,
    /// Isotope Nd151 of Neodymium
    Nd151,
    /// Isotope Nd152 of Neodymium
    Nd152,
    /// Isotope Nd153 of Neodymium
    Nd153,
    /// Isotope Nd154 of Neodymium
    Nd154,
    /// Isotope Nd155 of Neodymium
    Nd155,
    /// Isotope Nd156 of Neodymium
    Nd156,
    /// Isotope Nd157 of Neodymium
    Nd157,
    /// Isotope Nd158 of Neodymium
    Nd158,
    /// Isotope Nd159 of Neodymium
    Nd159,
    /// Isotope Nd160 of Neodymium
    Nd160,
    /// Isotope Nd161 of Neodymium
    Nd161,
}
impl super::RelativeAtomicMass for NeodymiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Nd124 => 123.9522f64,
            Self::Nd125 => 124.9489f64,
            Self::Nd126 => 125.94311f64,
            Self::Nd127 => 126.94038f64,
            Self::Nd128 => 127.93525f64,
            Self::Nd129 => 128.9331f64,
            Self::Nd130 => 129.928506f64,
            Self::Nd131 => 130.927248f64,
            Self::Nd132 => 131.923321f64,
            Self::Nd133 => 132.922348f64,
            Self::Nd134 => 133.91879f64,
            Self::Nd135 => 134.918181f64,
            Self::Nd136 => 135.914976f64,
            Self::Nd137 => 136.914562f64,
            Self::Nd138 => 137.91195f64,
            Self::Nd139 => 138.911954f64,
            Self::Nd140 => 139.90955f64,
            Self::Nd141 => 140.9096147f64,
            Self::Nd142 => 141.907729f64,
            Self::Nd143 => 142.90982f64,
            Self::Nd144 => 143.910093f64,
            Self::Nd145 => 144.9125793f64,
            Self::Nd146 => 145.9131226f64,
            Self::Nd147 => 146.9161061f64,
            Self::Nd148 => 147.9168993f64,
            Self::Nd149 => 148.9201548f64,
            Self::Nd150 => 149.9209022f64,
            Self::Nd151 => 150.9238403f64,
            Self::Nd152 => 151.924692f64,
            Self::Nd153 => 152.927718f64,
            Self::Nd154 => 153.92948f64,
            Self::Nd155 => 154.9331357f64,
            Self::Nd156 => 155.93508f64,
            Self::Nd157 => 156.939386f64,
            Self::Nd158 => 157.94197f64,
            Self::Nd159 => 158.94653f64,
            Self::Nd160 => 159.9494f64,
            Self::Nd161 => 160.95428f64,
        }
    }
}
impl super::ElementVariant for NeodymiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Nd
    }
}
impl super::MassNumber for NeodymiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Nd124 => 124u16,
            Self::Nd125 => 125u16,
            Self::Nd126 => 126u16,
            Self::Nd127 => 127u16,
            Self::Nd128 => 128u16,
            Self::Nd129 => 129u16,
            Self::Nd130 => 130u16,
            Self::Nd131 => 131u16,
            Self::Nd132 => 132u16,
            Self::Nd133 => 133u16,
            Self::Nd134 => 134u16,
            Self::Nd135 => 135u16,
            Self::Nd136 => 136u16,
            Self::Nd137 => 137u16,
            Self::Nd138 => 138u16,
            Self::Nd139 => 139u16,
            Self::Nd140 => 140u16,
            Self::Nd141 => 141u16,
            Self::Nd142 => 142u16,
            Self::Nd143 => 143u16,
            Self::Nd144 => 144u16,
            Self::Nd145 => 145u16,
            Self::Nd146 => 146u16,
            Self::Nd147 => 147u16,
            Self::Nd148 => 148u16,
            Self::Nd149 => 149u16,
            Self::Nd150 => 150u16,
            Self::Nd151 => 151u16,
            Self::Nd152 => 152u16,
            Self::Nd153 => 153u16,
            Self::Nd154 => 154u16,
            Self::Nd155 => 155u16,
            Self::Nd156 => 156u16,
            Self::Nd157 => 157u16,
            Self::Nd158 => 158u16,
            Self::Nd159 => 159u16,
            Self::Nd160 => 160u16,
            Self::Nd161 => 161u16,
        }
    }
}
impl super::IsotopicComposition for NeodymiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Nd142 => Some(0.27152f64),
            Self::Nd143 => Some(0.12174f64),
            Self::Nd144 => Some(0.23798f64),
            Self::Nd145 => Some(0.08293f64),
            Self::Nd146 => Some(0.17189f64),
            Self::Nd148 => Some(0.05756f64),
            Self::Nd150 => Some(0.05638f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for NeodymiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Nd142
    }
}
impl From<NeodymiumIsotope> for crate::Isotope {
    fn from(isotope: NeodymiumIsotope) -> Self {
        crate::Isotope::Nd(isotope)
    }
}
impl From<NeodymiumIsotope> for crate::Element {
    fn from(_isotope: NeodymiumIsotope) -> Self {
        crate::Element::Nd
    }
}
impl TryFrom<u64> for NeodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            124u64 => Ok(Self::Nd124),
            125u64 => Ok(Self::Nd125),
            126u64 => Ok(Self::Nd126),
            127u64 => Ok(Self::Nd127),
            128u64 => Ok(Self::Nd128),
            129u64 => Ok(Self::Nd129),
            130u64 => Ok(Self::Nd130),
            131u64 => Ok(Self::Nd131),
            132u64 => Ok(Self::Nd132),
            133u64 => Ok(Self::Nd133),
            134u64 => Ok(Self::Nd134),
            135u64 => Ok(Self::Nd135),
            136u64 => Ok(Self::Nd136),
            137u64 => Ok(Self::Nd137),
            138u64 => Ok(Self::Nd138),
            139u64 => Ok(Self::Nd139),
            140u64 => Ok(Self::Nd140),
            141u64 => Ok(Self::Nd141),
            142u64 => Ok(Self::Nd142),
            143u64 => Ok(Self::Nd143),
            144u64 => Ok(Self::Nd144),
            145u64 => Ok(Self::Nd145),
            146u64 => Ok(Self::Nd146),
            147u64 => Ok(Self::Nd147),
            148u64 => Ok(Self::Nd148),
            149u64 => Ok(Self::Nd149),
            150u64 => Ok(Self::Nd150),
            151u64 => Ok(Self::Nd151),
            152u64 => Ok(Self::Nd152),
            153u64 => Ok(Self::Nd153),
            154u64 => Ok(Self::Nd154),
            155u64 => Ok(Self::Nd155),
            156u64 => Ok(Self::Nd156),
            157u64 => Ok(Self::Nd157),
            158u64 => Ok(Self::Nd158),
            159u64 => Ok(Self::Nd159),
            160u64 => Ok(Self::Nd160),
            161u64 => Ok(Self::Nd161),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Nd, value)),
        }
    }
}
impl TryFrom<u8> for NeodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for NeodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for NeodymiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for NeodymiumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nd124 => write!(f, "Nd124"),
            Self::Nd125 => write!(f, "Nd125"),
            Self::Nd126 => write!(f, "Nd126"),
            Self::Nd127 => write!(f, "Nd127"),
            Self::Nd128 => write!(f, "Nd128"),
            Self::Nd129 => write!(f, "Nd129"),
            Self::Nd130 => write!(f, "Nd130"),
            Self::Nd131 => write!(f, "Nd131"),
            Self::Nd132 => write!(f, "Nd132"),
            Self::Nd133 => write!(f, "Nd133"),
            Self::Nd134 => write!(f, "Nd134"),
            Self::Nd135 => write!(f, "Nd135"),
            Self::Nd136 => write!(f, "Nd136"),
            Self::Nd137 => write!(f, "Nd137"),
            Self::Nd138 => write!(f, "Nd138"),
            Self::Nd139 => write!(f, "Nd139"),
            Self::Nd140 => write!(f, "Nd140"),
            Self::Nd141 => write!(f, "Nd141"),
            Self::Nd142 => write!(f, "Nd142"),
            Self::Nd143 => write!(f, "Nd143"),
            Self::Nd144 => write!(f, "Nd144"),
            Self::Nd145 => write!(f, "Nd145"),
            Self::Nd146 => write!(f, "Nd146"),
            Self::Nd147 => write!(f, "Nd147"),
            Self::Nd148 => write!(f, "Nd148"),
            Self::Nd149 => write!(f, "Nd149"),
            Self::Nd150 => write!(f, "Nd150"),
            Self::Nd151 => write!(f, "Nd151"),
            Self::Nd152 => write!(f, "Nd152"),
            Self::Nd153 => write!(f, "Nd153"),
            Self::Nd154 => write!(f, "Nd154"),
            Self::Nd155 => write!(f, "Nd155"),
            Self::Nd156 => write!(f, "Nd156"),
            Self::Nd157 => write!(f, "Nd157"),
            Self::Nd158 => write!(f, "Nd158"),
            Self::Nd159 => write!(f, "Nd159"),
            Self::Nd160 => write!(f, "Nd160"),
            Self::Nd161 => write!(f, "Nd161"),
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
        for isotope in NeodymiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in NeodymiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Nd, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in NeodymiumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in NeodymiumIsotope::iter() {
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
        let most_abundant = NeodymiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in NeodymiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Nd(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in NeodymiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Nd);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in NeodymiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = NeodymiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = NeodymiumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = NeodymiumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(NeodymiumIsotope::try_from(0_u16).is_err());
        assert!(NeodymiumIsotope::try_from(1000_u16).is_err());
        assert!(NeodymiumIsotope::try_from(0_u32).is_err());
        assert!(NeodymiumIsotope::try_from(1000_u32).is_err());
        assert!(NeodymiumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in NeodymiumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
