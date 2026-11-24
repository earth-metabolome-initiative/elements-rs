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

#[cfg(any(feature = "diesel-postgres", feature = "diesel-mariadb", feature = "diesel-mysql"))]
macro_rules! impl_raw_text {
    ($backend:ty) => {
        impl FromSql<Text, $backend> for Element {
            fn from_sql(value: <$backend as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                Ok(core::str::from_utf8(value.as_bytes())?.parse()?)
            }
        }

        impl FromSql<Text, $backend> for Isotope {
            fn from_sql(value: <$backend as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
                Ok(core::str::from_utf8(value.as_bytes())?.parse()?)
            }
        }

        impl ToSql<Text, $backend> for Isotope {
            fn to_sql<'bind>(
                &'bind self,
                output: &mut serialize::Output<'bind, '_, $backend>,
            ) -> serialize::Result {
                use std::io::Write;

                write!(output, "{self}")?;
                Ok(serialize::IsNull::No)
            }
        }
    };
}

#[cfg(feature = "diesel-postgres")]
impl_raw_text!(diesel::pg::Pg);
#[cfg(feature = "diesel-mariadb")]
impl_raw_text!(diesel::mariadb::Mariadb);
#[cfg(feature = "diesel-mysql")]
impl_raw_text!(diesel::mysql::Mysql);

#[cfg(feature = "diesel-sqlite")]
mod sqlite {
    use alloc::string::ToString;

    use diesel::sqlite::Sqlite;

    use super::{Backend, Element, FromSql, Isotope, Text, ToSql, deserialize, serialize};

    impl FromSql<Text, Sqlite> for Element {
        fn from_sql(mut value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            Ok(value.read_text().parse()?)
        }
    }

    impl FromSql<Text, Sqlite> for Isotope {
        fn from_sql(mut value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
            Ok(value.read_text().parse()?)
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
