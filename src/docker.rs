use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bollard::{container, exec, models, Docker};
use futures::StreamExt;
use std::time::Duration;

use crate::store::{PortMap, Store};

#[async_trait]
pub trait StoreRuntime<T>
where
    T: Store + Sync,
{
    async fn start(&self, store: &T) -> Result<()>;
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse>;
    async fn run(&self, store: &T) -> Result<()> {
        self.create(store).await?;
        self.start(store).await
    }
    async fn stop(&self, store: &T) -> Result<()>;
    async fn restart(&self, store: &T) -> Result<()>;
    async fn reset(&self, store: &T) -> Result<i64>;
    async fn ping(&self, store: &T) -> Result<i64>;
    async fn wait_ready(&self, store: &T) -> Result<()> {
        while self.ping(store).await? != 0 {
            tokio::time::sleep(Duration::SECOND).await
        }
        Ok(())
    }
}

#[async_trait]
impl<T> StoreRuntime<T> for Docker
where
    T: Store + Sync,
{
    async fn start(&self, store: &T) -> Result<()> {
        Ok(self.start_container::<String>(&store.container_name(), None).await?)
    }
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse> {
        let config = container::Config {
            image: Some(store.image()),
            env: Some(store.envs()),
            host_config: Some(models::HostConfig {
                auto_remove: Some(true),
                port_bindings: Some(
                    store
                        .port_map()
                        .into_iter()
                        .map(|pm| {
                            let (protocol, from, to) = match pm {
                                PortMap::Tcp(from, to) => ("tcp", from, to),
                                PortMap::Udp(from, to) => ("udp", from, to),
                            };
                            (
                                format!("{from}/{protocol}"),
                                Some(vec![models::PortBinding {
                                    host_port: Some(format!("{to}")),
                                    ..Default::default()
                                }]),
                            )
                        })
                        .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        Ok(self
            .create_container(
                Some(container::CreateContainerOptions { name: store.container_name() }),
                config,
            )
            .await?)
    }
    async fn stop(&self, store: &T) -> Result<()> {
        Ok(self.stop_container(&store.container_name(), None).await?)
    }
    async fn restart(&self, store: &T) -> Result<()> {
        Ok(self.restart_container(&store.container_name(), None).await?)
    }
    async fn reset(&self, store: &T) -> Result<i64> {
        exec(self, &store.container_name(), store.reset_cmd()).await
    }
    async fn ping(&self, store: &T) -> Result<i64> {
        exec(self, &store.container_name(), store.ping_cmd()).await
    }
}

pub async fn exec(docker: &Docker, container: &str, cmd: Vec<String>) -> Result<i64> {
    let id = docker
        .create_exec::<String>(container, exec::CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(cmd),
            ..Default::default()
        })
        .await?
        .id;
    match docker.start_exec(&id, None).await? {
        exec::StartExecResults::Attached { mut output, .. } =>
            while let Some(Ok(msg)) = output.next().await {
                print!("{}", msg);
            },
        exec::StartExecResults::Detached => bail!("should not be detached"),
    }
    docker
        .inspect_exec(&id)
        .await?
        .exit_code
        .ok_or_else(|| anyhow!("exit code is empty"))
}
