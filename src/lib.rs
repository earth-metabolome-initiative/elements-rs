#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "arbitrary", feature = "std")), no_std)]

#[cfg(test)]
extern crate alloc;

mod allowed_valences;
mod as_ref;
mod atomic_number;
mod atomic_radius;
mod bonds_number;
mod charged_valences;
mod classification;
mod covalent_radius;
mod display;
mod electronegativity;
mod element_mask;
pub mod errors;
mod from;
mod from_str;
pub mod isotopes;
mod name;
pub use element_mask::ElementMask;
mod from_stream;
mod orbitals;
pub use orbitals::{AtomicOrbital, AtomicOrbitalType};
mod oxidation_states;
mod principal_quantum_number;
mod standard_atomic_weight;
mod symbol;
mod try_from;
mod valence_electrons;
mod van_der_waals_radius;
pub use allowed_valences::AllowedValences;
pub use atomic_number::AtomicNumber;
pub use atomic_radius::AtomicRadius;
pub use bonds_number::BondsNumber;
pub use charged_valences::ChargedValences;
pub use classification::{ElementCategory, ElementClassification};
pub use covalent_radius::CovalentRadius;
pub use electronegativity::Electronegativity;
pub use isotopes::{
    ElementVariant, Isotope, IsotopicComposition, MassNumber, MostAbundantIsotope,
    RelativeAtomicMass,
};
pub use orbitals::Orbitals;
pub use oxidation_states::OxidationStates;
pub use principal_quantum_number::PrincipalQuantumNumber;
pub use valence_electrons::ValenceElectrons;
pub use van_der_waals_radius::VanDerWaalsRadius;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "mem_size", derive(mem_dbg::MemSize))]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_size", mem_size(flat))]
/// All 118 elements of the periodic table.
///
/// ```rust
/// use elements_rs::Element;
///
/// let oxygen = Element::O;
/// assert_eq!(oxygen.name(), "Oxygen");
/// assert_eq!(oxygen.standard_atomic_weight(), 15.999);
/// ```
pub enum Element {
    /// Hydrogen
    H,
    /// Helium
    He,
    /// Lithium
    Li,
    /// Beryllium
    Be,
    /// Boron
    B,
    /// Carbon
    C,
    /// Nitrogen
    N,
    /// Oxygen
    O,
    /// Fluorine
    F,
    /// Neon
    Ne,
    /// Sodium
    Na,
    /// Magnesium
    Mg,
    /// Aluminium
    Al,
    /// Silicon
    Si,
    /// Phosphorus
    P,
    /// Sulfur
    S,
    /// Chlorine
    Cl,
    /// Argon
    Ar,
    /// Potassium
    K,
    /// Calcium
    Ca,
    /// Scandium
    Sc,
    /// Titanium
    Ti,
    /// Vanadium
    V,
    /// Chromium
    Cr,
    /// Manganese
    Mn,
    /// Iron
    Fe,
    /// Cobalt
    Co,
    /// Nickel
    Ni,
    /// Copper
    Cu,
    /// Zinc
    Zn,
    /// Gallium
    Ga,
    /// Germanium
    Ge,
    /// Arsenic
    As,
    /// Selenium
    Se,
    /// Bromine
    Br,
    /// Krypton
    Kr,
    /// Rubidium
    Rb,
    /// Strontium
    Sr,
    /// Yttrium
    Y,
    /// Zirconium
    Zr,
    /// Niobium
    Nb,
    /// Molybdenum
    Mo,
    /// Technetium
    Tc,
    /// Ruthenium
    Ru,
    /// Rhodium
    Rh,
    /// Palladium
    Pd,
    /// Silver
    Ag,
    /// Cadmium
    Cd,
    /// Indium
    In,
    /// Tin
    Sn,
    /// Antimony
    Sb,
    /// Tellurium
    Te,
    /// Iodine
    I,
    /// Xenon
    Xe,
    /// Caesium
    Cs,
    /// Barium
    Ba,
    /// Lanthanum
    La,
    /// Cerium
    Ce,
    /// Praseodymium
    Pr,
    /// Neodymium
    Nd,
    /// Promethium
    Pm,
    /// Samarium
    Sm,
    /// Europium
    Eu,
    /// Gadolinium
    Gd,
    /// Terbium
    Tb,
    /// Dysprosium
    Dy,
    /// Holmium
    Ho,
    /// Erbium
    Er,
    /// Thulium
    Tm,
    /// Ytterbium
    Yb,
    /// Lutetium
    Lu,
    /// Hafnium
    Hf,
    /// Tantalum
    Ta,
    /// Tungsten
    W,
    /// Rhenium
    Re,
    /// Osmium
    Os,
    /// Iridium
    Ir,
    /// Platinum
    Pt,
    /// Gold
    Au,
    /// Mercury
    Hg,
    /// Thallium
    Tl,
    /// Lead
    Pb,
    /// Bismuth
    Bi,
    /// Polonium
    Po,
    /// Astatine
    At,
    /// Radon
    Rn,
    /// Francium
    Fr,
    /// Radium
    Ra,
    /// Actinium
    Ac,
    /// Thorium
    Th,
    /// Protactinium
    Pa,
    /// Uranium
    U,
    /// Neptunium
    Np,
    /// Plutonium
    Pu,
    /// Americium
    Am,
    /// Curium
    Cm,
    /// Berkelium
    Bk,
    /// Californium
    Cf,
    /// Einsteinium
    Es,
    /// Fermium
    Fm,
    /// Mendelevium
    Md,
    /// Nobelium
    No,
    /// Lawrencium
    Lr,
    /// Rutherfordium
    Rf,
    /// Dubnium
    Db,
    /// Seaborgium
    Sg,
    /// Bohrium
    Bh,
    /// Hassium
    Hs,
    /// Meitnerium
    Mt,
    /// Darmstadtium
    Ds,
    /// Roentgenium
    Rg,
    /// Copernicium
    Cn,
    /// Nihonium
    Nh,
    /// Flerovium
    Fl,
    /// Moscovium
    Mc,
    /// Livermorium
    Lv,
    /// Tennessine
    Ts,
    /// Oganesson
    Og,
}

impl AsRef<Element> for Element {
    fn as_ref(&self) -> &Element {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn test_as_ref_element() {
        let element = Element::C;
        let element_ref: &Element = element.as_ref();
        assert_eq!(element_ref, &Element::C);
    }

    #[cfg(feature = "mem_size")]
    #[test]
    fn test_mem_size_derives() {
        use mem_dbg::{MemSize, SizeFlags};

        let element = Element::C;
        assert_eq!(element.mem_size(SizeFlags::empty()), core::mem::size_of_val(&element));

        let isotope = crate::Isotope::C(crate::isotopes::CarbonIsotope::C12);
        assert_eq!(isotope.mem_size(SizeFlags::empty()), core::mem::size_of_val(&isotope));
    }

    #[cfg(feature = "mem_dbg")]
    #[test]
    fn test_mem_dbg_derives() {
        use mem_dbg::{DbgFlags, MemDbg};

        let mut output = alloc::string::String::new();
        Element::C.mem_dbg_on(&mut output, DbgFlags::default()).unwrap();
        assert!(!output.is_empty());
    }
}
