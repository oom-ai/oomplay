use std::{fs::File, io::BufReader};

use crate::{cli::ConfigOpt, config::ConfigMap};
use anyhow::Error;

impl TryFrom<ConfigOpt> for ConfigMap {
    type Error = Error;

    fn try_from(opt: ConfigOpt) -> Result<Self, Self::Error> {
        let file = File::open(opt.config)?;
        Ok(serde_yaml::from_reader(BufReader::new(file))?)
    }
}
