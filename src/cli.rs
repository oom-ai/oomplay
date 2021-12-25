use clap::{AppSettings, Args, Parser};
use clap_generate::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

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
    /// Backends
    #[clap(possible_values = Backend::VARIANTS, required = true)]
    pub backends: Vec<Backend>,
}

#[derive(Debug, Display, EnumString, EnumVariantNames, Parser)]
#[strum(serialize_all = "snake_case")]
pub enum Backend {
    Redis,
    Postgres,
    Mysql,
    Dynamodb,
    Cassandra,
}
