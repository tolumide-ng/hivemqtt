mod properties;
mod options;

pub use options::SubscriptionOptions;
pub use properties::SubcribeProperties;

use bytes::Bytes;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};


#[derive(Debug, Default)]
pub struct  Subscribe {
    packet_identifier: u16,
    properties: SubcribeProperties,
    payload: Vec<(String, SubscriptionOptions)>,
}


impl BufferIO for Subscribe {
    /// (Length of Variable Header + Length of the Payload)
    fn length(&self) -> usize {
        let mut len = 2 + self.properties.length(); // packet identifier + properties
        len += self.payload.iter().fold(0, |acc, x| acc + (1 + (2 + x.0.len()))); // u8(len) + (string(2) + topic.len())

        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::Subscribe, 0b10, self.length()).write(buf)?;

        self.packet_identifier.write(buf);
        self.properties.write(buf)?;

        for (topic, options) in &self.payload {
            topic.write(buf); options.write(buf)?;
        }

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let mut packet = Self::default();
        packet.packet_identifier = u16::read(buf)?;
        packet.properties = SubcribeProperties::read(buf)?;

        loop {
            let topic = String::read(buf)?;
            let options = SubscriptionOptions::read(buf)?;

            packet.payload.push((topic, options));
            if buf.is_empty() { break; }
        }

        Ok(packet)
    }
}