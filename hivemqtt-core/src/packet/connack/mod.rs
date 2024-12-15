mod properties;
mod reason_code;


use bytes::BufMut;
use properties::ConnAckProperties;
use reason_code::ConnAckReasonCode;

use crate::{commons::{packets::Packet, variable_byte_integer::{variable_integer, variable_length}}, traits::bufferio::BufferIO};



pub struct ConnAck {
    /// 3.2.2.1.1 Connect Acknowledge flag
    pub session_present: bool, // bit 0 of the COnnect Acknowledge flag
    pub reason: ConnAckReasonCode,
    pub properties: ConnAckProperties,
}


impl BufferIO for ConnAck {
    /// In this case:
    /// This is the length of the Variable Header
    fn length(&self) -> usize {
        let mut len = 1 + 1; // session present + reason code

        len += self.properties.length();
        len += variable_length(self.properties.length());
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::ConnAck));
        let _ = variable_integer(buf, self.length()); // Variable Header encoded as Variable Byte Integer
        buf.put_u8(self.session_present as u8);
        buf.put_u8(self.reason as u8);
        self.properties.w(buf);
    }
}