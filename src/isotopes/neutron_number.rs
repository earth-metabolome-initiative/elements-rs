//! Neutron counts derived from isotope mass and atomic numbers.

use crate::AtomicNumber;

use super::{ElementVariant, MassNumber, NeutronNumber};

impl<T: ElementVariant + MassNumber> NeutronNumber for T {
    #[inline]
    fn neutron_number(&self) -> u16 {
        self.mass_number() - u16::from(self.element().atomic_number())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::{
        AtomicNumber, Element,
        isotopes::{CarbonIsotope, HydrogenIsotope, Isotope, MassNumber, NeutronNumber},
    };

    #[test]
    fn neutron_counts_for_specific_and_general_isotopes() {
        assert_eq!(HydrogenIsotope::H1.neutron_number(), 0);
        assert_eq!(HydrogenIsotope::D.neutron_number(), 1);
        assert_eq!(CarbonIsotope::C12.neutron_number(), 6);
        assert_eq!(Isotope::C(CarbonIsotope::C14).neutron_number(), 8);
    }

    #[test]
    fn neutron_counts_for_all_isotopes() {
        for element in Element::iter() {
            for isotope in element.isotopes() {
                assert_eq!(
                    u16::from(isotope.atomic_number()) + isotope.neutron_number(),
                    isotope.mass_number(),
                    "incorrect neutron count for {isotope:?}",
                );
            }
        }
    }
}
