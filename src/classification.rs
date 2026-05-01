//! Classification of chemical elements into categories.

use crate::isotopes::ElementVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "mem_size", derive(mem_dbg::MemSize))]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_size", mem_size(flat))]
/// Category of a chemical element in the periodic table.
pub enum ElementCategory {
    /// Alkali metals (group 1, excluding hydrogen).
    AlkaliMetal,
    /// Alkaline earth metals (group 2).
    AlkalineEarthMetal,
    /// Transition metals (d-block elements).
    TransitionMetal,
    /// Post-transition metals (p-block metals).
    PostTransitionMetal,
    /// Metalloids (semi-metals).
    Metalloid,
    /// Reactive non-metals (excluding halogens).
    NonMetal,
    /// Halogens (group 17).
    Halogen,
    /// Noble gases (group 18).
    NobleGas,
    /// Lanthanides (La through Lu).
    Lanthanide,
    /// Actinides (Ac through Lr).
    Actinide,
}

/// Classification of elements into periodic table categories.
pub trait ElementClassification {
    /// Returns the classification category for the element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ElementCategory, ElementClassification};
    ///
    /// assert_eq!(Element::Fe.classification(), ElementCategory::TransitionMetal);
    /// assert_eq!(Element::H.classification(), ElementCategory::NonMetal);
    /// ```
    fn classification(&self) -> ElementCategory;

    /// Returns `true` if the element is a metal.
    ///
    /// Metals include alkali metals, alkaline earth metals, transition metals,
    /// post-transition metals, lanthanides, and actinides.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ElementClassification};
    ///
    /// assert!(Element::Fe.is_metal());
    /// assert!(!Element::H.is_metal());
    /// ```
    fn is_metal(&self) -> bool {
        matches!(
            self.classification(),
            ElementCategory::AlkaliMetal
                | ElementCategory::AlkalineEarthMetal
                | ElementCategory::TransitionMetal
                | ElementCategory::PostTransitionMetal
                | ElementCategory::Lanthanide
                | ElementCategory::Actinide
        )
    }
}

impl ElementClassification for crate::Element {
    fn classification(&self) -> ElementCategory {
        match self {
            // Non-metals
            Self::H | Self::C | Self::N | Self::O | Self::P | Self::S | Self::Se => {
                ElementCategory::NonMetal
            }

            // Alkali metals
            Self::Li | Self::Na | Self::K | Self::Rb | Self::Cs | Self::Fr => {
                ElementCategory::AlkaliMetal
            }

            // Alkaline earth metals
            Self::Be | Self::Mg | Self::Ca | Self::Sr | Self::Ba | Self::Ra => {
                ElementCategory::AlkalineEarthMetal
            }

            // Metalloids
            Self::B | Self::Si | Self::Ge | Self::As | Self::Sb | Self::Te => {
                ElementCategory::Metalloid
            }

            // Halogens
            Self::F | Self::Cl | Self::Br | Self::I | Self::At | Self::Ts => {
                ElementCategory::Halogen
            }

            // Noble gases
            Self::He | Self::Ne | Self::Ar | Self::Kr | Self::Xe | Self::Rn | Self::Og => {
                ElementCategory::NobleGas
            }

            // Transition metals (Sc-Zn, Y-Cd, Hf-Hg, Rf-Cn)
            Self::Sc
            | Self::Ti
            | Self::V
            | Self::Cr
            | Self::Mn
            | Self::Fe
            | Self::Co
            | Self::Ni
            | Self::Cu
            | Self::Zn
            | Self::Y
            | Self::Zr
            | Self::Nb
            | Self::Mo
            | Self::Tc
            | Self::Ru
            | Self::Rh
            | Self::Pd
            | Self::Ag
            | Self::Cd
            | Self::Hf
            | Self::Ta
            | Self::W
            | Self::Re
            | Self::Os
            | Self::Ir
            | Self::Pt
            | Self::Au
            | Self::Hg
            | Self::Rf
            | Self::Db
            | Self::Sg
            | Self::Bh
            | Self::Hs
            | Self::Mt
            | Self::Ds
            | Self::Rg
            | Self::Cn => ElementCategory::TransitionMetal,

            // Post-transition metals
            Self::Al
            | Self::Ga
            | Self::In
            | Self::Sn
            | Self::Tl
            | Self::Pb
            | Self::Bi
            | Self::Po
            | Self::Nh
            | Self::Fl
            | Self::Mc
            | Self::Lv => ElementCategory::PostTransitionMetal,

            // Lanthanides (La-Lu)
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
            | Self::Lu => ElementCategory::Lanthanide,

            // Actinides (Ac-Lr)
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
            | Self::Lr => ElementCategory::Actinide,
        }
    }
}

impl ElementClassification for crate::Isotope {
    fn classification(&self) -> ElementCategory {
        self.element().classification()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::{ElementCategory, ElementClassification};

    #[test]
    fn test_all_elements_valid() {
        for element in crate::Element::iter() {
            let _ = element.classification();
        }
    }

    #[test]
    fn test_iron_transition_metal() {
        assert_eq!(crate::Element::Fe.classification(), ElementCategory::TransitionMetal);
    }

    #[test]
    fn test_hydrogen_non_metal() {
        assert_eq!(crate::Element::H.classification(), ElementCategory::NonMetal);
    }

    #[test]
    fn test_astatine_halogen() {
        assert_eq!(crate::Element::At.classification(), ElementCategory::Halogen);
    }

    #[test]
    fn test_tennessine_halogen() {
        assert_eq!(crate::Element::Ts.classification(), ElementCategory::Halogen);
    }

    #[test]
    fn test_oganesson_noble_gas() {
        assert_eq!(crate::Element::Og.classification(), ElementCategory::NobleGas);
    }

    #[test]
    fn test_all_categories_covered() {
        assert_eq!(crate::Element::Na.classification(), ElementCategory::AlkaliMetal);
        assert_eq!(crate::Element::Ca.classification(), ElementCategory::AlkalineEarthMetal);
        assert_eq!(crate::Element::Al.classification(), ElementCategory::PostTransitionMetal);
        assert_eq!(crate::Element::Si.classification(), ElementCategory::Metalloid);
        assert_eq!(crate::Element::La.classification(), ElementCategory::Lanthanide);
        assert_eq!(crate::Element::U.classification(), ElementCategory::Actinide);
    }

    #[test]
    fn test_is_metal_true() {
        assert!(crate::Element::Fe.is_metal());
        assert!(crate::Element::Cu.is_metal());
        assert!(crate::Element::La.is_metal());
        assert!(crate::Element::U.is_metal());
    }

    #[test]
    fn test_is_metal_false() {
        assert!(!crate::Element::H.is_metal());
        assert!(!crate::Element::C.is_metal());
        assert!(!crate::Element::B.is_metal());
        assert!(!crate::Element::He.is_metal());
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let element_class = element.classification();
            for isotope in element.isotopes() {
                let isotope_class = isotope.classification();
                assert_eq!(
                    element_class, isotope_class,
                    "Classification mismatch for isotope {isotope:?} of element {element:?}",
                );
            }
        }
    }
}
