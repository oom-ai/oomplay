#![feature(box_syntax)]
#![feature(duration_constants)]
#![feature(async_closure)]

mod backend;
mod cli;
mod docker;
mod store;
mod update;
mod util;

#[macro_use]
extern crate log;
use crate::update::update;
use anyhow::Result;
use bollard::Docker;
use clap::{IntoApp, Parser};
use cli::App;
use colored::Colorize;
use docker::StoreRuntime;
use self_update::Status;
use std::io;
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
        App::Init { database } =>
            for store in unique_stores(&database) {
                info!("🎮 Initializing {} ...", store.name().blue().bold());
                with_flock(&store.name(), async move || docker.init(store).await).await?;
                info!("🟢 {}", "Store is ready.".bold());
            },
        App::Stop { database } =>
            for store in unique_stores(&database) {
                info!("🔌 Stopping {} ...", store.name().blue().bold());
                with_flock(&store.name(), async move || docker.stop(store).await).await?;
                info!("🔴 {}", "Stopped.".bold());
            },
        App::Update => match update().await? {
            Status::Updated(v) => info!("🎉 Upgrade to version {v} successfully!"),
            Status::UpToDate(v) => info!("✨ The current version {v} is up to date."),
        },
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
