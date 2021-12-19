#[allow(dead_code)]
pub enum PortMap {
    Tcp(u16, u16),
    Udp(u16, u16),
}

pub trait Store {
    fn container_name(&self) -> String;
    fn image(&self) -> String;
    fn envs(&self) -> Vec<String>;
    fn port_map(&self) -> Vec<PortMap>;
    fn reset_cmd(&self) -> Vec<String>;
    fn ping_cmd(&self) -> Vec<String>;
}
