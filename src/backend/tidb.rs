use crate::{docker::PortBinding, store::Store, svec};
use std::env;

pub enum TiDB {
    External,
    Internal,
}

impl Store for TiDB {
    fn name(&self) -> String {
        match self {
            TiDB::External => "tidb-ext".to_string(),
            TiDB::Internal => "tidb".to_string(),
        }
    }

    fn image(&self) -> String {
        "oomai/tiup:5.3.0".to_string()
    }

    fn network(&self) -> String {
        match self {
            TiDB::External => "host".to_string(),
            TiDB::Internal => "bridge".to_string(),
        }
    }

    fn port_map(&self) -> Vec<PortBinding> {
        match self {
            TiDB::External => vec![],
            TiDB::Internal => vec![(4000, 24000)],
        }
    }

    fn envs(&self) -> Vec<String> {
        match self {
            TiDB::External => {
                let host = env::var("TIDB_HOST").unwrap_or_else(|_| "127.0.0.1".into());
                svec![format!("TIDB_HOST={host}")]
            }
            TiDB::Internal => svec![],
        }
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        match self {
            TiDB::External => Some(svec!["sleep", "infinity"]),
            TiDB::Internal => Some(svec![
                "tiup",
                "playground",
                "--tag=oomplay",
                "--host=0.0.0.0",
                "--without-monitor",
                "--mode=tidb",
                "--tiflash=0",
                "--ticdc=0",
            ]),
        }
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                mysql -h ${TIDB_HOST:-$HOSTNAME} -P 4000 -e "
                    CREATE USER IF NOT EXISTS 'oomplay'@'%' IDENTIFIED BY 'oomplay';
                    GRANT ALL PRIVILEGES ON *.* TO 'oomplay'@'%' WITH GRANT OPTION;
                    DROP DATABASE IF EXISTS oomplay;
                    CREATE DATABASE oomplay;
                "
            "#
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "mysqladmin -h ${TIDB_HOST:-$HOSTNAME} -P 4000 ping"]
    }
}
