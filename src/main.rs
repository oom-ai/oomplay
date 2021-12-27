#![feature(box_syntax)]
#![feature(duration_constants)]
#![feature(async_closure)]

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
use colored::Colorize;
use docker::StoreRuntime;
use fslock::LockFile;
use std::{env, io};
use util::unique_stores;

#[tokio::main]
async fn main() {
    colog::init();

    if let Err(e) = try_main().await {
        error!("❗ {}", e.to_string().trim());
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let mut flock = env::temp_dir();
    flock.push("oomplay");
    let mut flock = LockFile::open(&flock)?;
    if !flock.try_lock()? {
        warn!("⌛ {}", "Waiting for another instance to finished ...".bold());
        flock.lock()?;
    }

    match App::parse() {
        App::Init { database } =>
            for store in unique_stores(&database) {
                info!("🎮 Initializing {} ...", store.name().blue().bold());
                docker.init(store).await?;
                info!("🟢 {}", "Store is ready.".bold());
            },
        App::Stop { database } =>
            for store in unique_stores(&database) {
                info!("🔌 Stopping {} ...", store.name().blue().bold());
                docker.stop(store).await?;
                info!("🔴 {}", "Stopped.".bold());
            },
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    flock.unlock()?;
    Ok(())
}
