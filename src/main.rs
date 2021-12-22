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
use backend::*;
use bollard::Docker;
use clap::{IntoApp, Parser};
use cli::App;
use docker::StoreRuntime;
use std::io;

use crate::cli::{Backend, BackendMap};

#[tokio::main]
async fn main() {
    colog::init();

    if let Err(e) = try_main().await {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        error!("❗ {}", e.to_string().trim());
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    match App::parse() {
        App::Start { backends } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🎮 Starting playground '{name}' ...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker.run(&Postgres { port, user, password, database }).await?;
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker.run(&Mysql { port, user, password, database }).await?;
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker.run(&Redis { port, password, database }).await?;
                    }
                }
            }
            info!("✨ All playgrounds initialized successfully!");
        }
        App::Clear { backends, recreate } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🧹 Clearing playground '{name}' ...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker
                            .clear(&Postgres { port, user, password, database }, recreate)
                            .await?;
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker
                            .clear(&Mysql { port, user, password, database }, recreate)
                            .await?;
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker.clear(&Redis { port, password, database }, recreate).await?;
                    }
                }
            }
            info!("✨ All playgrounds cleared.");
        }
        App::Stop { backends } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🔌 Stopping playground '{name}' ...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker.stop(&Postgres { port, user, password, database }).await?;
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker.stop(&Mysql { port, user, password, database }).await?;
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker.stop(&Redis { port, password, database }).await?;
                    }
                }
            }
            info!("🛑 All playgrounds stopped.");
        }
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
