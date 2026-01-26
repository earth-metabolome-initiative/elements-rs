//! Implementation of `Into<u8>` for `Element`.

use crate::Element;

impl From<crate::Element> for u8 {
    /// Converts an element to its atomic number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let atomic_number: u8 = Element::H.into();
    /// assert_eq!(atomic_number, 1);
    ///
    /// let oxygen: u8 = Element::O.into();
    /// assert_eq!(oxygen, 8);
    /// ```
    #[allow(clippy::too_many_lines)]
    fn from(element: crate::Element) -> Self {
        match element {
            crate::Element::H => 1,
            crate::Element::He => 2,
            crate::Element::Li => 3,
            crate::Element::Be => 4,
            crate::Element::B => 5,
            crate::Element::C => 6,
            crate::Element::N => 7,
            crate::Element::O => 8,
            crate::Element::F => 9,
            crate::Element::Ne => 10,
            crate::Element::Na => 11,
            crate::Element::Mg => 12,
            crate::Element::Al => 13,
            crate::Element::Si => 14,
            crate::Element::P => 15,
            crate::Element::S => 16,
            crate::Element::Cl => 17,
            crate::Element::Ar => 18,
            crate::Element::K => 19,
            crate::Element::Ca => 20,
            crate::Element::Sc => 21,
            crate::Element::Ti => 22,
            crate::Element::V => 23,
            crate::Element::Cr => 24,
            crate::Element::Mn => 25,
            crate::Element::Fe => 26,
            crate::Element::Co => 27,
            crate::Element::Ni => 28,
            crate::Element::Cu => 29,
            crate::Element::Zn => 30,
            crate::Element::Ga => 31,
            crate::Element::Ge => 32,
            crate::Element::As => 33,
            crate::Element::Se => 34,
            crate::Element::Br => 35,
            crate::Element::Kr => 36,
            crate::Element::Rb => 37,
            crate::Element::Sr => 38,
            crate::Element::Y => 39,
            crate::Element::Zr => 40,
            crate::Element::Nb => 41,
            crate::Element::Mo => 42,
            crate::Element::Tc => 43,
            crate::Element::Ru => 44,
            crate::Element::Rh => 45,
            crate::Element::Pd => 46,
            crate::Element::Ag => 47,
            crate::Element::Cd => 48,
            crate::Element::In => 49,
            crate::Element::Sn => 50,
            crate::Element::Sb => 51,
            crate::Element::Te => 52,
            crate::Element::I => 53,
            crate::Element::Xe => 54,
            crate::Element::Cs => 55,
            crate::Element::Ba => 56,
            crate::Element::La => 57,
            crate::Element::Ce => 58,
            crate::Element::Pr => 59,
            crate::Element::Nd => 60,
            crate::Element::Pm => 61,
            crate::Element::Sm => 62,
            crate::Element::Eu => 63,
            crate::Element::Gd => 64,
            crate::Element::Tb => 65,
            crate::Element::Dy => 66,
            crate::Element::Ho => 67,
            crate::Element::Er => 68,
            crate::Element::Tm => 69,
            crate::Element::Yb => 70,
            crate::Element::Lu => 71,
            crate::Element::Hf => 72,
            crate::Element::Ta => 73,
            crate::Element::W => 74,
            crate::Element::Re => 75,
            crate::Element::Os => 76,
            crate::Element::Ir => 77,
            crate::Element::Pt => 78,
            crate::Element::Au => 79,
            crate::Element::Hg => 80,
            crate::Element::Tl => 81,
            crate::Element::Pb => 82,
            crate::Element::Bi => 83,
            crate::Element::Po => 84,
            crate::Element::At => 85,
            crate::Element::Rn => 86,
            crate::Element::Fr => 87,
            crate::Element::Ra => 88,
            crate::Element::Ac => 89,
            crate::Element::Th => 90,
            crate::Element::Pa => 91,
            crate::Element::U => 92,
            crate::Element::Np => 93,
            crate::Element::Pu => 94,
            crate::Element::Am => 95,
            crate::Element::Cm => 96,
            crate::Element::Bk => 97,
            crate::Element::Cf => 98,
            crate::Element::Es => 99,
            crate::Element::Fm => 100,
            crate::Element::Md => 101,
            crate::Element::No => 102,
            crate::Element::Lr => 103,
            crate::Element::Rf => 104,
            crate::Element::Db => 105,
            crate::Element::Sg => 106,
            crate::Element::Bh => 107,
            crate::Element::Hs => 108,
            crate::Element::Mt => 109,
            crate::Element::Ds => 110,
            crate::Element::Rg => 111,
            crate::Element::Cn => 112,
            crate::Element::Nh => 113,
            crate::Element::Fl => 114,
            crate::Element::Mc => 115,
            crate::Element::Lv => 116,
            crate::Element::Ts => 117,
            crate::Element::Og => 118,
        }
    }
}

/// Macro implementing the `From<Element> for {integer}` trait for integer
/// types.
macro_rules! impl_try_from_u8_for_element {
    ($($t:ty),*) => {
        $(
            impl From<crate::Element> for $t {
                fn from(element: crate::Element) -> Self {
                    let atomic_number: u8 = element.into();
                    <$t>::from(atomic_number)
                }
            }
        )*
    };
}

impl_try_from_u8_for_element!(u16, u32, u64, u128, usize, i16, i32, i64, i128, isize);

impl TryFrom<u8> for Element {
    type Error = crate::errors::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(atomic_number: u8) -> Result<Self, Self::Error> {
        Ok(match atomic_number {
            1 => crate::Element::H,
            2 => crate::Element::He,
            3 => crate::Element::Li,
            4 => crate::Element::Be,
            5 => crate::Element::B,
            6 => crate::Element::C,
            7 => crate::Element::N,
            8 => crate::Element::O,
            9 => crate::Element::F,
            10 => crate::Element::Ne,
            11 => crate::Element::Na,
            12 => crate::Element::Mg,
            13 => crate::Element::Al,
            14 => crate::Element::Si,
            15 => crate::Element::P,
            16 => crate::Element::S,
            17 => crate::Element::Cl,
            18 => crate::Element::Ar,
            19 => crate::Element::K,
            20 => crate::Element::Ca,
            21 => crate::Element::Sc,
            22 => crate::Element::Ti,
            23 => crate::Element::V,
            24 => crate::Element::Cr,
            25 => crate::Element::Mn,
            26 => crate::Element::Fe,
            27 => crate::Element::Co,
            28 => crate::Element::Ni,
            29 => crate::Element::Cu,
            30 => crate::Element::Zn,
            31 => crate::Element::Ga,
            32 => crate::Element::Ge,
            33 => crate::Element::As,
            34 => crate::Element::Se,
            35 => crate::Element::Br,
            36 => crate::Element::Kr,
            37 => crate::Element::Rb,
            38 => crate::Element::Sr,
            39 => crate::Element::Y,
            40 => crate::Element::Zr,
            41 => crate::Element::Nb,
            42 => crate::Element::Mo,
            43 => crate::Element::Tc,
            44 => crate::Element::Ru,
            45 => crate::Element::Rh,
            46 => crate::Element::Pd,
            47 => crate::Element::Ag,
            48 => crate::Element::Cd,
            49 => crate::Element::In,
            50 => crate::Element::Sn,
            51 => crate::Element::Sb,
            52 => crate::Element::Te,
            53 => crate::Element::I,
            54 => crate::Element::Xe,
            55 => crate::Element::Cs,
            56 => crate::Element::Ba,
            57 => crate::Element::La,
            58 => crate::Element::Ce,
            59 => crate::Element::Pr,
            60 => crate::Element::Nd,
            61 => crate::Element::Pm,
            62 => crate::Element::Sm,
            63 => crate::Element::Eu,
            64 => crate::Element::Gd,
            65 => crate::Element::Tb,
            66 => crate::Element::Dy,
            67 => crate::Element::Ho,
            68 => crate::Element::Er,
            69 => crate::Element::Tm,
            70 => crate::Element::Yb,
            71 => crate::Element::Lu,
            72 => crate::Element::Hf,
            73 => crate::Element::Ta,
            74 => crate::Element::W,
            75 => crate::Element::Re,
            76 => crate::Element::Os,
            77 => crate::Element::Ir,
            78 => crate::Element::Pt,
            79 => crate::Element::Au,
            80 => crate::Element::Hg,
            81 => crate::Element::Tl,
            82 => crate::Element::Pb,
            83 => crate::Element::Bi,
            84 => crate::Element::Po,
            85 => crate::Element::At,
            86 => crate::Element::Rn,
            87 => crate::Element::Fr,
            88 => crate::Element::Ra,
            89 => crate::Element::Ac,
            90 => crate::Element::Th,
            91 => crate::Element::Pa,
            92 => crate::Element::U,
            93 => crate::Element::Np,
            94 => crate::Element::Pu,
            95 => crate::Element::Am,
            96 => crate::Element::Cm,
            97 => crate::Element::Bk,
            98 => crate::Element::Cf,
            99 => crate::Element::Es,
            100 => crate::Element::Fm,
            101 => crate::Element::Md,
            102 => crate::Element::No,
            103 => crate::Element::Lr,
            104 => crate::Element::Rf,
            105 => crate::Element::Db,
            106 => crate::Element::Sg,
            107 => crate::Element::Bh,
            108 => crate::Element::Hs,
            109 => crate::Element::Mt,
            110 => crate::Element::Ds,
            111 => crate::Element::Rg,
            112 => crate::Element::Cn,
            113 => crate::Element::Nh,
            114 => crate::Element::Fl,
            115 => crate::Element::Mc,
            116 => crate::Element::Lv,
            117 => crate::Element::Ts,
            118 => crate::Element::Og,
            _ => {
                return Err(crate::errors::Error::AtomicNumber(atomic_number));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_from_element_to_integer() {
        for element in Element::iter() {
            let atomic_number_u8: u8 = element.into();
            // Each element should have a valid atomic number between 1 and 118
            assert!((1..=118).contains(&atomic_number_u8));
            let atomic_number_u16: u16 = element.into();
            assert_eq!(atomic_number_u16 as u8, atomic_number_u8);
            let atomic_number_u32: u32 = element.into();
            assert_eq!(atomic_number_u32 as u8, atomic_number_u8);
            let atomic_number_u64: u64 = element.into();
            assert_eq!(atomic_number_u64 as u8, atomic_number_u8);
            let atomic_number_u128: u128 = element.into();
            assert_eq!(atomic_number_u128 as u8, atomic_number_u8);
            let atomic_number_usize: usize = element.into();
            assert_eq!(atomic_number_usize as u8, atomic_number_u8);
            let atomic_number_i16: i16 = element.into();
            assert_eq!(atomic_number_i16 as u8, atomic_number_u8);
            let atomic_number_i32: i32 = element.into();
            assert_eq!(atomic_number_i32 as u8, atomic_number_u8);
            let atomic_number_i64: i64 = element.into();
            assert_eq!(atomic_number_i64 as u8, atomic_number_u8);
            let atomic_number_i128: i128 = element.into();
            assert_eq!(atomic_number_i128 as u8, atomic_number_u8);
            let atomic_number_isize: isize = element.into();
            assert_eq!(atomic_number_isize as u8, atomic_number_u8);
        }
    }

    #[test]
    fn test_try_from_u8_to_element() {
        for element in Element::iter() {
            let atomic_number: u8 = element.into();
            let result: Result<Element, crate::errors::Error> = atomic_number.try_into();
            assert_eq!(result, Ok(element));
        }
    }

    #[test]
    fn test_roundtrip_via_u8() {
        for element in Element::iter() {
            let atomic_number: u8 = element.into();
            let roundtrip_element: Element = atomic_number.try_into().unwrap();
            assert_eq!(roundtrip_element, element);
        }
    }

    #[test]
    fn test_invalid_atomic_numbers() {
        // Test atomic number 0 (invalid)
        let result: Result<Element, crate::errors::Error> = 0u8.try_into();
        assert_eq!(result, Err(crate::errors::Error::AtomicNumber(0)));

        // Test atomic number 119 (invalid, beyond known elements)
        let result: Result<Element, crate::errors::Error> = 119u8.try_into();
        assert_eq!(result, Err(crate::errors::Error::AtomicNumber(119)));

        // Test atomic number 255 (invalid, maximum u8 value)
        let result: Result<Element, crate::errors::Error> = 255u8.try_into();
        assert_eq!(result, Err(crate::errors::Error::AtomicNumber(255)));
    }
}
