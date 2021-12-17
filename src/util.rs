use std::{fs::File, io::BufReader};

use crate::{cli::ConfigOpt, config::Config};
use anyhow::{bail, Error};

impl TryFrom<ConfigOpt> for Config {
    type Error = Error;

    fn try_from(opt: ConfigOpt) -> Result<Self, Self::Error> {
        let file = File::open(opt.config)?;
        Ok(serde_yaml::from_reader(BufReader::new(file))?)
    }
}
