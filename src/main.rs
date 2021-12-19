#![feature(duration_constants)]

mod backend;
mod cli;
mod config;
mod docker;
mod store;
mod util;

use anyhow::Result;
use backend::postgres::Postgres;
use bollard::Docker;
use clap::Parser;
use cli::App;
use docker::StoreRuntime;
use std::io;

use crate::config::{Backend, ConfigMap};

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        std::process::exit(1)
    }
}

async fn try_main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    match App::parse() {
        App::Start { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                println!("start {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        docker.run(&pg).await?;
                        docker.wait_ready(&pg).await?;
                    }
                    _ => todo!(),
                }
            }
        }
        App::Stop { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                println!("stop {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        docker.stop(&pg).await?
                    }
                    _ => todo!(),
                }
            }
        }
        App::Reset { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                println!("reset {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        docker.reset(&pg).await?;
                    }
                    _ => todo!(),
                }
            }
        }
        App::Init { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                println!("init {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        docker.init(&pg).await?;
                    }
                    _ => todo!(),
                }
            }
        }
    }
    Ok(())
}
