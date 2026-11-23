//! Diesel integration with an in-memory SQLite database.
#![cfg(feature = "diesel-sqlite")]

use diesel::{connection::SimpleConnection, prelude::*};
use elements_rs::{
    Element, Isotope,
    isotopes::{CarbonIsotope, HydrogenIsotope},
};

mod support;
use support::species;

#[test]
fn sqlite_round_trips_element_and_isotope_text() {
    let connection = &mut SqliteConnection::establish(":memory:").unwrap();
    connection
        .batch_execute(
            "CREATE TABLE species (
                id INTEGER PRIMARY KEY,
                element TEXT NOT NULL,
                isotope TEXT NOT NULL
            )",
        )
        .unwrap();

    diesel::insert_into(species::table)
        .values((
            species::element.eq(Element::Cl),
            species::isotope.eq(Isotope::from(CarbonIsotope::C13)),
        ))
        .execute(connection)
        .unwrap();
    let columns = species::table.select((species::element, species::isotope));
    assert_eq!(columns.first::<(String, String)>(connection).unwrap(), ("Cl".into(), "C13".into()));
    assert_eq!(
        columns.first::<(Element, Isotope)>(connection).unwrap(),
        (Element::Cl, CarbonIsotope::C13.into())
    );

    for (symbol, isotope) in [("D", HydrogenIsotope::D), ("T", HydrogenIsotope::T)] {
        diesel::update(species::table)
            .set((species::element.eq("H"), species::isotope.eq(symbol)))
            .execute(connection)
            .unwrap();
        assert_eq!(
            columns.first::<(Element, Isotope)>(connection).unwrap(),
            (Element::H, isotope.into())
        );
    }

    diesel::update(species::table).set(species::element.eq("Xx")).execute(connection).unwrap();
    assert!(matches!(
        species::table.select(species::element).first::<Element>(connection),
        Err(diesel::result::Error::DeserializationError(_))
    ));
    for malformed in ["C99", "C"] {
        diesel::update(species::table)
            .set(species::isotope.eq(malformed))
            .execute(connection)
            .unwrap();
        assert!(matches!(
            species::table.select(species::isotope).first::<Isotope>(connection),
            Err(diesel::result::Error::DeserializationError(_))
        ));
    }
}
