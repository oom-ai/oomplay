#[allow(dead_code)]
pub enum PortMap {
    Tcp(u16, u16),
    Udp(u16, u16),
}

pub trait Store {
    fn name(&self) -> String;
    fn image(&self) -> String;
    fn port_map(&self) -> Vec<PortMap>;
    fn reset_cmd(&self) -> Vec<String>;
    fn ping_cmd(&self) -> Vec<String>;

    fn envs(&self) -> Vec<String> {
        vec![]
    }
    fn cmd(&self) -> Option<Vec<String>> {
        None
    }
}
