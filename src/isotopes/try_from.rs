//! Implementations of the `TryFrom` trait for the `Isotope` enumeration.

use super::HydrogenIsotope;

impl TryFrom<char> for crate::Isotope {
    type Error = crate::errors::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'D' => HydrogenIsotope::D.into(),
            'T' => HydrogenIsotope::T.into(),
            _ => {
                return Err(crate::errors::Error::CharacterIsotope(value));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_try_from_char() {
        for char in ['D', 'T'] {
            let result = crate::Isotope::try_from(char);
            assert!(result.is_ok(), "Failed to convert char '{}' to Isotope", char);
        }

        // Test an invalid character
        let invalid_char = 'X';
        let result = crate::Isotope::try_from(invalid_char);
        assert!(matches!(result, Err(crate::errors::Error::CharacterIsotope('X'))));
    }
}
