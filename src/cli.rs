use clap::{AppSettings, Parser};
use clap_complete::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub enum App {
    /// Initialize playgrounds
    Init {
        /// Playground type
        #[clap(possible_values = Playground::VARIANTS, required = true)]
        playground: Vec<Playground>,

        /// Number of parallel jobs, defaults to # of CPUs
        #[clap(short, long, default_value_t = num_cpus::get(), value_name = "N")]
        jobs: usize,
    },

    /// Stop playgrounds
    Stop {
        /// Playground type
        #[clap(possible_values = Playground::VARIANTS, default_values = Playground::VARIANTS, hide_default_value = true)]
        playground: Vec<Playground>,

        /// Number of parallel jobs, defaults to # of CPUs
        #[clap(short, long, default_value_t = num_cpus::get(), value_name = "N")]
        jobs: usize,
    },

    /// List supported playgrounds
    List,

    /// Output shell completion code
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Display, EnumString, EnumVariantNames, Hash, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum Playground {
    Redis,
    Postgres,
    Mysql,
    Dynamodb,
    Cassandra,
    Tidb,
    Tikv,
}
