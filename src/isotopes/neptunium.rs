//! Isotopes of the element Neptunium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
/// Isotopes of the element Neptunium
pub enum NeptuniumIsotope {
    /// Isotope Np219 of Neptunium
    Np219,
    /// Isotope Np220 of Neptunium
    Np220,
    /// Isotope Np221 of Neptunium
    Np221,
    /// Isotope Np222 of Neptunium
    Np222,
    /// Isotope Np223 of Neptunium
    Np223,
    /// Isotope Np224 of Neptunium
    Np224,
    /// Isotope Np225 of Neptunium
    Np225,
    /// Isotope Np226 of Neptunium
    Np226,
    /// Isotope Np227 of Neptunium
    Np227,
    /// Isotope Np228 of Neptunium
    Np228,
    /// Isotope Np229 of Neptunium
    Np229,
    /// Isotope Np230 of Neptunium
    Np230,
    /// Isotope Np231 of Neptunium
    Np231,
    /// Isotope Np232 of Neptunium
    Np232,
    /// Isotope Np233 of Neptunium
    Np233,
    /// Isotope Np234 of Neptunium
    Np234,
    /// Isotope Np235 of Neptunium
    Np235,
    /// Isotope Np236 of Neptunium
    Np236,
    /// Isotope Np237 of Neptunium
    Np237,
    /// Isotope Np238 of Neptunium
    Np238,
    /// Isotope Np239 of Neptunium
    Np239,
    /// Isotope Np240 of Neptunium
    Np240,
    /// Isotope Np241 of Neptunium
    Np241,
    /// Isotope Np242 of Neptunium
    Np242,
    /// Isotope Np243 of Neptunium
    Np243,
    /// Isotope Np244 of Neptunium
    Np244,
    /// Isotope Np245 of Neptunium
    Np245,
}
impl super::RelativeAtomicMass for NeptuniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Np219 => 219.03143f64,
            Self::Np220 => 220.03254f64,
            Self::Np221 => 221.03204f64,
            Self::Np222 => 222.0333f64,
            Self::Np223 => 223.03285f64,
            Self::Np224 => 224.03422f64,
            Self::Np225 => 225.033911f64,
            Self::Np226 => 226.035188f64,
            Self::Np227 => 227.034957f64,
            Self::Np228 => 228.036067f64,
            Self::Np229 => 229.036264f64,
            Self::Np230 => 230.037828f64,
            Self::Np231 => 231.038245f64,
            Self::Np232 => 232.04011f64,
            Self::Np233 => 233.040741f64,
            Self::Np234 => 234.0428953f64,
            Self::Np235 => 235.0440635f64,
            Self::Np236 => 236.04657f64,
            Self::Np237 => 237.0481736f64,
            Self::Np238 => 238.0509466f64,
            Self::Np239 => 239.0529392f64,
            Self::Np240 => 240.056165f64,
            Self::Np241 => 241.058253f64,
            Self::Np242 => 242.06164f64,
            Self::Np243 => 243.06428f64,
            Self::Np244 => 244.06785f64,
            Self::Np245 => 245.0708f64,
        }
    }
}
impl super::ElementVariant for NeptuniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Np
    }
}
impl super::MassNumber for NeptuniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Np219 => 219u16,
            Self::Np220 => 220u16,
            Self::Np221 => 221u16,
            Self::Np222 => 222u16,
            Self::Np223 => 223u16,
            Self::Np224 => 224u16,
            Self::Np225 => 225u16,
            Self::Np226 => 226u16,
            Self::Np227 => 227u16,
            Self::Np228 => 228u16,
            Self::Np229 => 229u16,
            Self::Np230 => 230u16,
            Self::Np231 => 231u16,
            Self::Np232 => 232u16,
            Self::Np233 => 233u16,
            Self::Np234 => 234u16,
            Self::Np235 => 235u16,
            Self::Np236 => 236u16,
            Self::Np237 => 237u16,
            Self::Np238 => 238u16,
            Self::Np239 => 239u16,
            Self::Np240 => 240u16,
            Self::Np241 => 241u16,
            Self::Np242 => 242u16,
            Self::Np243 => 243u16,
            Self::Np244 => 244u16,
            Self::Np245 => 245u16,
        }
    }
}
impl super::IsotopicComposition for NeptuniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for NeptuniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Np245
    }
}
impl From<NeptuniumIsotope> for crate::Isotope {
    fn from(isotope: NeptuniumIsotope) -> Self {
        crate::Isotope::Np(isotope)
    }
}
impl From<NeptuniumIsotope> for crate::Element {
    fn from(_isotope: NeptuniumIsotope) -> Self {
        crate::Element::Np
    }
}
impl TryFrom<u64> for NeptuniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            219u64 => Ok(Self::Np219),
            220u64 => Ok(Self::Np220),
            221u64 => Ok(Self::Np221),
            222u64 => Ok(Self::Np222),
            223u64 => Ok(Self::Np223),
            224u64 => Ok(Self::Np224),
            225u64 => Ok(Self::Np225),
            226u64 => Ok(Self::Np226),
            227u64 => Ok(Self::Np227),
            228u64 => Ok(Self::Np228),
            229u64 => Ok(Self::Np229),
            230u64 => Ok(Self::Np230),
            231u64 => Ok(Self::Np231),
            232u64 => Ok(Self::Np232),
            233u64 => Ok(Self::Np233),
            234u64 => Ok(Self::Np234),
            235u64 => Ok(Self::Np235),
            236u64 => Ok(Self::Np236),
            237u64 => Ok(Self::Np237),
            238u64 => Ok(Self::Np238),
            239u64 => Ok(Self::Np239),
            240u64 => Ok(Self::Np240),
            241u64 => Ok(Self::Np241),
            242u64 => Ok(Self::Np242),
            243u64 => Ok(Self::Np243),
            244u64 => Ok(Self::Np244),
            245u64 => Ok(Self::Np245),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Np, value)),
        }
    }
}
impl TryFrom<u8> for NeptuniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for NeptuniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for NeptuniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for NeptuniumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Np219 => write!(f, "Np219"),
            Self::Np220 => write!(f, "Np220"),
            Self::Np221 => write!(f, "Np221"),
            Self::Np222 => write!(f, "Np222"),
            Self::Np223 => write!(f, "Np223"),
            Self::Np224 => write!(f, "Np224"),
            Self::Np225 => write!(f, "Np225"),
            Self::Np226 => write!(f, "Np226"),
            Self::Np227 => write!(f, "Np227"),
            Self::Np228 => write!(f, "Np228"),
            Self::Np229 => write!(f, "Np229"),
            Self::Np230 => write!(f, "Np230"),
            Self::Np231 => write!(f, "Np231"),
            Self::Np232 => write!(f, "Np232"),
            Self::Np233 => write!(f, "Np233"),
            Self::Np234 => write!(f, "Np234"),
            Self::Np235 => write!(f, "Np235"),
            Self::Np236 => write!(f, "Np236"),
            Self::Np237 => write!(f, "Np237"),
            Self::Np238 => write!(f, "Np238"),
            Self::Np239 => write!(f, "Np239"),
            Self::Np240 => write!(f, "Np240"),
            Self::Np241 => write!(f, "Np241"),
            Self::Np242 => write!(f, "Np242"),
            Self::Np243 => write!(f, "Np243"),
            Self::Np244 => write!(f, "Np244"),
            Self::Np245 => write!(f, "Np245"),
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
        for isotope in NeptuniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in NeptuniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Np, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in NeptuniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in NeptuniumIsotope::iter() {
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
        let most_abundant = NeptuniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in NeptuniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Np(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in NeptuniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Np);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in NeptuniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = NeptuniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = NeptuniumIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = NeptuniumIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(NeptuniumIsotope::try_from(0_u16).is_err());
        assert!(NeptuniumIsotope::try_from(1000_u16).is_err());
        assert!(NeptuniumIsotope::try_from(0_u32).is_err());
        assert!(NeptuniumIsotope::try_from(1000_u32).is_err());
        assert!(NeptuniumIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in NeptuniumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
