use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Mysql {
    pub port:     u16,
    pub user:     String,
    pub password: String,
    pub database: String,
}

impl Store for Mysql {
    fn name(&self) -> String {
        "oomstore-playground-mysql".to_string()
    }

    fn image(&self) -> String {
        "mysql:8.0".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec![
            format!("MYSQL_ALLOW_EMPTY_PASSWORD={}", "yes"),
            format!("MYSQL_USER={}", self.user),
            format!("MYSQL_PASSWORD={}", self.password),
        ]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(3306, self.port)]
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec!["mysql", "-e", format!("DROP DATABASE IF EXISTS {}", self.database)]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["mysqladmin", "ping", "-h", "localhost"]
    }
}
