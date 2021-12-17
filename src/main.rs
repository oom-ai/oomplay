mod cli;
mod config;
mod util;

use anyhow::Result;
use clap::Parser;
use cli::App;
use config::Config;
use std::io;

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
    match App::parse() {
        App::Start { config } => {
            let config: Config = config.try_into()?;
            println!("{:#?}", config);
        }
        App::Stop { config } => todo!(),
        App::Restart { config } => todo!(),
        App::Reset { config } => todo!(),
        App::Apply { config } => todo!(),
    }
    Ok(())
}
