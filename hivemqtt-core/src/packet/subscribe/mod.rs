mod properties;
mod options;

pub use options::SubscriptionOptions;
pub use properties::SubcribeProperties;

use bytes::Bytes;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};


#[derive(Debug, Default, PartialEq, Eq)]
pub struct  Subscribe {
    packet_identifier: u16,
    properties: SubcribeProperties,
    /// It is protocl error to have a subscribe packet that doesn't have atleast one payload (topic, subscriptionOptions)
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
        if self.payload.len() == 0 { return Err(MQTTError::ProtocolError("Must contain at least one topic/subscription option pair"))}

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

        if packet.payload.len() == 0 {return Err(MQTTError::ProtocolError("Must contain at least one topic/subscription option pair"))}
        Ok(packet)
    }
}


#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn should_return_an_error_if_theres_no_payload() {}

    #[test]
    fn default_read_write_subscribe_packet() {
        let mut packet = Subscribe::default();
        packet.packet_identifier = 2378;
        packet.payload = vec![("autos".into(), SubscriptionOptions::default())];

        let mut buf = BytesMut::with_capacity(20);
        packet.write(&mut buf).unwrap();

        println!("buf {buf:?}");

        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();
        
        assert_eq!(buf.to_vec(), b"\x82\n\tJ\0\0\x05autos\0".to_vec());
        assert_eq!(fixed_header.flags, 0b10);
        assert_eq!(fixed_header.packet_type, Packet::Subscribe);
        let read_packet = Subscribe::read(&mut read_buf).unwrap();
        assert_eq!(packet, read_packet);
    }

    #[test]
    fn read_write_subscribe_packet() {
        let mut packet = Subscribe::default();
        packet.packet_identifier = 2378;
        // packet.payload 

        let mut buf = BytesMut::with_capacity(20);
        // packet.write(&mut buf).unwrap();

        // assert_eq!(buf.to_vec(), b"\x82\x02\tJ\0".to_vec());
        // let read_packet = Subscribe::read(&mut Bytes::from_iter(buf.to_vec())).unwrap();
        // assert_eq!(packet, read_packet);
    }
}