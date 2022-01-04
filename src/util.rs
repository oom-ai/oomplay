use anyhow::Result;
use colored::Colorize;
use fslock::LockFile;
use futures::Future;
use itertools::Itertools;
use std::env;

use crate::{backend::*, cli::Playground, store::Store};
type StoreRef<'a> = &'a (dyn Store + Sync);

pub fn unique_stores(playground: &[Playground]) -> impl Iterator<Item = StoreRef> {
    playground.iter().unique().map(|backend| match backend {
        Playground::Redis => &Redis as StoreRef,
        Playground::PostgreSQL => &PostgreSQL as StoreRef,
        Playground::MySQL => &MySQL as StoreRef,
        Playground::DynamoDB => &DynamoDB as StoreRef,
        Playground::Cassandra => &Cassandra as StoreRef,
        Playground::TiDB => &TiDB::Internal as StoreRef,
        Playground::TiDBExt => &TiDB::External as StoreRef,
        Playground::TiKV => &TiKV::Internal as StoreRef,
        Playground::TiKVExt => &TiKV::External as StoreRef,
    })
}

pub async fn with_flock<F, Fut>(name: String, f: F) -> Result<()>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let mut flock = env::temp_dir();
    flock.push(format!("oomplay-{}", name));
    let mut flock = LockFile::open(&flock)?;
    if !flock.try_lock()? {
        warn!("⌛ {}", "Waiting for another instance to finished ...".bold());
        flock.lock()?;
    };
    f().await
}
