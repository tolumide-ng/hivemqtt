mod properties;
mod reason_code;

use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;
use properties::UnSubAckProperties;
pub use reason_code::UnSubAckReasonCode;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read}};

/// Sent by the Server to the Client to confirm receipt of an UNSUBSCRIBE packet
pub struct UnSubAck {
    packet_identifier: u16,
    properties: UnSubAckProperties,
    payload: Vec<UnSubAckReasonCode>,
}


impl BufferIO for UnSubAck {
    // Length of the Variable Header plus the length of the Payload 
    fn length(&self) -> usize {
        2 + self.properties.length() + Self::get_variable_length(self.properties.length()) + self.payload.len()
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::UnSubAck as u8);
        let _ = Self::write_variable_integer(buf, self.length());

        self.properties.w(buf);
        self.payload.iter().for_each(|rc| buf.put_u8(*rc as u8));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = UnSubAckProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            payload.push(UnSubAckReasonCode::try_from(u8::read(buf)?).map_err(|e| MQTTError::UnknownData(e))?);
            if buf.is_empty() { break; }
        }


        Ok(Self { packet_identifier, properties, payload })
    }
}