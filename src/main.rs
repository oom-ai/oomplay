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
use std::io;

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
    match App::parse() {
        App::Init { backends } =>
            for store in backends.store_iter() {
                info!("🎮 Initializing {} ...", store.name().blue().bold());
                docker.init(store).await?;
                info!("🟢 {}", "Store is ready".bold());
            },
        App::Stop { backends } =>
            for store in backends.store_iter() {
                info!("🔌 Stopping {} ...", store.name().blue().bold());
                docker.stop(store).await?;
                info!("🔴 {}", "Stopped".bold());
            },
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
