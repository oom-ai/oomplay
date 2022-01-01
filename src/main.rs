#![feature(duration_constants)]

mod backend;
mod cli;
mod docker;
mod store;
mod util;

#[macro_use]
extern crate log;
use anyhow::Result;
use bollard::Docker;
use clap::{IntoApp, Parser};
use cli::App;
use docker::StoreRuntime;
use futures::{stream, StreamExt, TryStreamExt};
use std::{io, io::Write};
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
    let docker = &Docker::connect_with_local_defaults()?;
    match App::parse() {
        App::Init { playground, jobs } => {
            stream::iter(unique_stores(&playground))
                .map(|store| with_flock(store.name(), || docker.init(store)))
                .buffer_unordered(jobs)
                .try_collect::<Vec<_>>()
                .await?;
        }
        App::Stop { playground, jobs } => {
            stream::iter(unique_stores(&playground))
                .map(|store| with_flock(store.name(), || docker.stop(store)))
                .buffer_unordered(jobs)
                .try_collect::<Vec<_>>()
                .await?;
        }
        App::List => cli::Playground::VARIANTS
            .iter()
            .try_for_each(|db| writeln!(io::stdout(), "{}", db))?,
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_complete::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
