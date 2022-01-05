use crate::{docker::Mount, store::Store, svec};

pub struct SQLite;

impl Store for SQLite {
    fn name(&self) -> String {
        "sqlite".to_string()
    }

    fn image(&self) -> String {
        "alpine".to_string()
    }

    fn mounts(&self) -> Vec<Mount> {
        vec![Mount { source: "/tmp".to_string(), target: "/tmp".to_string() }]
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        Some(svec!["sleep", "infinity"])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec!["sh", "-c", "rm -rf /tmp/oomplay.db"]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["true"]
    }
}
