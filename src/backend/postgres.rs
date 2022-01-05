use crate::{docker::PortBinding, store::Store, svec};

pub struct Postgres;

impl Store for Postgres {
    fn name(&self) -> String {
        "postgres".to_string()
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

    fn port_map(&self) -> Vec<PortBinding> {
        vec![(5432, 25432)]
    }

    fn reset_cmd(&self) -> Vec<String> {
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
