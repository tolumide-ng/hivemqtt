use crate::commons::qos::QoS;

/// The Connect Flags bytes provides information on the MQTT connection, 
/// and indicates the presence or absence of fields in the payload 
pub struct ConnectFlags {
    /// 3.1.2.4
    clean_start: bool,
    /// 3.1.2.5
    will_flag: bool,
    /// 3.1.2.6
    will_qos: QoS,
    /// 3.1.2.7
    will_retain: bool,
    /// 3.1.2.8
    username: bool,
    /// 3.1.2.9
    password: bool,
}

// impl ConnectFlags {
//     pub fn new(username: bool, password: bool, will_retain: bool, will_flag: bool, clean_start: bool) {}
// }

impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let flags = (value.clean_start as u8) << 1 |
        (value.will_flag as u8) << 2 |
        ((value.will_flag as u8) << 3 & (value.will_qos as u8) << 3) |
        ((value.will_flag as u8) << 5 & (value.will_retain as u8) << 5) |
        (value.password as u8) << 6 | (value.username as u8) << 7;

        flags
    }
}