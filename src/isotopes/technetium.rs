//! Isotopes of the element Technetium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Isotopes of the element Technetium
pub enum TechnetiumIsotope {
    /// Isotope Tc85 of Technetium
    Tc85,
    /// Isotope Tc86 of Technetium
    Tc86,
    /// Isotope Tc87 of Technetium
    Tc87,
    /// Isotope Tc88 of Technetium
    Tc88,
    /// Isotope Tc89 of Technetium
    Tc89,
    /// Isotope Tc90 of Technetium
    Tc90,
    /// Isotope Tc91 of Technetium
    Tc91,
    /// Isotope Tc92 of Technetium
    Tc92,
    /// Isotope Tc93 of Technetium
    Tc93,
    /// Isotope Tc94 of Technetium
    Tc94,
    /// Isotope Tc95 of Technetium
    Tc95,
    /// Isotope Tc96 of Technetium
    Tc96,
    /// Isotope Tc97 of Technetium
    Tc97,
    /// Isotope Tc98 of Technetium
    Tc98,
    /// Isotope Tc99 of Technetium
    Tc99,
    /// Isotope Tc100 of Technetium
    Tc100,
    /// Isotope Tc101 of Technetium
    Tc101,
    /// Isotope Tc102 of Technetium
    Tc102,
    /// Isotope Tc103 of Technetium
    Tc103,
    /// Isotope Tc104 of Technetium
    Tc104,
    /// Isotope Tc105 of Technetium
    Tc105,
    /// Isotope Tc106 of Technetium
    Tc106,
    /// Isotope Tc107 of Technetium
    Tc107,
    /// Isotope Tc108 of Technetium
    Tc108,
    /// Isotope Tc109 of Technetium
    Tc109,
    /// Isotope Tc110 of Technetium
    Tc110,
    /// Isotope Tc111 of Technetium
    Tc111,
    /// Isotope Tc112 of Technetium
    Tc112,
    /// Isotope Tc113 of Technetium
    Tc113,
    /// Isotope Tc114 of Technetium
    Tc114,
    /// Isotope Tc115 of Technetium
    Tc115,
    /// Isotope Tc116 of Technetium
    Tc116,
    /// Isotope Tc117 of Technetium
    Tc117,
    /// Isotope Tc118 of Technetium
    Tc118,
    /// Isotope Tc119 of Technetium
    Tc119,
    /// Isotope Tc120 of Technetium
    Tc120,
}
impl super::RelativeAtomicMass for TechnetiumIsotope {
    #[inline]
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Tc85 => 84.95058f64,
            Self::Tc86 => 85.94493f64,
            Self::Tc87 => 86.9380672f64,
            Self::Tc88 => 87.93378f64,
            Self::Tc89 => 88.9276487f64,
            Self::Tc90 => 89.9240739f64,
            Self::Tc91 => 90.9184254f64,
            Self::Tc92 => 91.9152698f64,
            Self::Tc93 => 92.910246f64,
            Self::Tc94 => 93.9096536f64,
            Self::Tc95 => 94.9076536f64,
            Self::Tc96 => 95.907868f64,
            Self::Tc97 => 96.9063667f64,
            Self::Tc98 => 97.9072124f64,
            Self::Tc99 => 98.9062508f64,
            Self::Tc100 => 99.9076539f64,
            Self::Tc101 => 100.907309f64,
            Self::Tc102 => 101.9092097f64,
            Self::Tc103 => 102.909176f64,
            Self::Tc104 => 103.911425f64,
            Self::Tc105 => 104.911655f64,
            Self::Tc106 => 105.914358f64,
            Self::Tc107 => 106.9154606f64,
            Self::Tc108 => 107.9184957f64,
            Self::Tc109 => 108.920256f64,
            Self::Tc110 => 109.923744f64,
            Self::Tc111 => 110.925901f64,
            Self::Tc112 => 111.9299458f64,
            Self::Tc113 => 112.932569f64,
            Self::Tc114 => 113.93691f64,
            Self::Tc115 => 114.93998f64,
            Self::Tc116 => 115.94476f64,
            Self::Tc117 => 116.94806f64,
            Self::Tc118 => 117.95299f64,
            Self::Tc119 => 118.95666f64,
            Self::Tc120 => 119.96187f64,
        }
    }
}
impl super::ElementVariant for TechnetiumIsotope {
    #[inline]
    fn element(&self) -> crate::Element {
        crate::Element::Tc
    }
}
impl super::MassNumber for TechnetiumIsotope {
    #[inline]
    fn mass_number(&self) -> u16 {
        match self {
            Self::Tc85 => 85u16,
            Self::Tc86 => 86u16,
            Self::Tc87 => 87u16,
            Self::Tc88 => 88u16,
            Self::Tc89 => 89u16,
            Self::Tc90 => 90u16,
            Self::Tc91 => 91u16,
            Self::Tc92 => 92u16,
            Self::Tc93 => 93u16,
            Self::Tc94 => 94u16,
            Self::Tc95 => 95u16,
            Self::Tc96 => 96u16,
            Self::Tc97 => 97u16,
            Self::Tc98 => 98u16,
            Self::Tc99 => 99u16,
            Self::Tc100 => 100u16,
            Self::Tc101 => 101u16,
            Self::Tc102 => 102u16,
            Self::Tc103 => 103u16,
            Self::Tc104 => 104u16,
            Self::Tc105 => 105u16,
            Self::Tc106 => 106u16,
            Self::Tc107 => 107u16,
            Self::Tc108 => 108u16,
            Self::Tc109 => 109u16,
            Self::Tc110 => 110u16,
            Self::Tc111 => 111u16,
            Self::Tc112 => 112u16,
            Self::Tc113 => 113u16,
            Self::Tc114 => 114u16,
            Self::Tc115 => 115u16,
            Self::Tc116 => 116u16,
            Self::Tc117 => 117u16,
            Self::Tc118 => 118u16,
            Self::Tc119 => 119u16,
            Self::Tc120 => 120u16,
        }
    }
}
impl super::IsotopicComposition for TechnetiumIsotope {
    #[inline]
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for TechnetiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Tc120
    }
}
impl From<TechnetiumIsotope> for crate::Isotope {
    fn from(isotope: TechnetiumIsotope) -> Self {
        crate::Isotope::Tc(isotope)
    }
}
impl From<TechnetiumIsotope> for crate::Element {
    fn from(_isotope: TechnetiumIsotope) -> Self {
        crate::Element::Tc
    }
}
impl TryFrom<u64> for TechnetiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            85u64 => Ok(Self::Tc85),
            86u64 => Ok(Self::Tc86),
            87u64 => Ok(Self::Tc87),
            88u64 => Ok(Self::Tc88),
            89u64 => Ok(Self::Tc89),
            90u64 => Ok(Self::Tc90),
            91u64 => Ok(Self::Tc91),
            92u64 => Ok(Self::Tc92),
            93u64 => Ok(Self::Tc93),
            94u64 => Ok(Self::Tc94),
            95u64 => Ok(Self::Tc95),
            96u64 => Ok(Self::Tc96),
            97u64 => Ok(Self::Tc97),
            98u64 => Ok(Self::Tc98),
            99u64 => Ok(Self::Tc99),
            100u64 => Ok(Self::Tc100),
            101u64 => Ok(Self::Tc101),
            102u64 => Ok(Self::Tc102),
            103u64 => Ok(Self::Tc103),
            104u64 => Ok(Self::Tc104),
            105u64 => Ok(Self::Tc105),
            106u64 => Ok(Self::Tc106),
            107u64 => Ok(Self::Tc107),
            108u64 => Ok(Self::Tc108),
            109u64 => Ok(Self::Tc109),
            110u64 => Ok(Self::Tc110),
            111u64 => Ok(Self::Tc111),
            112u64 => Ok(Self::Tc112),
            113u64 => Ok(Self::Tc113),
            114u64 => Ok(Self::Tc114),
            115u64 => Ok(Self::Tc115),
            116u64 => Ok(Self::Tc116),
            117u64 => Ok(Self::Tc117),
            118u64 => Ok(Self::Tc118),
            119u64 => Ok(Self::Tc119),
            120u64 => Ok(Self::Tc120),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Tc, value)),
        }
    }
}
impl TryFrom<u8> for TechnetiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u16> for TechnetiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl TryFrom<u32> for TechnetiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u64::from(value))
    }
}
impl core::fmt::Display for TechnetiumIsotope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tc85 => write!(f, "Tc85"),
            Self::Tc86 => write!(f, "Tc86"),
            Self::Tc87 => write!(f, "Tc87"),
            Self::Tc88 => write!(f, "Tc88"),
            Self::Tc89 => write!(f, "Tc89"),
            Self::Tc90 => write!(f, "Tc90"),
            Self::Tc91 => write!(f, "Tc91"),
            Self::Tc92 => write!(f, "Tc92"),
            Self::Tc93 => write!(f, "Tc93"),
            Self::Tc94 => write!(f, "Tc94"),
            Self::Tc95 => write!(f, "Tc95"),
            Self::Tc96 => write!(f, "Tc96"),
            Self::Tc97 => write!(f, "Tc97"),
            Self::Tc98 => write!(f, "Tc98"),
            Self::Tc99 => write!(f, "Tc99"),
            Self::Tc100 => write!(f, "Tc100"),
            Self::Tc101 => write!(f, "Tc101"),
            Self::Tc102 => write!(f, "Tc102"),
            Self::Tc103 => write!(f, "Tc103"),
            Self::Tc104 => write!(f, "Tc104"),
            Self::Tc105 => write!(f, "Tc105"),
            Self::Tc106 => write!(f, "Tc106"),
            Self::Tc107 => write!(f, "Tc107"),
            Self::Tc108 => write!(f, "Tc108"),
            Self::Tc109 => write!(f, "Tc109"),
            Self::Tc110 => write!(f, "Tc110"),
            Self::Tc111 => write!(f, "Tc111"),
            Self::Tc112 => write!(f, "Tc112"),
            Self::Tc113 => write!(f, "Tc113"),
            Self::Tc114 => write!(f, "Tc114"),
            Self::Tc115 => write!(f, "Tc115"),
            Self::Tc116 => write!(f, "Tc116"),
            Self::Tc117 => write!(f, "Tc117"),
            Self::Tc118 => write!(f, "Tc118"),
            Self::Tc119 => write!(f, "Tc119"),
            Self::Tc120 => write!(f, "Tc120"),
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
        for isotope in TechnetiumIsotope::iter() {
            let mass = isotope.relative_atomic_mass();
            assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
        }
    }
    #[test]
    fn test_element() {
        for isotope in TechnetiumIsotope::iter() {
            let element = isotope.element();
            assert_eq!(element, crate::Element::Tc, "Element should be correct for {isotope:?}");
        }
    }
    #[test]
    fn test_mass_number() {
        for isotope in TechnetiumIsotope::iter() {
            let mass_number = isotope.mass_number();
            assert!(
                mass_number > 0 && mass_number < 300,
                "Mass number should be reasonable for {isotope:?}"
            );
        }
    }
    #[test]
    fn test_isotopic_composition() {
        for isotope in TechnetiumIsotope::iter() {
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
        let most_abundant = TechnetiumIsotope::most_abundant_isotope();
        let _ = most_abundant.relative_atomic_mass();
    }
    #[test]
    fn test_from_isotope() {
        for isotope in TechnetiumIsotope::iter() {
            let iso: crate::Isotope = isotope.into();
            match iso {
                crate::Isotope::Tc(i) => assert_eq!(i, isotope),
                _ => panic!("Wrong isotope type"),
            }
        }
    }
    #[test]
    fn test_from_element() {
        for isotope in TechnetiumIsotope::iter() {
            let elem: crate::Element = isotope.into();
            assert_eq!(elem, crate::Element::Tc);
        }
    }
    #[test]
    fn test_try_from_mass_number() {
        for isotope in TechnetiumIsotope::iter() {
            let mass = isotope.mass_number();
            let iso = TechnetiumIsotope::try_from(mass).unwrap();
            assert_eq!(iso, isotope);
        }
        assert!(TechnetiumIsotope::try_from(0_u16).is_err());
        assert!(TechnetiumIsotope::try_from(1000_u16).is_err());
    }
    #[test]
    fn test_display() {
        for isotope in TechnetiumIsotope::iter() {
            let s = alloc::format!("{isotope}");
            assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
        }
    }
}
