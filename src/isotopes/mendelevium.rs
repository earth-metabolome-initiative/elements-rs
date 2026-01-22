//! Isotopes of the element Mendelevium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Mendelevium
pub enum MendeleviumIsotope {
    /// Isotope Md245 of Mendelevium
    Md245,
    /// Isotope Md246 of Mendelevium
    Md246,
    /// Isotope Md247 of Mendelevium
    Md247,
    /// Isotope Md248 of Mendelevium
    Md248,
    /// Isotope Md249 of Mendelevium
    Md249,
    /// Isotope Md250 of Mendelevium
    Md250,
    /// Isotope Md251 of Mendelevium
    Md251,
    /// Isotope Md252 of Mendelevium
    Md252,
    /// Isotope Md253 of Mendelevium
    Md253,
    /// Isotope Md254 of Mendelevium
    Md254,
    /// Isotope Md255 of Mendelevium
    Md255,
    /// Isotope Md256 of Mendelevium
    Md256,
    /// Isotope Md257 of Mendelevium
    Md257,
    /// Isotope Md258 of Mendelevium
    Md258,
    /// Isotope Md259 of Mendelevium
    Md259,
    /// Isotope Md260 of Mendelevium
    Md260,
    /// Isotope Md261 of Mendelevium
    Md261,
    /// Isotope Md262 of Mendelevium
    Md262,
}
impl super::RelativeAtomicMass for MendeleviumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Md245 => 245.08081f64,
            Self::Md246 => 246.08171f64,
            Self::Md247 => 247.08152f64,
            Self::Md248 => 248.08282f64,
            Self::Md249 => 249.08291f64,
            Self::Md250 => 250.08441f64,
            Self::Md251 => 251.084774f64,
            Self::Md252 => 252.08643f64,
            Self::Md253 => 253.087144f64,
            Self::Md254 => 254.08959f64,
            Self::Md255 => 255.0910841f64,
            Self::Md256 => 256.09389f64,
            Self::Md257 => 257.0955424f64,
            Self::Md258 => 258.0984315f64,
            Self::Md259 => 259.10051f64,
            Self::Md260 => 260.10365f64,
            Self::Md261 => 261.10583f64,
            Self::Md262 => 262.1091f64,
        }
    }
}
impl super::ElementVariant for MendeleviumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Md
    }
}
impl super::MassNumber for MendeleviumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Md245 => 245u16,
            Self::Md246 => 246u16,
            Self::Md247 => 247u16,
            Self::Md248 => 248u16,
            Self::Md249 => 249u16,
            Self::Md250 => 250u16,
            Self::Md251 => 251u16,
            Self::Md252 => 252u16,
            Self::Md253 => 253u16,
            Self::Md254 => 254u16,
            Self::Md255 => 255u16,
            Self::Md256 => 256u16,
            Self::Md257 => 257u16,
            Self::Md258 => 258u16,
            Self::Md259 => 259u16,
            Self::Md260 => 260u16,
            Self::Md261 => 261u16,
            Self::Md262 => 262u16,
        }
    }
}
impl super::IsotopicComposition for MendeleviumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for MendeleviumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Md262
    }
}
impl From<MendeleviumIsotope> for crate::Isotope {
    fn from(isotope: MendeleviumIsotope) -> Self {
        crate::Isotope::Md(isotope)
    }
}
impl From<MendeleviumIsotope> for crate::Element {
    fn from(_isotope: MendeleviumIsotope) -> Self {
        crate::Element::Md
    }
}
impl TryFrom<u64> for MendeleviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            245u64 => Ok(Self::Md245),
            246u64 => Ok(Self::Md246),
            247u64 => Ok(Self::Md247),
            248u64 => Ok(Self::Md248),
            249u64 => Ok(Self::Md249),
            250u64 => Ok(Self::Md250),
            251u64 => Ok(Self::Md251),
            252u64 => Ok(Self::Md252),
            253u64 => Ok(Self::Md253),
            254u64 => Ok(Self::Md254),
            255u64 => Ok(Self::Md255),
            256u64 => Ok(Self::Md256),
            257u64 => Ok(Self::Md257),
            258u64 => Ok(Self::Md258),
            259u64 => Ok(Self::Md259),
            260u64 => Ok(Self::Md260),
            261u64 => Ok(Self::Md261),
            262u64 => Ok(Self::Md262),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Md, value)),
        }
    }
}
impl TryFrom<u8> for MendeleviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for MendeleviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for MendeleviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for MendeleviumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Md245 => write!(f, "Md245"),
            Self::Md246 => write!(f, "Md246"),
            Self::Md247 => write!(f, "Md247"),
            Self::Md248 => write!(f, "Md248"),
            Self::Md249 => write!(f, "Md249"),
            Self::Md250 => write!(f, "Md250"),
            Self::Md251 => write!(f, "Md251"),
            Self::Md252 => write!(f, "Md252"),
            Self::Md253 => write!(f, "Md253"),
            Self::Md254 => write!(f, "Md254"),
            Self::Md255 => write!(f, "Md255"),
            Self::Md256 => write!(f, "Md256"),
            Self::Md257 => write!(f, "Md257"),
            Self::Md258 => write!(f, "Md258"),
            Self::Md259 => write!(f, "Md259"),
            Self::Md260 => write!(f, "Md260"),
            Self::Md261 => write!(f, "Md261"),
            Self::Md262 => write!(f, "Md262"),
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
        for isotope in MendeleviumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in MendeleviumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Md, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in MendeleviumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in MendeleviumIsotope::iter() {
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
        let most_abundant = MendeleviumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in MendeleviumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Md(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in MendeleviumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Md);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in MendeleviumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = MendeleviumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(MendeleviumIsotope::try_from(0_u16).is_err());
        assert!(MendeleviumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in MendeleviumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
