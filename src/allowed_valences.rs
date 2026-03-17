//! Allowed valences (neutral-charge bond counts) for chemical elements.

use crate::isotopes::ElementVariant;

/// Discrete allowed valence counts at neutral charge.
///
/// Returns an empty slice for transition metals, lanthanides, actinides,
/// and superheavy elements (Z >= 104), which have no fixed valence model.
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
    /// assert!(Element::Fe.allowed_valences().is_empty());
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
            Self::B => &[3],
            Self::C => &[4],
            Self::N => &[3, 5],
            Self::O => &[2],
            Self::F => &[1],
            Self::Ne => &[0],
            Self::Na => &[1],
            Self::Mg => &[2],
            Self::Al => &[3],
            Self::Si => &[4],
            Self::P => &[3, 5],
            Self::S => &[2, 4, 6],
            Self::Cl => &[1, 3, 5, 7],
            Self::Ar => &[0],
            Self::K => &[1],
            Self::Ca => &[2],
            // Transition metals (Sc-Zn): no fixed valence model
            Self::Sc
            | Self::Ti
            | Self::V
            | Self::Cr
            | Self::Mn
            | Self::Fe
            | Self::Co
            | Self::Ni
            | Self::Cu
            | Self::Zn => &[],
            Self::Ga => &[3],
            Self::Ge => &[2, 4],
            Self::As => &[3, 5],
            Self::Se => &[2, 4, 6],
            Self::Br => &[1, 3, 5, 7],
            Self::Kr => &[0, 2],
            Self::Rb => &[1],
            Self::Sr => &[2],
            // Transition metals (Y-Cd): no fixed valence model
            Self::Y
            | Self::Zr
            | Self::Nb
            | Self::Mo
            | Self::Tc
            | Self::Ru
            | Self::Rh
            | Self::Pd
            | Self::Ag
            | Self::Cd => &[],
            Self::In => &[3],
            Self::Sn => &[2, 4],
            Self::Sb => &[3, 5],
            Self::Te => &[2, 4, 6],
            Self::I => &[1, 3, 5, 7],
            Self::Xe => &[0, 2, 4, 6],
            Self::Cs => &[1],
            Self::Ba => &[2],
            // Lanthanides (La-Lu): no fixed valence model
            Self::La
            | Self::Ce
            | Self::Pr
            | Self::Nd
            | Self::Pm
            | Self::Sm
            | Self::Eu
            | Self::Gd
            | Self::Tb
            | Self::Dy
            | Self::Ho
            | Self::Er
            | Self::Tm
            | Self::Yb
            | Self::Lu => &[],
            // Transition metals (Hf-Hg): no fixed valence model
            Self::Hf
            | Self::Ta
            | Self::W
            | Self::Re
            | Self::Os
            | Self::Ir
            | Self::Pt
            | Self::Au
            | Self::Hg => &[],
            Self::Tl => &[1, 3],
            Self::Pb => &[2, 4],
            Self::Bi => &[3, 5],
            Self::Po => &[2, 4, 6],
            Self::At => &[1, 3, 5, 7],
            Self::Rn => &[0, 2],
            Self::Fr => &[1],
            Self::Ra => &[2],
            // Actinides (Ac-Lr): no fixed valence model
            Self::Ac
            | Self::Th
            | Self::Pa
            | Self::U
            | Self::Np
            | Self::Pu
            | Self::Am
            | Self::Cm
            | Self::Bk
            | Self::Cf
            | Self::Es
            | Self::Fm
            | Self::Md
            | Self::No
            | Self::Lr => &[],
            // Superheavy elements (Rf-Cn, Nh-Og): no fixed valence model
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
    fn test_iron_empty() {
        assert!(crate::Element::Fe.allowed_valences().is_empty());
    }

    #[test]
    fn test_bromine() {
        assert_eq!(crate::Element::Br.allowed_valences(), &[1, 3, 5, 7]);
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
