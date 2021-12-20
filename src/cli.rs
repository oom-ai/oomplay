use std::path::PathBuf;

use clap::{Args, Parser};
use clap_generate::Shell;

#[derive(Parser)]
#[clap(about, version)]
pub enum App {
    /// Start playgrounds
    #[clap(display_order = 1)]
    Start {
        #[clap(flatten)]
        config: ConfigOpt,
    },

    /// Stop playgrounds
    #[clap(display_order = 2)]
    Stop {
        #[clap(flatten)]
        config: ConfigOpt,
    },

    /// Reset playgrounds
    #[clap(display_order = 3)]
    Reset {
        #[clap(flatten)]
        config: ConfigOpt,
    },

    /// Start or reset playgrounds
    #[clap(display_order = 4)]
    Init {
        #[clap(flatten)]
        config: ConfigOpt,
    },

    /// Output shell completion code
    #[clap(display_order = 100)]
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Args)]
pub struct ConfigOpt {
    /// Config file path
    #[clap(short, long, display_order = 0)]
    pub config: PathBuf,
}
