use anyhow::{anyhow, Result};
use colored::Colorize;
use fslock::LockFile;
use futures::Future;
use itertools::Itertools;
use std::{env, ffi::OsStr};

use crate::{backend::*, cli::Playground, store::Store};
type StoreRef<'a> = &'a (dyn Store + Sync);

#[rustfmt::skip]
pub fn unique_stores(playground: &[Playground]) -> impl Iterator<Item = StoreRef> {
    playground.iter().unique().map(|backend| match backend {
        Playground::Redis     => &Redis          as StoreRef,
        Playground::Postgres  => &Postgres       as StoreRef,
        Playground::MySQL     => &MySQL          as StoreRef,
        Playground::DynamoDB  => &DynamoDB       as StoreRef,
        Playground::Cassandra => &Cassandra      as StoreRef,
        Playground::TiDB      => &TiDB::Internal as StoreRef,
        Playground::TiDBExt   => &TiDB::External as StoreRef,
        Playground::TiKV      => &TiKV::Internal as StoreRef,
        Playground::TiKVExt   => &TiKV::External as StoreRef,
        Playground::SQLite    => &SQLite         as StoreRef,
        Playground::Snowflake => &Snowflake      as StoreRef,
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

pub fn get_env<K: AsRef<OsStr>>(key: K) -> Result<String> {
    env::var(key.as_ref()).map_err(|e| match e {
        env::VarError::NotPresent => anyhow!("environment variable {:?} not found", key.as_ref()),
        e => anyhow!(e),
    })
}
