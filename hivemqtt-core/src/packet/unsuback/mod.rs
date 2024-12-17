mod properties;
mod reason_code;

use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;
use properties::UnSubAckProperties;
pub use reason_code::UnSubAckReasonCode;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

/// Sent by the Server to the Client to confirm receipt of an UNSUBSCRIBE packet
#[derive(Debug, Default)]
pub struct UnSubAck {
    pub packet_identifier: u16,
    pub properties: UnSubAckProperties,
    pub payload: Vec<UnSubAckReasonCode>,
}


impl BufferIO for UnSubAck {
    // Length of the Variable Header plus the length of the Payload 
    fn length(&self) -> usize {
        2 + self.properties.length() + Self::get_variable_length(self.properties.length()) + self.payload.len()
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::UnSubAck, 0, self.length()).write(buf)?;

        // packet identifier, properties
        self.packet_identifier.write(buf);
        self.properties.write(buf)?;

        self.payload.iter().for_each(|p| u8::from(*p).write(buf));
        
        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let mut packet = Self::default();
        packet.packet_identifier = u16::read(buf)?;
        packet.properties = UnSubAckProperties::read(buf)?;

        loop {
            if buf.is_empty() { break; }
            packet.payload.push(UnSubAckReasonCode::try_from(u8::read(buf)?).map_err(MQTTError::UnknownData)?);
        }

        Ok(packet)
    }
}