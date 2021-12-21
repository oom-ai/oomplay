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

impl Mysql {
    fn root_password(&self) -> &str {
        match self.user.as_str() {
            "root" => &self.password,
            _ => "mysql",
        }
    }
}

impl Store for Mysql {
    fn name(&self) -> String {
        "oomstore-playground-mysql".to_string()
    }

    fn image(&self) -> String {
        "mysql:8.0".to_string()
    }

    fn envs(&self) -> Vec<String> {
        match self.user.as_str() {
            "root" => svec![format!("MYSQL_ROOT_PASSWORD={}", self.root_password())],
            _ => svec![
                format!("MYSQL_ROOT_PASSWORD={}", self.root_password()),
                format!("MYSQL_USER={}", self.user),
                format!("MYSQL_PASSWORD={}", self.password),
            ],
        }
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(3306, self.port)]
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "mysql",
            format!("-p{}", self.root_password()),
            "-e",
            format!("DROP DATABASE IF EXISTS {}", self.database)
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        // sometimes reset_cmd fails even `ping` or `select 1` succeeded,
        // so we use `show databases` to make sure the server is actually ready.
        svec!["mysql", format!("-p{}", self.root_password()), "-e", "show databases"]
    }
}
