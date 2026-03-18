//! Allowed valences (neutral-charge bond counts) for chemical elements.
//!
//! Values are sourced from the InChI 1.07 C library `get_el_valence()` table
//! in `INCHI-1-SRC/INCHI_BASE/src/util.c` (`ElData[].cValence[NEUTRAL_STATE]`).

use crate::isotopes::ElementVariant;

/// Discrete allowed valence counts at neutral charge.
///
/// Returns an empty slice for superheavy elements (Z >= 104), which have no
/// fixed valence model in the InChI specification.
pub trait AllowedValences {
    /// Returns the allowed valences for the element at neutral charge.
    ///
    /// The returned slice is sorted in ascending order. An empty slice
    /// indicates that the element has no fixed valence model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{AllowedValences, Element};
    ///
    /// assert_eq!(Element::C.allowed_valences(), &[4]);
    /// assert_eq!(Element::N.allowed_valences(), &[3, 5]);
    /// assert_eq!(Element::Fe.allowed_valences(), &[2, 3, 4, 6]);
    /// ```
    fn allowed_valences(&self) -> &'static [u8];
}

impl AllowedValences for crate::Element {
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    fn allowed_valences(&self) -> &'static [u8] {
        match self {
            Self::H => &[1],
            Self::He => &[0],
            Self::Li => &[1],
            Self::Be => &[2],
            Self::B => &[3, 4],
            Self::C => &[4],
            Self::N => &[3, 5],
            Self::O => &[2],
            Self::F => &[1],
            Self::Ne => &[0],
            Self::Na => &[1],
            Self::Mg => &[2],
            Self::Al => &[3],
            Self::Si => &[4, 6],
            Self::P => &[3, 5, 6],
            Self::S => &[2, 4, 6],
            Self::Cl => &[1, 3, 5, 7],
            Self::Ar => &[0],
            Self::K => &[1],
            Self::Ca => &[2],
            // Period 4 transition metals (3d)
            Self::Sc => &[3],
            Self::Ti => &[3, 4],
            Self::V => &[2, 3, 4, 5],
            Self::Cr => &[2, 3, 6],
            Self::Mn => &[2, 3, 4, 6],
            Self::Fe => &[2, 3, 4, 6],
            Self::Co => &[2, 3],
            Self::Ni => &[2, 3],
            Self::Cu => &[1, 2],
            Self::Zn => &[2],
            Self::Ga => &[3],
            Self::Ge => &[4],
            Self::As => &[3, 5],
            Self::Se => &[2, 4, 6],
            Self::Br => &[1, 3, 5, 7],
            Self::Kr => &[0],
            Self::Rb => &[1],
            Self::Sr => &[2],
            // Period 5 transition metals (4d)
            Self::Y => &[3],
            Self::Zr => &[4],
            Self::Nb => &[3, 5],
            Self::Mo => &[3, 4, 5, 6],
            Self::Tc => &[7],
            Self::Ru => &[2, 3, 4, 6],
            Self::Rh => &[2, 3, 4],
            Self::Pd => &[2, 4],
            Self::Ag => &[1],
            Self::Cd => &[2],
            Self::In => &[3],
            Self::Sn => &[2, 4],
            Self::Sb => &[3, 5],
            Self::Te => &[2, 4, 6],
            Self::I => &[1, 3, 5, 7],
            Self::Xe => &[0],
            Self::Cs => &[1],
            Self::Ba => &[2],
            // Lanthanides (La-Lu)
            Self::La => &[3],
            Self::Ce => &[3, 4],
            Self::Pr => &[3, 4],
            Self::Nd => &[3],
            Self::Pm => &[3],
            Self::Sm => &[2, 3],
            Self::Eu => &[2, 3],
            Self::Gd => &[3],
            Self::Tb => &[3, 4],
            Self::Dy => &[3],
            Self::Ho => &[3],
            Self::Er => &[3],
            Self::Tm => &[2, 3],
            Self::Yb => &[2, 3],
            Self::Lu => &[3],
            // Period 6 transition metals (5d)
            Self::Hf => &[4],
            Self::Ta => &[5],
            Self::W => &[3, 4, 5, 6],
            Self::Re => &[2, 4, 6, 7],
            Self::Os => &[2, 3, 4, 6],
            Self::Ir => &[2, 3, 4, 6],
            Self::Pt => &[2, 4],
            Self::Au => &[1, 3],
            Self::Hg => &[1, 2],
            Self::Tl => &[1, 3],
            Self::Pb => &[2, 4],
            Self::Bi => &[3, 5],
            Self::Po => &[2, 4, 6],
            Self::At => &[1, 3, 5, 7],
            Self::Rn => &[0],
            Self::Fr => &[1],
            Self::Ra => &[2],
            // Actinides (Ac-No)
            Self::Ac => &[3],
            Self::Th => &[3, 4],
            Self::Pa => &[3, 4, 5],
            Self::U => &[3, 4, 5, 6],
            Self::Np => &[3, 4, 5, 6],
            Self::Pu => &[3, 4, 5, 6],
            Self::Am => &[3, 4, 5, 6],
            Self::Cm => &[3],
            Self::Bk => &[3, 4],
            Self::Cf => &[3],
            Self::Es => &[3],
            Self::Fm => &[3],
            Self::Md => &[3],
            Self::No => &[1],
            Self::Lr => &[],
            // Superheavy elements (Rf-Og): no fixed valence model
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
            | Self::Og => &[],
        }
    }
}

impl AllowedValences for crate::Isotope {
    fn allowed_valences(&self) -> &'static [u8] {
        self.element().allowed_valences()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::AllowedValences;

    #[test]
    fn test_sorted_ascending() {
        for element in crate::Element::iter() {
            let valences = element.allowed_valences();
            for window in valences.windows(2) {
                assert!(
                    window[0] < window[1],
                    "Allowed valences should be sorted ascending for {element:?}: {valences:?}",
                );
            }
        }
    }

    #[test]
    fn test_carbon() {
        assert_eq!(crate::Element::C.allowed_valences(), &[4]);
    }

    #[test]
    fn test_nitrogen() {
        assert_eq!(crate::Element::N.allowed_valences(), &[3, 5]);
    }

    #[test]
    fn test_sulfur() {
        assert_eq!(crate::Element::S.allowed_valences(), &[2, 4, 6]);
    }

    #[test]
    fn test_iron() {
        assert_eq!(crate::Element::Fe.allowed_valences(), &[2, 3, 4, 6]);
    }

    #[test]
    fn test_bromine() {
        assert_eq!(crate::Element::Br.allowed_valences(), &[1, 3, 5, 7]);
    }

    #[test]
    fn test_boron() {
        assert_eq!(crate::Element::B.allowed_valences(), &[3, 4]);
    }

    #[test]
    fn test_noble_gases_zero() {
        assert_eq!(crate::Element::Kr.allowed_valences(), &[0]);
        assert_eq!(crate::Element::Xe.allowed_valences(), &[0]);
        assert_eq!(crate::Element::Rn.allowed_valences(), &[0]);
    }

    #[test]
    fn test_germanium() {
        assert_eq!(crate::Element::Ge.allowed_valences(), &[4]);
    }

    #[test]
    fn test_lanthanide() {
        assert_eq!(crate::Element::Ce.allowed_valences(), &[3, 4]);
        assert_eq!(crate::Element::Gd.allowed_valences(), &[3]);
    }

    #[test]
    fn test_actinide() {
        assert_eq!(crate::Element::U.allowed_valences(), &[3, 4, 5, 6]);
        assert_eq!(crate::Element::No.allowed_valences(), &[1]);
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let element_valences = element.allowed_valences();
            for isotope in element.isotopes() {
                let isotope_valences = isotope.allowed_valences();
                assert_eq!(
                    element_valences, isotope_valences,
                    "Allowed valences mismatch for isotope {isotope:?} of element {element:?}",
                );
            }
        }
    }
}
