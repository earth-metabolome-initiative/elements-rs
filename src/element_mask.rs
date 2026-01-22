//! Submodule providing a bitmask based on a `u128` to represent sets of
//! elements observed in a molecule.

use crate::Element;

/// Struct representing a set of elements as a bitmask.
///
/// Each bit in the `u128` corresponds to an element in the periodic table,
/// with the least significant bit representing Hydrogen (H, atomic number 1)
/// and the most significant bit representing Oganesson (Og, atomic number 118).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementMask(u128);

impl From<Element> for ElementMask {
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ElementMask};
    ///
    /// let mask: ElementMask = Element::H.into();
    /// assert!(mask.contains(Element::H));
    /// ```
    fn from(element: Element) -> Self {
        let mut mask = ElementMask::default();
        mask.insert(element);
        mask
    }
}

impl ElementMask {
    /// Returns whether the bitmask contains the specified element.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to check for in the bitmask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ElementMask};
    ///
    /// let mut mask = ElementMask::default();
    /// mask.insert(Element::H);
    /// assert!(mask.contains(Element::H));
    /// assert!(!mask.contains(Element::He));
    /// ```
    #[must_use]
    pub fn contains(&self, element: Element) -> bool {
        let atomic_number: u8 = element.into();
        let index = u32::from(atomic_number - 1);
        (self.0 & (1 << index)) != 0
    }

    /// Inserts an element into the bitmask and returns
    /// whether the element was not already present.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to insert into the bitmask.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elements_rs::{Element, ElementMask};
    ///
    /// let mut mask = ElementMask::default();
    /// assert!(mask.insert(Element::H)); // Was not present
    /// assert!(!mask.insert(Element::H)); // Already present
    /// ```
    pub fn insert(&mut self, element: Element) -> bool {
        let atomic_number: u8 = element.into();
        let index = u32::from(atomic_number - 1_u8);
        let was_present = self.contains(element);
        self.0 |= 1 << index;
        !was_present
    }
}

impl FromIterator<Element> for ElementMask {
    /// # Examples
    ///
    /// ```rust
    /// # extern crate alloc;
    /// use elements_rs::{Element, ElementMask};
    ///
    /// let elements = alloc::vec![Element::H, Element::He];
    /// let mask: ElementMask = elements.into_iter().collect();
    /// assert!(mask.contains(Element::H));
    /// assert!(mask.contains(Element::He));
    /// ```
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        let mut mask = ElementMask::default();
        for element in iter {
            mask.insert(element);
        }
        mask
    }
}

impl core::iter::IntoIterator for ElementMask {
    type Item = Element;
    type IntoIter = ElementMaskIterator;

    /// # Examples
    ///
    /// ```rust
    /// # extern crate alloc;
    /// use elements_rs::{Element, ElementMask};
    ///
    /// let mut mask = ElementMask::default();
    /// mask.insert(Element::H);
    /// mask.insert(Element::He);
    /// let elements: Vec<Element> = mask.into_iter().collect();
    /// assert_eq!(elements, alloc::vec![Element::H, Element::He]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        ElementMaskIterator { mask: self.0, index: 0 }
    }
}

/// Iterator over the elements in an `ElementMask`.
pub struct ElementMaskIterator {
    mask: u128,
    index: u8,
}

impl Iterator for ElementMaskIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 128 {
            if (self.mask & (1 << self.index)) != 0 {
                let element = Element::try_from(self.index + 1).unwrap();
                self.index += 1;
                return Some(element);
            }
            self.index += 1;
        }
        None
    }
}
