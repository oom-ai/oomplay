use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bollard::{container, exec, image, models, Docker};
use futures::{StreamExt, TryStreamExt};
use std::time::Duration;

use crate::store::{PortMap, Store};

#[async_trait]
pub trait StoreRuntime<T>
where
    T: Store + Sync,
{
    async fn start(&self, store: &T) -> Result<()>;
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse>;
    async fn run(&self, store: &T) -> Result<()>;
    async fn stop(&self, store: &T) -> Result<()>;
    async fn reset(&self, store: &T) -> Result<i64>;
    async fn ping(&self, store: &T) -> Result<i64>;
    async fn wait_ready(&self, store: &T) -> Result<()> {
        loop {
            tokio::time::sleep(Duration::SECOND).await;
            match self.ping(store).await {
                Ok(0) => return Ok(()),
                Ok(n) => {
                    debug!("ping exited with none-zero code {}", n);
                    continue;
                }
                Err(e) => {
                    debug!("docker exec failed: {}", e);
                    continue;
                }
            }
        }
    }
    async fn init(&self, store: &T) -> Result<()> {
        match self.reset(store).await {
            Ok(0) => Ok(()),
            _ => {
                self.run(store).await?;
                self.wait_ready(store).await
            }
        }
    }
}

#[async_trait]
impl<T> StoreRuntime<T> for Docker
where
    T: Store + Sync,
{
    async fn start(&self, store: &T) -> Result<()> {
        Ok(self.start_container::<String>(&store.name(), None).await?)
    }
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse> {
        let config = container::Config {
            image: Some(store.image()),
            env: Some(store.envs()),
            cmd: store.cmd(),
            host_config: Some(models::HostConfig {
                auto_remove: Some(true),
                port_bindings: Some(
                    store
                        .port_map()
                        .into_iter()
                        .map(|pm| {
                            let (from, to) = match pm {
                                PortMap::Tcp(from, to) => (format!("{from}/tcp"), to.to_string()),
                                PortMap::Udp(from, to) => (format!("{from}/udp"), to.to_string()),
                            };
                            (
                                from,
                                Some(vec![models::PortBinding { host_port: Some(to), ..Default::default() }]),
                            )
                        })
                        .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        Ok(self
            .create_container(Some(container::CreateContainerOptions { name: store.name() }), config)
            .await?)
    }
    async fn run(&self, store: &T) -> Result<()> {
        if self.inspect_image(&store.image()).await.is_err() {
            pull(self, &store.image()).await?
        };
        self.create(store).await?;
        self.start(store).await
    }
    async fn stop(&self, store: &T) -> Result<()> {
        Ok(self.stop_container(&store.name(), None).await?)
    }
    async fn reset(&self, store: &T) -> Result<i64> {
        exec(self, &store.name(), store.reset_cmd()).await
    }
    async fn ping(&self, store: &T) -> Result<i64> {
        exec(self, &store.name(), store.ping_cmd()).await
    }
}

async fn exec(docker: &Docker, container: &str, cmd: Vec<String>) -> Result<i64> {
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
                debug!("{}", msg);
            },
        exec::StartExecResults::Detached => bail!("should not be detached"),
    }
    docker
        .inspect_exec(&id)
        .await?
        .exit_code
        .ok_or_else(|| anyhow!("exit code is empty"))
}

async fn pull(docker: &Docker, image: &str) -> Result<()> {
    docker
        .create_image(
            Some(image::CreateImageOptions { from_image: image, ..Default::default() }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;
    Ok(())
}
