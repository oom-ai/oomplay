#![feature(duration_constants)]
#![feature(async_closure)]

mod backend;
mod cli;
mod docker;
mod store;
mod util;

#[macro_use]
extern crate log;
use anyhow::{Error, Result};
use bollard::Docker;
use clap::{IntoApp, Parser};
use cli::App;
use colored::Colorize;
use docker::StoreRuntime;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use tokio::time::Instant;

use std::io;
use strum::VariantNames;
use util::unique_stores;

use crate::util::with_flock;

#[tokio::main]
async fn main() {
    colog::init();

    if let Err(e) = try_main().await {
        error!("❗ {}", e.to_string().trim());
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread();
    let docker = &Docker::connect_with_local_defaults()?;
    match App::parse() {
        App::Init { playground, jobs } => {
            futures::stream::iter(unique_stores(&playground).into_iter().map(async move |store| {
                info!("🎮 Initializing {} ...", store.name().blue().bold());
                let now = Instant::now();
                with_flock(&store.name(), async move || docker.init(store).await).await?;
                info!("🟢 {} is ready. ({:?})", store.name().bold(), now.elapsed());
                Ok::<_, Error>(())
            }))
            .buffer_unordered(jobs)
            .try_collect::<Vec<_>>()
            .await?;
        }
        App::Stop { playground, jobs } => {
            futures::stream::iter(unique_stores(&playground).into_iter().map(async move |store| {
                info!("🔌 Stopping {} ...", store.name().blue().bold());
                with_flock(&store.name(), async move || docker.stop(store).await).await?;
                info!("🔴 {} stopped.", store.name().bold());
                Ok::<_, Error>(())
            }))
            .buffer_unordered(jobs)
            .try_collect::<Vec<_>>()
            .await?;
        }
        App::List => cli::Playground::VARIANTS
            .iter()
            .try_for_each(|db| writeln!(io::stdout(), "{}", db))?,
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
