use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Redis {
    pub port:     u16,
    pub password: String,
    pub database: u32,
}

impl Store for Redis {
    fn name(&self) -> String {
        "oomplay-redis".to_string()
    }

    fn image(&self) -> String {
        "redis:alpine".to_string()
    }

    fn envs(&self) -> Vec<String> {
        svec![format!("REDISCLI_AUTH={}", self.password)]
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(6379, self.port)]
    }

    fn cmd(&self) -> Option<Vec<String>> {
        Some(svec!["redis-server", "--requirepass", self.password])
    }

    fn reset_cmd(&self) -> Vec<String> {
        svec!["redis-cli", "-n", self.database, "flushdb"]
    }

    fn recreate_cmd(&self) -> Vec<String> {
        self.reset_cmd()
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["redis-cli", "-c", "ping"]
    }
}
