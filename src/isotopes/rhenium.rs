//! Isotopes of the element Rhenium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Rhenium
pub enum RheniumIsotope {
    /// Isotope Re159 of Rhenium
    Re159,
    /// Isotope Re160 of Rhenium
    Re160,
    /// Isotope Re161 of Rhenium
    Re161,
    /// Isotope Re162 of Rhenium
    Re162,
    /// Isotope Re163 of Rhenium
    Re163,
    /// Isotope Re164 of Rhenium
    Re164,
    /// Isotope Re165 of Rhenium
    Re165,
    /// Isotope Re166 of Rhenium
    Re166,
    /// Isotope Re167 of Rhenium
    Re167,
    /// Isotope Re168 of Rhenium
    Re168,
    /// Isotope Re169 of Rhenium
    Re169,
    /// Isotope Re170 of Rhenium
    Re170,
    /// Isotope Re171 of Rhenium
    Re171,
    /// Isotope Re172 of Rhenium
    Re172,
    /// Isotope Re173 of Rhenium
    Re173,
    /// Isotope Re174 of Rhenium
    Re174,
    /// Isotope Re175 of Rhenium
    Re175,
    /// Isotope Re176 of Rhenium
    Re176,
    /// Isotope Re177 of Rhenium
    Re177,
    /// Isotope Re178 of Rhenium
    Re178,
    /// Isotope Re179 of Rhenium
    Re179,
    /// Isotope Re180 of Rhenium
    Re180,
    /// Isotope Re181 of Rhenium
    Re181,
    /// Isotope Re182 of Rhenium
    Re182,
    /// Isotope Re183 of Rhenium
    Re183,
    /// Isotope Re184 of Rhenium
    Re184,
    /// Isotope Re185 of Rhenium
    Re185,
    /// Isotope Re186 of Rhenium
    Re186,
    /// Isotope Re187 of Rhenium
    Re187,
    /// Isotope Re188 of Rhenium
    Re188,
    /// Isotope Re189 of Rhenium
    Re189,
    /// Isotope Re190 of Rhenium
    Re190,
    /// Isotope Re191 of Rhenium
    Re191,
    /// Isotope Re192 of Rhenium
    Re192,
    /// Isotope Re193 of Rhenium
    Re193,
    /// Isotope Re194 of Rhenium
    Re194,
    /// Isotope Re195 of Rhenium
    Re195,
    /// Isotope Re196 of Rhenium
    Re196,
    /// Isotope Re197 of Rhenium
    Re197,
    /// Isotope Re198 of Rhenium
    Re198,
}
impl super::RelativeAtomicMass for RheniumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Re159 => 158.98418f64,
            Self::Re160 => 159.98182f64,
            Self::Re161 => 160.97757f64,
            Self::Re162 => 161.97584f64,
            Self::Re163 => 162.97208f64,
            Self::Re164 => 163.970453f64,
            Self::Re165 => 164.967103f64,
            Self::Re166 => 165.965761f64,
            Self::Re167 => 166.962595f64,
            Self::Re168 => 167.961573f64,
            Self::Re169 => 168.958766f64,
            Self::Re170 => 169.95822f64,
            Self::Re171 => 170.955716f64,
            Self::Re172 => 171.95542f64,
            Self::Re173 => 172.953243f64,
            Self::Re174 => 173.953115f64,
            Self::Re175 => 174.951381f64,
            Self::Re176 => 175.951623f64,
            Self::Re177 => 176.950328f64,
            Self::Re178 => 177.950989f64,
            Self::Re179 => 178.949989f64,
            Self::Re180 => 179.950792f64,
            Self::Re181 => 180.950058f64,
            Self::Re182 => 181.95121f64,
            Self::Re183 => 182.9508196f64,
            Self::Re184 => 183.9525228f64,
            Self::Re185 => 184.9529545f64,
            Self::Re186 => 185.9549856f64,
            Self::Re187 => 186.9557501f64,
            Self::Re188 => 187.9581115f64,
            Self::Re189 => 188.959226f64,
            Self::Re190 => 189.961744f64,
            Self::Re191 => 190.963122f64,
            Self::Re192 => 191.966088f64,
            Self::Re193 => 192.967541f64,
            Self::Re194 => 193.97076f64,
            Self::Re195 => 194.97254f64,
            Self::Re196 => 195.9758f64,
            Self::Re197 => 196.97799f64,
            Self::Re198 => 197.9816f64,
        }
    }
}
impl super::ElementVariant for RheniumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Re
    }
}
impl super::MassNumber for RheniumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Re159 => 159u16,
            Self::Re160 => 160u16,
            Self::Re161 => 161u16,
            Self::Re162 => 162u16,
            Self::Re163 => 163u16,
            Self::Re164 => 164u16,
            Self::Re165 => 165u16,
            Self::Re166 => 166u16,
            Self::Re167 => 167u16,
            Self::Re168 => 168u16,
            Self::Re169 => 169u16,
            Self::Re170 => 170u16,
            Self::Re171 => 171u16,
            Self::Re172 => 172u16,
            Self::Re173 => 173u16,
            Self::Re174 => 174u16,
            Self::Re175 => 175u16,
            Self::Re176 => 176u16,
            Self::Re177 => 177u16,
            Self::Re178 => 178u16,
            Self::Re179 => 179u16,
            Self::Re180 => 180u16,
            Self::Re181 => 181u16,
            Self::Re182 => 182u16,
            Self::Re183 => 183u16,
            Self::Re184 => 184u16,
            Self::Re185 => 185u16,
            Self::Re186 => 186u16,
            Self::Re187 => 187u16,
            Self::Re188 => 188u16,
            Self::Re189 => 189u16,
            Self::Re190 => 190u16,
            Self::Re191 => 191u16,
            Self::Re192 => 192u16,
            Self::Re193 => 193u16,
            Self::Re194 => 194u16,
            Self::Re195 => 195u16,
            Self::Re196 => 196u16,
            Self::Re197 => 197u16,
            Self::Re198 => 198u16,
        }
    }
}
impl super::IsotopicComposition for RheniumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Re185 => Some(0.374f64),
            Self::Re187 => Some(0.626f64),
            _ => None,
        }
    }
}
impl super::MostAbundantIsotope for RheniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Re187
    }
}
impl From<RheniumIsotope> for crate::Isotope {
    fn from(isotope: RheniumIsotope) -> Self {
        crate::Isotope::Re(isotope)
    }
}
impl From<RheniumIsotope> for crate::Element {
    fn from(_isotope: RheniumIsotope) -> Self {
        crate::Element::Re
    }
}
impl TryFrom<u64> for RheniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            159u64 => Ok(Self::Re159),
            160u64 => Ok(Self::Re160),
            161u64 => Ok(Self::Re161),
            162u64 => Ok(Self::Re162),
            163u64 => Ok(Self::Re163),
            164u64 => Ok(Self::Re164),
            165u64 => Ok(Self::Re165),
            166u64 => Ok(Self::Re166),
            167u64 => Ok(Self::Re167),
            168u64 => Ok(Self::Re168),
            169u64 => Ok(Self::Re169),
            170u64 => Ok(Self::Re170),
            171u64 => Ok(Self::Re171),
            172u64 => Ok(Self::Re172),
            173u64 => Ok(Self::Re173),
            174u64 => Ok(Self::Re174),
            175u64 => Ok(Self::Re175),
            176u64 => Ok(Self::Re176),
            177u64 => Ok(Self::Re177),
            178u64 => Ok(Self::Re178),
            179u64 => Ok(Self::Re179),
            180u64 => Ok(Self::Re180),
            181u64 => Ok(Self::Re181),
            182u64 => Ok(Self::Re182),
            183u64 => Ok(Self::Re183),
            184u64 => Ok(Self::Re184),
            185u64 => Ok(Self::Re185),
            186u64 => Ok(Self::Re186),
            187u64 => Ok(Self::Re187),
            188u64 => Ok(Self::Re188),
            189u64 => Ok(Self::Re189),
            190u64 => Ok(Self::Re190),
            191u64 => Ok(Self::Re191),
            192u64 => Ok(Self::Re192),
            193u64 => Ok(Self::Re193),
            194u64 => Ok(Self::Re194),
            195u64 => Ok(Self::Re195),
            196u64 => Ok(Self::Re196),
            197u64 => Ok(Self::Re197),
            198u64 => Ok(Self::Re198),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Re, value)),
        }
    }
}
impl TryFrom<u8> for RheniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for RheniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for RheniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for RheniumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Re159 => write!(f, "Re159"),
            Self::Re160 => write!(f, "Re160"),
            Self::Re161 => write!(f, "Re161"),
            Self::Re162 => write!(f, "Re162"),
            Self::Re163 => write!(f, "Re163"),
            Self::Re164 => write!(f, "Re164"),
            Self::Re165 => write!(f, "Re165"),
            Self::Re166 => write!(f, "Re166"),
            Self::Re167 => write!(f, "Re167"),
            Self::Re168 => write!(f, "Re168"),
            Self::Re169 => write!(f, "Re169"),
            Self::Re170 => write!(f, "Re170"),
            Self::Re171 => write!(f, "Re171"),
            Self::Re172 => write!(f, "Re172"),
            Self::Re173 => write!(f, "Re173"),
            Self::Re174 => write!(f, "Re174"),
            Self::Re175 => write!(f, "Re175"),
            Self::Re176 => write!(f, "Re176"),
            Self::Re177 => write!(f, "Re177"),
            Self::Re178 => write!(f, "Re178"),
            Self::Re179 => write!(f, "Re179"),
            Self::Re180 => write!(f, "Re180"),
            Self::Re181 => write!(f, "Re181"),
            Self::Re182 => write!(f, "Re182"),
            Self::Re183 => write!(f, "Re183"),
            Self::Re184 => write!(f, "Re184"),
            Self::Re185 => write!(f, "Re185"),
            Self::Re186 => write!(f, "Re186"),
            Self::Re187 => write!(f, "Re187"),
            Self::Re188 => write!(f, "Re188"),
            Self::Re189 => write!(f, "Re189"),
            Self::Re190 => write!(f, "Re190"),
            Self::Re191 => write!(f, "Re191"),
            Self::Re192 => write!(f, "Re192"),
            Self::Re193 => write!(f, "Re193"),
            Self::Re194 => write!(f, "Re194"),
            Self::Re195 => write!(f, "Re195"),
            Self::Re196 => write!(f, "Re196"),
            Self::Re197 => write!(f, "Re197"),
            Self::Re198 => write!(f, "Re198"),
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
        for isotope in RheniumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in RheniumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Re, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in RheniumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in RheniumIsotope::iter() {
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
        let most_abundant = RheniumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in RheniumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Re(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in RheniumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Re);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in RheniumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = RheniumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(RheniumIsotope::try_from(0_u16).is_err());
        assert!(RheniumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in RheniumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
