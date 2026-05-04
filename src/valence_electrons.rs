//! Group-style valence-electron counts for chemical elements.
//!
//! For main-group elements this matches the number of electrons in the
//! outermost principal shell. For transition metals, lanthanides, and
//! actinides, this table can include chemically available d/f electrons and is
//! not the same thing as the allowed-valence table used by the bond-order
//! solver.

use crate::isotopes::ElementVariant;

/// Group-style valence-electron count for an element.
pub trait ValenceElectrons: Sized {
    /// Returns the number of valence electrons.
    ///
    /// This is a valence-electron descriptor, not a list of allowed bond
    /// valences. For example, transition-metal values can include d electrons.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ValenceElectrons};
    ///
    /// assert_eq!(Element::H.valence_electrons(), 1);
    /// assert_eq!(Element::C.valence_electrons(), 4);
    /// ```
    fn valence_electrons(&self) -> u8;
}

impl ValenceElectrons for crate::Element {
    fn valence_electrons(&self) -> u8 {
        match self {
            Self::H | Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs | Self::Fr => 1,
            Self::He | Self::Be | Self::Mg | Self::Ca | Self::Sr | Self::Ba | Self::Ra => 2,
            Self::B
            | Self::Al
            | Self::Sc
            | Self::Ga
            | Self::Y
            | Self::In
            | Self::La
            | Self::Lu
            | Self::Tl
            | Self::Ac
            | Self::Lr
            | Self::Nh => 3,
            Self::C
            | Self::Si
            | Self::Ti
            | Self::Ge
            | Self::Zr
            | Self::Sn
            | Self::Ce
            | Self::Hf
            | Self::Pb
            | Self::Th
            | Self::Rf
            | Self::Fl => 4,
            Self::N
            | Self::P
            | Self::V
            | Self::As
            | Self::Nb
            | Self::Sb
            | Self::Pr
            | Self::Ta
            | Self::Bi
            | Self::Pa
            | Self::Db
            | Self::Mc => 5,
            Self::O
            | Self::S
            | Self::Cr
            | Self::Se
            | Self::Mo
            | Self::Te
            | Self::Nd
            | Self::W
            | Self::Po
            | Self::U
            | Self::Sg
            | Self::Lv => 6,
            Self::F
            | Self::Cl
            | Self::Mn
            | Self::Br
            | Self::Tc
            | Self::I
            | Self::Pm
            | Self::Re
            | Self::At
            | Self::Np
            | Self::Bh
            | Self::Ts => 7,
            Self::Ne
            | Self::Ar
            | Self::Fe
            | Self::Kr
            | Self::Ru
            | Self::Xe
            | Self::Sm
            | Self::Os
            | Self::Rn
            | Self::Pu
            | Self::Hs
            | Self::Og => 8,
            Self::Co | Self::Rh | Self::Eu | Self::Ir | Self::Am | Self::Mt => 9,
            Self::Ni | Self::Pd | Self::Gd | Self::Pt | Self::Cm | Self::Ds => 10,
            Self::Cu | Self::Ag | Self::Tb | Self::Au | Self::Bk | Self::Rg => 11,
            Self::Zn | Self::Cd | Self::Dy | Self::Hg | Self::Cf | Self::Cn => 12,
            Self::Ho | Self::Es => 13,
            Self::Er | Self::Fm => 14,
            Self::Tm | Self::Md => 15,
            Self::Yb | Self::No => 16,
        }
    }
}

impl ValenceElectrons for crate::Isotope {
    fn valence_electrons(&self) -> u8 {
        self.element().valence_electrons()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::ValenceElectrons;
    use crate::AllowedValences;

    fn outermost_shell_electrons(element: crate::Element) -> u8 {
        let orbitals = element.orbitals();
        let outermost_shell = orbitals
            .iter()
            .map(crate::AtomicOrbital::principal_quantum_number)
            .max()
            .expect("elements should have orbital data");

        orbitals
            .iter()
            .filter(|orbital| orbital.principal_quantum_number() == outermost_shell)
            .map(crate::AtomicOrbital::number_of_electrons)
            .sum()
    }

    #[test]
    fn test_valence_electrons() {
        for element in crate::Element::iter() {
            let valence = element.valence_electrons();
            assert!(
                (1..=16).contains(&valence),
                "Valence electrons should be between 1 and 16 for {element:?}",
            );
        }
    }

    #[test]
    fn test_s_block_valence_electrons_match_outermost_shell() {
        for element in [
            crate::Element::H,
            crate::Element::Li,
            crate::Element::Be,
            crate::Element::Na,
            crate::Element::Mg,
            crate::Element::K,
            crate::Element::Ca,
            crate::Element::Rb,
            crate::Element::Sr,
            crate::Element::Cs,
            crate::Element::Ba,
            crate::Element::Fr,
            crate::Element::Ra,
        ] {
            assert_eq!(
                element.valence_electrons(),
                outermost_shell_electrons(element),
                "s-block valence-electron count should match the outermost shell for {element:?}",
            );
        }
    }

    #[test]
    fn test_s_block_valence_electrons_match_single_allowed_valence() {
        for element in [
            crate::Element::H,
            crate::Element::Li,
            crate::Element::Be,
            crate::Element::Na,
            crate::Element::Mg,
            crate::Element::K,
            crate::Element::Ca,
            crate::Element::Rb,
            crate::Element::Sr,
            crate::Element::Cs,
            crate::Element::Ba,
            crate::Element::Fr,
            crate::Element::Ra,
        ] {
            assert_eq!(
                element.allowed_valences(),
                &[element.valence_electrons()],
                "s-block elements should have a single allowed valence matching valence electrons for {element:?}",
            );
        }
    }

    #[test]
    fn test_valence_electrons_delegation() {
        for element in crate::Element::iter() {
            let element_number = element.valence_electrons();
            let isotopes = element.isotopes();
            for isotope in isotopes {
                let isotope_number = isotope.valence_electrons();
                assert_eq!(
                    element_number, isotope_number,
                    "Valence electrons mismatch for isotope {isotope:?} of element {element:?}",
                );
            }
        }
    }
}
