use std::{collections::HashMap, path::PathBuf};

use clap::{AppSettings, Args, Parser};
use clap_generate::Shell;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub enum App {
    /// Initialize playgrounds
    Init {
        #[clap(flatten)]
        backends: BackendOpt,
    },

    /// Stop playgrounds
    Stop {
        #[clap(flatten)]
        backends: BackendOpt,
    },

    /// Clear playgrounds
    Clear {
        #[clap(flatten)]
        backends: BackendOpt,

        /// Drop database
        #[clap(short, long, global = true)]
        recreate: bool,
    },

    /// Output shell completion code
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Args)]
pub struct BackendOpt {
    /// file path containing backends
    #[clap(short, long, global = true)]
    pub file: Option<PathBuf>,

    /// Backend specified by cli
    #[clap(subcommand)]
    pub cli: Option<Backend>,
}

#[derive(Debug, Serialize, Deserialize, Display, Parser)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Backend {
    /// Redis store
    Redis {
        #[clap(short = 'P', long, default_value = "6379")]
        port: u16,

        #[clap(short, long, default_value = "redis")]
        password: String,

        #[clap(short, long, default_value = "0")]
        database: u32,
    },
    /// Postgres store
    Postgres {
        #[clap(short = 'P', long, default_value = "5432")]
        port: u16,

        #[clap(short, long, default_value = "postgres")]
        user: String,

        #[clap(short, long, default_value = "postgres")]
        password: String,

        #[clap(short, long, default_value = "oomplay")]
        database: String,
    },
    /// Mysql store
    Mysql {
        #[clap(short = 'P', long, default_value = "3306")]
        port: u16,

        #[clap(short, long, default_value = "mysql")]
        user: String,

        #[clap(short, long, default_value = "mysql")]
        password: String,

        #[clap(short, long, default_value = "oomplay")]
        database: String,
    },
}

pub type BackendMap = HashMap<String, Backend>;
