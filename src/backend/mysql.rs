use crate::{docker::PortMap, store::Store, svec};

pub struct MySQL;

impl Store for MySQL {
    fn name(&self) -> String {
        "mysql".to_string()
    }

    fn image(&self) -> String {
        "mysql:8.0".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec!["MYSQL_ALLOW_EMPTY_PASSWORD=yes"]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(3306, 23306)]
    }

    fn reset_cmd(&self) -> Vec<String> {
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
        svec!["mysqladmin", "ping"]
    }
}
