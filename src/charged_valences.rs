//! Charge-specific allowed valence counts for chemical elements.
//!
//! Values follow the InChI 1.7 `ElData[].cValence` table in
//! `INCHI-1-SRC/INCHI_BASE/src/util.c`.
//!
//! Unlike [`crate::AllowedValences`], which returns a compatibility superset
//! across common charge states, this trait exposes the exact valences valid at
//! one formal charge.
//!
//! InChI stores placeholder neutral `[1]` entries for `Lr` and `Rf` through
//! `Og`; this crate keeps treating those elements as having no fixed valence
//! model and returns `&[]` for them at every charge.

use crate::isotopes::ElementVariant;

/// Allowed valence counts at a specific formal charge.
pub trait ChargedValences {
    /// Returns the allowed valences at the given formal charge.
    ///
    /// `charge` must be in the range `[-2, 2]`. Values outside that range
    /// return an empty slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{ChargedValences, Element};
    ///
    /// assert_eq!(Element::N.valences_at_charge(0), &[3, 5]);
    /// assert_eq!(Element::N.valences_at_charge(1), &[4]);
    /// assert_eq!(Element::N.valences_at_charge(-1), &[2]);
    /// ```
    fn valences_at_charge(&self, charge: i8) -> &'static [u8];
}

impl crate::Element {
    #[allow(clippy::match_same_arms)]
    fn valences_at_minus_2(self) -> &'static [u8] {
        match self {
            Self::B => &[3],
            Self::C => &[2],
            Self::N => &[1],
            Self::Al | Self::Ga | Self::In | Self::Tl => &[3, 5],
            Self::Si => &[2],
            Self::P | Self::As | Self::Sb | Self::Bi => &[1, 3, 5, 7],
            Self::Ge | Self::Sn | Self::Pb => &[2, 4, 6],
            _ => &[],
        }
    }

    #[allow(clippy::match_same_arms)]
    fn valences_at_minus_1(self) -> &'static [u8] {
        match self {
            Self::B => &[4],
            Self::C => &[3],
            Self::N => &[2],
            Self::O => &[1],
            Self::Al | Self::Ga => &[4],
            Self::In | Self::Tl => &[2, 4],
            Self::Si | Self::Ge => &[3, 5],
            Self::P | Self::As | Self::Sb | Self::Bi => &[2, 4, 6],
            Self::S | Self::Se | Self::Te | Self::Po => &[1, 3, 5, 7],
            Self::Sn | Self::Pb => &[3, 5],
            _ => &[],
        }
    }

    #[allow(clippy::match_same_arms)]
    #[allow(clippy::too_many_lines)]
    fn valences_at_neutral(self) -> &'static [u8] {
        match self {
            Self::H => &[1],
            Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs | Self::Fr => &[1],
            Self::Be | Self::Mg | Self::Ca | Self::Sr | Self::Ba | Self::Ra => &[2],
            Self::B => &[3],
            Self::C => &[4],
            Self::N => &[3, 5],
            Self::O => &[2],
            Self::F => &[1],
            Self::Al | Self::Ga | Self::In => &[3],
            Self::Si | Self::Ge => &[4],
            Self::P | Self::As | Self::Sb | Self::Bi => &[3, 5],
            Self::S | Self::Se | Self::Te | Self::Po => &[2, 4, 6],
            Self::Cl | Self::Br | Self::I | Self::At => &[1, 3, 5, 7],
            Self::Sc | Self::Y | Self::La | Self::Ac => &[3],
            Self::Ti | Self::Ce | Self::Pr | Self::Tb | Self::Th | Self::Bk => &[3, 4],
            Self::V => &[2, 3, 4, 5],
            Self::Cr => &[2, 3, 6],
            Self::Mn | Self::Fe | Self::Ru | Self::Os | Self::Ir => &[2, 3, 4, 6],
            Self::Co | Self::Ni | Self::Sm | Self::Eu | Self::Tm | Self::Yb => &[2, 3],
            Self::Cu | Self::Hg => &[1, 2],
            Self::Zn | Self::Cd => &[2],
            Self::Zr | Self::Hf => &[4],
            Self::Nb => &[3, 5],
            Self::Mo | Self::W => &[3, 4, 5, 6],
            Self::Tc => &[7],
            Self::Rh => &[2, 3, 4],
            Self::Pd | Self::Pt => &[2, 4],
            Self::Ag => &[1],
            Self::Sn | Self::Pb => &[2, 4],
            Self::Tl => &[1, 3],
            Self::Nd
            | Self::Pm
            | Self::Gd
            | Self::Dy
            | Self::Ho
            | Self::Er
            | Self::Lu
            | Self::Cm
            | Self::Cf
            | Self::Es
            | Self::Fm
            | Self::Md => &[3],
            Self::Ta => &[5],
            Self::Re => &[2, 4, 6, 7],
            Self::Au => &[1, 3],
            Self::Pa => &[3, 4, 5],
            Self::U | Self::Np | Self::Pu | Self::Am => &[3, 4, 5, 6],
            Self::No => &[1],
            Self::He
            | Self::Ne
            | Self::Ar
            | Self::Kr
            | Self::Xe
            | Self::Rn
            | Self::Lr
            | Self::Rf
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

    #[allow(clippy::match_same_arms)]
    fn valences_at_plus_1(self) -> &'static [u8] {
        match self {
            Self::Be | Self::Mg | Self::Ca | Self::Sr | Self::Ba | Self::Ra => &[1],
            Self::B => &[2],
            Self::C => &[3],
            Self::N => &[4],
            Self::O => &[3, 5],
            Self::F => &[2],
            Self::Al => &[2],
            Self::Si | Self::Ge | Self::Sn | Self::Pb => &[3],
            Self::P | Self::As => &[4],
            Self::S | Self::Se | Self::Te | Self::Po => &[3, 5],
            Self::Cl | Self::Br | Self::I | Self::At => &[2, 4, 6],
            Self::Sb | Self::Bi => &[2, 4],
            _ => &[],
        }
    }

    #[allow(clippy::match_same_arms)]
    fn valences_at_plus_2(self) -> &'static [u8] {
        match self {
            Self::B | Self::Al | Self::Ga | Self::In => &[1],
            Self::C | Self::Si => &[2],
            Self::N | Self::P | Self::As | Self::Sb | Self::Bi => &[3],
            Self::O | Self::S | Self::Se => &[4],
            Self::F | Self::Cl | Self::Br | Self::I | Self::At => &[3, 5],
            Self::Te | Self::Po => &[2, 4],
            _ => &[],
        }
    }
}

impl ChargedValences for crate::Element {
    fn valences_at_charge(&self, charge: i8) -> &'static [u8] {
        match charge {
            -2 => (*self).valences_at_minus_2(),
            -1 => (*self).valences_at_minus_1(),
            0 => (*self).valences_at_neutral(),
            1 => (*self).valences_at_plus_1(),
            2 => (*self).valences_at_plus_2(),
            _ => &[],
        }
    }
}

impl ChargedValences for crate::Isotope {
    fn valences_at_charge(&self, charge: i8) -> &'static [u8] {
        self.element().valences_at_charge(charge)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::ChargedValences;

    type ChargeTable = [&'static [u8]; 5];

    fn assert_charge_table(element: crate::Element, expected: ChargeTable) {
        for (index, expected_valences) in expected.iter().enumerate() {
            let charge = i8::try_from(index).unwrap() - 2;
            assert_eq!(
                element.valences_at_charge(charge),
                *expected_valences,
                "Charged valences mismatch for {element:?} at charge {charge}",
            );
        }
    }

    fn assert_charge_tables(cases: &[(crate::Element, ChargeTable)]) {
        for (element, expected) in cases {
            assert_charge_table(*element, *expected);
        }
    }

    #[test]
    fn test_all_charged_valences_sorted() {
        for element in crate::Element::iter() {
            for charge in -2..=2_i8 {
                let valences = element.valences_at_charge(charge);
                for window in valences.windows(2) {
                    assert!(
                        window[0] < window[1],
                        "{element:?} at charge {charge}: not sorted: {valences:?}",
                    );
                }
            }
        }
    }

    #[test]
    fn test_out_of_range_charge() {
        for element in crate::Element::iter() {
            assert_eq!(element.valences_at_charge(-3), &[]);
            assert_eq!(element.valences_at_charge(3), &[]);
            assert_eq!(element.valences_at_charge(i8::MIN), &[]);
            assert_eq!(element.valences_at_charge(i8::MAX), &[]);
        }
    }

    #[test]
    fn test_hydrogen_and_alkali_charge_tables() {
        assert_charge_tables(&[
            (crate::Element::H, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Li, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Na, [&[], &[], &[1], &[], &[]]),
            (crate::Element::K, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Rb, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Cs, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Fr, [&[], &[], &[1], &[], &[]]),
        ]);
    }

    #[test]
    fn test_alkaline_earth_charge_tables() {
        assert_charge_tables(&[
            (crate::Element::Be, [&[], &[], &[2], &[1], &[]]),
            (crate::Element::Mg, [&[], &[], &[2], &[1], &[]]),
            (crate::Element::Ca, [&[], &[], &[2], &[1], &[]]),
            (crate::Element::Sr, [&[], &[], &[2], &[1], &[]]),
            (crate::Element::Ba, [&[], &[], &[2], &[1], &[]]),
            (crate::Element::Ra, [&[], &[], &[2], &[1], &[]]),
        ]);
    }

    #[test]
    fn test_non_metals_and_metalloids_charge_tables() {
        assert_charge_tables(&[
            (crate::Element::B, [&[3], &[4], &[3], &[2], &[1]]),
            (crate::Element::C, [&[2], &[3], &[4], &[3], &[2]]),
            (crate::Element::N, [&[1], &[2], &[3, 5], &[4], &[3]]),
            (crate::Element::O, [&[], &[1], &[2], &[3, 5], &[4]]),
            (crate::Element::F, [&[], &[], &[1], &[2], &[3, 5]]),
            (crate::Element::Al, [&[3, 5], &[4], &[3], &[2], &[1]]),
            (crate::Element::Si, [&[2], &[3, 5], &[4], &[3], &[2]]),
            (crate::Element::P, [&[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[4], &[3]]),
            (crate::Element::S, [&[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[4]]),
            (crate::Element::Cl, [&[], &[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5]]),
            (crate::Element::Ga, [&[3, 5], &[4], &[3], &[], &[1]]),
            (crate::Element::Ge, [&[2, 4, 6], &[3, 5], &[4], &[3], &[]]),
            (crate::Element::As, [&[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[4], &[3]]),
            (crate::Element::Se, [&[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[4]]),
            (crate::Element::Br, [&[], &[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5]]),
            (crate::Element::In, [&[3, 5], &[2, 4], &[3], &[], &[1]]),
            (crate::Element::Sn, [&[2, 4, 6], &[3, 5], &[2, 4], &[3], &[]]),
            (crate::Element::Sb, [&[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[2, 4], &[3]]),
            (crate::Element::Te, [&[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[2, 4]]),
            (crate::Element::I, [&[], &[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5]]),
            (crate::Element::Tl, [&[3, 5], &[2, 4], &[1, 3], &[], &[]]),
            (crate::Element::Pb, [&[2, 4, 6], &[3, 5], &[2, 4], &[3], &[]]),
            (crate::Element::Bi, [&[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[2, 4], &[3]]),
            (crate::Element::Po, [&[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5], &[2, 4]]),
            (crate::Element::At, [&[], &[], &[1, 3, 5, 7], &[2, 4, 6], &[3, 5]]),
        ]);
    }

    #[test]
    fn test_transition_metals_are_neutral_only() {
        assert_charge_tables(&[
            (crate::Element::Sc, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Ti, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::V, [&[], &[], &[2, 3, 4, 5], &[], &[]]),
            (crate::Element::Cr, [&[], &[], &[2, 3, 6], &[], &[]]),
            (crate::Element::Mn, [&[], &[], &[2, 3, 4, 6], &[], &[]]),
            (crate::Element::Fe, [&[], &[], &[2, 3, 4, 6], &[], &[]]),
            (crate::Element::Co, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Ni, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Cu, [&[], &[], &[1, 2], &[], &[]]),
            (crate::Element::Zn, [&[], &[], &[2], &[], &[]]),
            (crate::Element::Y, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Zr, [&[], &[], &[4], &[], &[]]),
            (crate::Element::Nb, [&[], &[], &[3, 5], &[], &[]]),
            (crate::Element::Mo, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Tc, [&[], &[], &[7], &[], &[]]),
            (crate::Element::Ru, [&[], &[], &[2, 3, 4, 6], &[], &[]]),
            (crate::Element::Rh, [&[], &[], &[2, 3, 4], &[], &[]]),
            (crate::Element::Pd, [&[], &[], &[2, 4], &[], &[]]),
            (crate::Element::Ag, [&[], &[], &[1], &[], &[]]),
            (crate::Element::Cd, [&[], &[], &[2], &[], &[]]),
            (crate::Element::Hf, [&[], &[], &[4], &[], &[]]),
            (crate::Element::Ta, [&[], &[], &[5], &[], &[]]),
            (crate::Element::W, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Re, [&[], &[], &[2, 4, 6, 7], &[], &[]]),
            (crate::Element::Os, [&[], &[], &[2, 3, 4, 6], &[], &[]]),
            (crate::Element::Ir, [&[], &[], &[2, 3, 4, 6], &[], &[]]),
            (crate::Element::Pt, [&[], &[], &[2, 4], &[], &[]]),
            (crate::Element::Au, [&[], &[], &[1, 3], &[], &[]]),
            (crate::Element::Hg, [&[], &[], &[1, 2], &[], &[]]),
        ]);
    }

    #[test]
    fn test_lanthanides_are_neutral_only() {
        assert_charge_tables(&[
            (crate::Element::La, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Ce, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::Pr, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::Nd, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Pm, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Sm, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Eu, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Gd, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Tb, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::Dy, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Ho, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Er, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Tm, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Yb, [&[], &[], &[2, 3], &[], &[]]),
            (crate::Element::Lu, [&[], &[], &[3], &[], &[]]),
        ]);
    }

    #[test]
    fn test_actinides_are_neutral_only() {
        assert_charge_tables(&[
            (crate::Element::Ac, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Th, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::Pa, [&[], &[], &[3, 4, 5], &[], &[]]),
            (crate::Element::U, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Np, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Pu, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Am, [&[], &[], &[3, 4, 5, 6], &[], &[]]),
            (crate::Element::Cm, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Bk, [&[], &[], &[3, 4], &[], &[]]),
            (crate::Element::Cf, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Es, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Fm, [&[], &[], &[3], &[], &[]]),
            (crate::Element::Md, [&[], &[], &[3], &[], &[]]),
            (crate::Element::No, [&[], &[], &[1], &[], &[]]),
        ]);
    }

    #[test]
    fn test_noble_gases_are_empty_at_all_charges() {
        assert_charge_tables(&[
            (crate::Element::He, [&[], &[], &[], &[], &[]]),
            (crate::Element::Ne, [&[], &[], &[], &[], &[]]),
            (crate::Element::Ar, [&[], &[], &[], &[], &[]]),
            (crate::Element::Kr, [&[], &[], &[], &[], &[]]),
            (crate::Element::Xe, [&[], &[], &[], &[], &[]]),
            (crate::Element::Rn, [&[], &[], &[], &[], &[]]),
        ]);
    }

    #[test]
    fn test_superheavy_elements_have_no_fixed_model() {
        assert_charge_tables(&[
            (crate::Element::Lr, [&[], &[], &[], &[], &[]]),
            (crate::Element::Rf, [&[], &[], &[], &[], &[]]),
            (crate::Element::Db, [&[], &[], &[], &[], &[]]),
            (crate::Element::Sg, [&[], &[], &[], &[], &[]]),
            (crate::Element::Bh, [&[], &[], &[], &[], &[]]),
            (crate::Element::Hs, [&[], &[], &[], &[], &[]]),
            (crate::Element::Mt, [&[], &[], &[], &[], &[]]),
            (crate::Element::Ds, [&[], &[], &[], &[], &[]]),
            (crate::Element::Rg, [&[], &[], &[], &[], &[]]),
            (crate::Element::Cn, [&[], &[], &[], &[], &[]]),
            (crate::Element::Nh, [&[], &[], &[], &[], &[]]),
            (crate::Element::Fl, [&[], &[], &[], &[], &[]]),
            (crate::Element::Mc, [&[], &[], &[], &[], &[]]),
            (crate::Element::Lv, [&[], &[], &[], &[], &[]]),
            (crate::Element::Ts, [&[], &[], &[], &[], &[]]),
            (crate::Element::Og, [&[], &[], &[], &[], &[]]),
        ]);
    }

    #[test]
    fn test_critical_examples() {
        assert_eq!(crate::Element::C.valences_at_charge(1), &[3]);
        assert_eq!(crate::Element::O.valences_at_charge(1), &[3, 5]);
        assert_eq!(crate::Element::N.valences_at_charge(-2), &[1]);
        assert_eq!(crate::Element::N.valences_at_charge(-1), &[2]);
        assert_eq!(crate::Element::N.valences_at_charge(0), &[3, 5]);
        assert_eq!(crate::Element::N.valences_at_charge(1), &[4]);
        assert_eq!(crate::Element::N.valences_at_charge(2), &[3]);
        assert_eq!(crate::Element::S.valences_at_charge(-2), &[]);
        assert_eq!(crate::Element::S.valences_at_charge(-1), &[1, 3, 5, 7]);
        assert_eq!(crate::Element::S.valences_at_charge(0), &[2, 4, 6]);
        assert_eq!(crate::Element::S.valences_at_charge(1), &[3, 5]);
        assert_eq!(crate::Element::S.valences_at_charge(2), &[4]);
    }

    #[test]
    fn test_isoelectronic_pattern_positive() {
        let pairs = [
            (crate::Element::C, crate::Element::B),
            (crate::Element::N, crate::Element::C),
            (crate::Element::O, crate::Element::N),
            (crate::Element::F, crate::Element::O),
            (crate::Element::Si, crate::Element::Al),
            (crate::Element::P, crate::Element::Si),
            (crate::Element::S, crate::Element::P),
            (crate::Element::Cl, crate::Element::S),
        ];

        for (charged, neutral_neighbor) in pairs {
            assert_eq!(
                charged.valences_at_charge(1),
                neutral_neighbor.valences_at_charge(0),
                "{charged:?}(+1) should equal {neutral_neighbor:?}(0)",
            );
        }
    }

    #[test]
    fn test_matches_open_babel_reference_cases() {
        assert_eq!(crate::Element::C.valences_at_charge(-1), &[3]);
        assert_eq!(crate::Element::C.valences_at_charge(1), &[3]);
        assert_eq!(crate::Element::N.valences_at_charge(-1), &[2]);
        assert_eq!(crate::Element::N.valences_at_charge(1), &[4]);
        assert_eq!(crate::Element::O.valences_at_charge(-1), &[1]);
        assert_eq!(crate::Element::O.valences_at_charge(1), &[3, 5]);
        assert_eq!(crate::Element::S.valences_at_charge(-1), &[1, 3, 5, 7]);
        assert_eq!(crate::Element::S.valences_at_charge(1), &[3, 5]);
        assert_eq!(crate::Element::P.valences_at_charge(-1), &[2, 4, 6]);
        assert_eq!(crate::Element::P.valences_at_charge(1), &[4]);
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            for isotope in element.isotopes() {
                for charge in -2..=2_i8 {
                    assert_eq!(
                        isotope.valences_at_charge(charge),
                        element.valences_at_charge(charge),
                        "Charged valences mismatch for isotope {isotope:?} of element {element:?} at charge {charge}",
                    );
                }
            }
        }
    }
}
