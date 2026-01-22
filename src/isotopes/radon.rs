//! Isotopes of the element Radon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Radon
pub enum RadonIsotope {
    /// Isotope Rn193 of Radon
    Rn193,
    /// Isotope Rn194 of Radon
    Rn194,
    /// Isotope Rn195 of Radon
    Rn195,
    /// Isotope Rn196 of Radon
    Rn196,
    /// Isotope Rn197 of Radon
    Rn197,
    /// Isotope Rn198 of Radon
    Rn198,
    /// Isotope Rn199 of Radon
    Rn199,
    /// Isotope Rn200 of Radon
    Rn200,
    /// Isotope Rn201 of Radon
    Rn201,
    /// Isotope Rn202 of Radon
    Rn202,
    /// Isotope Rn203 of Radon
    Rn203,
    /// Isotope Rn204 of Radon
    Rn204,
    /// Isotope Rn205 of Radon
    Rn205,
    /// Isotope Rn206 of Radon
    Rn206,
    /// Isotope Rn207 of Radon
    Rn207,
    /// Isotope Rn208 of Radon
    Rn208,
    /// Isotope Rn209 of Radon
    Rn209,
    /// Isotope Rn210 of Radon
    Rn210,
    /// Isotope Rn211 of Radon
    Rn211,
    /// Isotope Rn212 of Radon
    Rn212,
    /// Isotope Rn213 of Radon
    Rn213,
    /// Isotope Rn214 of Radon
    Rn214,
    /// Isotope Rn215 of Radon
    Rn215,
    /// Isotope Rn216 of Radon
    Rn216,
    /// Isotope Rn217 of Radon
    Rn217,
    /// Isotope Rn218 of Radon
    Rn218,
    /// Isotope Rn219 of Radon
    Rn219,
    /// Isotope Rn220 of Radon
    Rn220,
    /// Isotope Rn221 of Radon
    Rn221,
    /// Isotope Rn222 of Radon
    Rn222,
    /// Isotope Rn223 of Radon
    Rn223,
    /// Isotope Rn224 of Radon
    Rn224,
    /// Isotope Rn225 of Radon
    Rn225,
    /// Isotope Rn226 of Radon
    Rn226,
    /// Isotope Rn227 of Radon
    Rn227,
    /// Isotope Rn228 of Radon
    Rn228,
    /// Isotope Rn229 of Radon
    Rn229,
    /// Isotope Rn230 of Radon
    Rn230,
    /// Isotope Rn231 of Radon
    Rn231,
}
impl super::RelativeAtomicMass for RadonIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rn193 => 193.009708f64,
            Self::Rn194 => 194.006144f64,
            Self::Rn195 => 195.005422f64,
            Self::Rn196 => 196.002116f64,
            Self::Rn197 => 197.001585f64,
            Self::Rn198 => 197.998679f64,
            Self::Rn199 => 198.99839f64,
            Self::Rn200 => 199.99569f64,
            Self::Rn201 => 200.995628f64,
            Self::Rn202 => 201.993264f64,
            Self::Rn203 => 202.993388f64,
            Self::Rn204 => 203.99143f64,
            Self::Rn205 => 204.991719f64,
            Self::Rn206 => 205.990214f64,
            Self::Rn207 => 206.9907303f64,
            Self::Rn208 => 207.989635f64,
            Self::Rn209 => 208.990415f64,
            Self::Rn210 => 209.9896891f64,
            Self::Rn211 => 210.9906011f64,
            Self::Rn212 => 211.9907039f64,
            Self::Rn213 => 212.9938831f64,
            Self::Rn214 => 213.995363f64,
            Self::Rn215 => 214.9987459f64,
            Self::Rn216 => 216.0002719f64,
            Self::Rn217 => 217.003928f64,
            Self::Rn218 => 218.0056016f64,
            Self::Rn219 => 219.0094804f64,
            Self::Rn220 => 220.0113941f64,
            Self::Rn221 => 221.0155371f64,
            Self::Rn222 => 222.0175782f64,
            Self::Rn223 => 223.0218893f64,
            Self::Rn224 => 224.024096f64,
            Self::Rn225 => 225.028486f64,
            Self::Rn226 => 226.030861f64,
            Self::Rn227 => 227.035304f64,
            Self::Rn228 => 228.037835f64,
            Self::Rn229 => 229.042257f64,
            Self::Rn230 => 230.04514f64,
            Self::Rn231 => 231.04987f64,
        }
    }
}
impl super::ElementVariant for RadonIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Rn
    }
}
impl super::MassNumber for RadonIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rn193 => 193u16,
            Self::Rn194 => 194u16,
            Self::Rn195 => 195u16,
            Self::Rn196 => 196u16,
            Self::Rn197 => 197u16,
            Self::Rn198 => 198u16,
            Self::Rn199 => 199u16,
            Self::Rn200 => 200u16,
            Self::Rn201 => 201u16,
            Self::Rn202 => 202u16,
            Self::Rn203 => 203u16,
            Self::Rn204 => 204u16,
            Self::Rn205 => 205u16,
            Self::Rn206 => 206u16,
            Self::Rn207 => 207u16,
            Self::Rn208 => 208u16,
            Self::Rn209 => 209u16,
            Self::Rn210 => 210u16,
            Self::Rn211 => 211u16,
            Self::Rn212 => 212u16,
            Self::Rn213 => 213u16,
            Self::Rn214 => 214u16,
            Self::Rn215 => 215u16,
            Self::Rn216 => 216u16,
            Self::Rn217 => 217u16,
            Self::Rn218 => 218u16,
            Self::Rn219 => 219u16,
            Self::Rn220 => 220u16,
            Self::Rn221 => 221u16,
            Self::Rn222 => 222u16,
            Self::Rn223 => 223u16,
            Self::Rn224 => 224u16,
            Self::Rn225 => 225u16,
            Self::Rn226 => 226u16,
            Self::Rn227 => 227u16,
            Self::Rn228 => 228u16,
            Self::Rn229 => 229u16,
            Self::Rn230 => 230u16,
            Self::Rn231 => 231u16,
        }
    }
}
impl super::IsotopicComposition for RadonIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for RadonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rn231
    }
}
impl From<RadonIsotope> for crate::Isotope {
    fn from(isotope: RadonIsotope) -> Self {
        crate::Isotope::Rn(isotope)
    }
}
impl From<RadonIsotope> for crate::Element {
    fn from(_isotope: RadonIsotope) -> Self {
        crate::Element::Rn
    }
}
impl TryFrom<u64> for RadonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            193u64 => Ok(Self::Rn193),
            194u64 => Ok(Self::Rn194),
            195u64 => Ok(Self::Rn195),
            196u64 => Ok(Self::Rn196),
            197u64 => Ok(Self::Rn197),
            198u64 => Ok(Self::Rn198),
            199u64 => Ok(Self::Rn199),
            200u64 => Ok(Self::Rn200),
            201u64 => Ok(Self::Rn201),
            202u64 => Ok(Self::Rn202),
            203u64 => Ok(Self::Rn203),
            204u64 => Ok(Self::Rn204),
            205u64 => Ok(Self::Rn205),
            206u64 => Ok(Self::Rn206),
            207u64 => Ok(Self::Rn207),
            208u64 => Ok(Self::Rn208),
            209u64 => Ok(Self::Rn209),
            210u64 => Ok(Self::Rn210),
            211u64 => Ok(Self::Rn211),
            212u64 => Ok(Self::Rn212),
            213u64 => Ok(Self::Rn213),
            214u64 => Ok(Self::Rn214),
            215u64 => Ok(Self::Rn215),
            216u64 => Ok(Self::Rn216),
            217u64 => Ok(Self::Rn217),
            218u64 => Ok(Self::Rn218),
            219u64 => Ok(Self::Rn219),
            220u64 => Ok(Self::Rn220),
            221u64 => Ok(Self::Rn221),
            222u64 => Ok(Self::Rn222),
            223u64 => Ok(Self::Rn223),
            224u64 => Ok(Self::Rn224),
            225u64 => Ok(Self::Rn225),
            226u64 => Ok(Self::Rn226),
            227u64 => Ok(Self::Rn227),
            228u64 => Ok(Self::Rn228),
            229u64 => Ok(Self::Rn229),
            230u64 => Ok(Self::Rn230),
            231u64 => Ok(Self::Rn231),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rn, value)),
        }
    }
}
impl TryFrom<u8> for RadonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for RadonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for RadonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for RadonIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Rn193 => write!(f, "Rn193"),
            Self::Rn194 => write!(f, "Rn194"),
            Self::Rn195 => write!(f, "Rn195"),
            Self::Rn196 => write!(f, "Rn196"),
            Self::Rn197 => write!(f, "Rn197"),
            Self::Rn198 => write!(f, "Rn198"),
            Self::Rn199 => write!(f, "Rn199"),
            Self::Rn200 => write!(f, "Rn200"),
            Self::Rn201 => write!(f, "Rn201"),
            Self::Rn202 => write!(f, "Rn202"),
            Self::Rn203 => write!(f, "Rn203"),
            Self::Rn204 => write!(f, "Rn204"),
            Self::Rn205 => write!(f, "Rn205"),
            Self::Rn206 => write!(f, "Rn206"),
            Self::Rn207 => write!(f, "Rn207"),
            Self::Rn208 => write!(f, "Rn208"),
            Self::Rn209 => write!(f, "Rn209"),
            Self::Rn210 => write!(f, "Rn210"),
            Self::Rn211 => write!(f, "Rn211"),
            Self::Rn212 => write!(f, "Rn212"),
            Self::Rn213 => write!(f, "Rn213"),
            Self::Rn214 => write!(f, "Rn214"),
            Self::Rn215 => write!(f, "Rn215"),
            Self::Rn216 => write!(f, "Rn216"),
            Self::Rn217 => write!(f, "Rn217"),
            Self::Rn218 => write!(f, "Rn218"),
            Self::Rn219 => write!(f, "Rn219"),
            Self::Rn220 => write!(f, "Rn220"),
            Self::Rn221 => write!(f, "Rn221"),
            Self::Rn222 => write!(f, "Rn222"),
            Self::Rn223 => write!(f, "Rn223"),
            Self::Rn224 => write!(f, "Rn224"),
            Self::Rn225 => write!(f, "Rn225"),
            Self::Rn226 => write!(f, "Rn226"),
            Self::Rn227 => write!(f, "Rn227"),
            Self::Rn228 => write!(f, "Rn228"),
            Self::Rn229 => write!(f, "Rn229"),
            Self::Rn230 => write!(f, "Rn230"),
            Self::Rn231 => write!(f, "Rn231"),
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
        for isotope in RadonIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in RadonIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Rn, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in RadonIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in RadonIsotope::iter() {
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
        let most_abundant = RadonIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in RadonIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Rn(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in RadonIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Rn);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in RadonIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = RadonIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
            let iso_u32 = RadonIsotope::try_from(u32::from(mass)).unwrap();
            assert_eq!(iso_u32, isotope);
            if let Ok(mass_u8) = u8::try_from(mass) {
                let iso_u8 = RadonIsotope::try_from(mass_u8).unwrap();
                assert_eq!(iso_u8, isotope);
            }
        }
        assert!(RadonIsotope::try_from(0_u16).is_err());
        assert!(RadonIsotope::try_from(1000_u16).is_err());
        assert!(RadonIsotope::try_from(0_u32).is_err());
        assert!(RadonIsotope::try_from(1000_u32).is_err());
        assert!(RadonIsotope::try_from(0_u8).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in RadonIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
