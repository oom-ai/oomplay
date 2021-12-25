use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Postgres;

impl Store for Postgres {
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
        vec![PortMap::Tcp(25432, 5432)]
    }

    fn drop_cmd(&self) -> Vec<String> {
        svec!["psql", "-c", "DROP DATABASE IF EXISTS oomplay"]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                createdb oomplay;
                dropdb oomplay;
                createdb oomplay;
                psql -tc '\du oomplay' | grep oomplay && exit
                psql -c "CREATE ROLE oomplay WITH LOGIN SUPERUSER PASSWORD 'oomplay'";
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        // `init_cmd` may fail even `pg_isready`succeeded
        svec!["psql", "-c", "SELECT 1"]
    }
}
