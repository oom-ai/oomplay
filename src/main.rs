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
        App::Init { backends } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🎮 Initializing playground '{name}' ...");
                match backend {
                    Backend::Postgres => {
                        docker.init(&Postgres).await?;
                    }
                    Backend::Mysql => {
                        docker.init(&Mysql).await?;
                    }
                    Backend::Redis => {
                        docker.init(&Redis).await?;
                    }
                    Backend::Dynamodb { port } => {
                        docker.init(&DynamoDB { port }).await?;
                    }
                    Backend::Cassandra { port, keyspace, .. } => {
                        docker.init(&Cassandra { port, keyspace }).await?;
                    }
                }
                info!("✨ Initialized playground '{name}'");
            }
        }
        App::Clear { backends } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🧹 Cleaning up playground '{name}' ...");
                match backend {
                    Backend::Postgres => {
                        docker.destory(&Postgres).await?;
                    }
                    Backend::Mysql => {
                        docker.destory(&Mysql).await?;
                    }
                    Backend::Redis => {
                        docker.destory(&Redis).await?;
                    }
                    Backend::Dynamodb { port } => {
                        docker.destory(&DynamoDB { port }).await?;
                    }
                    Backend::Cassandra { port, keyspace, .. } => {
                        docker.destory(&Cassandra { port, keyspace }).await?;
                    }
                }
                info!("✨ Cleaned up playground '{name}'");
            }
        }
        App::Stop { backends } => {
            let backends: BackendMap = backends.try_into()?;
            for (name, backend) in backends.into_iter() {
                info!("🔌 Stopping playground '{name}' ...");
                match backend {
                    Backend::Postgres => {
                        docker.stop(&Postgres).await?;
                    }
                    Backend::Mysql => {
                        docker.stop(&Mysql).await?;
                    }
                    Backend::Redis => {
                        docker.stop(&Redis).await?;
                    }
                    Backend::Dynamodb { port } => {
                        docker.stop(&DynamoDB { port }).await?;
                    }
                    Backend::Cassandra { port, keyspace, .. } => {
                        docker.stop(&Cassandra { port, keyspace }).await?;
                    }
                }
                info!("🛑 Stopped playground '{name}'");
            }
        }
        App::Completion { shell } => {
            let app = &mut App::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
