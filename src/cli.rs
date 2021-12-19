use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser)]
#[clap(about, version)]
pub enum App {
    /// Start playground
    Start {
        #[clap(flatten)]
        config: ConfigOpt,
    },
    /// Stop playground
    Stop {
        #[clap(flatten)]
        config: ConfigOpt,
    },
    /// Reset playground
    Reset {
        #[clap(flatten)]
        config: ConfigOpt,
    },
    /// Start or reset playground
    Init {
        #[clap(flatten)]
        config: ConfigOpt,
    },
}

#[derive(Debug, Args)]
pub struct ConfigOpt {
    /// config file path
    #[clap(short, long, display_order = 0)]
    pub config: PathBuf,
}
