//! Submodule implementing the `monoisotopic_mass` method for the `Element`
//! enumeration.

use crate::isotopes::RelativeAtomicMass;

impl crate::Element {
    /// Returns the monoisotopic mass of the element.
    ///
    /// This is the exact mass of the element's most abundant isotope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// assert_eq!(Element::C.monoisotopic_mass(), 12.0);
    /// ```
    #[must_use]
    #[inline]
    pub fn monoisotopic_mass(&self) -> f64 {
        self.most_abundant_isotope().relative_atomic_mass()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::isotopes::RelativeAtomicMass;

    #[test]
    fn test_monoisotopic_mass() {
        for element in crate::Element::iter() {
            let monoisotopic_mass = element.monoisotopic_mass();
            let isotope_mass = element.most_abundant_isotope().relative_atomic_mass();
            let relative_atomic_mass = element.relative_atomic_mass();

            assert_eq!(monoisotopic_mass.to_bits(), isotope_mass.to_bits());
            assert_eq!(monoisotopic_mass.to_bits(), relative_atomic_mass.to_bits());
        }
    }
}
