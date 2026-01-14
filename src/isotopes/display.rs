//! Submodule implementing the `Display` trait for the `Isotope`
//! enum.

use crate::isotopes::{ElementVariant, MassNumber};

impl core::fmt::Display for super::Isotope {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let element = self.element();
        let mass_number = self.mass_number();
        write!(f, "{}-{}", element, mass_number)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_display() {
        for element in crate::Element::iter() {
            let isotopes = element.isotopes();
            for isotope in isotopes {
                let display = format!("{}", isotope);
                assert!(!display.is_empty(), "Display should not be empty for {:?}", isotope);
                // Verify that the display string contains the element symbol
                let element_symbol = format!("{}", element);
                assert!(
                    display.contains(&element_symbol),
                    "Display '{}' for isotope {:?} should contain element symbol '{}'",
                    display,
                    isotope,
                    element_symbol
                );
            }
        }
    }
}
