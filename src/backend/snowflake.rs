use anyhow::Result;

use crate::{store::Store, svec, util::get_env};

pub struct Snowflake;

impl Store for Snowflake {
    fn name(&self) -> String {
        "snowflake-ext".to_string()
    }

    fn image(&self) -> String {
        "bbtv/snowsql:latest".to_string()
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec!["sleep", "infinity"])
    }

    fn envs(&self) -> Result<Vec<String>> {
        Ok(svec![
            format!("SNOWSQL_ACCOUNT={}", get_env("SNOWFLAKE_ACCOUNT")?),
            format!("SNOWSQL_USER={}", get_env("SNOWFLAKE_USER")?),
            format!("SNOWSQL_PWD={}", get_env("SNOWFLAKE_PASSWORD")?),
            format!("SNOWSQL_DATABASE={}", get_env("SNOWFLAKE_DATABASE")?),
        ])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                snowsql -a $SNOWSQL_ACCOUNT -u $SNOWSQL_USER -q '
                            DROP DATABASE IF EXISTS "$SNOWFLAKE_DATABASE";
                            CREATE DATABASE "$SNOWFLAKE_DATABASE";
                '
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "snowsql -a $SNOWSQL_ACCOUNT -u $SNOWSQL_USER -q 'SELECT 1'",]
    }
}
