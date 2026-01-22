//! Isotopes of the element Arsenic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Arsenic
pub enum ArsenicIsotope {
    /// Isotope As60 of Arsenic
    As60,
    /// Isotope As61 of Arsenic
    As61,
    /// Isotope As62 of Arsenic
    As62,
    /// Isotope As63 of Arsenic
    As63,
    /// Isotope As64 of Arsenic
    As64,
    /// Isotope As65 of Arsenic
    As65,
    /// Isotope As66 of Arsenic
    As66,
    /// Isotope As67 of Arsenic
    As67,
    /// Isotope As68 of Arsenic
    As68,
    /// Isotope As69 of Arsenic
    As69,
    /// Isotope As70 of Arsenic
    As70,
    /// Isotope As71 of Arsenic
    As71,
    /// Isotope As72 of Arsenic
    As72,
    /// Isotope As73 of Arsenic
    As73,
    /// Isotope As74 of Arsenic
    As74,
    /// Isotope As75 of Arsenic
    As75,
    /// Isotope As76 of Arsenic
    As76,
    /// Isotope As77 of Arsenic
    As77,
    /// Isotope As78 of Arsenic
    As78,
    /// Isotope As79 of Arsenic
    As79,
    /// Isotope As80 of Arsenic
    As80,
    /// Isotope As81 of Arsenic
    As81,
    /// Isotope As82 of Arsenic
    As82,
    /// Isotope As83 of Arsenic
    As83,
    /// Isotope As84 of Arsenic
    As84,
    /// Isotope As85 of Arsenic
    As85,
    /// Isotope As86 of Arsenic
    As86,
    /// Isotope As87 of Arsenic
    As87,
    /// Isotope As88 of Arsenic
    As88,
    /// Isotope As89 of Arsenic
    As89,
    /// Isotope As90 of Arsenic
    As90,
    /// Isotope As91 of Arsenic
    As91,
    /// Isotope As92 of Arsenic
    As92,
}
impl super::RelativeAtomicMass for ArsenicIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::As60 => 59.99388f64,
            Self::As61 => 60.98112f64,
            Self::As62 => 61.97361f64,
            Self::As63 => 62.9639f64,
            Self::As64 => 63.95743f64,
            Self::As65 => 64.949611f64,
            Self::As66 => 65.9441488f64,
            Self::As67 => 66.93925111f64,
            Self::As68 => 67.9367741f64,
            Self::As69 => 68.932246f64,
            Self::As70 => 69.930926f64,
            Self::As71 => 70.9271138f64,
            Self::As72 => 71.9267523f64,
            Self::As73 => 72.9238291f64,
            Self::As74 => 73.9239286f64,
            Self::As75 => 74.92159457f64,
            Self::As76 => 75.92239202f64,
            Self::As77 => 76.9206476f64,
            Self::As78 => 77.921828f64,
            Self::As79 => 78.9209484f64,
            Self::As80 => 79.9224746f64,
            Self::As81 => 80.9221323f64,
            Self::As82 => 81.9247412f64,
            Self::As83 => 82.9252069f64,
            Self::As84 => 83.9293033f64,
            Self::As85 => 84.9321637f64,
            Self::As86 => 85.9367015f64,
            Self::As87 => 86.9402917f64,
            Self::As88 => 87.94555f64,
            Self::As89 => 88.94976f64,
            Self::As90 => 89.95563f64,
            Self::As91 => 90.96039f64,
            Self::As92 => 91.96674f64,
        }
    }
}
impl super::ElementVariant for ArsenicIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::As
    }
}
impl super::MassNumber for ArsenicIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::As60 => 60u16,
            Self::As61 => 61u16,
            Self::As62 => 62u16,
            Self::As63 => 63u16,
            Self::As64 => 64u16,
            Self::As65 => 65u16,
            Self::As66 => 66u16,
            Self::As67 => 67u16,
            Self::As68 => 68u16,
            Self::As69 => 69u16,
            Self::As70 => 70u16,
            Self::As71 => 71u16,
            Self::As72 => 72u16,
            Self::As73 => 73u16,
            Self::As74 => 74u16,
            Self::As75 => 75u16,
            Self::As76 => 76u16,
            Self::As77 => 77u16,
            Self::As78 => 78u16,
            Self::As79 => 79u16,
            Self::As80 => 80u16,
            Self::As81 => 81u16,
            Self::As82 => 82u16,
            Self::As83 => 83u16,
            Self::As84 => 84u16,
            Self::As85 => 85u16,
            Self::As86 => 86u16,
            Self::As87 => 87u16,
            Self::As88 => 88u16,
            Self::As89 => 89u16,
            Self::As90 => 90u16,
            Self::As91 => 91u16,
            Self::As92 => 92u16,
        }
    }
}
impl super::IsotopicComposition for ArsenicIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::As75 => Some(1f64),
            Self::As60
            | Self::As61
            | Self::As62
            | Self::As63
            | Self::As64
            | Self::As65
            | Self::As66
            | Self::As67
            | Self::As68
            | Self::As69
            | Self::As70
            | Self::As71
            | Self::As72
            | Self::As73
            | Self::As74
            | Self::As76
            | Self::As77
            | Self::As78
            | Self::As79
            | Self::As80
            | Self::As81
            | Self::As82
            | Self::As83
            | Self::As84
            | Self::As85
            | Self::As86
            | Self::As87
            | Self::As88
            | Self::As89
            | Self::As90
            | Self::As91
            | Self::As92 => None,
        }
    }
}
impl super::MostAbundantIsotope for ArsenicIsotope {
    fn most_abundant_isotope() -> Self {
        Self::As75
    }
}
impl From<ArsenicIsotope> for crate::Isotope {
    fn from(isotope: ArsenicIsotope) -> Self {
        crate::Isotope::As(isotope)
    }
}
impl From<ArsenicIsotope> for crate::Element {
    fn from(_isotope: ArsenicIsotope) -> Self {
        crate::Element::As
    }
}
impl TryFrom<u64> for ArsenicIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            60u64 => Ok(Self::As60),
            61u64 => Ok(Self::As61),
            62u64 => Ok(Self::As62),
            63u64 => Ok(Self::As63),
            64u64 => Ok(Self::As64),
            65u64 => Ok(Self::As65),
            66u64 => Ok(Self::As66),
            67u64 => Ok(Self::As67),
            68u64 => Ok(Self::As68),
            69u64 => Ok(Self::As69),
            70u64 => Ok(Self::As70),
            71u64 => Ok(Self::As71),
            72u64 => Ok(Self::As72),
            73u64 => Ok(Self::As73),
            74u64 => Ok(Self::As74),
            75u64 => Ok(Self::As75),
            76u64 => Ok(Self::As76),
            77u64 => Ok(Self::As77),
            78u64 => Ok(Self::As78),
            79u64 => Ok(Self::As79),
            80u64 => Ok(Self::As80),
            81u64 => Ok(Self::As81),
            82u64 => Ok(Self::As82),
            83u64 => Ok(Self::As83),
            84u64 => Ok(Self::As84),
            85u64 => Ok(Self::As85),
            86u64 => Ok(Self::As86),
            87u64 => Ok(Self::As87),
            88u64 => Ok(Self::As88),
            89u64 => Ok(Self::As89),
            90u64 => Ok(Self::As90),
            91u64 => Ok(Self::As91),
            92u64 => Ok(Self::As92),
            _ => Err(crate::errors::Error::Isotope(crate::Element::As, value)),
        }
    }
}
impl TryFrom<u8> for ArsenicIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for ArsenicIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for ArsenicIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for ArsenicIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::As60 => write!(f, "As60"),
            Self::As61 => write!(f, "As61"),
            Self::As62 => write!(f, "As62"),
            Self::As63 => write!(f, "As63"),
            Self::As64 => write!(f, "As64"),
            Self::As65 => write!(f, "As65"),
            Self::As66 => write!(f, "As66"),
            Self::As67 => write!(f, "As67"),
            Self::As68 => write!(f, "As68"),
            Self::As69 => write!(f, "As69"),
            Self::As70 => write!(f, "As70"),
            Self::As71 => write!(f, "As71"),
            Self::As72 => write!(f, "As72"),
            Self::As73 => write!(f, "As73"),
            Self::As74 => write!(f, "As74"),
            Self::As75 => write!(f, "As75"),
            Self::As76 => write!(f, "As76"),
            Self::As77 => write!(f, "As77"),
            Self::As78 => write!(f, "As78"),
            Self::As79 => write!(f, "As79"),
            Self::As80 => write!(f, "As80"),
            Self::As81 => write!(f, "As81"),
            Self::As82 => write!(f, "As82"),
            Self::As83 => write!(f, "As83"),
            Self::As84 => write!(f, "As84"),
            Self::As85 => write!(f, "As85"),
            Self::As86 => write!(f, "As86"),
            Self::As87 => write!(f, "As87"),
            Self::As88 => write!(f, "As88"),
            Self::As89 => write!(f, "As89"),
            Self::As90 => write!(f, "As90"),
            Self::As91 => write!(f, "As91"),
            Self::As92 => write!(f, "As92"),
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
        for isotope in ArsenicIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in ArsenicIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::As, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in ArsenicIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in ArsenicIsotope::iter() {
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
        let most_abundant = ArsenicIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in ArsenicIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::As(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in ArsenicIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::As);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in ArsenicIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = ArsenicIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(ArsenicIsotope::try_from(0_u16).is_err());
        assert!(ArsenicIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in ArsenicIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
