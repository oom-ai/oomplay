#[allow(dead_code)]
pub enum PortMap {
    Tcp(u16, u16),
    Udp(u16, u16),
}

pub trait Store {
    fn name(&self) -> String;
    fn full_name(&self) -> String {
        format!("oomplay-{}", self.name())
    }

    fn image(&self) -> String;

    fn network(&self) -> String {
        "bridge".to_string()
    }

    fn port_map(&self) -> Vec<PortMap> {
        Vec::new()
    }

    fn envs(&self) -> Vec<String> {
        Vec::new()
    }

    fn entry_cmd(&self) -> Option<Vec<String>> {
        None
    }

    fn ping_cmd(&self) -> Vec<String>;
    fn reset_cmd(&self) -> Vec<String>;
}
