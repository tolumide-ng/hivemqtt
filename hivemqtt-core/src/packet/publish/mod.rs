mod properties;
pub use properties::PublishProperties;

use bytes::Bytes;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property, qos::QoS}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, Default)]
pub(crate) struct Publish {
    dup: bool,
    retain: bool,
    qos: QoS,
    topic: String,
    packet_identifier: Option<u16>,
    properties: PublishProperties,
    payload: Bytes,
}

impl BufferIO for Publish {
    /// variable header, length of the payload, encoded as Variable Byte Integer
    fn length(&self) -> usize {
        let mut len = if self.qos != QoS::Zero { self.topic.len() + 2 } else { 0 };
        len += self.properties.length() + self.properties.variable_length() + self.payload.len();
        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::Publish, (self.dup as u8) << 3 | (self.qos as u8) << 1 | (self.retain as u8), self.length()).write(buf)?;

        self.topic.write(buf);
        if self.qos != QoS::Zero {
            self.packet_identifier.ok_or_else(|| MQTTError::PublishPacketId)?.write(buf);
        }

        self.properties.write(buf)?;
        self.payload.write(buf);
        Ok(())
    }

    /// Publish does not implement `read` only read_with_flag
    fn read_with_flag(buf: &mut Bytes, flag: u8) -> Result<Self, MQTTError> {
        let mut packet = Self::default();

        packet.topic = String::read(buf)?;
        packet.dup = (flag & 0b1000) != 0;
        packet.qos = QoS::try_from((flag & 0b0110) >> 1)?;
        packet.retain = (flag & 0b1) != 0;

        if packet.qos != QoS::Zero {
            packet.packet_identifier = Some(u16::read(buf)?);
        }
        
        packet.properties = PublishProperties::read(buf)?;
        packet.payload = Bytes::read(buf)?;
        Ok(packet)
    }
}