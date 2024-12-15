mod properties;
mod options;

pub use options::SubscriptionOptions;
pub use properties::SubcribeProperties;

use bytes::{BufMut, Bytes};

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read}};


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

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Subscribe as u8 | 1 << 1);
        //  Encoded as Variable Byte Integer
        let _ = Self::write_variable_integer(buf, self.length());
        
        buf.put_u16(self.packet_identifier);
        self.properties.w(buf);

        for (topic, options) in &self.payload {
            self.ws(buf, topic.as_bytes());
            options.w(buf);
        }
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = SubcribeProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            let topic = String::read(buf)?;
            let options = SubscriptionOptions::read(buf)?;

            payload.push((topic, options));

            if buf.is_empty() { break }
        }

        Ok(Self { packet_identifier, properties, payload })
    }
}