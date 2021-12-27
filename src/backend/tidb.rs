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
        vec![
            PortMap::Tcp(4000, 24000), // TiDB
            PortMap::Tcp(2379, 22379), // PD
            PortMap::Tcp(9090, 29090), // Prometheus
            PortMap::Tcp(3000, 23000), // Grafana
        ]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"mysql -h $(hostname -i) -P 4000 -e "
                CREATE USER IF NOT EXISTS 'oomplay'@'%' IDENTIFIED BY 'oomplay';
                GRANT ALL PRIVILEGES ON *.* TO 'oomplay'@'%' WITH GRANT OPTION;
                DROP DATABASE IF EXISTS oomplay;
            ""#
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "mysqladmin -h $(hostname -i) -P 4000 ping"]
    }
}
