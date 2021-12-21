use std::{fs::File, io::BufReader};

use crate::cli::{BackendMap, BackendOpt};
use anyhow::Error;

impl TryFrom<BackendOpt> for BackendMap {
    type Error = Error;

    fn try_from(opt: BackendOpt) -> Result<Self, Self::Error> {
        let mut map = match opt.file {
            Some(file) => serde_yaml::from_reader(BufReader::new(File::open(file)?))?,
            None => BackendMap::new(),
        };
        if let Some(backend) = opt.cli {
            map.insert(backend.to_string(), backend);
        }
        Ok(map)
    }
}
