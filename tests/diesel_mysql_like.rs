//! Diesel integration with live MariaDB and MySQL databases.
#![cfg(any(feature = "diesel-mariadb", feature = "diesel-mysql"))]

use diesel::{connection::SimpleConnection, prelude::*};
use elements_rs::{Element, Isotope, isotopes::CarbonIsotope};

mod support;
use support::species;

macro_rules! check_text_round_trip {
    ($connection:ty, $url_variable:literal) => {{
        let database_url = std::env::var($url_variable).expect(concat!("set ", $url_variable));
        let connection = &mut <$connection>::establish(&database_url).unwrap();
        connection
            .batch_execute(
                "CREATE TEMPORARY TABLE species (
                    id INTEGER PRIMARY KEY AUTO_INCREMENT,
                    element TEXT NOT NULL,
                    isotope TEXT NOT NULL
                )",
            )
            .unwrap();

        diesel::insert_into(species::table)
            .values((
                species::element.eq(Element::C),
                species::isotope.eq(Isotope::from(CarbonIsotope::C13)),
            ))
            .execute(connection)
            .unwrap();
        let columns = species::table.select((species::element, species::isotope));
        assert_eq!(
            columns.first::<(String, String)>(connection).unwrap(),
            ("C".into(), "C13".into())
        );
        assert_eq!(
            columns.first::<(Element, Isotope)>(connection).unwrap(),
            (Element::C, CarbonIsotope::C13.into())
        );

        diesel::update(species::table).set(species::isotope.eq("C99")).execute(connection).unwrap();
        assert!(matches!(
            species::table.select(species::isotope).first::<Isotope>(connection),
            Err(diesel::result::Error::DeserializationError(_))
        ));
    }};
}

#[cfg(feature = "diesel-mariadb")]
#[test]
#[ignore = "requires a live MariaDB server and MARIADB_DATABASE_URL"]
fn mariadb_round_trips_element_and_isotope_text() {
    check_text_round_trip!(diesel::mariadb::MariadbConnection, "MARIADB_DATABASE_URL");
}

#[cfg(feature = "diesel-mysql")]
#[test]
#[ignore = "requires a live MySQL server and MYSQL_DATABASE_URL"]
fn mysql_round_trips_element_and_isotope_text() {
    check_text_round_trip!(diesel::mysql::MysqlConnection, "MYSQL_DATABASE_URL");
}
