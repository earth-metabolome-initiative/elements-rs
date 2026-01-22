//! Submodule implementing the `MassNumber` trait for the `Isotope`
//! enum.

impl super::MassNumber for super::Isotope {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn mass_number(&self) -> u16 {
        match self {
            Self::H(isotope) => isotope.mass_number(),
            Self::He(isotope) => isotope.mass_number(),
            Self::Li(isotope) => isotope.mass_number(),
            Self::Be(isotope) => isotope.mass_number(),
            Self::B(isotope) => isotope.mass_number(),
            Self::C(isotope) => isotope.mass_number(),
            Self::N(isotope) => isotope.mass_number(),
            Self::O(isotope) => isotope.mass_number(),
            Self::F(isotope) => isotope.mass_number(),
            Self::Ne(isotope) => isotope.mass_number(),
            Self::Na(isotope) => isotope.mass_number(),
            Self::Mg(isotope) => isotope.mass_number(),
            Self::Al(isotope) => isotope.mass_number(),
            Self::Si(isotope) => isotope.mass_number(),
            Self::P(isotope) => isotope.mass_number(),
            Self::S(isotope) => isotope.mass_number(),
            Self::Cl(isotope) => isotope.mass_number(),
            Self::Ar(isotope) => isotope.mass_number(),
            Self::K(isotope) => isotope.mass_number(),
            Self::Ca(isotope) => isotope.mass_number(),
            Self::Sc(isotope) => isotope.mass_number(),
            Self::Ti(isotope) => isotope.mass_number(),
            Self::V(isotope) => isotope.mass_number(),
            Self::Cr(isotope) => isotope.mass_number(),
            Self::Mn(isotope) => isotope.mass_number(),
            Self::Fe(isotope) => isotope.mass_number(),
            Self::Co(isotope) => isotope.mass_number(),
            Self::Ni(isotope) => isotope.mass_number(),
            Self::Cu(isotope) => isotope.mass_number(),
            Self::Zn(isotope) => isotope.mass_number(),
            Self::Ga(isotope) => isotope.mass_number(),
            Self::Ge(isotope) => isotope.mass_number(),
            Self::As(isotope) => isotope.mass_number(),
            Self::Se(isotope) => isotope.mass_number(),
            Self::Br(isotope) => isotope.mass_number(),
            Self::Kr(isotope) => isotope.mass_number(),
            Self::Rb(isotope) => isotope.mass_number(),
            Self::Sr(isotope) => isotope.mass_number(),
            Self::Y(isotope) => isotope.mass_number(),
            Self::Zr(isotope) => isotope.mass_number(),
            Self::Nb(isotope) => isotope.mass_number(),
            Self::Mo(isotope) => isotope.mass_number(),
            Self::Tc(isotope) => isotope.mass_number(),
            Self::Ru(isotope) => isotope.mass_number(),
            Self::Rh(isotope) => isotope.mass_number(),
            Self::Pd(isotope) => isotope.mass_number(),
            Self::Ag(isotope) => isotope.mass_number(),
            Self::Cd(isotope) => isotope.mass_number(),
            Self::In(isotope) => isotope.mass_number(),
            Self::Sn(isotope) => isotope.mass_number(),
            Self::Sb(isotope) => isotope.mass_number(),
            Self::Te(isotope) => isotope.mass_number(),
            Self::I(isotope) => isotope.mass_number(),
            Self::Xe(isotope) => isotope.mass_number(),
            Self::Cs(isotope) => isotope.mass_number(),
            Self::Ba(isotope) => isotope.mass_number(),
            Self::La(isotope) => isotope.mass_number(),
            Self::Ce(isotope) => isotope.mass_number(),
            Self::Pr(isotope) => isotope.mass_number(),
            Self::Nd(isotope) => isotope.mass_number(),
            Self::Pm(isotope) => isotope.mass_number(),
            Self::Sm(isotope) => isotope.mass_number(),
            Self::Eu(isotope) => isotope.mass_number(),
            Self::Gd(isotope) => isotope.mass_number(),
            Self::Tb(isotope) => isotope.mass_number(),
            Self::Dy(isotope) => isotope.mass_number(),
            Self::Ho(isotope) => isotope.mass_number(),
            Self::Er(isotope) => isotope.mass_number(),
            Self::Tm(isotope) => isotope.mass_number(),
            Self::Yb(isotope) => isotope.mass_number(),
            Self::Lu(isotope) => isotope.mass_number(),
            Self::Hf(isotope) => isotope.mass_number(),
            Self::Ta(isotope) => isotope.mass_number(),
            Self::W(isotope) => isotope.mass_number(),
            Self::Re(isotope) => isotope.mass_number(),
            Self::Os(isotope) => isotope.mass_number(),
            Self::Ir(isotope) => isotope.mass_number(),
            Self::Pt(isotope) => isotope.mass_number(),
            Self::Au(isotope) => isotope.mass_number(),
            Self::Hg(isotope) => isotope.mass_number(),
            Self::Tl(isotope) => isotope.mass_number(),
            Self::Pb(isotope) => isotope.mass_number(),
            Self::Bi(isotope) => isotope.mass_number(),
            Self::Po(isotope) => isotope.mass_number(),
            Self::At(isotope) => isotope.mass_number(),
            Self::Rn(isotope) => isotope.mass_number(),
            Self::Fr(isotope) => isotope.mass_number(),
            Self::Ra(isotope) => isotope.mass_number(),
            Self::Ac(isotope) => isotope.mass_number(),
            Self::Th(isotope) => isotope.mass_number(),
            Self::Pa(isotope) => isotope.mass_number(),
            Self::U(isotope) => isotope.mass_number(),
            Self::Np(isotope) => isotope.mass_number(),
            Self::Pu(isotope) => isotope.mass_number(),
            Self::Am(isotope) => isotope.mass_number(),
            Self::Cm(isotope) => isotope.mass_number(),
            Self::Bk(isotope) => isotope.mass_number(),
            Self::Cf(isotope) => isotope.mass_number(),
            Self::Es(isotope) => isotope.mass_number(),
            Self::Fm(isotope) => isotope.mass_number(),
            Self::Md(isotope) => isotope.mass_number(),
            Self::No(isotope) => isotope.mass_number(),
            Self::Lr(isotope) => isotope.mass_number(),
            Self::Rf(isotope) => isotope.mass_number(),
            Self::Db(isotope) => isotope.mass_number(),
            Self::Sg(isotope) => isotope.mass_number(),
            Self::Bh(isotope) => isotope.mass_number(),
            Self::Hs(isotope) => isotope.mass_number(),
            Self::Mt(isotope) => isotope.mass_number(),
            Self::Ds(isotope) => isotope.mass_number(),
            Self::Rg(isotope) => isotope.mass_number(),
            Self::Cn(isotope) => isotope.mass_number(),
            Self::Nh(isotope) => isotope.mass_number(),
            Self::Fl(isotope) => isotope.mass_number(),
            Self::Mc(isotope) => isotope.mass_number(),
            Self::Lv(isotope) => isotope.mass_number(),
            Self::Ts(isotope) => isotope.mass_number(),
            Self::Og(isotope) => isotope.mass_number(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::isotopes::{Isotope, MassNumber};

    #[test]
    fn test_mass_number_delegation() {
        // Test that the Isotope enum correctly delegates to individual isotope
        // implementations Test representative isotopes from different elements

        // Hydrogen isotopes
        let h1 = Isotope::H(crate::isotopes::HydrogenIsotope::H1);
        assert_eq!(h1.mass_number(), 1);

        let d2 = Isotope::H(crate::isotopes::HydrogenIsotope::D);
        assert_eq!(d2.mass_number(), 2);

        let t3 = Isotope::H(crate::isotopes::HydrogenIsotope::T);
        assert_eq!(t3.mass_number(), 3);

        // Carbon isotopes
        let c12 = Isotope::C(crate::isotopes::CarbonIsotope::C12);
        assert_eq!(c12.mass_number(), 12);

        let c13 = Isotope::C(crate::isotopes::CarbonIsotope::C13);
        assert_eq!(c13.mass_number(), 13);

        let c14 = Isotope::C(crate::isotopes::CarbonIsotope::C14);
        assert_eq!(c14.mass_number(), 14);

        // Oxygen isotopes
        let o16 = Isotope::O(crate::isotopes::OxygenIsotope::O16);
        assert_eq!(o16.mass_number(), 16);

        // Argon isotopes
        let ar36 = Isotope::Ar(crate::isotopes::ArgonIsotope::Ar36);
        assert_eq!(ar36.mass_number(), 36);

        let ar40 = Isotope::Ar(crate::isotopes::ArgonIsotope::Ar40);
        assert_eq!(ar40.mass_number(), 40);
    }

    #[test]
    fn test_mass_number_strum() {
        use strum::IntoEnumIterator;

        use crate::isotopes::{ElementVariant, MassNumber};

        for element in crate::Element::iter() {
            let isotopes = element.isotopes();
            for isotope in isotopes {
                let mass_number = isotope.mass_number();
                // Mass number should be positive and reasonable
                assert!(mass_number > 0, "Mass number for isotope {isotope:?} should be positive",);
                assert!(
                    mass_number < 300,
                    "Mass number for isotope {isotope:?} should be reasonable (< 300)",
                );
                // Verify that the isotope belongs to the correct element
                assert_eq!(
                    isotope.element(),
                    element,
                    "Isotope {isotope:?} should belong to element {element:?}",
                );
            }
        }
    }
}
