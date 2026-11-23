//! Diesel text storage for elements and isotopes.

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, ToSql},
    sql_types::Text,
};

use crate::{Element, Isotope};

impl<Database: Backend> ToSql<Text, Database> for Element
where
    str: ToSql<Text, Database>,
{
    fn to_sql<'bind>(
        &'bind self,
        output: &mut serialize::Output<'bind, '_, Database>,
    ) -> serialize::Result {
        self.symbol().to_sql(output)
    }
}

/// Parses a symbol such as `C13`, `D`, or `T` from a text column.
fn parse_isotope(text: &str) -> deserialize::Result<Isotope> {
    match text {
        "D" => return Ok(Isotope::try_from('D')?),
        "T" => return Ok(Isotope::try_from('T')?),
        _ => {}
    }

    let mass_start = text
        .find(|character: char| character.is_ascii_digit())
        .ok_or("missing isotope mass number")?;
    let (symbol, mass) = text.split_at(mass_start);
    Ok(Isotope::try_from((symbol.parse::<Element>()?, mass.parse::<u16>()?))?)
}

#[cfg(feature = "diesel-sqlite")]
mod sqlite {
    use alloc::string::ToString;

    use diesel::{backend::Backend, sqlite::Sqlite};

    use super::{Element, FromSql, Isotope, Text, ToSql, deserialize, parse_isotope, serialize};

    impl FromSql<Text, Sqlite> for Element {
        fn from_sql(mut value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            Ok(value.read_text().parse()?)
        }
    }

    impl FromSql<Text, Sqlite> for Isotope {
        fn from_sql(mut value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            parse_isotope(value.read_text())
        }
    }

    impl ToSql<Text, Sqlite> for Isotope {
        fn to_sql<'bind>(
            &'bind self,
            output: &mut serialize::Output<'bind, '_, Sqlite>,
        ) -> serialize::Result {
            output.set_value(self.to_string());
            Ok(serialize::IsNull::No)
        }
    }
}

#[cfg(feature = "diesel-postgres")]
mod postgres {
    use std::io::Write;

    use diesel::{backend::Backend, pg::Pg};

    use super::{Element, FromSql, Isotope, Text, ToSql, deserialize, parse_isotope, serialize};

    impl FromSql<Text, Pg> for Element {
        fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            Ok(core::str::from_utf8(value.as_bytes())?.parse()?)
        }
    }

    impl FromSql<Text, Pg> for Isotope {
        fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            parse_isotope(core::str::from_utf8(value.as_bytes())?)
        }
    }

    impl ToSql<Text, Pg> for Isotope {
        fn to_sql<'bind>(
            &'bind self,
            output: &mut serialize::Output<'bind, '_, Pg>,
        ) -> serialize::Result {
            write!(output, "{self}")?;
            Ok(serialize::IsNull::No)
        }
    }
}
