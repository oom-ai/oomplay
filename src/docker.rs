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
    async fn create(&self, store: &T) -> Result<()>;
    async fn init(&self, store: &T) -> Result<()>;
    async fn stop(&self, store: &T) -> Result<()>;
    async fn init_db(&self, store: &T) -> Result<()>;
    async fn check_health(&self, store: &T) -> Result<()>;

    async fn wait_ready(&self, store: &T) -> Result<()> {
        while let Err(e) = self.check_health(store).await {
            debug!("check health failed: {}", e);
            tokio::time::sleep(Duration::SECOND * 2).await;
        }
        Ok(())
    }
}

#[async_trait]
impl<T> StoreRuntime<T> for Docker
where
    T: Store + Sync + ?Sized,
{
    async fn start(&self, store: &T) -> Result<()> {
        info!("🚀 Starting container '{}' ...", store.name());
        Ok(self.start_container::<String>(&store.name(), None).await?)
    }

    async fn create(&self, store: &T) -> Result<()> {
        info!("📦 Creating container '{}' ...", store.name());
        let config = container::Config {
            image: Some(store.image()),
            env: Some(store.envs()),
            cmd: store.entry_cmd(),
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

        self.create_container(Some(container::CreateContainerOptions { name: store.name() }), config)
            .await?;
        Ok(())
    }

    async fn init(&self, store: &T) -> Result<()> {
        match self.check_health(store).await {
            Ok(_) => {
                info!("🔰 Store is already running");
                self.init_db(store).await?;
            }
            _ => {
                self.inspect_image(&store.image())
                    .map_ok(|_| ())
                    .or_else(async move |_| {
                        info!("🚚 Pulling image '{}' ...", store.image());
                        pull(self, &store.image()).await
                    })
                    .and_then(|_| self.create(store))
                    .and_then(|_| self.start(store))
                    .and_then(|_| self.wait_ready(store))
                    .and_then(|_| self.init_db(store))
                    .await?;
            }
        };
        Ok(())
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

    async fn init_db(&self, store: &T) -> Result<()> {
        info!("💫 Initializing database ...");
        // Sometimes `init_cmd` fails even after `ping_cmd` succeeded so we should retry here
        while let Err(e) = exec(self, &store.name(), store.init_cmd()).await {
            debug!("init database failed: {}", e);
            tokio::time::sleep(Duration::SECOND).await;
        }
        Ok(())
    }

    async fn check_health(&self, store: &T) -> Result<()> {
        info!("📡 Pinging database ...");
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
