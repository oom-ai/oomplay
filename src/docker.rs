use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bollard::{container, errors, exec, image, models, Docker};
use colored::Colorize;
use futures::prelude::*;
use std::time::Duration;
use tokio::time::Instant;

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
    async fn pull(&self, store: &T) -> Result<()>;
    async fn reset(&self, store: &T, retry: bool) -> Result<()>;
    async fn check_health(&self, store: &T) -> Result<()>;
    async fn check_image(&self, store: &T) -> Result<()>;

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
        info!("🚀 Starting container {} ...", store.name());
        match self.start_container::<String>(&store.name(), None).await {
            Ok(_) => Ok(()),
            Err(errors::Error::DockerResponseNotModifiedError { .. }) => {
                debug!("container {} already started", store.name());
                Ok(())
            }
            Err(e) => bail!(e),
        }
    }

    async fn create(&self, store: &T) -> Result<()> {
        info!("📦 Creating container {} ...", store.name());
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
        let options = container::CreateContainerOptions { name: store.name() };

        match self.create_container(Some(options), config).await {
            Ok(_) => Ok(()),
            Err(errors::Error::DockerResponseConflictError { .. }) => {
                debug!("container {} already exists", store.name());
                Ok(())
            }
            Err(e) => bail!(e),
        }
    }

    async fn init(&self, store: &T) -> Result<()> {
        info!("🎮 Initializing {} ...", store.name().blue().bold());
        let now = Instant::now();
        self.reset(store, false)
            .or_else(|_| {
                self.check_image(store)
                    .or_else(|_| self.pull(store))
                    .and_then(|_| self.create(store))
                    .and_then(|_| self.start(store))
                    .and_then(|_| self.reset(store, true))
            })
            .await?;
        info!("🟢 {} is ready. ({:?})", store.name().bold(), now.elapsed());
        Ok(())
    }

    async fn stop(&self, store: &T) -> Result<()> {
        info!("🔌 Stopping {} ...", store.name().blue().bold());
        let opt = Some(container::KillContainerOptions { signal: "SIGKILL" });
        self.kill_container(&store.name(), opt).await.or_else(|e| match e {
            errors::Error::DockerResponseNotFoundError { .. } => {
                debug!("'{}' already stopped", store.name());
                Ok(())
            }
            _ => bail!(e),
        })?;
        info!("🔴 Stopped {}.", store.name().bold());
        Ok(())
    }

    async fn reset(&self, store: &T, retry: bool) -> Result<()> {
        info!("💫 Resetting {} ...", store.name());
        // sometimes `init_cmd` fails even after `ping_cmd` succeeded so we may retry
        while let Err(e) = exec(self, &store.name(), store.reset_cmd()).await {
            info!("⌛ {} may not ready", store.name());
            debug!("init {} failed: {}", store.name(), e);
            if !retry {
                return Err(e);
            }
            tokio::time::sleep(Duration::SECOND).await;
        }
        info!("✨ {} has been reset.", store.name());
        Ok(())
    }

    async fn check_health(&self, store: &T) -> Result<()> {
        info!("📡 Pinging {} ...", store.name());
        exec(self, &store.name(), store.ping_cmd()).await
    }

    async fn pull(&self, store: &T) -> Result<()> {
        info!("🚚 Pulling image '{}' ...", store.name());
        let opts = image::CreateImageOptions { from_image: store.image(), ..Default::default() };
        self.create_image(Some(opts), None, None)
            .try_collect::<Vec<_>>()
            .await?;
        Ok(())
    }

    async fn check_image(&self, store: &T) -> Result<()> {
        Ok(self.inspect_image(&store.image()).map_ok(|_| ()).await?)
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
                debug!("exec {} output: {}", container, msg);
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
