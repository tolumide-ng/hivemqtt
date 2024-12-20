mod properties;
pub use properties::PublishProperties;

use bytes::Bytes;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property, qos::QoS}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, Default, PartialEq, Eq)]
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
        buf.extend(&self.payload);
        Ok(())
    }

    /// Publish does not implement `read` only read_with_flag
    fn read_with_fixedheader(buf: &mut Bytes, header: FixedHeader) -> Result<Self, MQTTError> {
        let mut packet = Self::default();
        let flag = header.flags;

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


#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn read_write_publish() {
        let packet = Publish {
            dup: true, retain: true, qos: QoS::One,
            topic: String::from("packagin_plant/#"),
            packet_identifier: Some(8930),
            payload: b"veryLarge payload".to_vec().into(),
            properties: PublishProperties {
                payload_format_indicator: Some(13),
                topic_alias: Some(02),
            ..Default::default()
            },
        };

        let mut buf = BytesMut::new();
        packet.write(&mut buf).unwrap();

        let expected = b";)\0\x10packagin_plant/#\"\xe2\x05\x01\r#\0\x02\0\x11veryLarge payloadveryLarge payload".to_vec();
        assert_eq!(buf.to_vec(), expected);

        let mut expected = Bytes::from_iter(b";)\0\x10packagin_plant/#\"\xe2\x05\x01\r#\0\x02\0\x11veryLarge payloadveryLarge payload".to_vec()[2..].to_vec());
        let created_packed = Publish::read_with_fixedheader(&mut expected, FixedHeader { packet_type: Packet::Publish, flags: 0b1011, remaining_length: 99 }).unwrap();
        assert_eq!(created_packed, packet);
    }
}