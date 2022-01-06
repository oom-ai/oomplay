use anyhow::Result;
use itertools::Itertools;

use crate::{store::Store, svec, util::get_env};

pub struct BigQuery;

impl Store for BigQuery {
    fn name(&self) -> String {
        "bigquery".to_string()
    }

    fn image(&self) -> String {
        "demisto/bigquery:1.0.0.24037".to_string()
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec![
            "bash",
            "-c",
            r#"echo "$BIGQUERY_CREDENTIALS" > /etc/.credentials; exec sleep infinity"#
        ])
    }

    fn envs(&self) -> Result<Vec<String>> {
        Ok(svec![
            format!("BIGQUERY_CREDENTIALS={}", get_env("BIGQUERY_CREDENTIALS")?),
            format!("BIGQUERY_DATASET_ID={}", get_env("BIGQUERY_DATASET_ID")?),
            format!("GOOGLE_APPLICATION_CREDENTIALS={}", "/etc/.credentials"),
        ])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "python3",
            "-c",
            r#"
                from google.cloud import bigquery; import os;
                dataset_id = os.getenv('BIGQUERY_DATASET_ID')
                client = bigquery.Client()
                client.delete_dataset(dataset_id, delete_contents=True, not_found_ok=True)
                client.create_dataset(dataset_id)
            "#
            .lines()
            .map(|l| l.trim())
            .join("\n"),
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec![
            "python3",
            "-c",
            r#"
                from google.cloud import bigquery; import os;
                dataset_id = os.getenv('BIGQUERY_DATASET_ID');
                print(list(item.dataset_id for item in bigquery.Client().list_datasets()))
            "#
            .lines()
            .map(|l| l.trim())
            .join("\n"),
        ]
    }
}
