use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bollard::{container, errors::Error::DockerResponseNotFoundError, exec, image, models, Docker};
use futures::prelude::*;
use std::time::Duration;

use crate::store::{PortMap, Store};

#[async_trait]
pub trait StoreRuntime<T>
where
    T: Store + Sync + ?Sized,
{
    async fn start(&self, store: &T) -> Result<()>;
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse>;
    async fn run(&self, store: &T) -> Result<()>;
    async fn stop(&self, store: &T) -> Result<()>;
    async fn reset(&self, store: &T) -> Result<()>;
    async fn ping(&self, store: &T) -> Result<()>;
    async fn wait_ready(&self, store: &T) -> Result<()> {
        while let Err(e) = self.ping(store).await {
            info!("⏳ Wait for the store to be ready ...");
            debug!("ping failed: {}", e);
            tokio::time::sleep(Duration::SECOND * 2).await;
        }
        Ok(())
    }
    async fn init(&self, store: &T) -> Result<()> {
        self.reset(store)
            .or_else(async move |e| {
                debug!("reset failed: {}", e);
                self.run(store).await?;
                self.wait_ready(store).await?;
                info!("❇️ Store is ready");
                Ok(())
            })
            .await
    }
}

#[async_trait]
impl<T> StoreRuntime<T> for Docker
where
    T: Store + Sync + ?Sized,
{
    async fn start(&self, store: &T) -> Result<()> {
        info!("🕹️ Starting container '{}' ...", store.name());
        Ok(self.start_container::<String>(&store.name(), None).await?)
    }
    async fn create(&self, store: &T) -> Result<models::ContainerCreateResponse> {
        info!("🛠️ Creating container '{}' ...", store.name());
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
        self.stop_container(&store.name(), None).await.or_else(|e| match e {
            DockerResponseNotFoundError { .. } => {
                debug!("'{}' already stopped", store.name());
                Ok(())
            }
            _ => Err(e.into()),
        })
    }
    async fn reset(&self, store: &T) -> Result<()> {
        info!("♻️ Reseting store ...");
        exec(self, &store.name(), store.reset_cmd()).await
    }
    async fn ping(&self, store: &T) -> Result<()> {
        exec(self, &store.name(), store.ping_cmd()).await
    }
}

async fn exec(docker: &Docker, container: &str, cmd: Vec<String>) -> Result<()> {
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
        .and_then(|code| match code {
            0 => Ok(()),
            _ => bail!("none-zero exit code: {}", code),
        })
}

async fn pull(docker: &Docker, image: &str) -> Result<()> {
    info!("📦 Pulling image '{}' ...", image);
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
