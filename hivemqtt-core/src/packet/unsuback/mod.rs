mod properties;
mod reason_code;

use properties::UnSubAckProperties;
pub use reason_code::UnSubAckReasonCode;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

/// Sent by the Server to the Client to confirm receipt of an UNSUBSCRIBE packet
#[derive(Debug, Default, PartialEq, Eq)]
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


#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use crate::packet::unsuback::UnSubAck;

    use super::*;

    #[test]
    fn read_write_unsuback() {
        let mut packet = UnSubAck::default();
        packet.packet_identifier = 0x2E;
        packet.payload = vec![UnSubAckReasonCode::Success, UnSubAckReasonCode::TopicFilterInvalid];
        packet.properties = UnSubAckProperties {reason_string: Some("reason_string here and there".into()), user_property: vec![("key".into(), "value".into())] };

        let mut buf = BytesMut::with_capacity(50);
        packet.write(&mut buf).unwrap();
        let expected = b"\xb01\0.,\x1f\0\x1creason_string here and there&\0\x03key\0\x05value\0\x8f".to_vec();

        assert_eq!(buf.to_vec(), expected);

        let mut read_buf = Bytes::from_iter(expected);
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.packet_type, Packet::UnSubAck);
        assert_eq!(fixed_header.remaining_length, 49);

        let read_packet = UnSubAck::read(&mut read_buf).unwrap();
        assert_eq!(read_packet, packet);
    }
}