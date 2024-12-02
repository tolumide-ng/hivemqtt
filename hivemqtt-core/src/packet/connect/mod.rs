pub(crate) mod will;
pub(crate) mod properties;


use bytes::BufMut;
use hivemqtt_macros::DataSize;

use crate::{commons::{packets::Packet, qos::QoS, variable_byte_integer::{variable_integer, variable_length}, version::Version}, constants::PROTOCOL_NAME, traits::write::ControlPacket};
use crate::packet::connect::{properties::ConnectProperties, will::Will};


#[derive(Debug, Default)]
pub(crate) struct ConnectFlags {
    pub(super) username: bool,
    pub(super) password: bool,
    pub(super) will_retain: bool,
    pub(super) will_qos: QoS,
    pub(super) will_flag: bool,
    pub(super) clean_start: bool,
}

impl ConnectFlags {
    const USERNAME_MASK: u8 = 1 << 7;
    const PASSWORD_MASK: u8 = 1 << 6;
    const WILL_RETAIN_MASK: u8 = 1 << 5;
    const QOS_MASK: u8 = 1 << 4 | 1 << 3;
    const WILL_FLAG_MASK: u8 = 1 << 2;
    const CLEAN_START_MASK: u8 = 1 << 1;
}

impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let flags = u8::from(value.username) << 7 | u8::from(value.password) << 6 | u8::from(value.will_retain) << 5 | u8::from(value.will_qos) << 4 | 
        u8::from(value.will_flag) << 2 | u8::from(value.clean_start) << 1;
        flags
    }
}


impl From<u8> for ConnectFlags {
    fn from(value: u8) -> Self {
        let username = (value & Self::USERNAME_MASK) != 0;
        let password = (value & Self::PASSWORD_MASK) != 0;
        let will_retain = (value & Self::WILL_RETAIN_MASK) != 0;
        let will_qos = QoS::from((value & Self::QOS_MASK) >> 3);
        let will_flag = (value & Self::WILL_FLAG_MASK) != 0;
        let clean_start = (value & Self::CLEAN_START_MASK) != 0;
        Self { username, password, will_retain, will_qos, will_flag, clean_start }
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
#[derive(Debug, DataSize)]
pub struct Connect {
    /// 3.1.2.2
    #[bytes(0)]
    version: Version,
    /// 3.1.3.1
    #[bytes(wl_2)]
    client_id: String,
    /// 3.1.3.2
    will: Option<Will>,
    #[bytes(wl_2)]
    username: Option<String>,
    #[bytes(wl_2)]
    password: Option<String>,

    /// 3.1.2.4
    clean_start: bool,
    /// 3.1.2.10
    keep_alive: u16,
    /// 3.1.2.11: Connection properties
    conn_ppts: ConnectProperties,
}

impl ControlPacket for Connect {
    fn length(&self) -> usize {
        let mut len = (2 + PROTOCOL_NAME.len()) + 1 + 1 + 2; // version + connect flags + keep alive
        len += self.conn_ppts.length();
        len += variable_length(self.conn_ppts.length());
        if let Some(will) = &self.will { len += will.length() }
        len += self.len(); // client id + username + password

        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Connect.into());
        let _ = variable_integer(buf, self.length());
        self.ws(buf, PROTOCOL_NAME.as_bytes());
        buf.put_u8(self.version as u8);
        
        let mut flags = ConnectFlags {
            clean_start: self.clean_start,
            password: self.password.is_some(),
            username: self.username.is_some(),
            ..Default::default()
        };

        if let Some(will) = &self.will {
            flags.will_retain = will.retain;
            flags.will_flag = true;
            flags.will_qos = will.qos;
        }
        
        buf.put_u8(u8::from(flags));
        buf.put_u16(self.keep_alive);
        self.conn_ppts.w(buf);
        self.ws(buf, self.client_id.as_bytes());
        if let Some(will) = &self.will { will.w(buf) }
        if let Some(username) = &self.username { self.ws(buf, username.as_bytes()) }
        if let Some(password) = &self.password { self.ws(buf, password.as_bytes()) }   
    }
}