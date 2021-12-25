mod cassandra;
mod dynamodb;
mod mysql;
mod postgres;
mod redis;

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
