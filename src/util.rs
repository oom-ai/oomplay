use crate::{backend::*, cli::Database, store::Store};
use itertools::Itertools;

type StoreRef<'a> = &'a (dyn Store + Sync);

pub fn unique_stores(databases: &[Database]) -> impl Iterator<Item = StoreRef> {
    databases.iter().unique().map(|backend| match backend {
        Database::Redis => &Redis as StoreRef,
        Database::Postgres => &Postgres as StoreRef,
        Database::Mysql => &Mysql as StoreRef,
        Database::Dynamodb => &DynamoDB as StoreRef,
        Database::Cassandra => &Cassandra as StoreRef,
    })
}
