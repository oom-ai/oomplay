use clap::{AppSettings, Parser};
use clap_generate::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub enum App {
    /// Initialize playgrounds
    Init {
        /// Databases
        #[clap(possible_values = Database::VARIANTS, required = true)]
        database: Vec<Database>,
    },

    /// Stop playgrounds
    Stop {
        /// Databases
        #[clap(possible_values = Database::VARIANTS, default_values = Database::VARIANTS, hide_default_value = true)]
        database: Vec<Database>,
    },

    /// Output shell completion code
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Display, EnumString, EnumVariantNames, Hash, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum Database {
    Redis,
    Postgres,
    Mysql,
    Dynamodb,
    Cassandra,
    Tidb,
}
