//! Atomic numbers (number of protons) for chemical elements.

use crate::isotopes::ElementVariant;

/// Number of protons in the nucleus.
pub trait AtomicNumber {
    /// Returns the atomic number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{
    ///     AtomicNumber, Element,
    ///     isotopes::{HydrogenIsotope, Isotope},
    /// };
    ///
    /// assert_eq!(Element::H.atomic_number(), 1);
    /// assert_eq!(Element::O.atomic_number(), 8);
    ///
    /// let deuterium = Isotope::H(HydrogenIsotope::D);
    /// assert_eq!(deuterium.atomic_number(), 1);
    /// ```
    fn atomic_number(&self) -> u8;
}

impl AtomicNumber for crate::Element {
    #[inline]
    fn atomic_number(&self) -> u8 {
        (*self).into()
    }
}

impl AtomicNumber for crate::Isotope {
    #[inline]
    fn atomic_number(&self) -> u8 {
        self.element().atomic_number()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::AtomicNumber;
    use crate::isotopes::{ElementVariant, Isotope, MassNumber};

    #[test]
    fn test_atomic_number_examples() {
        assert_eq!(crate::Element::H.atomic_number(), 1);
        assert_eq!(crate::Element::C.atomic_number(), 6);
        assert_eq!(crate::Element::O.atomic_number(), 8);
        assert_eq!(crate::Element::Fe.atomic_number(), 26);
        assert_eq!(crate::Element::U.atomic_number(), 92);
        assert_eq!(crate::Element::Og.atomic_number(), 118);
    }

    #[test]
    fn test_atomic_number_matches_integer_conversion() {
        for element in crate::Element::iter() {
            let atomic_number = element.atomic_number();

            assert_eq!(atomic_number, u8::from(element));
            assert!((1..=118).contains(&atomic_number));
        }
    }

    #[test]
    fn test_atomic_number_roundtrip() {
        for element in crate::Element::iter() {
            let atomic_number = element.atomic_number();

            assert_eq!(crate::Element::try_from(atomic_number), Ok(element));
        }
    }

    #[test]
    fn test_atomic_number_is_sequential() {
        for (index, element) in crate::Element::iter().enumerate() {
            assert_eq!(usize::from(element.atomic_number()), index + 1);
        }
    }

    #[test]
    fn test_isotope_atomic_number_delegation_examples() {
        let h2 = Isotope::H(crate::isotopes::HydrogenIsotope::D);
        assert_eq!(h2.atomic_number(), 1);

        let c14 = Isotope::C(crate::isotopes::CarbonIsotope::C14);
        assert_eq!(c14.atomic_number(), 6);

        let o18 = Isotope::O(crate::isotopes::OxygenIsotope::O18);
        assert_eq!(o18.atomic_number(), 8);

        let ar40 = Isotope::Ar(crate::isotopes::ArgonIsotope::Ar40);
        assert_eq!(ar40.atomic_number(), 18);

        let u238 = Isotope::U(crate::isotopes::UraniumIsotope::U238);
        assert_eq!(u238.atomic_number(), 92);
    }

    #[test]
    fn test_isotope_atomic_number_delegation() {
        for element in crate::Element::iter() {
            let atomic_number = element.atomic_number();

            for isotope in element.isotopes() {
                assert_eq!(
                    isotope.atomic_number(),
                    atomic_number,
                    "Atomic number mismatch for isotope {isotope:?} of element {element:?}",
                );
                assert_eq!(isotope.atomic_number(), isotope.element().atomic_number());
                assert_eq!(isotope.atomic_number(), u8::from(isotope.element()));
            }
        }
    }

    #[test]
    fn test_isotope_atomic_number_is_not_greater_than_mass_number() {
        for element in crate::Element::iter() {
            for isotope in element.isotopes() {
                assert!(
                    u16::from(isotope.atomic_number()) <= isotope.mass_number(),
                    "Atomic number should not exceed mass number for {isotope:?}",
                );
            }
        }
    }
}
