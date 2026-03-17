//! Covalent radii for chemical elements.

use crate::isotopes::ElementVariant;

/// Source-specific covalent radii for chemical elements.
///
/// Returned values are in angstrom.
pub trait CovalentRadius {
    /// Returns the Cordero single-bond covalent radius, if available.
    ///
    /// Values are returned in angstrom.
    ///
    /// Conventions used for this scalar API:
    /// - carbon uses the sp3 value from the source table
    /// - manganese, iron, and cobalt use the low-spin value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{CovalentRadius, Element};
    ///
    /// assert_eq!(Element::C.cordero_covalent_radius(), Some(0.76));
    /// assert_eq!(Element::Mn.cordero_covalent_radius(), Some(1.39));
    /// assert_eq!(Element::Bk.cordero_covalent_radius(), None);
    /// ```
    fn cordero_covalent_radius(&self) -> Option<f64>;
}

#[inline]
fn radius_from_pm(element: crate::Element, values_pm: &[Option<u16>; 118]) -> Option<f64> {
    let atomic_number: u8 = element.into();
    values_pm[usize::from(atomic_number - 1)].map(|pm| f64::from(pm) / 100.0)
}

#[rustfmt::skip]
const CORDERO_PM: [Option<u16>; 118] = [
    Some(31),  Some(28),  Some(128), Some(96),  Some(84),  Some(76),  Some(71),  Some(66),
    Some(57),  Some(58),  Some(166), Some(141), Some(121), Some(111), Some(107), Some(105),
    Some(102), Some(106), Some(203), Some(176), Some(170), Some(160), Some(153), Some(139),
    Some(139), Some(132), Some(126), Some(124), Some(132), Some(122), Some(122), Some(120),
    Some(119), Some(120), Some(120), Some(116), Some(220), Some(195), Some(190), Some(175),
    Some(164), Some(154), Some(147), Some(146), Some(142), Some(139), Some(145), Some(144),
    Some(142), Some(139), Some(139), Some(138), Some(139), Some(140), Some(244), Some(215),
    Some(207), Some(204), Some(203), Some(201), Some(199), Some(198), Some(198), Some(196),
    Some(194), Some(192), Some(192), Some(189), Some(190), Some(187), Some(187), Some(175),
    Some(170), Some(162), Some(151), Some(144), Some(141), Some(136), Some(136), Some(132),
    Some(145), Some(146), Some(148), Some(140), Some(150), Some(150), Some(260), Some(221),
    Some(215), Some(206), Some(200), Some(196), Some(190), Some(187), Some(180), Some(169),
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,
];

impl CovalentRadius for crate::Element {
    fn cordero_covalent_radius(&self) -> Option<f64> {
        radius_from_pm(*self, &CORDERO_PM)
    }
}

impl CovalentRadius for crate::Isotope {
    fn cordero_covalent_radius(&self) -> Option<f64> {
        self.element().cordero_covalent_radius()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::CovalentRadius;

    fn assert_radius_eq(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("radius should be present");
        assert!((actual - expected).abs() < 1.0e-12, "expected {expected}, got {actual}");
    }

    #[test]
    fn test_cordero_examples() {
        assert_radius_eq(crate::Element::C.cordero_covalent_radius(), 0.76);
        assert_radius_eq(crate::Element::Mn.cordero_covalent_radius(), 1.39);
        assert_radius_eq(crate::Element::Fe.cordero_covalent_radius(), 1.32);
        assert_radius_eq(crate::Element::Cm.cordero_covalent_radius(), 1.69);
        assert!(crate::Element::Bk.cordero_covalent_radius().is_none());
    }

    #[test]
    fn test_positive_ranges() {
        for element in crate::Element::iter() {
            if let Some(radius) = element.cordero_covalent_radius() {
                assert!(radius > 0.0 && radius < 3.0);
            }
        }
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let radius = element.cordero_covalent_radius();
            for isotope in element.isotopes() {
                assert_eq!(isotope.cordero_covalent_radius(), radius);
            }
        }
    }
}
