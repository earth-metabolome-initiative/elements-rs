//! Submodule implementing the `ElementVariant` trait for the `Isotope`
//! enum.

impl super::ElementVariant for super::Isotope {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn element(&self) -> crate::Element {
        match self {
            Self::H(isotope) => isotope.element(),
            Self::He(isotope) => isotope.element(),
            Self::Li(isotope) => isotope.element(),
            Self::Be(isotope) => isotope.element(),
            Self::B(isotope) => isotope.element(),
            Self::C(isotope) => isotope.element(),
            Self::N(isotope) => isotope.element(),
            Self::O(isotope) => isotope.element(),
            Self::F(isotope) => isotope.element(),
            Self::Ne(isotope) => isotope.element(),
            Self::Na(isotope) => isotope.element(),
            Self::Mg(isotope) => isotope.element(),
            Self::Al(isotope) => isotope.element(),
            Self::Si(isotope) => isotope.element(),
            Self::P(isotope) => isotope.element(),
            Self::S(isotope) => isotope.element(),
            Self::Cl(isotope) => isotope.element(),
            Self::Ar(isotope) => isotope.element(),
            Self::K(isotope) => isotope.element(),
            Self::Ca(isotope) => isotope.element(),
            Self::Sc(isotope) => isotope.element(),
            Self::Ti(isotope) => isotope.element(),
            Self::V(isotope) => isotope.element(),
            Self::Cr(isotope) => isotope.element(),
            Self::Mn(isotope) => isotope.element(),
            Self::Fe(isotope) => isotope.element(),
            Self::Co(isotope) => isotope.element(),
            Self::Ni(isotope) => isotope.element(),
            Self::Cu(isotope) => isotope.element(),
            Self::Zn(isotope) => isotope.element(),
            Self::Ga(isotope) => isotope.element(),
            Self::Ge(isotope) => isotope.element(),
            Self::As(isotope) => isotope.element(),
            Self::Se(isotope) => isotope.element(),
            Self::Br(isotope) => isotope.element(),
            Self::Kr(isotope) => isotope.element(),
            Self::Rb(isotope) => isotope.element(),
            Self::Sr(isotope) => isotope.element(),
            Self::Y(isotope) => isotope.element(),
            Self::Zr(isotope) => isotope.element(),
            Self::Nb(isotope) => isotope.element(),
            Self::Mo(isotope) => isotope.element(),
            Self::Tc(isotope) => isotope.element(),
            Self::Ru(isotope) => isotope.element(),
            Self::Rh(isotope) => isotope.element(),
            Self::Pd(isotope) => isotope.element(),
            Self::Ag(isotope) => isotope.element(),
            Self::Cd(isotope) => isotope.element(),
            Self::In(isotope) => isotope.element(),
            Self::Sn(isotope) => isotope.element(),
            Self::Sb(isotope) => isotope.element(),
            Self::Te(isotope) => isotope.element(),
            Self::I(isotope) => isotope.element(),
            Self::Xe(isotope) => isotope.element(),
            Self::Cs(isotope) => isotope.element(),
            Self::Ba(isotope) => isotope.element(),
            Self::La(isotope) => isotope.element(),
            Self::Ce(isotope) => isotope.element(),
            Self::Pr(isotope) => isotope.element(),
            Self::Nd(isotope) => isotope.element(),
            Self::Pm(isotope) => isotope.element(),
            Self::Sm(isotope) => isotope.element(),
            Self::Eu(isotope) => isotope.element(),
            Self::Gd(isotope) => isotope.element(),
            Self::Tb(isotope) => isotope.element(),
            Self::Dy(isotope) => isotope.element(),
            Self::Ho(isotope) => isotope.element(),
            Self::Er(isotope) => isotope.element(),
            Self::Tm(isotope) => isotope.element(),
            Self::Yb(isotope) => isotope.element(),
            Self::Lu(isotope) => isotope.element(),
            Self::Hf(isotope) => isotope.element(),
            Self::Ta(isotope) => isotope.element(),
            Self::W(isotope) => isotope.element(),
            Self::Re(isotope) => isotope.element(),
            Self::Os(isotope) => isotope.element(),
            Self::Ir(isotope) => isotope.element(),
            Self::Pt(isotope) => isotope.element(),
            Self::Au(isotope) => isotope.element(),
            Self::Hg(isotope) => isotope.element(),
            Self::Tl(isotope) => isotope.element(),
            Self::Pb(isotope) => isotope.element(),
            Self::Bi(isotope) => isotope.element(),
            Self::Po(isotope) => isotope.element(),
            Self::At(isotope) => isotope.element(),
            Self::Rn(isotope) => isotope.element(),
            Self::Fr(isotope) => isotope.element(),
            Self::Ra(isotope) => isotope.element(),
            Self::Ac(isotope) => isotope.element(),
            Self::Th(isotope) => isotope.element(),
            Self::Pa(isotope) => isotope.element(),
            Self::U(isotope) => isotope.element(),
            Self::Np(isotope) => isotope.element(),
            Self::Pu(isotope) => isotope.element(),
            Self::Am(isotope) => isotope.element(),
            Self::Cm(isotope) => isotope.element(),
            Self::Bk(isotope) => isotope.element(),
            Self::Cf(isotope) => isotope.element(),
            Self::Es(isotope) => isotope.element(),
            Self::Fm(isotope) => isotope.element(),
            Self::Md(isotope) => isotope.element(),
            Self::No(isotope) => isotope.element(),
            Self::Lr(isotope) => isotope.element(),
            Self::Rf(isotope) => isotope.element(),
            Self::Db(isotope) => isotope.element(),
            Self::Sg(isotope) => isotope.element(),
            Self::Bh(isotope) => isotope.element(),
            Self::Hs(isotope) => isotope.element(),
            Self::Mt(isotope) => isotope.element(),
            Self::Ds(isotope) => isotope.element(),
            Self::Rg(isotope) => isotope.element(),
            Self::Cn(isotope) => isotope.element(),
            Self::Nh(isotope) => isotope.element(),
            Self::Fl(isotope) => isotope.element(),
            Self::Mc(isotope) => isotope.element(),
            Self::Lv(isotope) => isotope.element(),
            Self::Ts(isotope) => isotope.element(),
            Self::Og(isotope) => isotope.element(),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::isotopes::ElementVariant;

    #[test]
    fn test_element_variant() {
        for element in crate::Element::iter() {
            let isotopes = element.isotopes();
            for isotope in isotopes {
                assert_eq!(
                    isotope.element(),
                    element,
                    "Isotope {isotope:?} should return element {element:?}",
                );
            }
        }
    }

    #[test]
    fn test_element_variant_delegation() {
        // Test that the Isotope enum correctly delegates to individual isotope
        // implementations for the element() method. Test representative isotopes
        // from different elements.

        // Hydrogen isotopes
        let h1 = crate::isotopes::Isotope::H(crate::isotopes::HydrogenIsotope::H1);
        assert_eq!(h1.element(), crate::Element::H);

        let d2 = crate::isotopes::Isotope::H(crate::isotopes::HydrogenIsotope::D);
        assert_eq!(d2.element(), crate::Element::H);

        // Carbon isotopes
        let c12 = crate::isotopes::Isotope::C(crate::isotopes::CarbonIsotope::C12);
        assert_eq!(c12.element(), crate::Element::C);

        let c13 = crate::isotopes::Isotope::C(crate::isotopes::CarbonIsotope::C13);
        assert_eq!(c13.element(), crate::Element::C);

        // Oxygen isotopes
        let o16 = crate::isotopes::Isotope::O(crate::isotopes::OxygenIsotope::O16);
        assert_eq!(o16.element(), crate::Element::O);

        // Argon isotopes
        let ar40 = crate::isotopes::Isotope::Ar(crate::isotopes::ArgonIsotope::Ar40);
        assert_eq!(ar40.element(), crate::Element::Ar);

        // Uranium isotopes
        let u235 = crate::isotopes::Isotope::U(crate::isotopes::UraniumIsotope::U235);
        assert_eq!(u235.element(), crate::Element::U);

        let u238 = crate::isotopes::Isotope::U(crate::isotopes::UraniumIsotope::U238);
        assert_eq!(u238.element(), crate::Element::U);
    }
}
