//! Implements methods to create an `Element`
//! from a stream of characters.

use core::iter::Peekable;

use crate::Element;

impl Element {
    /// Creates an `Element` from a peekable stream of characters.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a peekable iterator over characters.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Element))` if an element symbol was successfully parsed.
    /// * `Ok(None)` if the stream is exhausted (no character at all to parse).
    ///
    /// # Errors
    ///
    /// * If the parsing fails, it returns the consumed character that caused
    ///   the failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use elements_rs::Element;
    ///
    /// let mut stream = "He".chars().peekable();
    /// assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::He)));
    ///
    /// let mut stream = "H".chars().peekable();
    /// assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::H)));
    ///
    /// let mut stream = "Hz".chars().peekable();
    /// assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::H)));
    /// assert_eq!(stream.next(), Some('z'));
    ///
    /// let mut stream = "!".chars().peekable();
    /// assert_eq!(Element::try_from_stream(&mut stream), Err('!'));
    ///
    /// let mut stream = "".chars().peekable();
    /// assert_eq!(Element::try_from_stream(&mut stream), Ok(None));
    /// ```
    pub fn try_from_stream<I>(stream: &mut Peekable<I>) -> Result<Option<Self>, char>
    where
        I: Iterator<Item = char>,
    {
        let Some(character) = stream.next() else {
            return Ok(None);
        };

        if let Some(peak) = stream.peek().copied()
            && let Ok(element) = Element::try_from([character, peak])
        {
            stream.next();
            Ok(Some(element))
        } else if let Ok(element) = Element::try_from(character) {
            Ok(Some(element))
        } else {
            Err(character)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_stream() {
        // Empty stream
        let mut stream = "".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Ok(None));

        // Valid 1-char element
        let mut stream = "H".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::H)));
        assert_eq!(stream.next(), None);

        // Valid 2-char element
        let mut stream = "He".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::He)));
        assert_eq!(stream.next(), None);

        // Valid 1-char element followed by char making invalid 2-char
        let mut stream = "Hz".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::H)));
        assert_eq!(stream.next(), Some('z'));

        // Valid 2-char element (greedy match over 1-char)
        // 'C' is Carbon, 'Co' is Cobalt. Should match Cobalt.
        let mut stream = "Co".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Ok(Some(Element::Co)));
        assert_eq!(stream.next(), None);

        // Invalid character
        let mut stream = "!".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Err('!'));
        assert_eq!(stream.next(), None);

        // Invalid 2-char, but first char is invalid too
        let mut stream = "!H".chars().peekable();
        assert_eq!(Element::try_from_stream(&mut stream), Err('!'));
        assert_eq!(stream.next(), Some('H'));
    }
}
