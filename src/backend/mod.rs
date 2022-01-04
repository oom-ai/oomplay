mod cassandra;
mod dynamodb;
mod mysql;
mod postgresql;
mod redis;
mod tidb;
mod tikv;

pub use cassandra::Cassandra;
pub use dynamodb::DynamoDB;
pub use mysql::MySQL;
pub use postgresql::PostgreSQL;
pub use redis::Redis;
pub use tidb::TiDB;
pub use tikv::TiKV;

#[macro_export]
macro_rules! svec {
    ($($part:expr),* $(,)?) => {{
        vec![$($part.to_string(),)*]
    }};
}
