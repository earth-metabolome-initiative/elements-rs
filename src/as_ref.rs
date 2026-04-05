//! Submodule providing implementations of the `AsRef` trait
//! for the `Element` enumeration.

impl AsRef<str> for crate::Element {
    /// Returns the element symbol as a string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::Element;
    ///
    /// let symbol: &str = Element::H.as_ref();
    /// assert_eq!(symbol, "H");
    ///
    /// let oxygen_symbol: &str = Element::O.as_ref();
    /// assert_eq!(oxygen_symbol, "O");
    /// ```
    #[allow(clippy::too_many_lines)]
    fn as_ref(&self) -> &str {
        (*self).symbol()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_as_ref() {
        for element in crate::Element::iter() {
            let symbol: &str = element.as_ref();
            assert!(!symbol.is_empty(), "Symbol should not be empty for {element:?}");
            assert!(symbol.len() <= 2, "Symbol should be 1 or 2 characters for {element:?}");
        }
    }
}
