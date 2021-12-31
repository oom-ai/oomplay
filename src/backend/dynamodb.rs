use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct DynamoDB;

impl Store for DynamoDB {
    fn name(&self) -> String {
        "oomplay-localstack-dynamodb".to_string()
    }

    fn image(&self) -> String {
        "localstack/localstack:0.12.2".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec!["SERVICES=dynamodb", "AWS_DEFAULT_OUTPUT=text"]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(4566, 24566)]
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec![
            "bash",
            "-c",
            r#"
                mapfile -t tables < <(awslocal dynamodb list-tables | awk '{print $2}')
                for table in "${tables[@]}"; do
                    awslocal dynamodb delete-table --table-name "$table"
                done
            "#,
        ]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "awslocal dynamodb list-tables"]
    }
}
