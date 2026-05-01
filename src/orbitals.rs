//! Electron orbital configurations for elements.

mod element;

use crate::isotopes::ElementVariant;

/// Electron orbital configurations for atoms.
pub trait Orbitals {
    /// Returns the orbitals associated to the atom.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{
    ///     AtomicOrbitalType, Element, Orbitals,
    ///     isotopes::{HydrogenIsotope, Isotope},
    /// };
    ///
    /// let orbitals = Element::H.orbitals();
    /// assert_eq!(orbitals.len(), 1);
    /// assert_eq!(orbitals[0].orbital_type(), AtomicOrbitalType::S);
    ///
    /// let deuterium = Isotope::H(HydrogenIsotope::D);
    /// assert_eq!(deuterium.orbitals(), orbitals);
    /// ```
    fn orbitals(&self) -> &'static [AtomicOrbital];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "mem_size", derive(mem_dbg::MemSize))]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_size", mem_size(flat))]
/// Atomic orbital type (s, p, d, or f).
pub enum AtomicOrbitalType {
    /// s orbital
    S,
    /// p orbital
    P,
    /// d orbital
    D,
    /// f orbital
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "mem_size", derive(mem_dbg::MemSize))]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_size", mem_size(flat))]
/// An atomic orbital with its quantum number and electron count.
pub struct AtomicOrbital {
    /// The principal quantum number of the orbital
    principal_quantum_number: u8,
    /// The type of the orbital as defined by the azimuthal quantum number
    orbital_type: AtomicOrbitalType,
    /// The number of electrons in the orbital
    number_of_electrons: u8,
}

impl AtomicOrbital {
    #[must_use]
    /// Creates a new [`AtomicOrbital`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{AtomicOrbital, AtomicOrbitalType};
    ///
    /// let orbital = AtomicOrbital::new(1, AtomicOrbitalType::S, 2);
    /// assert_eq!(orbital.principal_quantum_number(), 1);
    /// assert_eq!(orbital.orbital_type(), AtomicOrbitalType::S);
    /// assert_eq!(orbital.number_of_electrons(), 2);
    /// ```
    pub const fn new(
        principal_quantum_number: u8,
        orbital_type: AtomicOrbitalType,
        number_of_electrons: u8,
    ) -> Self {
        Self { principal_quantum_number, orbital_type, number_of_electrons }
    }

    /// Returns the principal quantum number (n).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let orbitals = Element::H.orbitals();
    /// assert_eq!(orbitals[0].principal_quantum_number(), 1);
    /// ```
    #[must_use]
    pub fn principal_quantum_number(&self) -> u8 {
        self.principal_quantum_number
    }

    /// Returns the orbital type (s, p, d, or f).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{AtomicOrbitalType, Element};
    ///
    /// let orbitals = Element::H.orbitals();
    /// assert_eq!(orbitals[0].orbital_type(), AtomicOrbitalType::S);
    /// ```
    #[must_use]
    pub fn orbital_type(&self) -> AtomicOrbitalType {
        self.orbital_type
    }

    /// Returns the number of electrons in this orbital.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let orbitals = Element::H.orbitals();
    /// assert_eq!(orbitals[0].number_of_electrons(), 1);
    /// ```
    #[must_use]
    pub fn number_of_electrons(&self) -> u8 {
        self.number_of_electrons
    }
}

impl Orbitals for crate::Element {
    fn orbitals(&self) -> &'static [AtomicOrbital] {
        crate::Element::orbitals(self)
    }
}

impl Orbitals for crate::Isotope {
    fn orbitals(&self) -> &'static [AtomicOrbital] {
        self.element().orbitals()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Orbitals;

    #[test]
    fn test_orbitals() {
        for element in crate::Element::iter() {
            let orbitals = element.orbitals();
            assert!(!orbitals.is_empty(), "Orbitals should not be empty for {element:?}");
            let total_electrons: u32 =
                orbitals.iter().map(|o| u32::from(o.number_of_electrons())).sum();
            let atomic_number = u32::from(u8::from(element));
            assert_eq!(
                total_electrons, atomic_number,
                "Total electrons in orbitals should equal atomic number for {element:?}",
            );
        }
    }

    #[test]
    fn test_trait_matches_element_method() {
        for element in crate::Element::iter() {
            assert_eq!(
                <crate::Element as Orbitals>::orbitals(&element),
                crate::Element::orbitals(&element),
            );
        }
    }

    #[test]
    fn test_isotope_orbitals_examples() {
        let d = crate::Isotope::H(crate::isotopes::HydrogenIsotope::D);
        let d_orbitals = d.orbitals();
        assert_eq!(d_orbitals.len(), 1);
        assert_eq!(d_orbitals[0].principal_quantum_number(), 1);
        assert_eq!(d_orbitals[0].number_of_electrons(), 1);

        let c13 = crate::Isotope::C(crate::isotopes::CarbonIsotope::C13);
        let c_orbitals = c13.orbitals();
        assert_eq!(c_orbitals.len(), 3);
        assert_eq!(
            c_orbitals.iter().map(|orbital| u32::from(orbital.number_of_electrons())).sum::<u32>(),
            6,
        );

        let u238 = crate::Isotope::U(crate::isotopes::UraniumIsotope::U238);
        let u_orbitals = u238.orbitals();
        assert_eq!(
            u_orbitals.iter().map(|orbital| u32::from(orbital.number_of_electrons())).sum::<u32>(),
            92,
        );
    }

    #[test]
    fn test_isotope_orbitals_delegation() {
        for element in crate::Element::iter() {
            let orbitals = element.orbitals();
            let total_electrons: u32 =
                orbitals.iter().map(|orbital| u32::from(orbital.number_of_electrons())).sum();

            for isotope in element.isotopes() {
                let isotope_orbitals = isotope.orbitals();

                assert_eq!(
                    isotope_orbitals, orbitals,
                    "Orbitals mismatch for isotope {isotope:?} of element {element:?}",
                );
                assert_eq!(
                    isotope_orbitals
                        .iter()
                        .map(|orbital| u32::from(orbital.number_of_electrons()))
                        .sum::<u32>(),
                    total_electrons,
                    "Total electrons mismatch for isotope {isotope:?} of element {element:?}",
                );
            }
        }
    }
}
