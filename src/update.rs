use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub async fn update() -> Result<self_update::Status> {
    tokio::task::spawn_blocking(|| {
        let bin = env!("CARGO_PKG_REPOSITORY");
        let repo = env!("CARGO_PKG_REPOSITORY");
        let target = self_update::get_target();
        let repo_caps = Regex::new(r#"github.com/(?P<owner>[^/]+)/(?P<name>[^/]+)$"#)
            .unwrap()
            .captures(repo)
            .unwrap();
        let repo_owner = repo_caps.name("owner").unwrap().as_str();
        let repo_name = repo_caps.name("name").unwrap().as_str();

        Ok(self_update::backends::github::Update::configure()
            .repo_owner(repo_owner)
            .repo_name(repo_name)
            .target(target)
            .bin_path_in_archive(Path::new(bin).join(bin).to_str().unwrap())
            .bin_name(bin)
            .show_download_progress(true)
            .current_version(self_update::cargo_crate_version!())
            .build()?
            .update()?)
    })
    .await?
}
