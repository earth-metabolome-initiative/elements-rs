diesel::table! {
    /// Chemical species stored as text.
    species (id) {
        /// Row identifier.
        id -> Integer,
        /// Canonical element symbol.
        element -> Text,
        /// Isotope symbol.
        isotope -> Text,
    }
}
