mod backend;
mod cli;
mod config;
mod util;

use anyhow::Result;
use backend::postgres::Postgres;
use bollard::Docker;
use clap::Parser;
use cli::App;
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
            for (_k, v) in config_map.into_iter() {
                match v {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        pg.run(&docker).await?
                    }
                    _ => todo!(),
                }
            }
        }
        App::Stop { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (_k, v) in config_map.into_iter() {
                match v {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        pg.stop(&docker).await?
                    }
                    _ => todo!(),
                }
            }
        }
        App::Restart { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (_k, v) in config_map.into_iter() {
                match v {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        pg.restart(&docker).await?
                    }
                    _ => todo!(),
                }
            }
        }
        App::Reset { config } => {
            let config_map: ConfigMap = config.try_into()?;
            for (_k, v) in config_map.into_iter() {
                match v {
                    Backend::Postgres { port, user, password, database, .. } => {
                        let pg = Postgres { port, user, password, database };
                        pg.reset(&docker).await?
                    }
                    _ => todo!(),
                }
            }
        }
        _ => todo!(),
    }
    Ok(())
}
