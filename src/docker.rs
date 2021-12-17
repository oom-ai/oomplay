use anyhow::{bail, Result};
use bollard::{exec, Docker};
use futures::StreamExt;

pub async fn exec(docker: &Docker, container: &str, cmd: &[&str]) -> Result<()> {
    let id = docker
        .create_exec::<String>(container, exec::CreateExecOptions {
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
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
    Ok(())
}
