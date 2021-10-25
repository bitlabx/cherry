use sqlx::{Any, Arguments as SqlxArguments, Database, Mssql, MySql, Postgres, Sqlite, Type};
use sqlx::database::HasArguments;
use sqlx::encode::Encode;

pub enum WrapArguments<'a> {
    MySqlArguments(MySqlArguments<'a>),
    PgArguments(PgArguments<'a>),
    SqliteArguments(SqliteArguments<'a>),
    MssqlArguments(MssqlArguments<'a>),
}

pub trait Arguments<'q>: Send {
    type Database: Database;
    fn new() -> Self;
    fn from<T>(value: T) -> Self
        where T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>;
    fn add<T>(&mut self, value: T) -> &mut Self
        where T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>;
}

pub struct MySqlArguments<'q> {
    pub(crate) inner: <MySql as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}
pub struct PgArguments<'q> {
    pub(crate) inner: <Postgres as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}
pub struct SqliteArguments<'q> {
    pub(crate) inner: <Sqlite as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}
pub struct MssqlArguments<'q> {
    pub(crate) inner: <Mssql as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}
pub struct AnyArguments<'q> {
    pub(crate) inner: <Any as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}


// #[macro_export]
macro_rules! impl_arguments {
    ($d:ty, $a:ty) => {
        // pub struct $a {
        //     pub(crate) inner: <$d as HasArguments<'q>>::Arguments,
        //     pub(crate) count: usize,
        // }

        impl<'q> Arguments<'q> for $a {
            type Database = $d;

            fn new() -> Self {
                Self {
                    inner: <Self::Database as HasArguments<'q>>::Arguments::default(),
                    count: 0,
                }
            }

            fn from<T>(value: T) -> Self
                where T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database> {
                let mut arg = Self::new();
                arg.add(value);
                arg
            }

            fn add<T>(&mut self, value: T) -> &mut Self
                where T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database> {
                self.inner.add(value);
                self.count += 1;
                self
            }
        }
    }
}

impl_arguments!(MySql, MySqlArguments<'q>);
impl_arguments!(Postgres, PgArguments<'q>);
impl_arguments!(Sqlite, SqliteArguments<'q>);
impl_arguments!(Mssql, MssqlArguments<'q>);
impl_arguments!(Any, AnyArguments<'q>);