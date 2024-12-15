mod properties;
mod reason_code;


use bytes::BufMut;
use properties::ConnAckProperties;
use reason_code::ConnAckReasonCode;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, variable_byte_integer::{variable_integer, variable_length}}, traits::{bufferio::BufferIO, read::Read, write::Write}};


#[derive(Default)]
pub struct ConnAck {
    /// 3.2.2.1.1 Connect Acknowledge flag
    pub session_present: bool, // bit 0 of the COnnect Acknowledge flag
    pub reason: ConnAckReasonCode,
    pub properties: ConnAckProperties,
}


impl BufferIO for ConnAck {
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


    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::ConnAck, 0, self.length()).write(buf)?;

        u8::from(self.session_present).write(buf);
        (self.reason as u8).write(buf);
        self.properties.write(buf)?;

        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        // Assumption is that the fixed header as been read already
        let mut packet = Self::default();
        packet.session_present = u8::read(buf)? != 0;
        let reason = u8::read(buf)?;
        packet.reason = ConnAckReasonCode::try_from(reason).map_err(|_| MQTTError::UnknownData(format!("Unrecognized reason code: {reason}")))?;
        packet.properties = ConnAckProperties::read(buf)?;

        Ok(packet)
    }
}