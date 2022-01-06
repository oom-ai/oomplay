mod bigquery;
mod cassandra;
mod dynamodb;
mod mysql;
mod postgres;
mod redis;
mod snowflake;
mod sqlite;
mod tidb;
mod tikv;

pub use bigquery::BigQuery;
pub use cassandra::Cassandra;
pub use dynamodb::DynamoDB;
pub use mysql::MySQL;
pub use postgres::Postgres;
pub use redis::Redis;
pub use snowflake::Snowflake;
pub use sqlite::SQLite;
pub use tidb::TiDB;
pub use tikv::TiKV;

#[macro_export]
macro_rules! svec {
    ($($part:expr),* $(,)?) => {{
        vec![$($part.to_string(),)*]
    }};
}
