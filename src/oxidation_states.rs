//! Valid oxidation states for elements.

use crate::isotopes::ElementVariant;

/// Valid oxidation states for chemical elements.
pub trait OxidationStates {
    /// Returns all valid oxidation states.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{
    ///     Element, OxidationStates,
    ///     isotopes::{HydrogenIsotope, Isotope},
    /// };
    ///
    /// let states = Element::H.oxidation_states();
    /// assert!(states.contains(&1));
    /// assert!(states.contains(&-1));
    ///
    /// let deuterium = Isotope::H(HydrogenIsotope::D);
    /// assert_eq!(deuterium.oxidation_states(), states);
    /// ```
    fn oxidation_states(&self) -> &'static [i16];

    /// Returns whether the oxidation state is valid for this element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{
    ///     Element, OxidationStates,
    ///     isotopes::{HydrogenIsotope, Isotope},
    /// };
    ///
    /// assert!(Element::O.is_valid_oxidation_state(-2));
    /// assert!(!Element::O.is_valid_oxidation_state(3));
    ///
    /// let deuterium = Isotope::H(HydrogenIsotope::D);
    /// assert!(deuterium.is_valid_oxidation_state(1));
    /// assert!(!deuterium.is_valid_oxidation_state(2));
    /// ```
    fn is_valid_oxidation_state(&self, state: i16) -> bool {
        self.oxidation_states().contains(&state)
    }
}

impl OxidationStates for crate::Element {
    #[inline]
    fn oxidation_states(&self) -> &'static [i16] {
        match self {
            Self::B => &[-5, -1, 0, 1, 2, 3],
            Self::C | Self::Si | Self::Ge | Self::Sn => &[-4, -3, -2, -1, 0, 1, 2, 3, 4],
            Self::N | Self::P | Self::As | Self::Sb | Self::Bi => &[-3, -2, -1, 0, 1, 2, 3, 4, 5],
            Self::O => &[-2, -1, 0, 1, 2],
            Self::F => &[-1, 0],
            Self::Al | Self::Ag => &[-2, -1, 0, 1, 2, 3],
            Self::S | Self::Se | Self::Te => &[-2, -1, 0, 1, 2, 3, 4, 5, 6],
            Self::Cl | Self::Tc | Self::I => &[-1, 0, 1, 2, 3, 4, 5, 6, 7],
            Self::Ti | Self::Ni | Self::Cu => &[-2, -1, 0, 1, 2, 3, 4],
            Self::V | Self::Co | Self::Nb | Self::Ta => &[-3, -1, 0, 1, 2, 3, 4, 5],
            Self::Cr | Self::Mo | Self::W => &[-4, -2, -1, 0, 1, 2, 3, 4, 5, 6],
            Self::Mn => &[-3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7],
            Self::Fe => &[-2, -1, 0, 1, 2, 3, 4, 5, 6, 7],
            Self::Zn | Self::Cd | Self::Hg => &[-2, 0, 1, 2],
            Self::Ga => &[-5, -4, -3, -2, -1, 0, 1, 2, 3],
            Self::Br => &[-1, 0, 1, 2, 3, 4, 5, 7],
            Self::Zr | Self::Hf => &[-2, 0, 1, 2, 3, 4],
            Self::Ru => &[-2, 0, 1, 2, 3, 4, 5, 6, 7, 8],
            Self::Rh | Self::Re => &[-3, -1, 0, 1, 2, 3, 4, 5, 6, 7],
            Self::Pd | Self::Pr => &[0, 1, 2, 3, 4, 5],
            Self::In | Self::Tl => &[-5, -2, -1, 0, 1, 2, 3],
            Self::Xe => &[0, 2, 4, 6, 8],
            Self::Tb => &[0, 1, 2, 3, 4],
            Self::Os => &[-4, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8],
            Self::Ir => &[-3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            Self::Pt => &[-3, -2, -1, 0, 1, 2, 3, 4, 5, 6],
            Self::Au => &[-3, -2, -1, 0, 1, 2, 3, 5],
            Self::Pb => &[-4, -2, -1, 0, 1, 2, 3, 4],
            Self::Po => &[-2, 0, 2, 4, 5, 6],
            Self::At => &[-1, 0, 1, 3, 5, 7],
            Self::Rn => &[0, 2, 6],
            Self::Fr => &[0, 1],
            Self::Ra => &[0, 2],
            Self::Ac | Self::Lr => &[0, 3],
            Self::Th => &[-1, 0, 1, 2, 3, 4],
            Self::Pa | Self::Bk | Self::Cf => &[0, 2, 3, 4, 5],
            Self::U => &[-1, 0, 1, 2, 3, 4, 5, 6],
            Self::Np | Self::Am => &[0, 2, 3, 4, 5, 6, 7],
            Self::Pu => &[0, 2, 3, 4, 5, 6, 7, 8],
            Self::Cm | Self::Sg => &[0, 3, 4, 5, 6],
            Self::Rf => &[0, 3, 4],
            Self::Db => &[0, 3, 4, 5],
            Self::Bh => &[0, 3, 4, 5, 7],
            Self::Hs => &[0, 3, 4, 6, 8],
            Self::Mt => &[0, 1, 3, 6],
            Self::Ds => &[0, 2, 4, 6],
            Self::Rg => &[-1, 0, 3, 5],
            Self::Cn => &[0, 2, 4],
            Self::Lv => &[-2, 0, 4],
            Self::Ts => &[-1, 0, 5],
            Self::Og => &[-1, 0, 1, 2, 4, 6],
            Self::Pm
            | Self::Eu
            | Self::Ho
            | Self::Er
            | Self::Lu
            | Self::Fm
            | Self::Md
            | Self::No => &[0, 2, 3],
            Self::Ce | Self::Nd | Self::Dy | Self::Es => &[0, 2, 3, 4],

            Self::Sc | Self::Y | Self::La | Self::Sm | Self::Gd | Self::Tm | Self::Yb => {
                &[0, 1, 2, 3]
            }

            Self::Be | Self::Mg | Self::Ca | Self::Kr | Self::Sr | Self::Ba => &[0, 1, 2],

            Self::H | Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs => &[-1, 0, 1],
            Self::He | Self::Ne | Self::Ar | Self::Nh | Self::Fl | Self::Mc => &[0],
        }
    }
}

impl OxidationStates for crate::Isotope {
    fn oxidation_states(&self) -> &'static [i16] {
        self.element().oxidation_states()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::OxidationStates;

    #[test]
    fn test_oxidation_states() {
        for element in crate::Element::iter() {
            let states = element.oxidation_states();
            assert!(!states.is_empty(), "Oxidation states should not be empty for {element:?}");
            let _ = element.is_valid_oxidation_state(0);
        }
    }

    #[test]
    fn test_element_oxidation_states_examples() {
        assert_eq!(crate::Element::H.oxidation_states(), &[-1, 0, 1]);
        assert!(crate::Element::O.is_valid_oxidation_state(-2));
        assert!(!crate::Element::O.is_valid_oxidation_state(3));
        assert_eq!(crate::Element::U.oxidation_states(), &[-1, 0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_element_trait_is_self_consistent() {
        for element in crate::Element::iter() {
            let states = <crate::Element as OxidationStates>::oxidation_states(&element);
            assert!(!states.is_empty());

            for state in -5..=9 {
                assert_eq!(
                    <crate::Element as OxidationStates>::is_valid_oxidation_state(&element, state),
                    states.contains(&state),
                );
            }
        }
    }

    #[test]
    fn test_isotope_oxidation_states_examples() {
        let d = crate::Isotope::H(crate::isotopes::HydrogenIsotope::D);
        assert_eq!(d.oxidation_states(), &[-1, 0, 1]);
        assert!(d.is_valid_oxidation_state(-1));
        assert!(!d.is_valid_oxidation_state(2));

        let c14 = crate::Isotope::C(crate::isotopes::CarbonIsotope::C14);
        assert_eq!(c14.oxidation_states(), &[-4, -3, -2, -1, 0, 1, 2, 3, 4]);
        assert!(c14.is_valid_oxidation_state(4));
        assert!(!c14.is_valid_oxidation_state(5));

        let u238 = crate::Isotope::U(crate::isotopes::UraniumIsotope::U238);
        assert_eq!(u238.oxidation_states(), &[-1, 0, 1, 2, 3, 4, 5, 6]);
        assert!(u238.is_valid_oxidation_state(6));
        assert!(!u238.is_valid_oxidation_state(7));
    }

    #[test]
    fn test_isotope_oxidation_states_delegation() {
        for element in crate::Element::iter() {
            let states = element.oxidation_states();

            for isotope in element.isotopes() {
                assert_eq!(
                    isotope.oxidation_states(),
                    states,
                    "Oxidation states mismatch for isotope {isotope:?} of element {element:?}",
                );

                for state in -5..=9 {
                    assert_eq!(
                        isotope.is_valid_oxidation_state(state),
                        element.is_valid_oxidation_state(state),
                        "Oxidation-state validity mismatch for isotope {isotope:?} at state {state}",
                    );
                }
            }
        }
    }
}
