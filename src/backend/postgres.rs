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
            format!("POSTGRES_PASSWORD={}", "postgres"),
            format!("POSTGRES_USER={}", "postgres"),
            format!("PGUSER={}", "postgres"),
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
            format!(
                r#"
                    createdb {user};
                    dropdb {database};
                    createdb {database};
                    psql -tc '\du {user}' | grep {user} && exit
                    psql -c "CREATE ROLE {user} WITH LOGIN SUPERUSER PASSWORD '{password}'";
                "#,
                user = "oomplay",
                password = "oomplay",
                database = "oomplay",
            ),
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        // `init_cmd` may fail even `pg_isready`succeeded
        svec!["psql", "-c", "SELECT 1"]
    }
}
