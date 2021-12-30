use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct PostgreSQL;

impl Store for PostgreSQL {
    fn name(&self) -> String {
        "oomplay-postgres".to_string()
    }

    fn image(&self) -> String {
        "postgres:14.0-alpine".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec![
            "POSTGRES_PASSWORD=postgres",
            "POSTGRES_USER=postgres",
            "PGUSER=postgres",
        ]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(5432, 25432)]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                psql -c 'DROP DATABASE oomplay';
                psql -c 'CREATE DATABASE oomplay';
                psql -tc '\du oomplay' | grep oomplay && exit
                psql -c "CREATE ROLE oomplay WITH LOGIN SUPERUSER PASSWORD 'oomplay'";
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["pg_isready"]
    }
}
