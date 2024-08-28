use crate::commons::qos::QoS;

/// The Connect Flags bytes provides information on the MQTT connection, 
/// and indicates the presence or absence of fields in the payload 
pub struct ConnectFlags {
    /// bit 7: If the username flag is set to 0, a username MUST NOT present in the Payload.
    /// If the username flag is set to 1, then a username MUST be present in the Payload. 
    username: bool,
    /// bit 6: If the password flag is set to 0, a password MUST NOT present in the Payload.
    /// If the password flag is set to 1, then a password MUST be present in the Payload.
    password: bool,
    /// bit 5: Specifies if the Will Message is to be reatined when it is published
    will_retain: bool,
    /// bits 3 - 4: Specifies the QoS level when publishing the Will Message 
    will_qos: QoS,
    /// bit 2: This indicates that a will message MUST be stored on the Server and associated with the Session.
    /// If this property is set to 1, the Will Properties, Will Topic, and Will Payload fields must be present in the Payload
    will_flag: bool,
    /// bit 1: This bit specifies whether the connection starts a new Session or continues an existing Session.
    /// The Session present flag in CONNACK is always set to 0 if Clean Start is set to 1
    clean_start: bool
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