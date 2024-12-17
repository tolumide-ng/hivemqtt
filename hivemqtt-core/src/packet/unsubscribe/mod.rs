mod properties;
pub use properties::UnSubscribeProperties;


use bytes::BufMut;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UnSubscribe {
    packet_identifier: u16,
    properties: UnSubscribeProperties,
    payload: Vec<String>,
}

impl BufferIO for UnSubscribe {
    /// Length of the Variable Header (2 bytes) plus the length of the Payload
    fn length(&self) -> usize {
        // packet identidier + string len
        2 + self.payload.iter().fold(0, |acc, x| acc + x.len() + 2) + self.properties.length() + self.properties.variable_length()
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        if self.payload.is_empty() {return Err(MQTTError::ProtocolError("The Payload of an UNSUBSCRIBE packet MUST contain at least one Topic Filter"))};
        FixedHeader::new(Packet::UnSubscribe, 0b10, self.length()).write(buf)?;
        
        self.packet_identifier.write(buf);
        self.properties.write(buf)?;
        self.payload.iter().for_each(|p| p.write(buf));
        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let mut packet = Self::default();
        
        packet.packet_identifier = u16::read(buf)?;
        packet.properties = UnSubscribeProperties::read(buf)?;
        
        loop {
            if buf.is_empty() { break; }
            packet.payload.push(String::read(buf)?);
        }

        if packet.payload.is_empty() {return Err(MQTTError::ProtocolError("The Payload of an UNSUBSCRIBE packet MUST contain at least one Topic Filter"))};

        Ok(packet)
    }
}
