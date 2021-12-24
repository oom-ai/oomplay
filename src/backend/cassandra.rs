use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Cassandra {
    pub port:     u16,
    pub keyspace: String,
}

impl Store for Cassandra {
    fn name(&self) -> String {
        "oomplay-cassandra".to_string()
    }

    fn image(&self) -> String {
        "cassandra:4.0".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(9042, self.port)]
    }

    fn drop_cmd(&self) -> Vec<String> {
        svec!["cqlsh", "-e", format!("DROP KEYSPACE IF EXISTS {}", self.keyspace)]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "cqlsh",
            "-e",
            format!(
                r#"
                    DROP KEYSPACE IF EXISTS {keyspace};
                    CREATE KEYSPACE IF NOT EXISTS {keyspace} WITH replication = {{
                        'class' : 'SimpleStrategy',
                        'replication_factor' : 1
                    }}
                "#,
                keyspace = self.keyspace,
            )
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["cqlsh", "-e", "describe keyspaces"]
    }
}
