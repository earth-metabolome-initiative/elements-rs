//!  Submodule provding the implementation of the `Display` trait for the
//! `Element` enum.

impl core::fmt::Display for crate::Element {
    /// Formats the element as its symbol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate alloc;
    /// use elements_rs::Element;
    ///
    /// let hydrogen = alloc::format!("{}", Element::H);
    /// assert_eq!(hydrogen, "H");
    ///
    /// let magnesium = alloc::format!("{}", Element::Mg);
    /// assert_eq!(magnesium, "Mg");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    #[test]
    fn test_display() {
        for element in crate::Element::iter() {
            let display = alloc::format!("{element}");
            assert!(!display.is_empty(), "Display should not be empty for {element:?}");
            assert_eq!(display, element.as_ref(), "Display should equal as_ref for {element:?}");
        }
    }
}
