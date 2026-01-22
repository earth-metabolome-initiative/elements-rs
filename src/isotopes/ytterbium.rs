//! Isotopes of the element Ytterbium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Ytterbium
pub enum YtterbiumIsotope {
    /// Isotope Yb148 of Ytterbium
    Yb148,
    /// Isotope Yb149 of Ytterbium
    Yb149,
    /// Isotope Yb150 of Ytterbium
    Yb150,
    /// Isotope Yb151 of Ytterbium
    Yb151,
    /// Isotope Yb152 of Ytterbium
    Yb152,
    /// Isotope Yb153 of Ytterbium
    Yb153,
    /// Isotope Yb154 of Ytterbium
    Yb154,
    /// Isotope Yb155 of Ytterbium
    Yb155,
    /// Isotope Yb156 of Ytterbium
    Yb156,
    /// Isotope Yb157 of Ytterbium
    Yb157,
    /// Isotope Yb158 of Ytterbium
    Yb158,
    /// Isotope Yb159 of Ytterbium
    Yb159,
    /// Isotope Yb160 of Ytterbium
    Yb160,
    /// Isotope Yb161 of Ytterbium
    Yb161,
    /// Isotope Yb162 of Ytterbium
    Yb162,
    /// Isotope Yb163 of Ytterbium
    Yb163,
    /// Isotope Yb164 of Ytterbium
    Yb164,
    /// Isotope Yb165 of Ytterbium
    Yb165,
    /// Isotope Yb166 of Ytterbium
    Yb166,
    /// Isotope Yb167 of Ytterbium
    Yb167,
    /// Isotope Yb168 of Ytterbium
    Yb168,
    /// Isotope Yb169 of Ytterbium
    Yb169,
    /// Isotope Yb170 of Ytterbium
    Yb170,
    /// Isotope Yb171 of Ytterbium
    Yb171,
    /// Isotope Yb172 of Ytterbium
    Yb172,
    /// Isotope Yb173 of Ytterbium
    Yb173,
    /// Isotope Yb174 of Ytterbium
    Yb174,
    /// Isotope Yb175 of Ytterbium
    Yb175,
    /// Isotope Yb176 of Ytterbium
    Yb176,
    /// Isotope Yb177 of Ytterbium
    Yb177,
    /// Isotope Yb178 of Ytterbium
    Yb178,
    /// Isotope Yb179 of Ytterbium
    Yb179,
    /// Isotope Yb180 of Ytterbium
    Yb180,
    /// Isotope Yb181 of Ytterbium
    Yb181,
}
impl super::RelativeAtomicMass for YtterbiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Yb148 => 147.96758f64,
            Self::Yb149 => 148.96436f64,
            Self::Yb150 => 149.95852f64,
            Self::Yb151 => 150.9554f64,
            Self::Yb152 => 151.95027f64,
            Self::Yb153 => 152.94932f64,
            Self::Yb154 => 153.946396f64,
            Self::Yb155 => 154.945783f64,
            Self::Yb156 => 155.942825f64,
            Self::Yb157 => 156.942645f64,
            Self::Yb158 => 157.9398705f64,
            Self::Yb159 => 158.940055f64,
            Self::Yb160 => 159.937557f64,
            Self::Yb161 => 160.937907f64,
            Self::Yb162 => 161.935774f64,
            Self::Yb163 => 162.93634f64,
            Self::Yb164 => 163.934495f64,
            Self::Yb165 => 164.93527f64,
            Self::Yb166 => 165.9338747f64,
            Self::Yb167 => 166.934953f64,
            Self::Yb168 => 167.9338896f64,
            Self::Yb169 => 168.9351825f64,
            Self::Yb170 => 169.9347664f64,
            Self::Yb171 => 170.9363302f64,
            Self::Yb172 => 171.9363859f64,
            Self::Yb173 => 172.9382151f64,
            Self::Yb174 => 173.9388664f64,
            Self::Yb175 => 174.9412808f64,
            Self::Yb176 => 175.9425764f64,
            Self::Yb177 => 176.9452656f64,
            Self::Yb178 => 177.946651f64,
            Self::Yb179 => 178.95004f64,
            Self::Yb180 => 179.95212f64,
            Self::Yb181 => 180.95589f64,
        }
    }
}
impl super::ElementVariant for YtterbiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Yb
    }
}
impl super::MassNumber for YtterbiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Yb148 => 148u16,
            Self::Yb149 => 149u16,
            Self::Yb150 => 150u16,
            Self::Yb151 => 151u16,
            Self::Yb152 => 152u16,
            Self::Yb153 => 153u16,
            Self::Yb154 => 154u16,
            Self::Yb155 => 155u16,
            Self::Yb156 => 156u16,
            Self::Yb157 => 157u16,
            Self::Yb158 => 158u16,
            Self::Yb159 => 159u16,
            Self::Yb160 => 160u16,
            Self::Yb161 => 161u16,
            Self::Yb162 => 162u16,
            Self::Yb163 => 163u16,
            Self::Yb164 => 164u16,
            Self::Yb165 => 165u16,
            Self::Yb166 => 166u16,
            Self::Yb167 => 167u16,
            Self::Yb168 => 168u16,
            Self::Yb169 => 169u16,
            Self::Yb170 => 170u16,
            Self::Yb171 => 171u16,
            Self::Yb172 => 172u16,
            Self::Yb173 => 173u16,
            Self::Yb174 => 174u16,
            Self::Yb175 => 175u16,
            Self::Yb176 => 176u16,
            Self::Yb177 => 177u16,
            Self::Yb178 => 178u16,
            Self::Yb179 => 179u16,
            Self::Yb180 => 180u16,
            Self::Yb181 => 181u16,
        }
    }
}
impl super::IsotopicComposition for YtterbiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Yb168 => Some(0.00123f64),
            Self::Yb170 => Some(0.02982f64),
            Self::Yb171 => Some(0.1409f64),
            Self::Yb172 => Some(0.2168f64),
            Self::Yb173 => Some(0.16103f64),
            Self::Yb174 => Some(0.32026f64),
            Self::Yb176 => Some(0.12996f64),
            Self::Yb148
            | Self::Yb149
            | Self::Yb150
            | Self::Yb151
            | Self::Yb152
            | Self::Yb153
            | Self::Yb154
            | Self::Yb155
            | Self::Yb156
            | Self::Yb157
            | Self::Yb158
            | Self::Yb159
            | Self::Yb160
            | Self::Yb161
            | Self::Yb162
            | Self::Yb163
            | Self::Yb164
            | Self::Yb165
            | Self::Yb166
            | Self::Yb167
            | Self::Yb169
            | Self::Yb175
            | Self::Yb177
            | Self::Yb178
            | Self::Yb179
            | Self::Yb180
            | Self::Yb181 => None,
        }
    }
}
impl super::MostAbundantIsotope for YtterbiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Yb174
    }
}
impl From<YtterbiumIsotope> for crate::Isotope {
    fn from(isotope: YtterbiumIsotope) -> Self {
        crate::Isotope::Yb(isotope)
    }
}
impl From<YtterbiumIsotope> for crate::Element {
    fn from(_isotope: YtterbiumIsotope) -> Self {
        crate::Element::Yb
    }
}
impl TryFrom<u64> for YtterbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            148u64 => Ok(Self::Yb148),
            149u64 => Ok(Self::Yb149),
            150u64 => Ok(Self::Yb150),
            151u64 => Ok(Self::Yb151),
            152u64 => Ok(Self::Yb152),
            153u64 => Ok(Self::Yb153),
            154u64 => Ok(Self::Yb154),
            155u64 => Ok(Self::Yb155),
            156u64 => Ok(Self::Yb156),
            157u64 => Ok(Self::Yb157),
            158u64 => Ok(Self::Yb158),
            159u64 => Ok(Self::Yb159),
            160u64 => Ok(Self::Yb160),
            161u64 => Ok(Self::Yb161),
            162u64 => Ok(Self::Yb162),
            163u64 => Ok(Self::Yb163),
            164u64 => Ok(Self::Yb164),
            165u64 => Ok(Self::Yb165),
            166u64 => Ok(Self::Yb166),
            167u64 => Ok(Self::Yb167),
            168u64 => Ok(Self::Yb168),
            169u64 => Ok(Self::Yb169),
            170u64 => Ok(Self::Yb170),
            171u64 => Ok(Self::Yb171),
            172u64 => Ok(Self::Yb172),
            173u64 => Ok(Self::Yb173),
            174u64 => Ok(Self::Yb174),
            175u64 => Ok(Self::Yb175),
            176u64 => Ok(Self::Yb176),
            177u64 => Ok(Self::Yb177),
            178u64 => Ok(Self::Yb178),
            179u64 => Ok(Self::Yb179),
            180u64 => Ok(Self::Yb180),
            181u64 => Ok(Self::Yb181),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Yb, value)),
        }
    }
}
impl TryFrom<u8> for YtterbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for YtterbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for YtterbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for YtterbiumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Yb148 => write!(f, "Yb148"),
            Self::Yb149 => write!(f, "Yb149"),
            Self::Yb150 => write!(f, "Yb150"),
            Self::Yb151 => write!(f, "Yb151"),
            Self::Yb152 => write!(f, "Yb152"),
            Self::Yb153 => write!(f, "Yb153"),
            Self::Yb154 => write!(f, "Yb154"),
            Self::Yb155 => write!(f, "Yb155"),
            Self::Yb156 => write!(f, "Yb156"),
            Self::Yb157 => write!(f, "Yb157"),
            Self::Yb158 => write!(f, "Yb158"),
            Self::Yb159 => write!(f, "Yb159"),
            Self::Yb160 => write!(f, "Yb160"),
            Self::Yb161 => write!(f, "Yb161"),
            Self::Yb162 => write!(f, "Yb162"),
            Self::Yb163 => write!(f, "Yb163"),
            Self::Yb164 => write!(f, "Yb164"),
            Self::Yb165 => write!(f, "Yb165"),
            Self::Yb166 => write!(f, "Yb166"),
            Self::Yb167 => write!(f, "Yb167"),
            Self::Yb168 => write!(f, "Yb168"),
            Self::Yb169 => write!(f, "Yb169"),
            Self::Yb170 => write!(f, "Yb170"),
            Self::Yb171 => write!(f, "Yb171"),
            Self::Yb172 => write!(f, "Yb172"),
            Self::Yb173 => write!(f, "Yb173"),
            Self::Yb174 => write!(f, "Yb174"),
            Self::Yb175 => write!(f, "Yb175"),
            Self::Yb176 => write!(f, "Yb176"),
            Self::Yb177 => write!(f, "Yb177"),
            Self::Yb178 => write!(f, "Yb178"),
            Self::Yb179 => write!(f, "Yb179"),
            Self::Yb180 => write!(f, "Yb180"),
            Self::Yb181 => write!(f, "Yb181"),
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
        for isotope in YtterbiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in YtterbiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Yb, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in YtterbiumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in YtterbiumIsotope::iter() {
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
        let most_abundant = YtterbiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in YtterbiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Yb(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in YtterbiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Yb);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in YtterbiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = YtterbiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(YtterbiumIsotope::try_from(0_u16).is_err());
        assert!(YtterbiumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in YtterbiumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
