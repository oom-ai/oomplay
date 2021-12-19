use crate::docker::exec;
use anyhow::Result;
use bollard::{container, models, Docker};
use std::{collections::HashMap, time::Duration};

pub struct Postgres {
    pub port:     u16,
    pub user:     String,
    pub password: String,
    pub database: String,
}

impl Postgres {
    pub fn container_name() -> String {
        "oomstore-playground-postgres".to_string()
    }

    pub async fn create(&self, docker: &Docker) -> Result<models::ContainerCreateResponse> {
        let config = container::Config {
            image: Some("postgres:14.0-alpine".to_string()),
            env: Some(vec![
                format!("POSTGRES_PASSWORD={}", self.password),
                format!("POSTGRES_USER={}", self.user),
                format!("PGUSER={}", self.user),
            ]),
            host_config: Some(models::HostConfig {
                auto_remove: Some(true),
                port_bindings: Some(HashMap::from([(
                    "5432/tcp".to_string(),
                    Some(vec![models::PortBinding {
                        host_port: Some(self.port.to_string()),
                        ..Default::default()
                    }]),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        };

        Ok(docker
            .create_container(
                Some(container::CreateContainerOptions { name: Self::container_name() }),
                config,
            )
            .await?)
    }

    pub async fn start(&self, docker: &Docker) -> Result<()> {
        Ok(docker.start_container::<String>(&Self::container_name(), None).await?)
    }

    pub async fn run(&self, docker: &Docker) -> Result<()> {
        self.create(docker).await?;
        self.start(docker).await?;
        Ok(())
    }

    pub async fn stop(&self, docker: &Docker) -> Result<()> {
        Ok(docker.stop_container(&Self::container_name(), None).await?)
    }

    pub async fn restart(&self, docker: &Docker) -> Result<()> {
        Ok(docker.restart_container(&Self::container_name(), None).await?)
    }

    pub async fn reset(&self, docker: &Docker) -> Result<i64> {
        exec(docker, &Self::container_name(), &[
            "psql",
            "-c",
            &format!("DROP DATABASE IF EXISTS {}", self.database),
        ])
        .await
    }

    pub async fn ping(&self, docker: &Docker) -> Result<i64> {
        exec(docker, &Self::container_name(), &["pg_isready"]).await
    }

    pub async fn wait_ready(&self, docker: &Docker) -> Result<()> {
        while self.ping(docker).await? != 0 {
            tokio::time::sleep(Duration::SECOND).await
        }
        Ok(())
    }
}
