//! Submodule implementing the `TryFrom<(Element, u16)>` trait for the `Isotope`
//! enumeration.

use crate::{
    Element,
    isotopes::{
        ActiniumIsotope, AluminiumIsotope, AmericiumIsotope, AntimonyIsotope, ArgonIsotope,
        ArsenicIsotope, AstatineIsotope, BariumIsotope, BerkeliumIsotope, BerylliumIsotope,
        BismuthIsotope, BohriumIsotope, BoronIsotope, BromineIsotope, CadmiumIsotope,
        CaesiumIsotope, CalciumIsotope, CaliforniumIsotope, CarbonIsotope, CeriumIsotope,
        ChlorineIsotope, ChromiumIsotope, CobaltIsotope, CoperniciumIsotope, CopperIsotope,
        CuriumIsotope, DarmstadtiumIsotope, DubniumIsotope, DysprosiumIsotope, EinsteiniumIsotope,
        ErbiumIsotope, EuropiumIsotope, FermiumIsotope, FleroviumIsotope, FluorineIsotope,
        FranciumIsotope, GadoliniumIsotope, GalliumIsotope, GermaniumIsotope, GoldIsotope,
        HafniumIsotope, HassiumIsotope, HeliumIsotope, HolmiumIsotope, HydrogenIsotope,
        IndiumIsotope, IodineIsotope, IridiumIsotope, IronIsotope, KryptonIsotope,
        LanthanumIsotope, LawrenciumIsotope, LeadIsotope, LithiumIsotope, LivermoriumIsotope,
        LutetiumIsotope, MagnesiumIsotope, ManganeseIsotope, MeitneriumIsotope, MendeleviumIsotope,
        MercuryIsotope, MolybdenumIsotope, MoscoviumIsotope, NeodymiumIsotope, NeonIsotope,
        NeptuniumIsotope, NickelIsotope, NihoniumIsotope, NiobiumIsotope, NitrogenIsotope,
        NobeliumIsotope, OganessonIsotope, OsmiumIsotope, OxygenIsotope, PalladiumIsotope,
        PhosphorusIsotope, PlatinumIsotope, PlutoniumIsotope, PoloniumIsotope, PotassiumIsotope,
        PraseodymiumIsotope, PromethiumIsotope, ProtactiniumIsotope, RadiumIsotope, RadonIsotope,
        RheniumIsotope, RhodiumIsotope, RoentgeniumIsotope, RubidiumIsotope, RutheniumIsotope,
        RutherfordiumIsotope, SamariumIsotope, ScandiumIsotope, SeaborgiumIsotope, SeleniumIsotope,
        SiliconIsotope, SilverIsotope, SodiumIsotope, StrontiumIsotope, SulfurIsotope,
        TantalumIsotope, TechnetiumIsotope, TelluriumIsotope, TennessineIsotope, TerbiumIsotope,
        ThalliumIsotope, ThoriumIsotope, ThuliumIsotope, TinIsotope, TitaniumIsotope,
        TungstenIsotope, UraniumIsotope, VanadiumIsotope, XenonIsotope, YtterbiumIsotope,
        YttriumIsotope, ZincIsotope, ZirconiumIsotope,
    },
};

impl TryFrom<(Element, u16)> for crate::Isotope {
    type Error = crate::errors::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from((element, mass): (Element, u16)) -> Result<Self, Self::Error> {
        Ok(match element {
            Element::H => Self::H(HydrogenIsotope::try_from(mass)?),
            Element::He => Self::He(HeliumIsotope::try_from(mass)?),
            Element::Li => Self::Li(LithiumIsotope::try_from(mass)?),
            Element::Be => Self::Be(BerylliumIsotope::try_from(mass)?),
            Element::B => Self::B(BoronIsotope::try_from(mass)?),
            Element::C => Self::C(CarbonIsotope::try_from(mass)?),
            Element::N => Self::N(NitrogenIsotope::try_from(mass)?),
            Element::O => Self::O(OxygenIsotope::try_from(mass)?),
            Element::F => Self::F(FluorineIsotope::try_from(mass)?),
            Element::Ne => Self::Ne(NeonIsotope::try_from(mass)?),
            Element::Na => Self::Na(SodiumIsotope::try_from(mass)?),
            Element::Mg => Self::Mg(MagnesiumIsotope::try_from(mass)?),
            Element::Al => Self::Al(AluminiumIsotope::try_from(mass)?),
            Element::Si => Self::Si(SiliconIsotope::try_from(mass)?),
            Element::P => Self::P(PhosphorusIsotope::try_from(mass)?),
            Element::S => Self::S(SulfurIsotope::try_from(mass)?),
            Element::Cl => Self::Cl(ChlorineIsotope::try_from(mass)?),
            Element::Ar => Self::Ar(ArgonIsotope::try_from(mass)?),
            Element::K => Self::K(PotassiumIsotope::try_from(mass)?),
            Element::Ca => Self::Ca(CalciumIsotope::try_from(mass)?),
            Element::Sc => Self::Sc(ScandiumIsotope::try_from(mass)?),
            Element::Ti => Self::Ti(TitaniumIsotope::try_from(mass)?),
            Element::V => Self::V(VanadiumIsotope::try_from(mass)?),
            Element::Cr => Self::Cr(ChromiumIsotope::try_from(mass)?),
            Element::Mn => Self::Mn(ManganeseIsotope::try_from(mass)?),
            Element::Fe => Self::Fe(IronIsotope::try_from(mass)?),
            Element::Co => Self::Co(CobaltIsotope::try_from(mass)?),
            Element::Ni => Self::Ni(NickelIsotope::try_from(mass)?),
            Element::Cu => Self::Cu(CopperIsotope::try_from(mass)?),
            Element::Zn => Self::Zn(ZincIsotope::try_from(mass)?),
            Element::Ga => Self::Ga(GalliumIsotope::try_from(mass)?),
            Element::Ge => Self::Ge(GermaniumIsotope::try_from(mass)?),
            Element::As => Self::As(ArsenicIsotope::try_from(mass)?),
            Element::Se => Self::Se(SeleniumIsotope::try_from(mass)?),
            Element::Br => Self::Br(BromineIsotope::try_from(mass)?),
            Element::Kr => Self::Kr(KryptonIsotope::try_from(mass)?),
            Element::Rb => Self::Rb(RubidiumIsotope::try_from(mass)?),
            Element::Sr => Self::Sr(StrontiumIsotope::try_from(mass)?),
            Element::Y => Self::Y(YttriumIsotope::try_from(mass)?),
            Element::Zr => Self::Zr(ZirconiumIsotope::try_from(mass)?),
            Element::Nb => Self::Nb(NiobiumIsotope::try_from(mass)?),
            Element::Mo => Self::Mo(MolybdenumIsotope::try_from(mass)?),
            Element::Tc => Self::Tc(TechnetiumIsotope::try_from(mass)?),
            Element::Ru => Self::Ru(RutheniumIsotope::try_from(mass)?),
            Element::Rh => Self::Rh(RhodiumIsotope::try_from(mass)?),
            Element::Pd => Self::Pd(PalladiumIsotope::try_from(mass)?),
            Element::Ag => Self::Ag(SilverIsotope::try_from(mass)?),
            Element::Cd => Self::Cd(CadmiumIsotope::try_from(mass)?),
            Element::In => Self::In(IndiumIsotope::try_from(mass)?),
            Element::Sn => Self::Sn(TinIsotope::try_from(mass)?),
            Element::Sb => Self::Sb(AntimonyIsotope::try_from(mass)?),
            Element::Te => Self::Te(TelluriumIsotope::try_from(mass)?),
            Element::I => Self::I(IodineIsotope::try_from(mass)?),
            Element::Xe => Self::Xe(XenonIsotope::try_from(mass)?),
            Element::Cs => Self::Cs(CaesiumIsotope::try_from(mass)?),
            Element::Ba => Self::Ba(BariumIsotope::try_from(mass)?),
            Element::La => Self::La(LanthanumIsotope::try_from(mass)?),
            Element::Ce => Self::Ce(CeriumIsotope::try_from(mass)?),
            Element::Pr => Self::Pr(PraseodymiumIsotope::try_from(mass)?),
            Element::Nd => Self::Nd(NeodymiumIsotope::try_from(mass)?),
            Element::Pm => Self::Pm(PromethiumIsotope::try_from(mass)?),
            Element::Sm => Self::Sm(SamariumIsotope::try_from(mass)?),
            Element::Eu => Self::Eu(EuropiumIsotope::try_from(mass)?),
            Element::Gd => Self::Gd(GadoliniumIsotope::try_from(mass)?),
            Element::Tb => Self::Tb(TerbiumIsotope::try_from(mass)?),
            Element::Dy => Self::Dy(DysprosiumIsotope::try_from(mass)?),
            Element::Ho => Self::Ho(HolmiumIsotope::try_from(mass)?),
            Element::Er => Self::Er(ErbiumIsotope::try_from(mass)?),
            Element::Tm => Self::Tm(ThuliumIsotope::try_from(mass)?),
            Element::Yb => Self::Yb(YtterbiumIsotope::try_from(mass)?),
            Element::Lu => Self::Lu(LutetiumIsotope::try_from(mass)?),
            Element::Hf => Self::Hf(HafniumIsotope::try_from(mass)?),
            Element::Ta => Self::Ta(TantalumIsotope::try_from(mass)?),
            Element::W => Self::W(TungstenIsotope::try_from(mass)?),
            Element::Re => Self::Re(RheniumIsotope::try_from(mass)?),
            Element::Os => Self::Os(OsmiumIsotope::try_from(mass)?),
            Element::Ir => Self::Ir(IridiumIsotope::try_from(mass)?),
            Element::Pt => Self::Pt(PlatinumIsotope::try_from(mass)?),
            Element::Au => Self::Au(GoldIsotope::try_from(mass)?),
            Element::Hg => Self::Hg(MercuryIsotope::try_from(mass)?),
            Element::Tl => Self::Tl(ThalliumIsotope::try_from(mass)?),
            Element::Pb => Self::Pb(LeadIsotope::try_from(mass)?),
            Element::Bi => Self::Bi(BismuthIsotope::try_from(mass)?),
            Element::Po => Self::Po(PoloniumIsotope::try_from(mass)?),
            Element::At => Self::At(AstatineIsotope::try_from(mass)?),
            Element::Rn => Self::Rn(RadonIsotope::try_from(mass)?),
            Element::Fr => Self::Fr(FranciumIsotope::try_from(mass)?),
            Element::Ra => Self::Ra(RadiumIsotope::try_from(mass)?),
            Element::Ac => Self::Ac(ActiniumIsotope::try_from(mass)?),
            Element::Th => Self::Th(ThoriumIsotope::try_from(mass)?),
            Element::Pa => Self::Pa(ProtactiniumIsotope::try_from(mass)?),
            Element::U => Self::U(UraniumIsotope::try_from(mass)?),
            Element::Np => Self::Np(NeptuniumIsotope::try_from(mass)?),
            Element::Pu => Self::Pu(PlutoniumIsotope::try_from(mass)?),
            Element::Am => Self::Am(AmericiumIsotope::try_from(mass)?),
            Element::Cm => Self::Cm(CuriumIsotope::try_from(mass)?),
            Element::Bk => Self::Bk(BerkeliumIsotope::try_from(mass)?),
            Element::Cf => Self::Cf(CaliforniumIsotope::try_from(mass)?),
            Element::Es => Self::Es(EinsteiniumIsotope::try_from(mass)?),
            Element::Fm => Self::Fm(FermiumIsotope::try_from(mass)?),
            Element::Md => Self::Md(MendeleviumIsotope::try_from(mass)?),
            Element::No => Self::No(NobeliumIsotope::try_from(mass)?),
            Element::Lr => Self::Lr(LawrenciumIsotope::try_from(mass)?),
            Element::Rf => Self::Rf(RutherfordiumIsotope::try_from(mass)?),
            Element::Db => Self::Db(DubniumIsotope::try_from(mass)?),
            Element::Sg => Self::Sg(SeaborgiumIsotope::try_from(mass)?),
            Element::Bh => Self::Bh(BohriumIsotope::try_from(mass)?),
            Element::Hs => Self::Hs(HassiumIsotope::try_from(mass)?),
            Element::Mt => Self::Mt(MeitneriumIsotope::try_from(mass)?),
            Element::Ds => Self::Ds(DarmstadtiumIsotope::try_from(mass)?),
            Element::Rg => Self::Rg(RoentgeniumIsotope::try_from(mass)?),
            Element::Cn => Self::Cn(CoperniciumIsotope::try_from(mass)?),
            Element::Nh => Self::Nh(NihoniumIsotope::try_from(mass)?),
            Element::Fl => Self::Fl(FleroviumIsotope::try_from(mass)?),
            Element::Mc => Self::Mc(MoscoviumIsotope::try_from(mass)?),
            Element::Lv => Self::Lv(LivermoriumIsotope::try_from(mass)?),
            Element::Ts => Self::Ts(TennessineIsotope::try_from(mass)?),
            Element::Og => Self::Og(OganessonIsotope::try_from(mass)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, isotopes::Isotope};

    #[test]
    fn test_try_from_element_delegation() {
        // Test that the Isotope enum correctly delegates to individual isotope
        // implementations Test representative isotopes from different elements

        // Hydrogen
        let h1 = Isotope::try_from((Element::H, 1)).unwrap();
        assert_eq!(h1, Isotope::H(crate::isotopes::HydrogenIsotope::H1));

        let d2 = Isotope::try_from((Element::H, 2)).unwrap();
        assert_eq!(d2, Isotope::H(crate::isotopes::HydrogenIsotope::D2));

        let t3 = Isotope::try_from((Element::H, 3)).unwrap();
        assert_eq!(t3, Isotope::H(crate::isotopes::HydrogenIsotope::T3));

        // Carbon
        let c12 = Isotope::try_from((Element::C, 12)).unwrap();
        assert_eq!(c12, Isotope::C(crate::isotopes::CarbonIsotope::C12));

        let c13 = Isotope::try_from((Element::C, 13)).unwrap();
        assert_eq!(c13, Isotope::C(crate::isotopes::CarbonIsotope::C13));

        let c14 = Isotope::try_from((Element::C, 14)).unwrap();
        assert_eq!(c14, Isotope::C(crate::isotopes::CarbonIsotope::C14));

        // Oxygen
        let o16 = Isotope::try_from((Element::O, 16)).unwrap();
        assert_eq!(o16, Isotope::O(crate::isotopes::OxygenIsotope::O16));

        // Argon
        let ar36 = Isotope::try_from((Element::Ar, 36)).unwrap();
        assert_eq!(ar36, Isotope::Ar(crate::isotopes::ArgonIsotope::Ar36));

        let ar40 = Isotope::try_from((Element::Ar, 40)).unwrap();
        assert_eq!(ar40, Isotope::Ar(crate::isotopes::ArgonIsotope::Ar40));
    }

    #[test]
    fn test_try_from_element_invalid_mass() {
        // Test invalid mass numbers for various elements
        assert!(Isotope::try_from((Element::H, 0)).is_err());
        assert!(Isotope::try_from((Element::H, 8)).is_err()); // H7 is the highest for hydrogen
        assert!(Isotope::try_from((Element::C, 7)).is_err()); // C8 is the lowest for carbon
        assert!(Isotope::try_from((Element::C, 24)).is_err()); // C23 is the highest for carbon
        assert!(Isotope::try_from((Element::O, 11)).is_err()); // O12 is the lowest for oxygen
        assert!(Isotope::try_from((Element::O, 29)).is_err()); // O28 is the highest for oxygen
    }
}
