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
    /// Databases
    #[clap(possible_values = Database::VARIANTS, required = true)]
    pub database: Vec<Database>,
}

#[derive(Debug, Display, EnumString, EnumVariantNames, Parser, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Database {
    Redis,
    Postgres,
    Mysql,
    Dynamodb,
    Cassandra,
}
