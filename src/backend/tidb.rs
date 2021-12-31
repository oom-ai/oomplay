use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct TiDB;

impl Store for TiDB {
    fn name(&self) -> String {
        "oomplay-tidb".to_string()
    }

    fn image(&self) -> String {
        "oomai/tiup:5.3.0".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(4000, 24000)]
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec![
            "tiup",
            "playground",
            "--tag=oomplay",
            "--host=0.0.0.0",
            "--without-monitor",
            "--mode=tidb",
            "--tiflash=0",
            "--ticdc=0",
        ])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                mysql -h $HOSTNAME -P 4000 -e "
                    CREATE USER IF NOT EXISTS 'oomplay'@'%' IDENTIFIED BY 'oomplay';
                    GRANT ALL PRIVILEGES ON *.* TO 'oomplay'@'%' WITH GRANT OPTION;
                    DROP DATABASE IF EXISTS oomplay;
                    CREATE DATABASE oomplay;
                "
            "#
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "mysqladmin -h $HOSTNAME -P 4000 ping"]
    }
}
