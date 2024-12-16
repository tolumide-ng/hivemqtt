mod properties;
mod reason_code;
use properties::SubAckProperties;
pub use reason_code::SubAckReasonCode;


use bytes::BufMut;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read}};

/// 3.9: Sent by the Server to the Client to confirm receipt and processing of a SUBSCRIBE packet.
pub struct SubAck {
    packet_identifier: u16,
    payload: Vec<SubAckReasonCode>,
    properties: SubAckProperties,
}

impl BufferIO for SubAck {
    // Length of the Variable Header plus the length of the Payload
    fn length(&self) -> usize {
        // packet identifier + ...
        2 + self.payload.len() + self.properties.length() + self.properties.variable_length()
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::SubAck));
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);
        self.properties.w(buf);
        buf.extend_from_slice(&(self.payload.iter().map(|x| *x as u8).collect::<Vec<u8>>()));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = SubAckProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            payload.push(SubAckReasonCode::try_from(u8::read(buf)?).map_err(|e| MQTTError::UnknownData(e))?);

            if buf.is_empty() { break; }
        }
        
        
        Ok(Self { packet_identifier, payload, properties })
    }
}