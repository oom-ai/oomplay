use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Postgres {
    pub port:     u16,
    pub user:     String,
    pub password: String,
    pub database: String,
}

impl Store for Postgres {
    fn name(&self) -> String {
        "oomplay-postgres".to_string()
    }

    fn image(&self) -> String {
        "postgres:14.0-alpine".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec![
            format!("POSTGRES_PASSWORD={}", self.password),
            format!("POSTGRES_USER={}", self.user),
            format!("PGUSER={}", self.user),
        ]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(5432, self.port)]
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec!["psql", "-c", format!("DROP DATABASE IF EXISTS {}", self.database)]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["pg_isready"]
    }
}
