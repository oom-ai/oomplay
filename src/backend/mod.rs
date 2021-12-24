pub mod cassandra;
pub mod dynamodb;
pub mod mysql;
pub mod postgres;
pub mod redis;

pub use cassandra::Cassandra;
pub use dynamodb::DynamoDB;
pub use mysql::Mysql;
pub use postgres::Postgres;
pub use redis::Redis;

#[macro_export]
macro_rules! svec {
    ($($part:expr),* $(,)?) => {{
        vec![$($part.to_string(),)*]
    }};
}
