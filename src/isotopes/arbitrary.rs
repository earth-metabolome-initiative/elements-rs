#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for crate::isotopes::Isotope {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        match u.int_in_range(0..=117u8)? {
            0u8 => Ok(crate::isotopes::Isotope::H(crate::isotopes::HydrogenIsotope::arbitrary(u)?)),
            1u8 => Ok(crate::isotopes::Isotope::He(crate::isotopes::HeliumIsotope::arbitrary(u)?)),
            2u8 => Ok(crate::isotopes::Isotope::Li(crate::isotopes::LithiumIsotope::arbitrary(u)?)),
            3u8 => {
                Ok(crate::isotopes::Isotope::Be(crate::isotopes::BerylliumIsotope::arbitrary(u)?))
            }
            4u8 => Ok(crate::isotopes::Isotope::B(crate::isotopes::BoronIsotope::arbitrary(u)?)),
            5u8 => Ok(crate::isotopes::Isotope::C(crate::isotopes::CarbonIsotope::arbitrary(u)?)),
            6u8 => Ok(crate::isotopes::Isotope::N(crate::isotopes::NitrogenIsotope::arbitrary(u)?)),
            7u8 => Ok(crate::isotopes::Isotope::O(crate::isotopes::OxygenIsotope::arbitrary(u)?)),
            8u8 => Ok(crate::isotopes::Isotope::F(crate::isotopes::FluorineIsotope::arbitrary(u)?)),
            9u8 => Ok(crate::isotopes::Isotope::Ne(crate::isotopes::NeonIsotope::arbitrary(u)?)),
            10u8 => Ok(crate::isotopes::Isotope::Na(crate::isotopes::SodiumIsotope::arbitrary(u)?)),
            11u8 => {
                Ok(crate::isotopes::Isotope::Mg(crate::isotopes::MagnesiumIsotope::arbitrary(u)?))
            }
            12u8 => {
                Ok(crate::isotopes::Isotope::Al(crate::isotopes::AluminiumIsotope::arbitrary(u)?))
            }
            13u8 => {
                Ok(crate::isotopes::Isotope::Si(crate::isotopes::SiliconIsotope::arbitrary(u)?))
            }
            14u8 => {
                Ok(crate::isotopes::Isotope::P(crate::isotopes::PhosphorusIsotope::arbitrary(u)?))
            }
            15u8 => Ok(crate::isotopes::Isotope::S(crate::isotopes::SulfurIsotope::arbitrary(u)?)),
            16u8 => {
                Ok(crate::isotopes::Isotope::Cl(crate::isotopes::ChlorineIsotope::arbitrary(u)?))
            }
            17u8 => Ok(crate::isotopes::Isotope::Ar(crate::isotopes::ArgonIsotope::arbitrary(u)?)),
            18u8 => {
                Ok(crate::isotopes::Isotope::K(crate::isotopes::PotassiumIsotope::arbitrary(u)?))
            }
            19u8 => {
                Ok(crate::isotopes::Isotope::Ca(crate::isotopes::CalciumIsotope::arbitrary(u)?))
            }
            20u8 => {
                Ok(crate::isotopes::Isotope::Sc(crate::isotopes::ScandiumIsotope::arbitrary(u)?))
            }
            21u8 => {
                Ok(crate::isotopes::Isotope::Ti(crate::isotopes::TitaniumIsotope::arbitrary(u)?))
            }
            22u8 => {
                Ok(crate::isotopes::Isotope::V(crate::isotopes::VanadiumIsotope::arbitrary(u)?))
            }
            23u8 => {
                Ok(crate::isotopes::Isotope::Cr(crate::isotopes::ChromiumIsotope::arbitrary(u)?))
            }
            24u8 => {
                Ok(crate::isotopes::Isotope::Mn(crate::isotopes::ManganeseIsotope::arbitrary(u)?))
            }
            25u8 => Ok(crate::isotopes::Isotope::Fe(crate::isotopes::IronIsotope::arbitrary(u)?)),
            26u8 => Ok(crate::isotopes::Isotope::Co(crate::isotopes::CobaltIsotope::arbitrary(u)?)),
            27u8 => Ok(crate::isotopes::Isotope::Ni(crate::isotopes::NickelIsotope::arbitrary(u)?)),
            28u8 => Ok(crate::isotopes::Isotope::Cu(crate::isotopes::CopperIsotope::arbitrary(u)?)),
            29u8 => Ok(crate::isotopes::Isotope::Zn(crate::isotopes::ZincIsotope::arbitrary(u)?)),
            30u8 => {
                Ok(crate::isotopes::Isotope::Ga(crate::isotopes::GalliumIsotope::arbitrary(u)?))
            }
            31u8 => {
                Ok(crate::isotopes::Isotope::Ge(crate::isotopes::GermaniumIsotope::arbitrary(u)?))
            }
            32u8 => {
                Ok(crate::isotopes::Isotope::As(crate::isotopes::ArsenicIsotope::arbitrary(u)?))
            }
            33u8 => {
                Ok(crate::isotopes::Isotope::Se(crate::isotopes::SeleniumIsotope::arbitrary(u)?))
            }
            34u8 => {
                Ok(crate::isotopes::Isotope::Br(crate::isotopes::BromineIsotope::arbitrary(u)?))
            }
            35u8 => {
                Ok(crate::isotopes::Isotope::Kr(crate::isotopes::KryptonIsotope::arbitrary(u)?))
            }
            36u8 => {
                Ok(crate::isotopes::Isotope::Rb(crate::isotopes::RubidiumIsotope::arbitrary(u)?))
            }
            37u8 => {
                Ok(crate::isotopes::Isotope::Sr(crate::isotopes::StrontiumIsotope::arbitrary(u)?))
            }
            38u8 => Ok(crate::isotopes::Isotope::Y(crate::isotopes::YttriumIsotope::arbitrary(u)?)),
            39u8 => {
                Ok(crate::isotopes::Isotope::Zr(crate::isotopes::ZirconiumIsotope::arbitrary(u)?))
            }
            40u8 => {
                Ok(crate::isotopes::Isotope::Nb(crate::isotopes::NiobiumIsotope::arbitrary(u)?))
            }
            41u8 => {
                Ok(crate::isotopes::Isotope::Mo(crate::isotopes::MolybdenumIsotope::arbitrary(u)?))
            }
            42u8 => {
                Ok(crate::isotopes::Isotope::Tc(crate::isotopes::TechnetiumIsotope::arbitrary(u)?))
            }
            43u8 => {
                Ok(crate::isotopes::Isotope::Ru(crate::isotopes::RutheniumIsotope::arbitrary(u)?))
            }
            44u8 => {
                Ok(crate::isotopes::Isotope::Rh(crate::isotopes::RhodiumIsotope::arbitrary(u)?))
            }
            45u8 => {
                Ok(crate::isotopes::Isotope::Pd(crate::isotopes::PalladiumIsotope::arbitrary(u)?))
            }
            46u8 => Ok(crate::isotopes::Isotope::Ag(crate::isotopes::SilverIsotope::arbitrary(u)?)),
            47u8 => {
                Ok(crate::isotopes::Isotope::Cd(crate::isotopes::CadmiumIsotope::arbitrary(u)?))
            }
            48u8 => Ok(crate::isotopes::Isotope::In(crate::isotopes::IndiumIsotope::arbitrary(u)?)),
            49u8 => Ok(crate::isotopes::Isotope::Sn(crate::isotopes::TinIsotope::arbitrary(u)?)),
            50u8 => {
                Ok(crate::isotopes::Isotope::Sb(crate::isotopes::AntimonyIsotope::arbitrary(u)?))
            }
            51u8 => {
                Ok(crate::isotopes::Isotope::Te(crate::isotopes::TelluriumIsotope::arbitrary(u)?))
            }
            52u8 => Ok(crate::isotopes::Isotope::I(crate::isotopes::IodineIsotope::arbitrary(u)?)),
            53u8 => Ok(crate::isotopes::Isotope::Xe(crate::isotopes::XenonIsotope::arbitrary(u)?)),
            54u8 => {
                Ok(crate::isotopes::Isotope::Cs(crate::isotopes::CaesiumIsotope::arbitrary(u)?))
            }
            55u8 => Ok(crate::isotopes::Isotope::Ba(crate::isotopes::BariumIsotope::arbitrary(u)?)),
            56u8 => {
                Ok(crate::isotopes::Isotope::La(crate::isotopes::LanthanumIsotope::arbitrary(u)?))
            }
            57u8 => Ok(crate::isotopes::Isotope::Ce(crate::isotopes::CeriumIsotope::arbitrary(u)?)),
            58u8 => {
                Ok(crate::isotopes::Isotope::Pr(crate::isotopes::PraseodymiumIsotope::arbitrary(
                    u,
                )?))
            }
            59u8 => {
                Ok(crate::isotopes::Isotope::Nd(crate::isotopes::NeodymiumIsotope::arbitrary(u)?))
            }
            60u8 => {
                Ok(crate::isotopes::Isotope::Pm(crate::isotopes::PromethiumIsotope::arbitrary(u)?))
            }
            61u8 => {
                Ok(crate::isotopes::Isotope::Sm(crate::isotopes::SamariumIsotope::arbitrary(u)?))
            }
            62u8 => {
                Ok(crate::isotopes::Isotope::Eu(crate::isotopes::EuropiumIsotope::arbitrary(u)?))
            }
            63u8 => {
                Ok(crate::isotopes::Isotope::Gd(crate::isotopes::GadoliniumIsotope::arbitrary(u)?))
            }
            64u8 => {
                Ok(crate::isotopes::Isotope::Tb(crate::isotopes::TerbiumIsotope::arbitrary(u)?))
            }
            65u8 => {
                Ok(crate::isotopes::Isotope::Dy(crate::isotopes::DysprosiumIsotope::arbitrary(u)?))
            }
            66u8 => {
                Ok(crate::isotopes::Isotope::Ho(crate::isotopes::HolmiumIsotope::arbitrary(u)?))
            }
            67u8 => Ok(crate::isotopes::Isotope::Er(crate::isotopes::ErbiumIsotope::arbitrary(u)?)),
            68u8 => {
                Ok(crate::isotopes::Isotope::Tm(crate::isotopes::ThuliumIsotope::arbitrary(u)?))
            }
            69u8 => {
                Ok(crate::isotopes::Isotope::Yb(crate::isotopes::YtterbiumIsotope::arbitrary(u)?))
            }
            70u8 => {
                Ok(crate::isotopes::Isotope::Lu(crate::isotopes::LutetiumIsotope::arbitrary(u)?))
            }
            71u8 => {
                Ok(crate::isotopes::Isotope::Hf(crate::isotopes::HafniumIsotope::arbitrary(u)?))
            }
            72u8 => {
                Ok(crate::isotopes::Isotope::Ta(crate::isotopes::TantalumIsotope::arbitrary(u)?))
            }
            73u8 => {
                Ok(crate::isotopes::Isotope::W(crate::isotopes::TungstenIsotope::arbitrary(u)?))
            }
            74u8 => {
                Ok(crate::isotopes::Isotope::Re(crate::isotopes::RheniumIsotope::arbitrary(u)?))
            }
            75u8 => Ok(crate::isotopes::Isotope::Os(crate::isotopes::OsmiumIsotope::arbitrary(u)?)),
            76u8 => {
                Ok(crate::isotopes::Isotope::Ir(crate::isotopes::IridiumIsotope::arbitrary(u)?))
            }
            77u8 => {
                Ok(crate::isotopes::Isotope::Pt(crate::isotopes::PlatinumIsotope::arbitrary(u)?))
            }
            78u8 => Ok(crate::isotopes::Isotope::Au(crate::isotopes::GoldIsotope::arbitrary(u)?)),
            79u8 => {
                Ok(crate::isotopes::Isotope::Hg(crate::isotopes::MercuryIsotope::arbitrary(u)?))
            }
            80u8 => {
                Ok(crate::isotopes::Isotope::Tl(crate::isotopes::ThalliumIsotope::arbitrary(u)?))
            }
            81u8 => Ok(crate::isotopes::Isotope::Pb(crate::isotopes::LeadIsotope::arbitrary(u)?)),
            82u8 => {
                Ok(crate::isotopes::Isotope::Bi(crate::isotopes::BismuthIsotope::arbitrary(u)?))
            }
            83u8 => {
                Ok(crate::isotopes::Isotope::Po(crate::isotopes::PoloniumIsotope::arbitrary(u)?))
            }
            84u8 => {
                Ok(crate::isotopes::Isotope::At(crate::isotopes::AstatineIsotope::arbitrary(u)?))
            }
            85u8 => Ok(crate::isotopes::Isotope::Rn(crate::isotopes::RadonIsotope::arbitrary(u)?)),
            86u8 => {
                Ok(crate::isotopes::Isotope::Fr(crate::isotopes::FranciumIsotope::arbitrary(u)?))
            }
            87u8 => Ok(crate::isotopes::Isotope::Ra(crate::isotopes::RadiumIsotope::arbitrary(u)?)),
            88u8 => {
                Ok(crate::isotopes::Isotope::Ac(crate::isotopes::ActiniumIsotope::arbitrary(u)?))
            }
            89u8 => {
                Ok(crate::isotopes::Isotope::Th(crate::isotopes::ThoriumIsotope::arbitrary(u)?))
            }
            90u8 => {
                Ok(crate::isotopes::Isotope::Pa(crate::isotopes::ProtactiniumIsotope::arbitrary(
                    u,
                )?))
            }
            91u8 => Ok(crate::isotopes::Isotope::U(crate::isotopes::UraniumIsotope::arbitrary(u)?)),
            92u8 => {
                Ok(crate::isotopes::Isotope::Np(crate::isotopes::NeptuniumIsotope::arbitrary(u)?))
            }
            93u8 => {
                Ok(crate::isotopes::Isotope::Pu(crate::isotopes::PlutoniumIsotope::arbitrary(u)?))
            }
            94u8 => {
                Ok(crate::isotopes::Isotope::Am(crate::isotopes::AmericiumIsotope::arbitrary(u)?))
            }
            95u8 => Ok(crate::isotopes::Isotope::Cm(crate::isotopes::CuriumIsotope::arbitrary(u)?)),
            96u8 => {
                Ok(crate::isotopes::Isotope::Bk(crate::isotopes::BerkeliumIsotope::arbitrary(u)?))
            }
            97u8 => {
                Ok(crate::isotopes::Isotope::Cf(crate::isotopes::CaliforniumIsotope::arbitrary(u)?))
            }
            98u8 => {
                Ok(crate::isotopes::Isotope::Es(crate::isotopes::EinsteiniumIsotope::arbitrary(u)?))
            }
            99u8 => {
                Ok(crate::isotopes::Isotope::Fm(crate::isotopes::FermiumIsotope::arbitrary(u)?))
            }
            100u8 => {
                Ok(crate::isotopes::Isotope::Md(crate::isotopes::MendeleviumIsotope::arbitrary(u)?))
            }
            101u8 => {
                Ok(crate::isotopes::Isotope::No(crate::isotopes::NobeliumIsotope::arbitrary(u)?))
            }
            102u8 => {
                Ok(crate::isotopes::Isotope::Lr(crate::isotopes::LawrenciumIsotope::arbitrary(u)?))
            }
            103u8 => {
                Ok(crate::isotopes::Isotope::Rf(crate::isotopes::RutherfordiumIsotope::arbitrary(
                    u,
                )?))
            }
            104u8 => {
                Ok(crate::isotopes::Isotope::Db(crate::isotopes::DubniumIsotope::arbitrary(u)?))
            }
            105u8 => {
                Ok(crate::isotopes::Isotope::Sg(crate::isotopes::SeaborgiumIsotope::arbitrary(u)?))
            }
            106u8 => {
                Ok(crate::isotopes::Isotope::Bh(crate::isotopes::BohriumIsotope::arbitrary(u)?))
            }
            107u8 => {
                Ok(crate::isotopes::Isotope::Hs(crate::isotopes::HassiumIsotope::arbitrary(u)?))
            }
            108u8 => {
                Ok(crate::isotopes::Isotope::Mt(crate::isotopes::MeitneriumIsotope::arbitrary(u)?))
            }
            109u8 => {
                Ok(crate::isotopes::Isotope::Ds(crate::isotopes::DarmstadtiumIsotope::arbitrary(
                    u,
                )?))
            }
            110u8 => {
                Ok(crate::isotopes::Isotope::Rg(crate::isotopes::RoentgeniumIsotope::arbitrary(u)?))
            }
            111u8 => {
                Ok(crate::isotopes::Isotope::Cn(crate::isotopes::CoperniciumIsotope::arbitrary(u)?))
            }
            112u8 => {
                Ok(crate::isotopes::Isotope::Nh(crate::isotopes::NihoniumIsotope::arbitrary(u)?))
            }
            113u8 => {
                Ok(crate::isotopes::Isotope::Fl(crate::isotopes::FleroviumIsotope::arbitrary(u)?))
            }
            114u8 => {
                Ok(crate::isotopes::Isotope::Mc(crate::isotopes::MoscoviumIsotope::arbitrary(u)?))
            }
            115u8 => {
                Ok(crate::isotopes::Isotope::Lv(crate::isotopes::LivermoriumIsotope::arbitrary(u)?))
            }
            116u8 => {
                Ok(crate::isotopes::Isotope::Ts(crate::isotopes::TennessineIsotope::arbitrary(u)?))
            }
            117u8 => {
                Ok(crate::isotopes::Isotope::Og(crate::isotopes::OganessonIsotope::arbitrary(u)?))
            }
            _ => unreachable!(),
        }
    }
}
