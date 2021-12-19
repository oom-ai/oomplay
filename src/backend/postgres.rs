use crate::store::{PortMap, Store};

pub struct Postgres {
    pub port:     u16,
    pub user:     String,
    pub password: String,
    pub database: String,
}

impl Store for Postgres {
    fn container_name(&self) -> String {
        "oomstore-playground-postgres".to_string()
    }

    fn image(&self) -> String {
        "postgres:14.0-alpine".to_string()
    }

    fn envs(&self) -> Vec<String> {
        vec![
            format!("POSTGRES_PASSWORD={}", self.password),
            format!("POSTGRES_USER={}", self.user),
            format!("PGUSER={}", self.user),
        ]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(self.port, self.port)]
    }

    fn reset_cmd(&self) -> Vec<String> {
        vec![
            "psql".into(),
            "-c".into(),
            format!("DROP DATABASE IF EXISTS {}", self.database),
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        vec!["pg_isready".into()]
    }
}
