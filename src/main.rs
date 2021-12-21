#![feature(duration_constants)]
#![feature(async_closure)]

mod backend;
mod cli;
mod config;
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

use crate::config::{Backend, ConfigMap};

#[tokio::main]
async fn main() {
    colog::init();

    if let Err(e) = try_main().await {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        error!("{}", e);
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    match App::parse() {
        App::Init { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                info!("init {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker.init(&Postgres { port, user, password, database }).await?;
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker.init(&Mysql { port, user, password, database }).await?;
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker.init(&Redis { port, password, database }).await?;
                    }
                }
            }
        }
        App::Stop { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                info!("stop {name}...");
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
        }
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
