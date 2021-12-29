use itertools::Itertools;

use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct TiKV;

impl Store for TiKV {
    fn name(&self) -> String {
        "oomplay-tikv".to_string()
    }

    fn image(&self) -> String {
        "oomai/tiup:5.3.0".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(2379, 22379)]
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec![
            "tiup",
            "playground",
            "--tag=oomplay",
            "--host=0.0.0.0",
            "--without-monitor",
            "--mode=tikv-slim",
            "--tiflash=0",
            "--ticdc=0",
        ])
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "python3",
            "-c",
            r#"
                import socket; from tikv_client import RawClient;
                RawClient.connect([f'{socket.gethostname()}:2379']).delete_range('')
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
                import socket; from tikv_client import RawClient;
                RawClient.connect([f'{socket.gethostname()}:2379']).get('')
            "#
            .lines()
            .map(|l| l.trim())
            .join("\n")
        ]
    }
}
