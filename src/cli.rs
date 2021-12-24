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

    /// Clean up playgrounds
    Clear {
        #[clap(flatten)]
        backends: BackendOpt,
    },

    /// Stop playgrounds
    Stop {
        #[clap(flatten)]
        backends: BackendOpt,
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
    /// Redis playground
    Redis {
        #[clap(short = 'P', long, default_value = "6379")]
        port: u16,

        #[clap(short, long, default_value = "redis")]
        password: String,

        #[clap(short, long, default_value = "oomplay")]
        database: u32,
    },
    /// Postgres playground
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
    /// MySQL playground
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
    /// DynamoDB playground
    Dynamodb {
        #[clap(short = 'P', long, default_value = "4566")]
        port: u16,
    },
    /// Cassandra
    Cassandra {
        #[clap(short = 'P', long, default_value = "9042")]
        port: u16,

        #[clap(short, long, default_value = "oomplay")]
        keyspace: String,
    },
}

pub type BackendMap = HashMap<String, Backend>;
