//! Isotopes of the element Boron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Boron
pub enum BoronIsotope {
    /// Isotope B6 of Boron
    B6,
    /// Isotope B7 of Boron
    B7,
    /// Isotope B8 of Boron
    B8,
    /// Isotope B9 of Boron
    B9,
    /// Isotope B10 of Boron
    B10,
    /// Isotope B11 of Boron
    B11,
    /// Isotope B12 of Boron
    B12,
    /// Isotope B13 of Boron
    B13,
    /// Isotope B14 of Boron
    B14,
    /// Isotope B15 of Boron
    B15,
    /// Isotope B16 of Boron
    B16,
    /// Isotope B17 of Boron
    B17,
    /// Isotope B18 of Boron
    B18,
    /// Isotope B19 of Boron
    B19,
    /// Isotope B20 of Boron
    B20,
    /// Isotope B21 of Boron
    B21,
}
impl super::RelativeAtomicMass for BoronIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::B6 => 6.0508f64,
            Self::B7 => 7.029712f64,
            Self::B8 => 8.0246073f64,
            Self::B9 => 9.01332965f64,
            Self::B10 => 10.01293695f64,
            Self::B11 => 11.00930536f64,
            Self::B12 => 12.0143527f64,
            Self::B13 => 13.0177802f64,
            Self::B14 => 14.025404f64,
            Self::B15 => 15.031088f64,
            Self::B16 => 16.039842f64,
            Self::B17 => 17.04699f64,
            Self::B18 => 18.05566f64,
            Self::B19 => 19.0631f64,
            Self::B20 => 20.07207f64,
            Self::B21 => 21.08129f64,
        }
    }
}
impl super::ElementVariant for BoronIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::B
    }
}
impl super::MassNumber for BoronIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::B6 => 6u16,
            Self::B7 => 7u16,
            Self::B8 => 8u16,
            Self::B9 => 9u16,
            Self::B10 => 10u16,
            Self::B11 => 11u16,
            Self::B12 => 12u16,
            Self::B13 => 13u16,
            Self::B14 => 14u16,
            Self::B15 => 15u16,
            Self::B16 => 16u16,
            Self::B17 => 17u16,
            Self::B18 => 18u16,
            Self::B19 => 19u16,
            Self::B20 => 20u16,
            Self::B21 => 21u16,
        }
    }
}
impl super::IsotopicComposition for BoronIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::B10 => Some(0.199f64),
            Self::B11 => Some(0.801f64),
            Self::B6
            | Self::B7
            | Self::B8
            | Self::B9
            | Self::B12
            | Self::B13
            | Self::B14
            | Self::B15
            | Self::B16
            | Self::B17
            | Self::B18
            | Self::B19
            | Self::B20
            | Self::B21 => None,
        }
    }
}
impl super::MostAbundantIsotope for BoronIsotope {
    fn most_abundant_isotope() -> Self {
        Self::B11
    }
}
impl From<BoronIsotope> for crate::Isotope {
    fn from(isotope: BoronIsotope) -> Self {
        crate::Isotope::B(isotope)
    }
}
impl From<BoronIsotope> for crate::Element {
    fn from(_isotope: BoronIsotope) -> Self {
        crate::Element::B
    }
}
impl TryFrom<u64> for BoronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            6u64 => Ok(Self::B6),
            7u64 => Ok(Self::B7),
            8u64 => Ok(Self::B8),
            9u64 => Ok(Self::B9),
            10u64 => Ok(Self::B10),
            11u64 => Ok(Self::B11),
            12u64 => Ok(Self::B12),
            13u64 => Ok(Self::B13),
            14u64 => Ok(Self::B14),
            15u64 => Ok(Self::B15),
            16u64 => Ok(Self::B16),
            17u64 => Ok(Self::B17),
            18u64 => Ok(Self::B18),
            19u64 => Ok(Self::B19),
            20u64 => Ok(Self::B20),
            21u64 => Ok(Self::B21),
            _ => Err(crate::errors::Error::Isotope(crate::Element::B, value)),
        }
    }
}
impl TryFrom<u8> for BoronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for BoronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for BoronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl std::fmt::Display for BoronIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B6 => write!(f, "B6"),
            Self::B7 => write!(f, "B7"),
            Self::B8 => write!(f, "B8"),
            Self::B9 => write!(f, "B9"),
            Self::B10 => write!(f, "B10"),
            Self::B11 => write!(f, "B11"),
            Self::B12 => write!(f, "B12"),
            Self::B13 => write!(f, "B13"),
            Self::B14 => write!(f, "B14"),
            Self::B15 => write!(f, "B15"),
            Self::B16 => write!(f, "B16"),
            Self::B17 => write!(f, "B17"),
            Self::B18 => write!(f, "B18"),
            Self::B19 => write!(f, "B19"),
            Self::B20 => write!(f, "B20"),
            Self::B21 => write!(f, "B21"),
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
        for isotope in BoronIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {:?}", isotope);
        }
    }
    #[test]
    fn test_element() {
        for isotope in BoronIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::B, "Element should be correct for {:?}", isotope);
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in BoronIsotope::iter() {
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
        for isotope in BoronIsotope::iter() {
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
        let most_abundant = BoronIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in BoronIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::B(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in BoronIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::B);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in BoronIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = BoronIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(BoronIsotope::try_from(0_u16).is_err());
        assert!(BoronIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in BoronIsotope::iter() {
            let s = format!("{}", isotope);
            assert!(!s.is_empty(), "Display should not be empty for {:?}", isotope);
        }
    }
}
