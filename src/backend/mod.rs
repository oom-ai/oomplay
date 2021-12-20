pub mod mysql;
pub mod postgres;

pub use mysql::Mysql;
pub use postgres::Postgres;

#[macro_export]
macro_rules! svec {
    ($($part:expr),+ $(,)?) => {{
        vec![$($part.to_string(),)*]
    }};
}
