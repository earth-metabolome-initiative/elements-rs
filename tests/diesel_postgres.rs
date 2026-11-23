//! Diesel integration with a live PostgreSQL database.
#![cfg(feature = "diesel-postgres")]

use diesel::{connection::SimpleConnection, prelude::*};
use elements_rs::{Element, Isotope, isotopes::HydrogenIsotope};

mod support;
use support::species;

#[test]
#[ignore = "requires a live PostgreSQL server and DATABASE_URL"]
fn postgres_round_trips_element_and_isotope_text() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL for PostgreSQL");
    let connection = &mut PgConnection::establish(&database_url).unwrap();
    connection
        .batch_execute(
            "CREATE TEMPORARY TABLE species (
                id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                element TEXT NOT NULL,
                isotope TEXT NOT NULL
            )",
        )
        .unwrap();

    diesel::insert_into(species::table)
        .values((
            species::element.eq(Element::H),
            species::isotope.eq(Isotope::from(HydrogenIsotope::T)),
        ))
        .execute(connection)
        .unwrap();
    let columns = species::table.select((species::element, species::isotope));
    assert_eq!(columns.first::<(String, String)>(connection).unwrap(), ("H".into(), "T".into()));
    assert_eq!(
        columns.first::<(Element, Isotope)>(connection).unwrap(),
        (Element::H, HydrogenIsotope::T.into())
    );
}
