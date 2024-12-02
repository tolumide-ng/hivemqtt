use bytes::BufMut;

use crate::{commons::version::Version, constants::PROTOCOL_NAME, traits::{variable_header::VariableHeader, write::Write}};

use super::{connect_flags::ConnectFlags, properties::ConnectProperties, will::Will};


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
    /// 3.1.2.2
    version: Version,
    /// 3.1.3.1
    client_id: String,
    /// 3.1.3.2
    will: Option<Will>,
    username: Option<String>,
    password: Option<String>,

    /// 3.1.2.4
    clean_start: bool,
    /// 3.1.2.10
    keep_alive: u16,
    /// 3.1.2.11: Connection properties
    conn_ppts: ConnectProperties,
}


impl Write for Connect {
    fn w(&self, buff: &mut bytes::BytesMut) {
        
    }

    fn w_vh(&self, buff: &mut bytes::BytesMut) {
        self.ws(buff, PROTOCOL_NAME.as_bytes());
        buff.put_u8(self.version as u8);
        
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
        
        buff.put_u8(u8::from(flags));
        buff.put_u16(self.keep_alive);
        self.conn_ppts.w(buff);
        self.ws(buff, self.client_id.as_bytes());
        if let Some(will) = &self.will { will.w(buff) }
        if let Some(username) = &self.username { self.ws(buff, username.as_bytes()) }
        if let Some(password) = &self.password { self.ws(buff, password.as_bytes()) }
    }
}