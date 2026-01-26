//! Build script for the `elements` crate.

use std::{io::Write, path::Path};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

fn normalize_symbol(symbol: &str) -> String {
    // We uppercase the first letter and lowercase the rest
    debug_assert!(symbol.len() <= 2, "Received symbol is too long: `{symbol}`");
    let mut chars = symbol.chars();
    let first_char = chars.next().unwrap_or_default().to_uppercase();
    let rest = chars.as_str().to_lowercase();
    let result = format!("{first_char}{rest}");
    debug_assert!(
        result.len() <= 2,
        "Element symbol is too long: `{result}`, started from `{symbol}`"
    );
    debug_assert_eq!(symbol.to_lowercase(), result.to_lowercase(),);
    result
}

#[allow(clippy::too_many_lines)]
/// Returns the name of the element from its symbol.
fn element_name_from_symbol(symbol: &str) -> &'static str {
    match symbol {
        "D" | "T" | "H" => "Hydrogen",
        "He" => "Helium",
        "Li" => "Lithium",
        "Be" => "Beryllium",
        "B" => "Boron",
        "C" => "Carbon",
        "N" => "Nitrogen",
        "O" => "Oxygen",
        "F" => "Fluorine",
        "Ne" => "Neon",
        "Na" => "Sodium",
        "Mg" => "Magnesium",
        "Al" => "Aluminium",
        "Si" => "Silicon",
        "P" => "Phosphorus",
        "S" => "Sulfur",
        "Cl" => "Chlorine",
        "Ar" => "Argon",
        "K" => "Potassium",
        "Ca" => "Calcium",
        "Sc" => "Scandium",
        "Ti" => "Titanium",
        "V" => "Vanadium",
        "Cr" => "Chromium",
        "Mn" => "Manganese",
        "Fe" => "Iron",
        "Co" => "Cobalt",
        "Ni" => "Nickel",
        "Cu" => "Copper",
        "Zn" => "Zinc",
        "Ga" => "Gallium",
        "Ge" => "Germanium",
        "As" => "Arsenic",
        "Se" => "Selenium",
        "Br" => "Bromine",
        "Kr" => "Krypton",
        "Rb" => "Rubidium",
        "Sr" => "Strontium",
        "Y" => "Yttrium",
        "Zr" => "Zirconium",
        "Nb" => "Niobium",
        "Mo" => "Molybdenum",
        "Tc" => "Technetium",
        "Ru" => "Ruthenium",
        "Rh" => "Rhodium",
        "Pd" => "Palladium",
        "Ag" => "Silver",
        "Cd" => "Cadmium",
        "In" => "Indium",
        "Sn" => "Tin",
        "Sb" => "Antimony",
        "Te" => "Tellurium",
        "I" => "Iodine",
        "Xe" => "Xenon",
        "Cs" => "Caesium",
        "Ba" => "Barium",
        "La" => "Lanthanum",
        "Ce" => "Cerium",
        "Pr" => "Praseodymium",
        "Nd" => "Neodymium",
        "Pm" => "Promethium",
        "Sm" => "Samarium",
        "Eu" => "Europium",
        "Gd" => "Gadolinium",
        "Tb" => "Terbium",
        "Dy" => "Dysprosium",
        "Ho" => "Holmium",
        "Er" => "Erbium",
        "Tm" => "Thulium",
        "Yb" => "Ytterbium",
        "Lu" => "Lutetium",
        "Hf" => "Hafnium",
        "Ta" => "Tantalum",
        "W" => "Tungsten",
        "Re" => "Rhenium",
        "Os" => "Osmium",
        "Ir" => "Iridium",
        "Pt" => "Platinum",
        "Au" => "Gold",
        "Hg" => "Mercury",
        "Tl" => "Thallium",
        "Pb" => "Lead",
        "Bi" => "Bismuth",
        "Po" => "Polonium",
        "At" => "Astatine",
        "Rn" => "Radon",
        "Fr" => "Francium",
        "Ra" => "Radium",
        "Ac" => "Actinium",
        "Th" => "Thorium",
        "Pa" => "Protactinium",
        "U" => "Uranium",
        "Np" => "Neptunium",
        "Pu" => "Plutonium",
        "Am" => "Americium",
        "Cm" => "Curium",
        "Bk" => "Berkelium",
        "Cf" => "Californium",
        "Es" => "Einsteinium",
        "Fm" => "Fermium",
        "Md" => "Mendelevium",
        "No" => "Nobelium",
        "Lr" => "Lawrencium",
        "Rf" => "Rutherfordium",
        "Db" => "Dubnium",
        "Sg" => "Seaborgium",
        "Bh" => "Bohrium",
        "Hs" => "Hassium",
        "Mt" => "Meitnerium",
        "Ds" => "Darmstadtium",
        "Rg" => "Roentgenium",
        "Cn" => "Copernicium",
        "Nh" => "Nihonium",
        "Fl" => "Flerovium",
        "Mc" => "Moscovium",
        "Lv" => "Livermorium",
        "Ts" => "Tennessine",
        "Og" => "Oganesson",
        _ => panic!("Unknown element symbol: {symbol}"),
    }
}

#[derive(serde::Deserialize, Debug)]
struct IsotopeMetadata {
    atomic_number: u8,
    atomic_symbol: String,
    mass_number: u16,
    relative_atomic_mass: f64,
    isotopic_composition: Option<f64>,
}

fn isotopes() -> Vec<IsotopeMetadata> {
    // We load using serde_json to avoid the need for a custom deserializer.
    let reader =
        std::fs::File::open("isotopes_data.json").expect("Failed to open isotopes_data.json");
    let mut isotopes: Vec<IsotopeMetadata> =
        serde_json::from_reader(reader).expect("Failed to parse isotopes_data.json");

    for isotope in &mut isotopes {
        // Normalize the atomic symbol
        isotope.atomic_symbol = normalize_symbol(&isotope.atomic_symbol);
    }

    isotopes
}

#[allow(clippy::too_many_lines)]
fn implement_isotope_enum(isotopes: &[IsotopeMetadata]) -> TokenStream {
    let isotope_names = isotopes
        .iter()
        .map(|isotope| {
            if isotope.atomic_symbol == "D" || isotope.atomic_symbol == "T" {
                isotope.atomic_symbol.clone()
            } else {
                format!("{}{}", isotope.atomic_symbol, isotope.mass_number)
            }
        })
        .collect::<Vec<_>>();

    // We generate the enum variants for each isotope
    let enum_variants = isotope_names
        .iter()
        .map(|isotope_name| {
            let isotope_ident = Ident::new(isotope_name, proc_macro2::Span::call_site());
            quote! {
                #isotope_ident
            }
        })
        .collect::<Vec<_>>();

    let enum_variants_with_documentation = isotopes
        .iter()
        .zip(isotope_names.iter())
        .map(|(isotope, isotope_name)| {
            let isotope_ident = Ident::new(isotope_name, proc_macro2::Span::call_site());
            let documentation = format!(
                "Isotope {} of {}",
                isotope_name,
                element_name_from_symbol(&isotope.atomic_symbol)
            );
            quote! {
                #[doc = #documentation]
                #isotope_ident
            }
        })
        .collect::<Vec<_>>();

    let relative_atomic_masses =
        isotopes.iter().map(|isotope| isotope.relative_atomic_mass).collect::<Vec<_>>();

    let mass_numbers = isotopes.iter().map(|isotope| isotope.mass_number).collect::<Vec<_>>();
    let mass_numbers_u64 =
        isotopes.iter().map(|isotope| u64::from(isotope.mass_number)).collect::<Vec<_>>();

    let known_isotopic_compositions: Vec<TokenStream> = isotopes
        .iter()
        .zip(isotope_names.iter())
        .filter_map(|(isotope, isotope_name)| {
            isotope.isotopic_composition.map(|isotopic_composition| {
                let isotope_ident = Ident::new(isotope_name, proc_macro2::Span::call_site());
                quote! {
                    Self::#isotope_ident => Some(#isotopic_composition)
                }
            })
        })
        .collect::<Vec<_>>();

    let most_abundant_isotope = isotopes
        .iter()
        .max_by(|a, b| {
            a.isotopic_composition
                .unwrap_or(0.0)
                .partial_cmp(&b.isotopic_composition.unwrap_or(0.0))
                .unwrap()
        })
        .unwrap();

    let most_abundant_isotope_ident = Ident::new(
        &format!("{}{}", most_abundant_isotope.atomic_symbol, most_abundant_isotope.mass_number),
        proc_macro2::Span::call_site(),
    );

    let element_symbol_ident: Ident =
        Ident::new(&isotopes[0].atomic_symbol, proc_macro2::Span::call_site());

    let isotope_ident = Ident::new(
        &format!("{}Isotope", element_name_from_symbol(&isotopes[0].atomic_symbol)),
        proc_macro2::Span::call_site(),
    );

    let isotope_documentation =
        format!("Isotopes of the element {}", element_name_from_symbol(&isotopes[0].atomic_symbol));

    let submodule_documentation =
        format!("Isotopes of the element {}", element_name_from_symbol(&isotopes[0].atomic_symbol));

    let isotopic_composition_impl = if known_isotopic_compositions.is_empty() {
        quote! {
            None
        }
    } else {
        quote! {
            match self {
                #(#known_isotopic_compositions),*,
                _ => None,
            }
        }
    };

    quote! {
        #![doc = #submodule_documentation]

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        #[doc = #isotope_documentation]
        pub enum #isotope_ident {
            #(#enum_variants_with_documentation),*
        }

        impl super::RelativeAtomicMass for #isotope_ident {
            #[inline]
            fn relative_atomic_mass(&self) -> f64 {
                match self {
                    #(Self::#enum_variants => #relative_atomic_masses),*
                }
            }
        }

        impl super::ElementVariant for #isotope_ident {
            #[inline]
            fn element(&self) -> crate::Element {
                crate::Element::#element_symbol_ident
            }
        }

        impl super::MassNumber for #isotope_ident {
            #[inline]
            fn mass_number(&self) -> u16 {
                match self {
                    #(Self::#enum_variants => #mass_numbers),*
                }
            }
        }

        impl super::IsotopicComposition for #isotope_ident {
            #[inline]
            fn isotopic_composition(&self) -> Option<f64> {
                #isotopic_composition_impl
            }
        }

        impl super::MostAbundantIsotope for #isotope_ident {
            fn most_abundant_isotope() -> Self {
                Self::#most_abundant_isotope_ident
            }
        }

        impl From<#isotope_ident> for crate::Isotope {
            fn from(isotope: #isotope_ident) -> Self {
                crate::Isotope::#element_symbol_ident(isotope)
            }
        }

        impl From<#isotope_ident> for crate::Element {
            fn from(_isotope: #isotope_ident) -> Self {
                crate::Element::#element_symbol_ident
            }
        }

        impl TryFrom<u64> for #isotope_ident {
            type Error = crate::errors::Error;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                match value {
                    #(
                        #mass_numbers_u64 => Ok(Self::#enum_variants),
                    )*
                    _ => Err(crate::errors::Error::Isotope(
                        crate::Element::#element_symbol_ident,
                        value,
                    )),
                }
            }
        }

        impl TryFrom<u8> for #isotope_ident {
            type Error = crate::errors::Error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::try_from(u64::from(value))
            }
        }

        impl TryFrom<u16> for #isotope_ident {
            type Error = crate::errors::Error;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                Self::try_from(u64::from(value))
            }
        }

        impl TryFrom<u32> for #isotope_ident {
            type Error = crate::errors::Error;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                Self::try_from(u64::from(value))
            }
        }

        impl core::fmt::Display for #isotope_ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(Self::#enum_variants => write!(f, #isotope_names),)*
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use strum::IntoEnumIterator;
            use crate::isotopes::{RelativeAtomicMass, ElementVariant, MassNumber, IsotopicComposition, MostAbundantIsotope};

            #[test]
            fn test_relative_atomic_mass() {
                for isotope in #isotope_ident::iter() {
                    let mass = isotope.relative_atomic_mass();
                    assert!(mass > 0.0, "Mass should be positive for {isotope:?}");
                }
            }

            #[test]
            fn test_element() {
                for isotope in #isotope_ident::iter() {
                    let element = isotope.element();
                    assert_eq!(element, crate::Element::#element_symbol_ident, "Element should be correct for {isotope:?}");
                }
            }

            #[test]
            fn test_mass_number() {
                for isotope in #isotope_ident::iter() {
                    let mass_number = isotope.mass_number();
                    assert!(mass_number > 0 && mass_number < 300, "Mass number should be reasonable for {isotope:?}");
                }
            }

            #[test]
            fn test_isotopic_composition() {
                for isotope in #isotope_ident::iter() {
                    let comp = isotope.isotopic_composition();
                    if let Some(c) = comp {
                        assert!(
                            (0.0..=1.0).contains(&c),
                            "Composition should be between 0 and 1 for {isotope:?}"
                        );
                    }
                }
            }

            #[test]
            fn test_most_abundant() {
                let most_abundant = #isotope_ident::most_abundant_isotope();
                // Just check it's a valid variant
                let _ = most_abundant.relative_atomic_mass();
            }

            #[test]
            fn test_from_isotope() {
                for isotope in #isotope_ident::iter() {
                    let iso: crate::Isotope = isotope.into();
                    // Check it matches
                    match iso {
                        crate::Isotope::#element_symbol_ident(i) => assert_eq!(i, isotope),
                        _ => panic!("Wrong isotope type"),
                    }
                }
            }

            #[test]
            fn test_from_element() {
                for isotope in #isotope_ident::iter() {
                    let elem: crate::Element = isotope.into();
                    assert_eq!(elem, crate::Element::#element_symbol_ident);
                }
            }

            #[test]
            fn test_try_from_mass_number() {
                for isotope in #isotope_ident::iter() {
                    let mass = isotope.mass_number();
                    let iso = #isotope_ident::try_from(mass).unwrap();
                    assert_eq!(iso, isotope);

                    let iso_u32 = #isotope_ident::try_from(u32::from(mass)).unwrap();
                    assert_eq!(iso_u32, isotope);

                    if let Ok(mass_u8) = u8::try_from(mass) {
                        let iso_u8 = #isotope_ident::try_from(mass_u8).unwrap();
                        assert_eq!(iso_u8, isotope);
                    }
                }
                // Test error cases
                assert!(#isotope_ident::try_from(0_u16).is_err()); // Too low
                assert!(#isotope_ident::try_from(1000_u16).is_err()); // Way too high

                assert!(#isotope_ident::try_from(0_u32).is_err()); // Too low
                assert!(#isotope_ident::try_from(1000_u32).is_err()); // Way too high

                assert!(#isotope_ident::try_from(0_u8).is_err()); // Too low
            }

            #[test]
            fn test_display() {
                for isotope in #isotope_ident::iter() {
                    let s = alloc::format!("{isotope}");
                    assert!(!s.is_empty(), "Display should not be empty for {isotope:?}");
                }
            }
        }
    }
}

pub fn main() {
    let isotopes = isotopes();
    // We group the isotopes by atomic number
    let isotopes_by_atomic_number: std::collections::HashMap<u8, Vec<IsotopeMetadata>> =
        isotopes.into_iter().fold(std::collections::HashMap::new(), |mut acc, isotope| {
            acc.entry(isotope.atomic_number).or_default().push(isotope);
            acc
        });

    let isotopes_module_dir = Path::new("../src/isotopes/");

    // Generate arbitrary.rs
    for (_atomic_number, isotopes) in isotopes_by_atomic_number {
        let element_name = element_name_from_symbol(&isotopes[0].atomic_symbol).to_lowercase();
        let tokens = implement_isotope_enum(&isotopes);
        let file_name = format!("{element_name}.rs");
        let file_path = isotopes_module_dir.join(&file_name);
        let mut file = std::fs::File::create(&file_path).expect("Failed to create file");
        writeln!(file, "{tokens}").expect("Failed to write to file");

        // We run rustfmt on the generated file
        let status = std::process::Command::new("rustfmt")
            .arg(file_path.to_str().unwrap())
            .status()
            .expect("Failed to run rustfmt");
        assert!(status.success(), "rustfmt failed with status: {status}");
    }
}
