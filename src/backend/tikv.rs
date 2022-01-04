use itertools::Itertools;

use crate::{docker::PortMap, store::Store, svec};

pub enum TiKV {
    External,
    Internal,
}

impl Store for TiKV {
    fn name(&self) -> String {
        match self {
            TiKV::External => "tikv-ext".to_string(),
            TiKV::Internal => "tikv".to_string(),
        }
    }

    fn image(&self) -> String {
        "oomai/tiup:5.3.0".to_string()
    }

    fn network(&self) -> String {
        match self {
            TiKV::External => "host".to_string(),
            TiKV::Internal => "bridge".to_string(),
        }
    }

    fn port_map(&self) -> Vec<PortMap> {
        match self {
            TiKV::External => vec![],
            TiKV::Internal => vec![PortMap::Tcp(2379, 22379)],
        }
    }

    fn envs(&self) -> Vec<String> {
        match self {
            TiKV::External => svec!["TIKV_HOST=127.0.0.1"],
            TiKV::Internal => svec![],
        }
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        match self {
            TiKV::External => Some(svec!["sleep", "infinity"]),
            TiKV::Internal => Some(svec![
                "tiup",
                "playground",
                "--tag=oomplay",
                "--host=0.0.0.0",
                "--without-monitor",
                "--tiflash=0",
                "--ticdc=0",
                "--mode=tikv-slim",
            ]),
        }
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "python3",
            "-c",
            r#"
                import os; from tikv_client import RawClient;
                host = os.getenv('TIKV_HOST', os.uname().nodename)
                RawClient.connect([f'{host}:2379']).delete_range('')
            "#
            .lines()
            .map(|l| l.trim())
            .join("\n")
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec![
            "python3",
            "-c",
            r#"
                import os; from tikv_client import RawClient;
                host = os.getenv('TIKV_HOST', os.uname().nodename)
                RawClient.connect([f'{host}:2379']).get('')
            "#
            .lines()
            .map(|l| l.trim())
            .join("\n")
        ]
    }
}
