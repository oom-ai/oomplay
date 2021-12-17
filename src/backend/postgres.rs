use anyhow::Result;
use bollard::{container, models, Docker};
use std::collections::HashMap;

pub struct Postgres {
    pub port:     u16,
    pub user:     String,
    pub password: String,
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
}
