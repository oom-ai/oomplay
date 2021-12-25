use crate::{
    backend::*,
    cli::{BackendOpt, Database},
    store::Store,
};
use itertools::Itertools;

type StoreRef<'a> = &'a (dyn Store + Sync);

impl BackendOpt {
    pub fn store_iter(&self) -> impl Iterator<Item = StoreRef> {
        self.database.iter().unique().map(|backend| match backend {
            Database::Redis => &Redis as StoreRef,
            Database::Postgres => &Postgres as StoreRef,
            Database::Mysql => &Mysql as StoreRef,
            Database::Dynamodb => &DynamoDB as StoreRef,
            Database::Cassandra => &Cassandra as StoreRef,
        })
    }
}
