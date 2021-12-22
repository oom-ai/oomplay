#[allow(dead_code)]
pub enum PortMap {
    Tcp(u16, u16),
    Udp(u16, u16),
}

pub trait Store {
    fn name(&self) -> String;
    fn image(&self) -> String;
    fn port_map(&self) -> Vec<PortMap>;
    fn ping_cmd(&self) -> Vec<String>;
    fn destory_cmd(&self) -> Vec<String>;
    fn recreate_cmd(&self) -> Vec<String>;

    fn envs(&self) -> Vec<String> {
        vec![]
    }
    fn cmd(&self) -> Option<Vec<String>> {
        None
    }
}
