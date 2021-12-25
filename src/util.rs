use crate::{
    backend::*,
    cli::{Backend, BackendOpt},
    store::Store,
};

type StoreRef<'a> = &'a (dyn Store + Sync);

impl BackendOpt {
    pub fn store_iter(&self) -> impl Iterator<Item = StoreRef> {
        self.backends.iter().map(|backend| match backend {
            Backend::Redis => &Redis as StoreRef,
            Backend::Postgres => &Postgres as StoreRef,
            Backend::Mysql => &Mysql as StoreRef,
            Backend::Dynamodb => &DynamoDB as StoreRef,
            Backend::Cassandra => &Cassandra as StoreRef,
        })
    }
}
