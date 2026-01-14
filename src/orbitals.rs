//! Electron orbital configurations for elements.

mod element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    const fn new(
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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_orbitals() {
        for element in crate::Element::iter() {
            let orbitals = element.orbitals();
            assert!(!orbitals.is_empty(), "Orbitals should not be empty for {:?}", element);
            let total_electrons: u32 =
                orbitals.iter().map(|o| o.number_of_electrons() as u32).sum();
            let atomic_number = u8::from(element) as u32;
            assert_eq!(
                total_electrons, atomic_number,
                "Total electrons in orbitals should equal atomic number for {:?}",
                element
            );
        }
    }
}
