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
        "oomplay-mysql".to_string()
    }

    fn image(&self) -> String {
        "mysql:8.0".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec![format!("MYSQL_ROOT_PASSWORD={}", self.root_password())]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(3306, self.port)]
    }

    fn drop_cmd(&self) -> Vec<String> {
        svec![
            "mysql",
            format!("-p{}", self.root_password()),
            "-e",
            format!("DROP DATABASE IF EXISTS `{}`", self.database)
        ]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "mysql",
            format!("-p{}", self.root_password()),
            "-e",
            format!(
                r#"
                    CREATE USER IF NOT EXISTS '{user}'@'%' IDENTIFIED BY '{password}';
                    GRANT ALL PRIVILEGES ON *.* TO '{user}'@'%' WITH GRANT OPTION;
                    DROP DATABASE IF EXISTS {database};
                    CREATE DATABASE {database};
                "#,
                user = self.user,
                password = self.password,
                database = self.database,
            ),
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        // `init_cmd` may fail even `ping` or `select 1` succeeded,
        svec!["mysql", format!("-p{}", self.root_password()), "-e", "show databases"]
    }
}
