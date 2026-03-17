#[derive(Debug, Clone, Default)]
pub struct ArpNode {
    pub mac_address: String,
    pub ping_ms: f32,
    pub ipv4_address: String,
    pub ipv4_target: String,
}

impl std::fmt::Display for ArpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<18} {:<16} {:.3}ms", self.mac_address, self.ipv4_address, self.ping_ms)
    }
}
