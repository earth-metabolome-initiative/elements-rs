//! Isotopes of the element Roentgenium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Roentgenium
pub enum RoentgeniumIsotope {
    /// Isotope Rg272 of Roentgenium
    Rg272,
    /// Isotope Rg273 of Roentgenium
    Rg273,
    /// Isotope Rg274 of Roentgenium
    Rg274,
    /// Isotope Rg275 of Roentgenium
    Rg275,
    /// Isotope Rg276 of Roentgenium
    Rg276,
    /// Isotope Rg277 of Roentgenium
    Rg277,
    /// Isotope Rg278 of Roentgenium
    Rg278,
    /// Isotope Rg279 of Roentgenium
    Rg279,
    /// Isotope Rg280 of Roentgenium
    Rg280,
    /// Isotope Rg281 of Roentgenium
    Rg281,
    /// Isotope Rg282 of Roentgenium
    Rg282,
    /// Isotope Rg283 of Roentgenium
    Rg283,
}
impl super::RelativeAtomicMass for RoentgeniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rg272 => 272.15327f64,
            Self::Rg273 => 273.15313f64,
            Self::Rg274 => 274.15525f64,
            Self::Rg275 => 275.15594f64,
            Self::Rg276 => 276.15833f64,
            Self::Rg277 => 277.15907f64,
            Self::Rg278 => 278.16149f64,
            Self::Rg279 => 279.16272f64,
            Self::Rg280 => 280.16514f64,
            Self::Rg281 => 281.16636f64,
            Self::Rg282 => 282.16912f64,
            Self::Rg283 => 283.17054f64,
        }
    }
}
impl super::ElementVariant for RoentgeniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Rg
    }
}
impl super::MassNumber for RoentgeniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rg272 => 272u16,
            Self::Rg273 => 273u16,
            Self::Rg274 => 274u16,
            Self::Rg275 => 275u16,
            Self::Rg276 => 276u16,
            Self::Rg277 => 277u16,
            Self::Rg278 => 278u16,
            Self::Rg279 => 279u16,
            Self::Rg280 => 280u16,
            Self::Rg281 => 281u16,
            Self::Rg282 => 282u16,
            Self::Rg283 => 283u16,
        }
    }
}
impl super::IsotopicComposition for RoentgeniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for RoentgeniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rg283
    }
}
impl From<RoentgeniumIsotope> for crate::Isotope {
    fn from(isotope: RoentgeniumIsotope) -> Self {
        crate::Isotope::Rg(isotope)
    }
}
impl From<RoentgeniumIsotope> for crate::Element {
    fn from(_isotope: RoentgeniumIsotope) -> Self {
        crate::Element::Rg
    }
}
impl TryFrom<u64> for RoentgeniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            272u64 => Ok(Self::Rg272),
            273u64 => Ok(Self::Rg273),
            274u64 => Ok(Self::Rg274),
            275u64 => Ok(Self::Rg275),
            276u64 => Ok(Self::Rg276),
            277u64 => Ok(Self::Rg277),
            278u64 => Ok(Self::Rg278),
            279u64 => Ok(Self::Rg279),
            280u64 => Ok(Self::Rg280),
            281u64 => Ok(Self::Rg281),
            282u64 => Ok(Self::Rg282),
            283u64 => Ok(Self::Rg283),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rg, value)),
        }
    }
}
impl TryFrom<u8> for RoentgeniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for RoentgeniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for RoentgeniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for RoentgeniumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rg272 => write!(f, "Rg272"),
            Self::Rg273 => write!(f, "Rg273"),
            Self::Rg274 => write!(f, "Rg274"),
            Self::Rg275 => write!(f, "Rg275"),
            Self::Rg276 => write!(f, "Rg276"),
            Self::Rg277 => write!(f, "Rg277"),
            Self::Rg278 => write!(f, "Rg278"),
            Self::Rg279 => write!(f, "Rg279"),
            Self::Rg280 => write!(f, "Rg280"),
            Self::Rg281 => write!(f, "Rg281"),
            Self::Rg282 => write!(f, "Rg282"),
            Self::Rg283 => write!(f, "Rg283"),
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
        for isotope in RoentgeniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in RoentgeniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Rg, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in RoentgeniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in RoentgeniumIsotope::iter() {
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
        let most_abundant = RoentgeniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in RoentgeniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Rg(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in RoentgeniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Rg);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in RoentgeniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = RoentgeniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(RoentgeniumIsotope::try_from(0_u16).is_err());
        assert!(RoentgeniumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in RoentgeniumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
