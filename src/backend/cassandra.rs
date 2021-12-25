use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Cassandra;

impl Store for Cassandra {
    fn name(&self) -> String {
        "oomplay-cassandra".to_string()
    }

    fn image(&self) -> String {
        "cassandra:4.0".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(29042, 9042)]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "cqlsh",
            "-e",
            r#"
                DROP KEYSPACE IF EXISTS oomplay;
                CREATE KEYSPACE IF NOT EXISTS oomplay WITH replication = {
                    'class': 'SimpleStrategy',
                    'replication_factor': 1
                }
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["cqlsh", "-e", "describe keyspaces"]
    }
}
