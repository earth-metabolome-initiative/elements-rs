//! Atomic radii for chemical elements.
//!
//! These datasets intentionally stay source-specific because "atomic radius"
//! does not refer to a single universal standard.

use crate::isotopes::ElementVariant;

/// Source-specific atomic radii for chemical elements.
///
/// Returned values are in angstrom.
pub trait AtomicRadius {
    /// Returns the empirical atomic radius published by Slater, if available.
    ///
    /// Values are returned in angstrom.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{AtomicRadius, Element};
    ///
    /// assert_eq!(Element::H.slater_atomic_radius(), Some(0.25));
    /// assert_eq!(Element::Kr.slater_atomic_radius(), None);
    /// ```
    fn slater_atomic_radius(&self) -> Option<f64>;

    /// Returns the Rahm non-interacting atomic radius, if available.
    ///
    /// Values are returned in angstrom.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{AtomicRadius, Element};
    ///
    /// assert_eq!(Element::O.rahm_atomic_radius(), Some(1.71));
    /// assert_eq!(Element::Bk.rahm_atomic_radius(), None);
    /// ```
    fn rahm_atomic_radius(&self) -> Option<f64>;
}

#[inline]
fn radius_from_pm(element: crate::Element, values_pm: &[Option<u16>; 118]) -> Option<f64> {
    let atomic_number: u8 = element.into();
    values_pm[usize::from(atomic_number - 1)].map(|pm| f64::from(pm) / 100.0)
}

#[rustfmt::skip]
const SLATER_EMPIRICAL_PM: [Option<u16>; 118] = [
    Some(25),  Some(120), Some(145), Some(105), Some(85),  Some(70),  Some(65),  Some(60),
    Some(50),  Some(160), Some(180), Some(150), Some(125), Some(110), Some(100), Some(100),
    Some(100), Some(71),  Some(220), Some(180), Some(160), Some(140), Some(135), Some(140),
    Some(140), Some(140), Some(135), Some(135), Some(135), Some(135), Some(130), Some(125),
    Some(115), Some(115), Some(115), None,      Some(235), Some(200), Some(180), Some(155),
    Some(145), Some(145), Some(135), Some(130), Some(135), Some(140), Some(160), Some(155),
    Some(155), Some(145), Some(145), Some(140), Some(140), None,      Some(260), Some(215),
    Some(195), Some(185), Some(185), Some(185), Some(185), Some(185), Some(185), Some(180),
    Some(175), Some(175), Some(175), Some(175), Some(175), Some(175), Some(175), Some(155),
    Some(145), Some(135), Some(135), Some(130), Some(135), Some(135), Some(135), Some(150),
    Some(190), Some(180), Some(160), Some(190), None,      None,      None,      Some(215),
    Some(195), Some(180), Some(180), Some(175), Some(175), Some(175), Some(175), Some(176),
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,
];

#[rustfmt::skip]
const RAHM_PM: [Option<u16>; 118] = [
    Some(154), Some(134), Some(220), Some(219), Some(205), Some(190), Some(179), Some(171),
    Some(163), Some(156), Some(225), Some(240), Some(239), Some(232), Some(223), Some(214),
    Some(206), Some(197), Some(234), Some(270), Some(263), Some(257), Some(252), Some(233),
    Some(242), Some(237), Some(233), Some(229), Some(217), Some(222), Some(233), Some(234),
    Some(231), Some(224), Some(219), Some(212), Some(240), Some(279), Some(274), Some(269),
    Some(251), Some(244), Some(252), Some(237), Some(233), Some(215), Some(225), Some(238),
    Some(246), Some(248), Some(246), Some(242), Some(238), Some(232), Some(249), Some(293),
    Some(284), Some(282), Some(286), Some(284), Some(283), Some(280), Some(280), Some(277),
    Some(276), Some(275), Some(273), Some(272), Some(271), Some(277), Some(270), Some(264),
    Some(258), Some(253), Some(249), Some(244), Some(240), Some(230), Some(226), Some(229),
    Some(242), Some(249), Some(250), Some(250), Some(247), Some(243), Some(258), Some(292),
    Some(293), Some(288), Some(285), Some(283), Some(281), Some(278), Some(276), Some(276),
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,      None,      None,
    None,      None,      None,      None,      None,      None,
];

impl AtomicRadius for crate::Element {
    fn slater_atomic_radius(&self) -> Option<f64> {
        radius_from_pm(*self, &SLATER_EMPIRICAL_PM)
    }

    fn rahm_atomic_radius(&self) -> Option<f64> {
        radius_from_pm(*self, &RAHM_PM)
    }
}

impl AtomicRadius for crate::Isotope {
    fn slater_atomic_radius(&self) -> Option<f64> {
        self.element().slater_atomic_radius()
    }

    fn rahm_atomic_radius(&self) -> Option<f64> {
        self.element().rahm_atomic_radius()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::AtomicRadius;

    fn assert_radius_eq(actual: Option<f64>, expected: f64) {
        let actual = actual.expect("radius should be present");
        assert!((actual - expected).abs() < 1.0e-12, "expected {expected}, got {actual}");
    }

    #[test]
    fn test_slater_examples() {
        assert_radius_eq(crate::Element::H.slater_atomic_radius(), 0.25);
        assert_radius_eq(crate::Element::C.slater_atomic_radius(), 0.70);
        assert!(crate::Element::Kr.slater_atomic_radius().is_none());
        assert_radius_eq(crate::Element::Cm.slater_atomic_radius(), 1.76);
    }

    #[test]
    fn test_rahm_examples() {
        assert_radius_eq(crate::Element::O.rahm_atomic_radius(), 1.71);
        assert_radius_eq(crate::Element::Xe.rahm_atomic_radius(), 2.32);
        assert_radius_eq(crate::Element::Cm.rahm_atomic_radius(), 2.76);
        assert!(crate::Element::Bk.rahm_atomic_radius().is_none());
    }

    #[test]
    fn test_positive_ranges() {
        for element in crate::Element::iter() {
            if let Some(radius) = element.slater_atomic_radius() {
                assert!(radius > 0.0 && radius < 4.0);
            }
            if let Some(radius) = element.rahm_atomic_radius() {
                assert!(radius > 0.0 && radius < 4.0);
            }
        }
    }

    #[test]
    fn test_isotope_delegation() {
        for element in crate::Element::iter() {
            let slater = element.slater_atomic_radius();
            let rahm = element.rahm_atomic_radius();
            for isotope in element.isotopes() {
                assert_eq!(isotope.slater_atomic_radius(), slater);
                assert_eq!(isotope.rahm_atomic_radius(), rahm);
            }
        }
    }
}
