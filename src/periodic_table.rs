use strum::VariantArray;

use crate::Element;

impl Element {
    /// Returns all elements in atomic number order as a static slice.
    #[must_use]
    pub const fn periodic_table() -> &'static [Self] {
        Self::VARIANTS
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::Element;

    #[test]
    fn test_periodic_table() {
        let periodic_table = Element::periodic_table();

        for (i, element) in Element::iter().enumerate() {
            assert_eq!(periodic_table[i], element);
        }
    }
    #[test]
    fn test_periodic_table_bounds() {
        let periodic_table = Element::periodic_table();

        assert_eq!(periodic_table.len(), 118);
        assert_eq!(periodic_table[0], Element::H);
        assert_eq!(periodic_table[1], Element::He);
        assert_eq!(periodic_table[117], Element::Og);
    }
}
