use itertools::Itertools;

use crate::{
    store::{PortMap, Store},
    svec,
};

pub enum TiKV {
    External,
    Internal,
}

impl Store for TiKV {
    fn name(&self) -> String {
        match self {
            TiKV::External => "oomplay-tikv-ext".to_string(),
            TiKV::Internal => "oomplay-tikv".to_string(),
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
        let host = match self {
            TiKV::External => "127.0.0.1",
            TiKV::Internal => "{socket.gethostname()}",
        };
        svec![
            "python3",
            "-c",
            format!(
                r#"
                    import socket; from tikv_client import RawClient;
                    RawClient.connect([f'{host}:2379']).delete_range('')
                "#
            )
            .lines()
            .map(|l| l.trim())
            .join("\n")
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        let host = match self {
            TiKV::External => "127.0.0.1",
            TiKV::Internal => "{socket.gethostname()}",
        };
        svec![
            "python3",
            "-c",
            format!(
                r#"
                    import socket; from tikv_client import RawClient;
                    RawClient.connect([f'{host}:2379']).get('')
                "#
            )
            .lines()
            .map(|l| l.trim())
            .join("\n")
        ]
    }
}
