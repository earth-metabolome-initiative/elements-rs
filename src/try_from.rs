//!  Submodule provding implementations of the `TryFrom` trait for the
//! `Element` enum.

impl TryFrom<char> for crate::Element {
    type Error = crate::errors::Error;

    /// Parses single-character element symbols.
    ///
    /// # Implementation details
    ///
    /// It supports both uppercase and lowercase letters since,
    /// while in chemical formulas element symbols are capitalized,
    /// in other contexts such as SMILES strings they may appear in lowercase
    /// to represent aromatic atoms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let hydrogen = Element::try_from('H').unwrap();
    /// assert_eq!(hydrogen, Element::H);
    ///
    /// let carbon = Element::try_from('C').unwrap();
    /// assert_eq!(carbon, Element::C);
    /// ```
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'H' => Self::H,
            'B' => Self::B,
            'C' => Self::C,
            'N' => Self::N,
            'O' => Self::O,
            'F' => Self::F,
            'P' => Self::P,
            'S' => Self::S,
            'K' => Self::K,
            'V' => Self::V,
            'Y' => Self::Y,
            'I' => Self::I,
            'W' => Self::W,
            'U' => Self::U,
            _ => {
                return Err(crate::errors::Error::Element([value, ' ']));
            }
        })
    }
}

impl TryFrom<[char; 2]> for crate::Element {
    type Error = crate::errors::Error;

    /// Parses two-character element symbols.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let helium = Element::try_from(['H', 'e']).unwrap();
    /// assert_eq!(helium, Element::He);
    ///
    /// let lithium = Element::try_from(['L', 'i']).unwrap();
    /// assert_eq!(lithium, Element::Li);
    /// ```
    #[allow(clippy::too_many_lines)]
    fn try_from(value: [char; 2]) -> Result<Self, Self::Error> {
        // Handle single-character symbols padded with space
        if value[1] == ' ' {
            return match value[0] {
                'H' => Ok(Self::H),
                'B' => Ok(Self::B),
                'C' => Ok(Self::C),
                'N' => Ok(Self::N),
                'O' => Ok(Self::O),
                'F' => Ok(Self::F),
                'P' => Ok(Self::P),
                'S' => Ok(Self::S),
                'K' => Ok(Self::K),
                'V' => Ok(Self::V),
                'Y' => Ok(Self::Y),
                'I' => Ok(Self::I),
                'W' => Ok(Self::W),
                'U' => Ok(Self::U),
                _ => Err(crate::errors::Error::Element(value)),
            };
        }

        // Handle two-character symbols
        Ok(match value {
            ['H', 'e'] => Self::He,
            ['L', 'i'] => Self::Li,
            ['B', 'e'] => Self::Be,
            ['N', 'e'] => Self::Ne,
            ['N', 'a'] => Self::Na,
            ['M', 'g'] => Self::Mg,
            ['A', 'l'] => Self::Al,
            ['S', 'i'] => Self::Si,
            ['C', 'l'] => Self::Cl,
            ['A', 'r'] => Self::Ar,
            ['C', 'a'] => Self::Ca,
            ['S', 'c'] => Self::Sc,
            ['T', 'i'] => Self::Ti,
            ['C', 'r'] => Self::Cr,
            ['M', 'n'] => Self::Mn,
            ['F', 'e'] => Self::Fe,
            ['C', 'o'] => Self::Co,
            ['N', 'i'] => Self::Ni,
            ['C', 'u'] => Self::Cu,
            ['Z', 'n'] => Self::Zn,
            ['G', 'a'] => Self::Ga,
            ['G', 'e'] => Self::Ge,
            ['A', 's'] => Self::As,
            ['S', 'e'] => Self::Se,
            ['B', 'r'] => Self::Br,
            ['K', 'r'] => Self::Kr,
            ['R', 'b'] => Self::Rb,
            ['S', 'r'] => Self::Sr,
            ['Z', 'r'] => Self::Zr,
            ['N', 'b'] => Self::Nb,
            ['M', 'o'] => Self::Mo,
            ['T', 'c'] => Self::Tc,
            ['R', 'u'] => Self::Ru,
            ['R', 'h'] => Self::Rh,
            ['P', 'd'] => Self::Pd,
            ['A', 'g'] => Self::Ag,
            ['C', 'd'] => Self::Cd,
            ['I', 'n'] => Self::In,
            ['S', 'n'] => Self::Sn,
            ['S', 'b'] => Self::Sb,
            ['T', 'e'] => Self::Te,
            ['X', 'e'] => Self::Xe,
            ['C', 's'] => Self::Cs,
            ['B', 'a'] => Self::Ba,
            ['L', 'a'] => Self::La,
            ['C', 'e'] => Self::Ce,
            ['P', 'r'] => Self::Pr,
            ['N', 'd'] => Self::Nd,
            ['P', 'm'] => Self::Pm,
            ['S', 'm'] => Self::Sm,
            ['E', 'u'] => Self::Eu,
            ['G', 'd'] => Self::Gd,
            ['T', 'b'] => Self::Tb,
            ['D', 'y'] => Self::Dy,
            ['H', 'o'] => Self::Ho,
            ['E', 'r'] => Self::Er,
            ['T', 'm'] => Self::Tm,
            ['Y', 'b'] => Self::Yb,
            ['L', 'u'] => Self::Lu,
            ['H', 'f'] => Self::Hf,
            ['T', 'a'] => Self::Ta,
            ['R', 'e'] => Self::Re,
            ['O', 's'] => Self::Os,
            ['I', 'r'] => Self::Ir,
            ['P', 't'] => Self::Pt,
            ['A', 'u'] => Self::Au,
            ['H', 'g'] => Self::Hg,
            ['T', 'l'] => Self::Tl,
            ['P', 'b'] => Self::Pb,
            ['B', 'i'] => Self::Bi,
            ['P', 'o'] => Self::Po,
            ['A', 't'] => Self::At,
            ['R', 'n'] => Self::Rn,
            ['F', 'r'] => Self::Fr,
            ['R', 'a'] => Self::Ra,
            ['A', 'c'] => Self::Ac,
            ['T', 'h'] => Self::Th,
            ['P', 'a'] => Self::Pa,
            ['N', 'p'] => Self::Np,
            ['P', 'u'] => Self::Pu,
            ['A', 'm'] => Self::Am,
            ['C', 'm'] => Self::Cm,
            ['B', 'k'] => Self::Bk,
            ['C', 'f'] => Self::Cf,
            ['E', 's'] => Self::Es,
            ['F', 'm'] => Self::Fm,
            ['M', 'd'] => Self::Md,
            ['N', 'o'] => Self::No,
            ['L', 'r'] => Self::Lr,
            ['R', 'f'] => Self::Rf,
            ['D', 'b'] => Self::Db,
            ['S', 'g'] => Self::Sg,
            ['B', 'h'] => Self::Bh,
            ['H', 's'] => Self::Hs,
            ['M', 't'] => Self::Mt,
            ['D', 's'] => Self::Ds,
            ['R', 'g'] => Self::Rg,
            ['C', 'n'] => Self::Cn,
            ['N', 'h'] => Self::Nh,
            ['F', 'l'] => Self::Fl,
            ['M', 'c'] => Self::Mc,
            ['L', 'v'] => Self::Lv,
            ['T', 's'] => Self::Ts,
            ['O', 'g'] => Self::Og,
            _ => {
                return Err(crate::errors::Error::Element(value));
            }
        })
    }
}

impl TryFrom<&str> for crate::Element {
    type Error = crate::errors::Error;

    /// Parses element symbols from string slices.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let oxygen = Element::try_from("O").unwrap();
    /// assert_eq!(oxygen, Element::O);
    ///
    /// let magnesium = Element::try_from("Mg").unwrap();
    /// assert_eq!(magnesium, Element::Mg);
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as core::str::FromStr>::from_str(value)
    }
}

impl TryFrom<crate::Element> for char {
    type Error = crate::errors::Error;

    /// Converts an element to its single-character symbol.
    ///
    /// This only works for elements with single-character symbols.
    /// For elements with two-character symbols, this returns an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let hydrogen_char: char = Element::H.try_into().unwrap();
    /// assert_eq!(hydrogen_char, 'H');
    ///
    /// let carbon_char: char = Element::C.try_into().unwrap();
    /// assert_eq!(carbon_char, 'C');
    ///
    /// // This will fail for two-character symbols
    /// let helium_result: Result<char, _> = Element::He.try_into();
    /// assert!(helium_result.is_err());
    /// ```
    fn try_from(value: crate::Element) -> Result<Self, Self::Error> {
        let symbol: &str = value.as_ref();
        if symbol.len() == 1 {
            Ok(symbol.chars().next().unwrap())
        } else {
            Err(crate::errors::Error::Element([
                symbol.chars().nth(0).unwrap(),
                symbol.chars().nth(1).unwrap(),
            ]))
        }
    }
}

impl TryFrom<crate::Element> for [char; 2] {
    type Error = crate::errors::Error;

    /// Converts an element to its symbol as a character array.
    ///
    /// Single-character symbols are padded with a space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let hydrogen_chars: [char; 2] = Element::H.try_into().unwrap();
    /// assert_eq!(hydrogen_chars, ['H', ' ']);
    ///
    /// let helium_chars: [char; 2] = Element::He.try_into().unwrap();
    /// assert_eq!(helium_chars, ['H', 'e']);
    /// ```
    fn try_from(value: crate::Element) -> Result<Self, Self::Error> {
        let symbol: &str = value.as_ref();
        let mut chars = symbol.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap_or(' ');
        Ok([first, second])
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_try_from_char() {
        // Test single-character symbols that should work
        let single_char_elements = alloc::vec![
            (crate::Element::H, 'H'),
            (crate::Element::B, 'B'),
            (crate::Element::C, 'C'),
            (crate::Element::N, 'N'),
            (crate::Element::O, 'O'),
            (crate::Element::F, 'F'),
            (crate::Element::P, 'P'),
            (crate::Element::S, 'S'),
            (crate::Element::K, 'K'),
            (crate::Element::V, 'V'),
            (crate::Element::Y, 'Y'),
            (crate::Element::I, 'I'),
            (crate::Element::W, 'W'),
            (crate::Element::U, 'U'),
        ];

        for (element, expected_char) in single_char_elements {
            let result: Result<char, _> = element.try_into();
            assert_eq!(result.unwrap(), expected_char, "Failed for {element:?}");
        }

        // We check an error case for a single-character symbol that does not exist
        assert!(matches!(
            crate::Element::try_from('X'),
            Err(crate::errors::Error::Element(['X', ' ']))
        ));

        // Test that two-character symbols fail
        let two_char_elements = alloc::vec![
            crate::Element::He,
            crate::Element::Li,
            crate::Element::Be,
            crate::Element::Ne,
            crate::Element::Na,
            crate::Element::Mg,
            crate::Element::Al,
            crate::Element::Si,
            crate::Element::Cl,
            crate::Element::Ar,
            crate::Element::Ca,
            crate::Element::Sc,
            crate::Element::Ti,
            crate::Element::Cr,
            crate::Element::Mn,
            crate::Element::Fe,
            crate::Element::Co,
            crate::Element::Ni,
            crate::Element::Cu,
            crate::Element::Zn,
            crate::Element::Ga,
            crate::Element::Ge,
            crate::Element::As,
            crate::Element::Se,
            crate::Element::Br,
            crate::Element::Kr,
            crate::Element::Rb,
            crate::Element::Sr,
            crate::Element::Zr,
            crate::Element::Nb,
            crate::Element::Mo,
            crate::Element::Tc,
            crate::Element::Ru,
            crate::Element::Rh,
            crate::Element::Pd,
            crate::Element::Ag,
            crate::Element::Cd,
            crate::Element::In,
            crate::Element::Sn,
            crate::Element::Sb,
            crate::Element::Te,
            crate::Element::Xe,
            crate::Element::Cs,
            crate::Element::Ba,
            crate::Element::La,
            crate::Element::Ce,
            crate::Element::Pr,
            crate::Element::Nd,
            crate::Element::Pm,
            crate::Element::Sm,
            crate::Element::Eu,
            crate::Element::Gd,
            crate::Element::Tb,
            crate::Element::Dy,
            crate::Element::Ho,
            crate::Element::Er,
            crate::Element::Tm,
            crate::Element::Yb,
            crate::Element::Lu,
            crate::Element::Hf,
            crate::Element::Ta,
            crate::Element::Re,
            crate::Element::Os,
            crate::Element::Ir,
            crate::Element::Pt,
            crate::Element::Au,
            crate::Element::Hg,
            crate::Element::Tl,
            crate::Element::Pb,
            crate::Element::Bi,
            crate::Element::Po,
            crate::Element::At,
            crate::Element::Rn,
            crate::Element::Fr,
            crate::Element::Ra,
            crate::Element::Ac,
            crate::Element::Th,
            crate::Element::Pa,
            crate::Element::Np,
            crate::Element::Pu,
            crate::Element::Am,
            crate::Element::Cm,
            crate::Element::Bk,
            crate::Element::Cf,
            crate::Element::Es,
            crate::Element::Fm,
            crate::Element::Md,
            crate::Element::No,
            crate::Element::Lr,
            crate::Element::Rf,
            crate::Element::Db,
            crate::Element::Sg,
            crate::Element::Bh,
            crate::Element::Hs,
            crate::Element::Mt,
            crate::Element::Ds,
            crate::Element::Rg,
            crate::Element::Cn,
            crate::Element::Nh,
            crate::Element::Fl,
            crate::Element::Mc,
            crate::Element::Lv,
            crate::Element::Ts,
            crate::Element::Og,
        ];

        for element in two_char_elements {
            let result: Result<char, _> = element.try_into();
            assert!(result.is_err(), "Should fail for two-character element {element:?}");
        }

        // We test a corner case for an invalid character array
        assert!(matches!(
            crate::Element::try_from(['X', 'Y']),
            Err(crate::errors::Error::Element(['X', 'Y']))
        ));

        // We test a corner case for an invalid character array
        assert!(matches!(
            crate::Element::try_from(['X', ' ']),
            Err(crate::errors::Error::Element(['X', ' ']))
        ));
    }

    #[test]
    fn test_try_from_element_to_char_array() {
        for element in crate::Element::iter() {
            let result: Result<[char; 2], _> = element.try_into();
            assert!(result.is_ok(), "Failed to convert {element:?} to [char; 2]");

            let chars = result.unwrap();
            let symbol: &str = element.as_ref();

            // First character should match
            assert_eq!(
                chars[0],
                symbol.chars().next().unwrap(),
                "First char mismatch for {element:?}",
            );

            // Second character should match symbol's second char or be space
            if symbol.len() == 2 {
                assert_eq!(
                    chars[1],
                    symbol.chars().nth(1).unwrap(),
                    "Second char mismatch for {element:?}",
                );
            } else {
                assert_eq!(
                    chars[1], ' ',
                    "Second char should be space for single-char element {element:?}",
                );
            }
        }
    }

    #[test]
    fn test_roundtrip_char_to_element() {
        // Test roundtrip for single-character elements
        let single_char_elements = alloc::vec![
            crate::Element::H,
            crate::Element::B,
            crate::Element::C,
            crate::Element::N,
            crate::Element::O,
            crate::Element::F,
            crate::Element::P,
            crate::Element::S,
            crate::Element::K,
            crate::Element::V,
            crate::Element::Y,
            crate::Element::I,
            crate::Element::W,
            crate::Element::U,
        ];

        for element in single_char_elements {
            let char_result: Result<char, _> = element.try_into();
            assert!(char_result.is_ok(), "Failed to convert {element:?} to char");

            let ch = char_result.unwrap();
            let back_to_element: Result<crate::Element, _> = ch.try_into();
            assert!(back_to_element.is_ok(), "Failed to convert char '{ch}' back to element");

            assert_eq!(back_to_element.unwrap(), element, "Roundtrip failed for {element:?}");
        }
    }

    #[test]
    fn test_roundtrip_char_array_to_element() {
        for element in crate::Element::iter() {
            let chars_result: Result<[char; 2], _> = element.try_into();
            assert!(chars_result.is_ok(), "Failed to convert {element:?} to [char; 2]");

            let chars = chars_result.unwrap();
            let back_to_element: Result<crate::Element, _> = chars.try_into();
            assert!(back_to_element.is_ok(), "Failed to convert {chars:?} back to element");

            assert_eq!(back_to_element.unwrap(), element, "Roundtrip failed for {element:?}");
        }
    }
}
