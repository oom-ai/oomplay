use crate::{
    store::{PortMap, Store},
    svec,
};

pub struct Redis;

impl Store for Redis {
    fn name(&self) -> String {
        "oomplay-redis".to_string()
    }

    fn image(&self) -> String {
        "redis:alpine".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        vec![PortMap::Tcp(26379, 6379)]
    }

    fn init_cmd(&self) -> Vec<String> {
        svec!["redis-cli", "flushdb"]
    }

    fn ping_cmd(&self) -> Vec<String> {
        svec!["redis-cli", "-c", "ping"]
    }
}
