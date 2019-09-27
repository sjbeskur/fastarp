#[derive(Debug, Serialize, Clone, Default)]
pub struct ArpNode{
   pub mac_address:  String,
   pub ping_ms:      f32,
   pub ipv4_address: String,   
   pub ipv4_target:  String,
}