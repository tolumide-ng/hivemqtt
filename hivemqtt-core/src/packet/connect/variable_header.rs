use bytes::Bytes;

use crate::commons::version::Version;

use super::{payload::WillProperties, properties::ConnectProperties};

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

/// ```text
/// +--------+--------------------------+---+---+---+---+---+---+---+---+
/// |               |    Description    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Protocol Name                                                 |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 1        | Length MSB(0)     |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 2        | Length LSB(4)     |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+ 
/// | byte 3        |       'M'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 4        |       'Q'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 5        |       'T'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 6        |       'T'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Protocol Version                                              |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 7        | Version(5)        |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// |               |   Description     | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// |               | User Name Flag(1) |   |   |   |   |   |   |   |   |
/// |               | Password Flag(1)  |   |   |   |   |   |   |   |   |
/// |               | Will Retain(0)    |   |   |   |   |   |   |   |   |
/// | byte 8        | Will QoS(01)      | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0 |
/// |               | Will Flag(1)      |   |   |   |   |   |   |   |   |
/// |               | Clean Start(1)    |   |   |   |   |   |   |   |   |
/// |               | Reserved(0)       |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Keep Alive                                            |   |       |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 9        | Keep Alive MSB(0) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 10       | Keep Alive LSB(10)| 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Properties                                            |   |       |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 11       | Length(5)         | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 12       | SEII(17)          | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 13       | SEI (10)          | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 14       |                   | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 15       |                   | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 16       |                   | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |
/// +-----------------------------------+---+---+---+---+---+---+---+---+
/// 
/// **SEII = Session Expiry Interval Identifier
/// **SEI  = Session Expiry Interval
/// ```
pub struct Connect {
    // ACTUAL VARIABLE HEADERS
    /// 3.1.2.2
    version: Version,
    /// 3.1.2.3
    connect_flags: ConnectFlags,
    /// 3.1.2.10 (2 bytes) - byte 9 & byte 10
    keep_alive: u16,
    /// 3.1.2.11.2 - 3.1.2.11.10
    properties: ConnectProperties,
    // CONNECT PAYLOAD
    /// 3.1.3.1
    client_identifier: Option<String>,
    /// 3.1.3.2
    last_will: WillProperties,
    /// 3.1.3.3 (if the will flag is 1, then this must be the next field in the payload)
    will_topic: Option<String>,
    /// 3.1.3.4
    will_payload: Option<Bytes>,
    username: Option<String>,
    password: Option<String>

}

// struct ConnectAuthentication {
//     /// Name of the Authentication method used for extended authentication. 
//     /// If the Client sets this value, the Client MUST NOT send any packets other than AUTH or 
//     /// DISCONNECT packets until it has received a CONNACK packet
//     authentication_method: String,
//     /// Binary Data containing the authentication data.
//     /// It is ProtocolError to include Authentication Data if there is no Authentication Method.
//     authentication_data: String,
// }

