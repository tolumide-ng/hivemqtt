mod properties;
mod reason_code;

pub use properties::{AuthProperties, AuthReasonCode};

use bytes::Bytes;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};


#[derive(Debug, Default, PartialEq, Eq)]
pub struct Auth {
    reason_code: AuthReasonCode,
    properties: AuthProperties,
}

impl BufferIO for Auth {
    fn length(&self) -> usize {
        1 + self.properties.length() + self.properties.variable_length()
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::Auth, 0, self.length()).write(buf)?;

        u8::from(self.reason_code).write(buf);
        self.properties.write(buf)?;

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let mut packet = Self::default();

        // reason code and property length can be omitted if reason_code is success and there are no properties
        if buf.is_empty() { return Ok(packet) }

        packet.reason_code = AuthReasonCode::try_from(u8::read(buf)?).map_err(MQTTError::UnknownData)?;
        packet.properties = AuthProperties::read(buf)?;

        Ok(packet)
    }
}




#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn read_write_when_reason_code_code_and_property_length_is_omitted() {
        // reason code and property length can be omitted if the reason code is 0x00(Success) and there are no properties
        let packet = Auth::default();
        let mut buf = BytesMut::with_capacity(10);
        packet.write(&mut buf).unwrap();

        assert_eq!(buf, b"\xf0\x02\0\0".to_vec());

        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 2);
        assert_eq!(fixed_header.packet_type, Packet::Auth);

        let read_packet = Auth::read(&mut read_buf).unwrap();
        assert_eq!(read_packet.reason_code, AuthReasonCode::Success);
        assert_eq!(packet, read_packet);
    }

    #[test]
    fn reason_code_and_property_length_are_omitted() {
        let mut buf = Bytes::from_iter(b"\xf0\0".to_vec());
        let fixed_header = FixedHeader::read(&mut buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 0);
        assert_eq!(fixed_header.packet_type, Packet::Auth);

        let packet = Auth::read(&mut buf).unwrap();

        assert_eq!(packet, Auth::default());
    }

    #[test]
    fn read_write() {
        let mut packet = Auth::default();
        packet.properties = AuthProperties {auth_data: Some("serious auth data ".into()), auth_method: Some("basicAuthenticationMethod".into()), reason_string: Some("rss".into()), user_property: vec![]};
        packet.reason_code =  AuthReasonCode::ReAuthenticate;

        let mut buf = BytesMut::with_capacity(0);

        packet.write(&mut buf).unwrap();

        let expected = b"\xf09\x197\x15\0\x19basicAuthenticationMethod\x16\0\x12serious auth data \x1f\0\x03rss".to_vec();

        assert_eq!(buf, expected);

        let mut read_buf = Bytes::from_iter(expected.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 57);
        assert_eq!(fixed_header.packet_type, Packet::Auth);

        let read_packet = Auth::read(&mut read_buf).unwrap();
        assert_eq!(read_packet.reason_code, AuthReasonCode::ReAuthenticate);
        assert_eq!(packet, read_packet);
    }
}