use anyhow::Result;

use crate::{store::Store, svec, util::get_env};

pub struct Redshift;

impl Store for Redshift {
    fn name(&self) -> String {
        "redshift-ext".to_string()
    }

    fn image(&self) -> String {
        "postgres:14.0-alpine".to_string()
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec!["sleep", "infinity"])
    }

    fn envs(&self) -> Result<Vec<String>> {
        Ok(svec![
            format!("PGHOST={}", get_env("REDSHIFT_HOST")?),
            format!(
                "PGPORT={}",
                get_env("REDSHIFT_PORT").unwrap_or_else(|_| "5439".to_string())
            ),
            format!("PGUSER={}", get_env("REDSHIFT_USER")?),
            format!("PGPASSWORD={}", get_env("REDSHIFT_PASSWORD")?),
            format!("PGDATABASE={}", get_env("REDSHIFT_DEFAULT_DATABASE")?),
            format!("PLAYDB={}", get_env("REDSHIFT_DATABASE")?),
        ])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "sh",
            "-c",
            r#"
                psql \
                    -c "DROP DATABASE \"$PLAYDB\"" \
                    -c "CREATE DATABASE \"$PLAYDB\""
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["pg_isready"]
    }
}
