//! van der Waals radii for chemical elements.

use crate::isotopes::ElementVariant;

/// Source-specific van der Waals radii for chemical elements.
///
/// Returned values are in angstrom.
pub trait VanDerWaalsRadius {
    /// Returns the Bondi van der Waals radius, if available.
    ///
    /// Values are returned in angstrom.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, VanDerWaalsRadius};
    ///
    /// assert_eq!(Element::H.bondi_van_der_waals_radius(), Some(1.20));
    /// assert_eq!(Element::Ca.bondi_van_der_waals_radius(), None);
    /// ```
    fn bondi_van_der_waals_radius(&self) -> Option<f64>;

    /// Returns the Mantina main-group extension radius, if available.
    ///
    /// Values are returned in angstrom.
    ///
    /// This keeps the 2009 main-group set separate from Bondi-only transition
    /// metal values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, VanDerWaalsRadius};
    ///
    /// assert_eq!(Element::H.mantina_van_der_waals_radius(), Some(1.10));
    /// assert_eq!(Element::Ca.mantina_van_der_waals_radius(), Some(2.31));
    /// assert_eq!(Element::Ni.mantina_van_der_waals_radius(), None);
    /// ```
    fn mantina_van_der_waals_radius(&self) -> Option<f64>;
}

#[inline]
fn radius_from_pm(element: crate::Element, values_pm: &[Option<u16>; 118]) -> Option<f64> {
    let atomic_number: u8 = element.into();
    values_pm[usize::from(atomic_number - 1)].map(|pm| f64::from(pm) / 100.0)
}

#[rustfmt::skip]
const BONDI_PM: [Option<u16>; 118] = [
    Some(120), Some(140), Some(182), None,      None,      Some(170), Some(155), Some(152),
    Some(147), Some(154), Some(227), Some(173), None,      Some(210), Some(180), Some(180),
    Some(175), Some(188), Some(275), None,      None,      None,      None,      None,
    None,      None,      None,      Some(163), Some(140), Some(139), Some(187), None,
    Some(185), Some(190), Some(185), Some(202), None,      None,      None,      None,
    None,      None,      None,      None,      None,      Some(163), Some(172), Some(158),
    Some(193), Some(217), None,      Some(206), Some(198), Some(216), None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      Some(175), Some(166),
    Some(155), Some(196), Some(202), None,      None,      None,      None,      None,
    None,      None,      None,      None,      Some(186), None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,
];

#[rustfmt::skip]
const MANTINA_PM: [Option<u16>; 118] = [
    Some(110), Some(140), Some(181), Some(153), Some(192), Some(170), Some(155), Some(152),
    Some(147), Some(154), Some(227), Some(173), Some(184), Some(210), Some(180), Some(180),
    Some(175), Some(188), Some(275), Some(231), None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      Some(187), Some(211),
    Some(185), Some(190), Some(183), Some(202), Some(303), Some(249), None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    Some(193), Some(217), Some(206), Some(206), Some(198), Some(216), Some(343), Some(268),
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    Some(196), Some(202), Some(207), Some(197), Some(202), Some(220), Some(348), Some(283),
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,
];

impl VanDerWaalsRadius for crate::Element {
    fn bondi_van_der_waals_radius(&self) -> Option<f64> {
        radius_from_pm(*self, &BONDI_PM)
    }

    fn mantina_van_der_waals_radius(&self) -> Option<f64> {
        radius_from_pm(*self, &MANTINA_PM)
    }
}

impl VanDerWaalsRadius for crate::Isotope {
    fn bondi_van_der_waals_radius(&self) -> Option<f64> {
        self.element().bondi_van_der_waals_radius()
    }

    fn mantina_van_der_waals_radius(&self) -> Option<f64> {
        self.element().mantina_van_der_waals_radius()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::VanDerWaalsRadius;

    fn assert_radius_eq(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("radius should be present");
        assert!((actual - expected).abs() < 1.0e-12, "expected {expected}, got {actual}");
    }

    #[test]
    fn test_bondi_examples() {
        assert_radius_eq(crate::Element::H.bondi_van_der_waals_radius(), 1.20);
        assert_radius_eq(crate::Element::Ni.bondi_van_der_waals_radius(), 1.63);
        assert!(crate::Element::Ca.bondi_van_der_waals_radius().is_none());
        assert!(crate::Element::Og.bondi_van_der_waals_radius().is_none());
    }

    #[test]
    fn test_mantina_examples() {
        assert_radius_eq(crate::Element::H.mantina_van_der_waals_radius(), 1.10);
        assert_radius_eq(crate::Element::Ca.mantina_van_der_waals_radius(), 2.31);
        assert!(crate::Element::Ni.mantina_van_der_waals_radius().is_none());
        assert!(crate::Element::Og.mantina_van_der_waals_radius().is_none());
    }

    #[test]
    fn test_positive_ranges() {
        for element in crate::Element::iter() {
            if let Some(radius) = element.bondi_van_der_waals_radius() {
                assert!(radius > 0.0 && radius < 4.0);
            }
            if let Some(radius) = element.mantina_van_der_waals_radius() {
                assert!(radius > 0.0 && radius < 4.0);
            }
        }
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let bondi = element.bondi_van_der_waals_radius();
            let mantina = element.mantina_van_der_waals_radius();
            for isotope in element.isotopes() {
                assert_eq!(isotope.bondi_van_der_waals_radius(), bondi);
                assert_eq!(isotope.mantina_van_der_waals_radius(), mantina);
            }
        }
    }
}
