//! Pauling electronegativity values for chemical elements.

use crate::isotopes::ElementVariant;

/// Pauling electronegativity of chemical elements.
pub trait Electronegativity {
    /// Returns the Pauling electronegativity of the element, if known.
    ///
    /// Returns `None` for elements without reliable Pauling-scale data:
    /// He, Ne, Ar (no compounds), Pm, Eu, Tb, Yb (no measured Pauling value),
    /// Rn, Lr, and all superheavy elements (Z >= 104).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Electronegativity, Element};
    ///
    /// assert_eq!(Element::H.pauling_electronegativity(), Some(2.20));
    /// assert_eq!(Element::He.pauling_electronegativity(), None);
    /// ```
    fn pauling_electronegativity(&self) -> Option<f64>;
}

impl Electronegativity for crate::Element {
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    fn pauling_electronegativity(&self) -> Option<f64> {
        match self {
            Self::H => Some(2.20),
            Self::He => None,
            Self::Li => Some(0.98),
            Self::Be => Some(1.57),
            Self::B => Some(2.04),
            Self::C => Some(2.55),
            Self::N => Some(3.04),
            Self::O => Some(3.44),
            Self::F => Some(3.98),
            Self::Ne => None,
            Self::Na => Some(0.93),
            Self::Mg => Some(1.31),
            Self::Al => Some(1.61),
            Self::Si => Some(1.90),
            Self::P => Some(2.19),
            Self::S => Some(2.58),
            Self::Cl => Some(3.16),
            Self::Ar => None,
            Self::K => Some(0.82),
            Self::Ca => Some(1.00),
            Self::Sc => Some(1.36),
            Self::Ti => Some(1.54),
            Self::V => Some(1.63),
            Self::Cr => Some(1.66),
            Self::Mn => Some(1.55),
            Self::Fe => Some(1.83),
            Self::Co => Some(1.88),
            Self::Ni => Some(1.91),
            Self::Cu => Some(1.90),
            Self::Zn => Some(1.65),
            Self::Ga => Some(1.81),
            Self::Ge => Some(2.01),
            Self::As => Some(2.18),
            Self::Se => Some(2.55),
            Self::Br => Some(2.96),
            Self::Kr => Some(3.00),
            Self::Rb => Some(0.82),
            Self::Sr => Some(0.95),
            Self::Y => Some(1.22),
            Self::Zr => Some(1.33),
            Self::Nb => Some(1.60),
            Self::Mo => Some(2.16),
            Self::Tc => Some(1.90),
            Self::Ru => Some(2.20),
            Self::Rh => Some(2.28),
            Self::Pd => Some(2.20),
            Self::Ag => Some(1.93),
            Self::Cd => Some(1.69),
            Self::In => Some(1.78),
            Self::Sn => Some(1.96),
            Self::Sb => Some(2.05),
            Self::Te => Some(2.10),
            Self::I => Some(2.66),
            Self::Xe => Some(2.60),
            Self::Cs => Some(0.79),
            Self::Ba => Some(0.89),
            Self::La => Some(1.10),
            Self::Ce => Some(1.12),
            Self::Pr => Some(1.13),
            Self::Nd => Some(1.14),
            Self::Pm => None,
            Self::Sm => Some(1.17),
            Self::Eu => None,
            Self::Gd => Some(1.20),
            Self::Tb => None,
            Self::Dy => Some(1.22),
            Self::Ho => Some(1.23),
            Self::Er => Some(1.24),
            Self::Tm => Some(1.25),
            Self::Yb => None,
            Self::Lu => Some(1.27),
            Self::Hf => Some(1.30),
            Self::Ta => Some(1.50),
            Self::W => Some(2.36),
            Self::Re => Some(1.90),
            Self::Os => Some(2.20),
            Self::Ir => Some(2.20),
            Self::Pt => Some(2.28),
            Self::Au => Some(2.54),
            Self::Hg => Some(2.00),
            Self::Tl => Some(1.62),
            Self::Pb => Some(2.33),
            Self::Bi => Some(2.02),
            Self::Po => Some(2.00),
            Self::At => Some(2.20),
            Self::Rn => None,
            Self::Fr => Some(0.70),
            Self::Ra => Some(0.90),
            Self::Ac => Some(1.10),
            Self::Th => Some(1.30),
            Self::Pa => Some(1.50),
            Self::U => Some(1.38),
            Self::Np => Some(1.36),
            Self::Pu => Some(1.28),
            Self::Am => Some(1.30),
            Self::Cm => Some(1.30),
            Self::Bk => Some(1.30),
            Self::Cf => Some(1.30),
            Self::Es => Some(1.30),
            Self::Fm => Some(1.30),
            Self::Md => Some(1.30),
            Self::No => Some(1.30),
            Self::Lr => None,
            // Superheavy elements (Z >= 104): no measured values
            Self::Rf
            | Self::Db
            | Self::Sg
            | Self::Bh
            | Self::Hs
            | Self::Mt
            | Self::Ds
            | Self::Rg
            | Self::Cn
            | Self::Nh
            | Self::Fl
            | Self::Mc
            | Self::Lv
            | Self::Ts
            | Self::Og => None,
        }
    }
}

impl Electronegativity for crate::Isotope {
    fn pauling_electronegativity(&self) -> Option<f64> {
        self.element().pauling_electronegativity()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Electronegativity;

    #[test]
    fn test_electronegativity_range() {
        for element in crate::Element::iter() {
            if let Some(en) = element.pauling_electronegativity() {
                assert!(
                    en > 0.0 && en <= 4.0,
                    "Electronegativity should be in (0.0, 4.0] for {element:?}, got {en}",
                );
            }
        }
    }

    #[test]
    fn test_fluorine_highest() {
        let f_en = crate::Element::F.pauling_electronegativity().unwrap();
        for element in crate::Element::iter() {
            if let Some(en) = element.pauling_electronegativity() {
                assert!(
                    en <= f_en,
                    "Fluorine should have highest electronegativity, but {element:?} has {en} > {f_en}",
                );
            }
        }
    }

    #[test]
    fn test_francium_lowest() {
        let fr_en = crate::Element::Fr.pauling_electronegativity().unwrap();
        for element in crate::Element::iter() {
            if let Some(en) = element.pauling_electronegativity() {
                assert!(
                    en >= fr_en,
                    "Fr should have lowest electronegativity, but {element:?} has {en} < {fr_en}",
                );
            }
        }
    }

    #[test]
    fn test_noble_gas_none() {
        assert!(crate::Element::He.pauling_electronegativity().is_none());
        assert!(crate::Element::Ne.pauling_electronegativity().is_none());
        assert!(crate::Element::Ar.pauling_electronegativity().is_none());
        assert!(crate::Element::Rn.pauling_electronegativity().is_none());
    }

    #[test]
    fn test_noble_gas_some() {
        assert!(crate::Element::Kr.pauling_electronegativity().is_some());
        assert!(crate::Element::Xe.pauling_electronegativity().is_some());
    }

    #[test]
    fn test_superheavy_none() {
        assert!(crate::Element::Rf.pauling_electronegativity().is_none());
        assert!(crate::Element::Og.pauling_electronegativity().is_none());
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let element_en = element.pauling_electronegativity();
            for isotope in element.isotopes() {
                let isotope_en = isotope.pauling_electronegativity();
                assert_eq!(
                    element_en, isotope_en,
                    "Electronegativity mismatch for isotope {isotope:?} of element {element:?}",
                );
            }
        }
    }
}
