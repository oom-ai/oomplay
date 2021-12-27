mod cassandra;
mod dynamodb;
mod mysql;
mod postgres;
mod redis;
mod tidb;

pub use cassandra::Cassandra;
pub use dynamodb::DynamoDB;
pub use mysql::Mysql;
pub use postgres::Postgres;
pub use redis::Redis;
pub use tidb::Tidb;

#[macro_export]
macro_rules! svec {
    ($($part:expr),* $(,)?) => {{
        vec![$($part.to_string(),)*]
    }};
}
