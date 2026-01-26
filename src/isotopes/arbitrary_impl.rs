#![cfg(feature = "arbitrary")]

impl<'a> arbitrary::Arbitrary<'a> for crate::isotopes::Isotope {
    #[allow(clippy::too_many_lines)]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        match u.int_in_range(0..=117u8)? {
            0u8 => Ok(crate::isotopes::HydrogenIsotope::arbitrary(u)?.into()),
            1u8 => Ok(crate::isotopes::HeliumIsotope::arbitrary(u)?.into()),
            2u8 => Ok(crate::isotopes::LithiumIsotope::arbitrary(u)?.into()),
            3u8 => Ok(crate::isotopes::BerylliumIsotope::arbitrary(u)?.into()),
            4u8 => Ok(crate::isotopes::BoronIsotope::arbitrary(u)?.into()),
            5u8 => Ok(crate::isotopes::CarbonIsotope::arbitrary(u)?.into()),
            6u8 => Ok(crate::isotopes::NitrogenIsotope::arbitrary(u)?.into()),
            7u8 => Ok(crate::isotopes::OxygenIsotope::arbitrary(u)?.into()),
            8u8 => Ok(crate::isotopes::FluorineIsotope::arbitrary(u)?.into()),
            9u8 => Ok(crate::isotopes::NeonIsotope::arbitrary(u)?.into()),
            10u8 => Ok(crate::isotopes::SodiumIsotope::arbitrary(u)?.into()),
            11u8 => Ok(crate::isotopes::MagnesiumIsotope::arbitrary(u)?.into()),
            12u8 => Ok(crate::isotopes::AluminiumIsotope::arbitrary(u)?.into()),
            13u8 => Ok(crate::isotopes::SiliconIsotope::arbitrary(u)?.into()),
            14u8 => Ok(crate::isotopes::PhosphorusIsotope::arbitrary(u)?.into()),
            15u8 => Ok(crate::isotopes::SulfurIsotope::arbitrary(u)?.into()),
            16u8 => Ok(crate::isotopes::ChlorineIsotope::arbitrary(u)?.into()),
            17u8 => Ok(crate::isotopes::ArgonIsotope::arbitrary(u)?.into()),
            18u8 => Ok(crate::isotopes::PotassiumIsotope::arbitrary(u)?.into()),
            19u8 => Ok(crate::isotopes::CalciumIsotope::arbitrary(u)?.into()),
            20u8 => Ok(crate::isotopes::ScandiumIsotope::arbitrary(u)?.into()),
            21u8 => Ok(crate::isotopes::TitaniumIsotope::arbitrary(u)?.into()),
            22u8 => Ok(crate::isotopes::VanadiumIsotope::arbitrary(u)?.into()),
            23u8 => Ok(crate::isotopes::ChromiumIsotope::arbitrary(u)?.into()),
            24u8 => Ok(crate::isotopes::ManganeseIsotope::arbitrary(u)?.into()),
            25u8 => Ok(crate::isotopes::IronIsotope::arbitrary(u)?.into()),
            26u8 => Ok(crate::isotopes::CobaltIsotope::arbitrary(u)?.into()),
            27u8 => Ok(crate::isotopes::NickelIsotope::arbitrary(u)?.into()),
            28u8 => Ok(crate::isotopes::CopperIsotope::arbitrary(u)?.into()),
            29u8 => Ok(crate::isotopes::ZincIsotope::arbitrary(u)?.into()),
            30u8 => Ok(crate::isotopes::GalliumIsotope::arbitrary(u)?.into()),
            31u8 => Ok(crate::isotopes::GermaniumIsotope::arbitrary(u)?.into()),
            32u8 => Ok(crate::isotopes::ArsenicIsotope::arbitrary(u)?.into()),
            33u8 => Ok(crate::isotopes::SeleniumIsotope::arbitrary(u)?.into()),
            34u8 => Ok(crate::isotopes::BromineIsotope::arbitrary(u)?.into()),
            35u8 => Ok(crate::isotopes::KryptonIsotope::arbitrary(u)?.into()),
            36u8 => Ok(crate::isotopes::RubidiumIsotope::arbitrary(u)?.into()),
            37u8 => Ok(crate::isotopes::StrontiumIsotope::arbitrary(u)?.into()),
            38u8 => Ok(crate::isotopes::YttriumIsotope::arbitrary(u)?.into()),
            39u8 => Ok(crate::isotopes::ZirconiumIsotope::arbitrary(u)?.into()),
            40u8 => Ok(crate::isotopes::NiobiumIsotope::arbitrary(u)?.into()),
            41u8 => Ok(crate::isotopes::MolybdenumIsotope::arbitrary(u)?.into()),
            42u8 => Ok(crate::isotopes::TechnetiumIsotope::arbitrary(u)?.into()),
            43u8 => Ok(crate::isotopes::RutheniumIsotope::arbitrary(u)?.into()),
            44u8 => Ok(crate::isotopes::RhodiumIsotope::arbitrary(u)?.into()),
            45u8 => Ok(crate::isotopes::PalladiumIsotope::arbitrary(u)?.into()),
            46u8 => Ok(crate::isotopes::SilverIsotope::arbitrary(u)?.into()),
            47u8 => Ok(crate::isotopes::CadmiumIsotope::arbitrary(u)?.into()),
            48u8 => Ok(crate::isotopes::IndiumIsotope::arbitrary(u)?.into()),
            49u8 => Ok(crate::isotopes::TinIsotope::arbitrary(u)?.into()),
            50u8 => Ok(crate::isotopes::AntimonyIsotope::arbitrary(u)?.into()),
            51u8 => Ok(crate::isotopes::TelluriumIsotope::arbitrary(u)?.into()),
            52u8 => Ok(crate::isotopes::IodineIsotope::arbitrary(u)?.into()),
            53u8 => Ok(crate::isotopes::XenonIsotope::arbitrary(u)?.into()),
            54u8 => Ok(crate::isotopes::CaesiumIsotope::arbitrary(u)?.into()),
            55u8 => Ok(crate::isotopes::BariumIsotope::arbitrary(u)?.into()),
            56u8 => Ok(crate::isotopes::LanthanumIsotope::arbitrary(u)?.into()),
            57u8 => Ok(crate::isotopes::CeriumIsotope::arbitrary(u)?.into()),
            58u8 => Ok(crate::isotopes::PraseodymiumIsotope::arbitrary(u)?.into()),
            59u8 => Ok(crate::isotopes::NeodymiumIsotope::arbitrary(u)?.into()),
            60u8 => Ok(crate::isotopes::PromethiumIsotope::arbitrary(u)?.into()),
            61u8 => Ok(crate::isotopes::SamariumIsotope::arbitrary(u)?.into()),
            62u8 => Ok(crate::isotopes::EuropiumIsotope::arbitrary(u)?.into()),
            63u8 => Ok(crate::isotopes::GadoliniumIsotope::arbitrary(u)?.into()),
            64u8 => Ok(crate::isotopes::TerbiumIsotope::arbitrary(u)?.into()),
            65u8 => Ok(crate::isotopes::DysprosiumIsotope::arbitrary(u)?.into()),
            66u8 => Ok(crate::isotopes::HolmiumIsotope::arbitrary(u)?.into()),
            67u8 => Ok(crate::isotopes::ErbiumIsotope::arbitrary(u)?.into()),
            68u8 => Ok(crate::isotopes::ThuliumIsotope::arbitrary(u)?.into()),
            69u8 => Ok(crate::isotopes::YtterbiumIsotope::arbitrary(u)?.into()),
            70u8 => Ok(crate::isotopes::LutetiumIsotope::arbitrary(u)?.into()),
            71u8 => Ok(crate::isotopes::HafniumIsotope::arbitrary(u)?.into()),
            72u8 => Ok(crate::isotopes::TantalumIsotope::arbitrary(u)?.into()),
            73u8 => Ok(crate::isotopes::TungstenIsotope::arbitrary(u)?.into()),
            74u8 => Ok(crate::isotopes::RheniumIsotope::arbitrary(u)?.into()),
            75u8 => Ok(crate::isotopes::OsmiumIsotope::arbitrary(u)?.into()),
            76u8 => Ok(crate::isotopes::IridiumIsotope::arbitrary(u)?.into()),
            77u8 => Ok(crate::isotopes::PlatinumIsotope::arbitrary(u)?.into()),
            78u8 => Ok(crate::isotopes::GoldIsotope::arbitrary(u)?.into()),
            79u8 => Ok(crate::isotopes::MercuryIsotope::arbitrary(u)?.into()),
            80u8 => Ok(crate::isotopes::ThalliumIsotope::arbitrary(u)?.into()),
            81u8 => Ok(crate::isotopes::LeadIsotope::arbitrary(u)?.into()),
            82u8 => Ok(crate::isotopes::BismuthIsotope::arbitrary(u)?.into()),
            83u8 => Ok(crate::isotopes::PoloniumIsotope::arbitrary(u)?.into()),
            84u8 => Ok(crate::isotopes::AstatineIsotope::arbitrary(u)?.into()),
            85u8 => Ok(crate::isotopes::RadonIsotope::arbitrary(u)?.into()),
            86u8 => Ok(crate::isotopes::FranciumIsotope::arbitrary(u)?.into()),
            87u8 => Ok(crate::isotopes::RadiumIsotope::arbitrary(u)?.into()),
            88u8 => Ok(crate::isotopes::ActiniumIsotope::arbitrary(u)?.into()),
            89u8 => Ok(crate::isotopes::ThoriumIsotope::arbitrary(u)?.into()),
            90u8 => Ok(crate::isotopes::ProtactiniumIsotope::arbitrary(u)?.into()),
            91u8 => Ok(crate::isotopes::UraniumIsotope::arbitrary(u)?.into()),
            92u8 => Ok(crate::isotopes::NeptuniumIsotope::arbitrary(u)?.into()),
            93u8 => Ok(crate::isotopes::PlutoniumIsotope::arbitrary(u)?.into()),
            94u8 => Ok(crate::isotopes::AmericiumIsotope::arbitrary(u)?.into()),
            95u8 => Ok(crate::isotopes::CuriumIsotope::arbitrary(u)?.into()),
            96u8 => Ok(crate::isotopes::BerkeliumIsotope::arbitrary(u)?.into()),
            97u8 => Ok(crate::isotopes::CaliforniumIsotope::arbitrary(u)?.into()),
            98u8 => Ok(crate::isotopes::EinsteiniumIsotope::arbitrary(u)?.into()),
            99u8 => Ok(crate::isotopes::FermiumIsotope::arbitrary(u)?.into()),
            100u8 => Ok(crate::isotopes::MendeleviumIsotope::arbitrary(u)?.into()),
            101u8 => Ok(crate::isotopes::NobeliumIsotope::arbitrary(u)?.into()),
            102u8 => Ok(crate::isotopes::LawrenciumIsotope::arbitrary(u)?.into()),
            103u8 => Ok(crate::isotopes::RutherfordiumIsotope::arbitrary(u)?.into()),
            104u8 => Ok(crate::isotopes::DubniumIsotope::arbitrary(u)?.into()),
            105u8 => Ok(crate::isotopes::SeaborgiumIsotope::arbitrary(u)?.into()),
            106u8 => Ok(crate::isotopes::BohriumIsotope::arbitrary(u)?.into()),
            107u8 => Ok(crate::isotopes::HassiumIsotope::arbitrary(u)?.into()),
            108u8 => Ok(crate::isotopes::MeitneriumIsotope::arbitrary(u)?.into()),
            109u8 => Ok(crate::isotopes::DarmstadtiumIsotope::arbitrary(u)?.into()),
            110u8 => Ok(crate::isotopes::RoentgeniumIsotope::arbitrary(u)?.into()),
            111u8 => Ok(crate::isotopes::CoperniciumIsotope::arbitrary(u)?.into()),
            112u8 => Ok(crate::isotopes::NihoniumIsotope::arbitrary(u)?.into()),
            113u8 => Ok(crate::isotopes::FleroviumIsotope::arbitrary(u)?.into()),
            114u8 => Ok(crate::isotopes::MoscoviumIsotope::arbitrary(u)?.into()),
            115u8 => Ok(crate::isotopes::LivermoriumIsotope::arbitrary(u)?.into()),
            116u8 => Ok(crate::isotopes::TennessineIsotope::arbitrary(u)?.into()),
            117u8 => Ok(crate::isotopes::OganessonIsotope::arbitrary(u)?.into()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use arbitrary::{Arbitrary, Unstructured};
    use rand::RngCore;

    #[test]
    fn test_arbitrary_isotope() {
        let mut rng = rand::rng();
        let mut buffer = [0u8; 1024];

        // Loop for a reasonable amount of time to cover possibilities
        for _ in 0..10_000 {
            rng.fill_bytes(&mut buffer);
            let mut u = Unstructured::new(&buffer);

            // We iterate until we run out of bytes or generating one fails (which shouldn't
            // happen often if buffer is large enough)
            let mut last_len = u.len();
            while let Ok(isotope) = crate::isotopes::Isotope::arbitrary(&mut u) {
                // Just ensuring it doesn't panic and produces a valid value
                let _ = isotope;
                let current_len = u.len();
                if current_len == last_len {
                    // Break to avoid infinite loop where arbitrary consumes 0 bytes
                    break;
                }
                last_len = current_len;
            }
        }
    }
}
