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
            format!("POSTGRES_PASSWORD={}", "postgres"),
            format!("POSTGRES_USER={}", "_root"),
            format!("PGUSER={}", "_root"),
        ]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(5432, self.port)]
    }

    fn drop_cmd(&self) -> Vec<String> {
        svec!["psql", "-c", format!(r#"DROP DATABASE IF EXISTS "{}""#, self.database)]
    }

    fn init_db_cmd(&self) -> Vec<String> {
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
                user = self.user,
                password = self.password,
                database = self.database,
            ),
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["pg_isready"]
    }
}
