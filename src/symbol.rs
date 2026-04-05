//! Canonical element symbols and aromatic SMILES spellings.

impl crate::Element {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Returns the canonical element symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// assert_eq!(Element::C.symbol(), "C");
    /// assert_eq!(Element::Cl.symbol(), "Cl");
    /// ```
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::H => "H",
            Self::He => "He",
            Self::Li => "Li",
            Self::Be => "Be",
            Self::B => "B",
            Self::C => "C",
            Self::N => "N",
            Self::O => "O",
            Self::F => "F",
            Self::Ne => "Ne",
            Self::Na => "Na",
            Self::Mg => "Mg",
            Self::Al => "Al",
            Self::Si => "Si",
            Self::P => "P",
            Self::S => "S",
            Self::Cl => "Cl",
            Self::Ar => "Ar",
            Self::K => "K",
            Self::Ca => "Ca",
            Self::Sc => "Sc",
            Self::Ti => "Ti",
            Self::V => "V",
            Self::Cr => "Cr",
            Self::Mn => "Mn",
            Self::Fe => "Fe",
            Self::Co => "Co",
            Self::Ni => "Ni",
            Self::Cu => "Cu",
            Self::Zn => "Zn",
            Self::Ga => "Ga",
            Self::Ge => "Ge",
            Self::As => "As",
            Self::Se => "Se",
            Self::Br => "Br",
            Self::Kr => "Kr",
            Self::Rb => "Rb",
            Self::Sr => "Sr",
            Self::Y => "Y",
            Self::Zr => "Zr",
            Self::Nb => "Nb",
            Self::Mo => "Mo",
            Self::Tc => "Tc",
            Self::Ru => "Ru",
            Self::Rh => "Rh",
            Self::Pd => "Pd",
            Self::Ag => "Ag",
            Self::Cd => "Cd",
            Self::In => "In",
            Self::Sn => "Sn",
            Self::Sb => "Sb",
            Self::Te => "Te",
            Self::I => "I",
            Self::Xe => "Xe",
            Self::Cs => "Cs",
            Self::Ba => "Ba",
            Self::La => "La",
            Self::Ce => "Ce",
            Self::Pr => "Pr",
            Self::Nd => "Nd",
            Self::Pm => "Pm",
            Self::Sm => "Sm",
            Self::Eu => "Eu",
            Self::Gd => "Gd",
            Self::Tb => "Tb",
            Self::Dy => "Dy",
            Self::Ho => "Ho",
            Self::Er => "Er",
            Self::Tm => "Tm",
            Self::Yb => "Yb",
            Self::Lu => "Lu",
            Self::Hf => "Hf",
            Self::Ta => "Ta",
            Self::W => "W",
            Self::Re => "Re",
            Self::Os => "Os",
            Self::Ir => "Ir",
            Self::Pt => "Pt",
            Self::Au => "Au",
            Self::Hg => "Hg",
            Self::Tl => "Tl",
            Self::Pb => "Pb",
            Self::Bi => "Bi",
            Self::Po => "Po",
            Self::At => "At",
            Self::Rn => "Rn",
            Self::Fr => "Fr",
            Self::Ra => "Ra",
            Self::Ac => "Ac",
            Self::Th => "Th",
            Self::Pa => "Pa",
            Self::U => "U",
            Self::Np => "Np",
            Self::Pu => "Pu",
            Self::Am => "Am",
            Self::Cm => "Cm",
            Self::Bk => "Bk",
            Self::Cf => "Cf",
            Self::Es => "Es",
            Self::Fm => "Fm",
            Self::Md => "Md",
            Self::No => "No",
            Self::Lr => "Lr",
            Self::Rf => "Rf",
            Self::Db => "Db",
            Self::Sg => "Sg",
            Self::Bh => "Bh",
            Self::Hs => "Hs",
            Self::Mt => "Mt",
            Self::Ds => "Ds",
            Self::Rg => "Rg",
            Self::Cn => "Cn",
            Self::Nh => "Nh",
            Self::Fl => "Fl",
            Self::Mc => "Mc",
            Self::Lv => "Lv",
            Self::Ts => "Ts",
            Self::Og => "Og",
        }
    }

    #[must_use]
    #[inline]
    /// Returns the length of the canonical element symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// assert_eq!(Element::C.symbol_len(), 1);
    /// assert_eq!(Element::Cl.symbol_len(), 2);
    /// ```
    pub const fn symbol_len(self) -> usize {
        self.symbol().len()
    }

    #[must_use]
    /// Returns the lowercase aromatic SMILES spelling for elements in the
    /// aromatic organic subset.
    ///
    /// Returns `None` for elements that do not have an aromatic lowercase
    /// spelling in SMILES.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// assert_eq!(Element::C.aromatic_smiles_symbol(), Some("c"));
    /// assert_eq!(Element::Se.aromatic_smiles_symbol(), Some("se"));
    /// assert_eq!(Element::Cl.aromatic_smiles_symbol(), None);
    /// ```
    pub const fn aromatic_smiles_symbol(self) -> Option<&'static str> {
        match self {
            Self::B => Some("b"),
            Self::C => Some("c"),
            Self::N => Some("n"),
            Self::O => Some("o"),
            Self::P => Some("p"),
            Self::S => Some("s"),
            Self::Se => Some("se"),
            Self::As => Some("as"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_symbol() {
        for element in crate::Element::iter() {
            let symbol: &str = element.as_ref();
            assert_eq!(element.symbol(), symbol);
        }
    }

    #[test]
    fn test_symbol_len() {
        for element in crate::Element::iter() {
            assert_eq!(element.symbol_len(), element.symbol().len());
        }
    }

    #[test]
    fn test_aromatic_smiles_symbol() {
        let aromatic_symbols = [
            (crate::Element::B, "b"),
            (crate::Element::C, "c"),
            (crate::Element::N, "n"),
            (crate::Element::O, "o"),
            (crate::Element::P, "p"),
            (crate::Element::S, "s"),
            (crate::Element::Se, "se"),
            (crate::Element::As, "as"),
        ];

        for (element, symbol) in aromatic_symbols {
            assert_eq!(element.aromatic_smiles_symbol(), Some(symbol));
        }

        for element in crate::Element::iter() {
            if !matches!(
                element,
                crate::Element::B
                    | crate::Element::C
                    | crate::Element::N
                    | crate::Element::O
                    | crate::Element::P
                    | crate::Element::S
                    | crate::Element::Se
                    | crate::Element::As
            ) {
                assert_eq!(element.aromatic_smiles_symbol(), None);
            }
        }
    }
}
