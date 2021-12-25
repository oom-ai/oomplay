use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Mysql;

impl Store for Mysql {
    fn name(&self) -> String {
        "oomplay-mysql".to_string()
    }

    fn image(&self) -> String {
        "mysql:8.0".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec!["MYSQL_ALLOW_EMPTY_PASSWORD=yes"]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(23306, 3306)]
    }

    fn drop_cmd(&self) -> Vec<String> {
        svec!["mysql", "-e", "DROP DATABASE IF EXISTS oomplay"]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec![
            "mysql",
            "-e",
            r#"
                CREATE USER IF NOT EXISTS 'oomplay'@'%' IDENTIFIED BY 'oomplay';
                GRANT ALL PRIVILEGES ON *.* TO 'oomplay'@'%' WITH GRANT OPTION;
                DROP DATABASE IF EXISTS oomplay;
                CREATE DATABASE oomplay;
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        // `init_cmd` may fail even `ping` or `select 1` succeeded,
        svec!["mysql", "-e", "show databases"]
    }
}
