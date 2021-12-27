use anyhow::Result;
use colored::Colorize;
use fslock::LockFile;
use futures::Future;
use itertools::Itertools;
use std::env;

use crate::{backend::*, cli::Database, store::Store};
type StoreRef<'a> = &'a (dyn Store + Sync);

pub fn unique_stores(databases: &[Database]) -> impl Iterator<Item = StoreRef> {
    databases.iter().unique().map(|backend| match backend {
        Database::Redis => &Redis as StoreRef,
        Database::Postgres => &Postgres as StoreRef,
        Database::Mysql => &Mysql as StoreRef,
        Database::Dynamodb => &DynamoDB as StoreRef,
        Database::Cassandra => &Cassandra as StoreRef,
        Database::Tidb => &Tidb as StoreRef,
    })
}

pub async fn with_flock<F, Fut>(name: &str, f: F) -> Result<()>
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
