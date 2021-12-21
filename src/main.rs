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
    macro_rules! docker_start {
        ($store:expr) => {
            let store = &$store;
            docker.run(store).await?;
            docker.wait_ready(store).await?;
        };
    }
    macro_rules! docker_init {
        ($store:expr) => {
            let store = &$store;
            docker.init(store).await?;
        };
    }
    macro_rules! docker_stop {
        ($store:expr) => {
            let store = &$store;
            docker.stop(store).await?;
        };
    }
    match App::parse() {
        App::Start { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                info!("start {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker_start!(Postgres { port, user, password, database });
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker_start!(Mysql { port, user, password, database });
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker_start!(Redis { port, password, database });
                    }
                }
            }
        }
        App::Init { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (name, backend) in config_map.into_iter() {
                info!("init {name}...");
                match backend {
                    Backend::Postgres { port, user, password, database, .. } => {
                        docker_init!(Postgres { port, user, password, database });
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker_init!(Mysql { port, user, password, database });
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker_init!(Redis { port, password, database });
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
                        docker_stop!(Postgres { port, user, password, database });
                    }
                    Backend::Mysql { port, user, password, database, .. } => {
                        docker_stop!(Mysql { port, user, password, database });
                    }
                    Backend::Redis { port, password, database, .. } => {
                        docker_stop!(Redis { port, password, database });
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
