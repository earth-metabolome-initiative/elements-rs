//! Isotopes of the element Einsteinium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Einsteinium
pub enum EinsteiniumIsotope {
    /// Isotope Es239 of Einsteinium
    Es239,
    /// Isotope Es240 of Einsteinium
    Es240,
    /// Isotope Es241 of Einsteinium
    Es241,
    /// Isotope Es242 of Einsteinium
    Es242,
    /// Isotope Es243 of Einsteinium
    Es243,
    /// Isotope Es244 of Einsteinium
    Es244,
    /// Isotope Es245 of Einsteinium
    Es245,
    /// Isotope Es246 of Einsteinium
    Es246,
    /// Isotope Es247 of Einsteinium
    Es247,
    /// Isotope Es248 of Einsteinium
    Es248,
    /// Isotope Es249 of Einsteinium
    Es249,
    /// Isotope Es250 of Einsteinium
    Es250,
    /// Isotope Es251 of Einsteinium
    Es251,
    /// Isotope Es252 of Einsteinium
    Es252,
    /// Isotope Es253 of Einsteinium
    Es253,
    /// Isotope Es254 of Einsteinium
    Es254,
    /// Isotope Es255 of Einsteinium
    Es255,
    /// Isotope Es256 of Einsteinium
    Es256,
    /// Isotope Es257 of Einsteinium
    Es257,
    /// Isotope Es258 of Einsteinium
    Es258,
}
impl super::RelativeAtomicMass for EinsteiniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Es239 => 239.06823f64,
            Self::Es240 => 240.06892f64,
            Self::Es241 => 241.06856f64,
            Self::Es242 => 242.06957f64,
            Self::Es243 => 243.06951f64,
            Self::Es244 => 244.07088f64,
            Self::Es245 => 245.07125f64,
            Self::Es246 => 246.0729f64,
            Self::Es247 => 247.073622f64,
            Self::Es248 => 248.075471f64,
            Self::Es249 => 249.076411f64,
            Self::Es250 => 250.07861f64,
            Self::Es251 => 251.0799936f64,
            Self::Es252 => 252.08298f64,
            Self::Es253 => 253.0848257f64,
            Self::Es254 => 254.0880222f64,
            Self::Es255 => 255.090275f64,
            Self::Es256 => 256.0936f64,
            Self::Es257 => 257.09598f64,
            Self::Es258 => 258.09952f64,
        }
    }
}
impl super::ElementVariant for EinsteiniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Es
    }
}
impl super::MassNumber for EinsteiniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Es239 => 239u16,
            Self::Es240 => 240u16,
            Self::Es241 => 241u16,
            Self::Es242 => 242u16,
            Self::Es243 => 243u16,
            Self::Es244 => 244u16,
            Self::Es245 => 245u16,
            Self::Es246 => 246u16,
            Self::Es247 => 247u16,
            Self::Es248 => 248u16,
            Self::Es249 => 249u16,
            Self::Es250 => 250u16,
            Self::Es251 => 251u16,
            Self::Es252 => 252u16,
            Self::Es253 => 253u16,
            Self::Es254 => 254u16,
            Self::Es255 => 255u16,
            Self::Es256 => 256u16,
            Self::Es257 => 257u16,
            Self::Es258 => 258u16,
        }
    }
}
impl super::IsotopicComposition for EinsteiniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for EinsteiniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Es258
    }
}
impl From<EinsteiniumIsotope> for crate::Isotope {
    fn from(isotope: EinsteiniumIsotope) -> Self {
        crate::Isotope::Es(isotope)
    }
}
impl From<EinsteiniumIsotope> for crate::Element {
    fn from(_isotope: EinsteiniumIsotope) -> Self {
        crate::Element::Es
    }
}
impl TryFrom<u64> for EinsteiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            239u64 => Ok(Self::Es239),
            240u64 => Ok(Self::Es240),
            241u64 => Ok(Self::Es241),
            242u64 => Ok(Self::Es242),
            243u64 => Ok(Self::Es243),
            244u64 => Ok(Self::Es244),
            245u64 => Ok(Self::Es245),
            246u64 => Ok(Self::Es246),
            247u64 => Ok(Self::Es247),
            248u64 => Ok(Self::Es248),
            249u64 => Ok(Self::Es249),
            250u64 => Ok(Self::Es250),
            251u64 => Ok(Self::Es251),
            252u64 => Ok(Self::Es252),
            253u64 => Ok(Self::Es253),
            254u64 => Ok(Self::Es254),
            255u64 => Ok(Self::Es255),
            256u64 => Ok(Self::Es256),
            257u64 => Ok(Self::Es257),
            258u64 => Ok(Self::Es258),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Es, value)),
        }
    }
}
impl TryFrom<u8> for EinsteiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for EinsteiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for EinsteiniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for EinsteiniumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Es239 => write!(f, "Es239"),
            Self::Es240 => write!(f, "Es240"),
            Self::Es241 => write!(f, "Es241"),
            Self::Es242 => write!(f, "Es242"),
            Self::Es243 => write!(f, "Es243"),
            Self::Es244 => write!(f, "Es244"),
            Self::Es245 => write!(f, "Es245"),
            Self::Es246 => write!(f, "Es246"),
            Self::Es247 => write!(f, "Es247"),
            Self::Es248 => write!(f, "Es248"),
            Self::Es249 => write!(f, "Es249"),
            Self::Es250 => write!(f, "Es250"),
            Self::Es251 => write!(f, "Es251"),
            Self::Es252 => write!(f, "Es252"),
            Self::Es253 => write!(f, "Es253"),
            Self::Es254 => write!(f, "Es254"),
            Self::Es255 => write!(f, "Es255"),
            Self::Es256 => write!(f, "Es256"),
            Self::Es257 => write!(f, "Es257"),
            Self::Es258 => write!(f, "Es258"),
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
        for isotope in EinsteiniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in EinsteiniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Es, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in EinsteiniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in EinsteiniumIsotope::iter() {
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
        let most_abundant = EinsteiniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in EinsteiniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Es(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in EinsteiniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Es);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in EinsteiniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = EinsteiniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = EinsteiniumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = EinsteiniumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(EinsteiniumIsotope::try_from(0_u16).is_err());
        assert!(EinsteiniumIsotope::try_from(1000_u16).is_err());
        assert!(EinsteiniumIsotope::try_from(0_u32).is_err());
        assert!(EinsteiniumIsotope::try_from(1000_u32).is_err());
        assert!(EinsteiniumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in EinsteiniumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
