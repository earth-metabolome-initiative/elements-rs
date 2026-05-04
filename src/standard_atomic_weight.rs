//! Atomic-weight convenience values for elements.
//!
//! For elements without a CIAAW standard atomic weight, this module still
//! exposes a representative single value for compatibility.

impl crate::Element {
    #[allow(clippy::too_many_lines)]
    /// Returns the crate's single-value atomic-weight convenience value.
    ///
    /// For elements with a CIAAW standard atomic weight, this is the
    /// single-value form used by the crate. For elements without a CIAAW
    /// standard atomic weight, this method still returns a representative
    /// fallback value for compatibility, typically following the bracketed
    /// value used in the [IUPAC periodic table].
    ///
    /// This means the result should not always be interpreted as a literal
    /// CIAAW standard atomic weight.
    ///
    /// [IUPAC periodic table]: https://iupac.org/what-we-do/periodic-table-of-elements/
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// assert_eq!(Element::H.standard_atomic_weight(), 1.008);
    /// assert_eq!(Element::O.standard_atomic_weight(), 15.999);
    /// ```
    #[must_use]
    pub fn standard_atomic_weight(&self) -> f64 {
        match self {
            Self::H => 1.008,
            Self::He => 4.002602,
            Self::Li => 6.94,
            Self::Be => 9.0121831,
            Self::B => 10.81,
            Self::C => 12.011,
            Self::N => 14.007,
            Self::O => 15.999,
            Self::F => 18.998403162,
            Self::Ne => 20.1797,
            Self::Na => 22.98976928,
            Self::Mg => 24.305,
            Self::Al => 26.9815384,
            Self::Si => 28.085,
            Self::P => 30.973761998,
            Self::S => 32.06,
            Self::Cl => 35.45,
            Self::Ar => 39.948,
            Self::K => 39.0983,
            Self::Ca => 40.078,
            Self::Sc => 44.955907,
            Self::Ti => 47.867,
            Self::V => 50.9415,
            Self::Cr => 51.9961,
            Self::Mn => 54.938043,
            Self::Fe => 55.845,
            Self::Co => 58.933194,
            Self::Ni => 58.6934,
            Self::Cu => 63.546,
            Self::Zn => 65.38,
            Self::Ga => 69.723,
            Self::Ge => 72.63,
            Self::As => 74.921595,
            Self::Se => 78.971,
            Self::Br => 79.904,
            Self::Kr => 83.798,
            Self::Rb => 85.4678,
            Self::Sr => 87.62,
            Self::Y => 88.905838,
            Self::Zr => 91.222,
            Self::Nb => 92.90637,
            Self::Mo => 95.95,
            Self::Tc => 97.90721,
            Self::Ru => 101.07,
            Self::Rh => 102.90549,
            Self::Pd => 106.42,
            Self::Ag => 107.8682,
            Self::Cd => 112.414,
            Self::In => 114.818,
            Self::Sn => 118.71,
            Self::Sb => 121.76,
            Self::Te => 127.6,
            Self::I => 126.90447,
            Self::Xe => 131.293,
            Self::Cs => 132.90545196,
            Self::Ba => 137.327,
            Self::La => 138.90547,
            Self::Ce => 140.116,
            Self::Pr => 140.90766,
            Self::Nd => 144.242,
            Self::Pm => 144.91276,
            Self::Sm => 150.36,
            Self::Eu => 151.964,
            Self::Gd => 157.249,
            Self::Tb => 158.925354,
            Self::Dy => 162.5,
            Self::Ho => 164.930329,
            Self::Er => 167.259,
            Self::Tm => 168.934219,
            Self::Yb => 173.045,
            Self::Lu => 174.96669,
            Self::Hf => 178.486,
            Self::Ta => 180.94788,
            Self::W => 183.84,
            Self::Re => 186.207,
            Self::Os => 190.23,
            Self::Ir => 192.217,
            Self::Pt => 195.084,
            Self::Au => 196.96657,
            Self::Hg => 200.592,
            Self::Tl => 204.38,
            Self::Pb => 207.2,
            Self::Bi => 208.9804,
            Self::Po => 209.0,
            Self::At => 210.0,
            Self::Rn => 222.0,
            Self::Fr => 223.0,
            Self::Ra => 226.0,
            Self::Ac => 227.0,
            Self::Th => 232.0377,
            Self::Pa => 231.03588,
            Self::U => 238.02891,
            Self::Np => 237.0,
            Self::Pu => 244.0,
            Self::Am => 243.0,
            Self::Cm | Self::Bk => 247.0,
            Self::Cf => 251.0,
            Self::Es => 252.0,
            Self::Fm => 257.0,
            Self::Md => 258.0,
            Self::No => 259.0,
            Self::Lr => 262.0,
            Self::Rf => 267.0,
            Self::Db => 268.0,
            Self::Sg | Self::Hs => 269.0,
            Self::Bh => 270.0,
            Self::Mt => 277.0,
            Self::Ds => 281.0,
            Self::Rg => 282.0,
            Self::Cn => 285.0,
            Self::Nh => 286.0,
            Self::Fl | Self::Mc => 290.0,
            Self::Lv => 293.0,
            Self::Ts | Self::Og => 294.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_atomic_weight_values_are_positive() {
        for element in crate::Element::iter() {
            let weight = element.standard_atomic_weight();
            assert!(weight > 0.0, "Atomic-weight value should be positive for {element:?}");
        }
    }

    #[test]
    fn test_updated_iupac_periodic_table_fallback_values() {
        for (element, expected) in [
            (crate::Element::Sg, 269.0),
            (crate::Element::Bh, 270.0),
            (crate::Element::Mt, 277.0),
            (crate::Element::Rg, 282.0),
            (crate::Element::Fl, 290.0),
            (crate::Element::Mc, 290.0),
        ] {
            let weight = element.standard_atomic_weight();
            assert!(
                (weight - expected).abs() < f64::EPSILON,
                "expected {expected} for {element:?}, got {weight}"
            );
        }
    }

    #[test]
    fn test_updated_ciaaw_single_values() {
        for (element, expected) in [
            (crate::Element::F, 18.998403162),
            (crate::Element::Na, 22.98976928),
            (crate::Element::Al, 26.9815384),
            (crate::Element::Sc, 44.955907),
            (crate::Element::Mn, 54.938043),
            (crate::Element::Y, 88.905838),
            (crate::Element::Zr, 91.222),
            (crate::Element::Rh, 102.90549),
            (crate::Element::Gd, 157.249),
            (crate::Element::Tb, 158.925354),
            (crate::Element::Ho, 164.930329),
            (crate::Element::Tm, 168.934219),
            (crate::Element::Lu, 174.96669),
            (crate::Element::Hf, 178.486),
            (crate::Element::Au, 196.96657),
        ] {
            let weight = element.standard_atomic_weight();
            assert!(
                (weight - expected).abs() <= f64::EPSILON,
                "expected {expected} for {element:?}, got {weight}"
            );
        }
    }
}
