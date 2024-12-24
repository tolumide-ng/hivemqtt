mod reason_code;
mod properties;

use bytes::Buf;
pub use properties::DisconnectProperties;
pub use reason_code::DisconnectReasonCode;


use crate::v5::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Disconnect {
    reason_code: DisconnectReasonCode,
    properties: DisconnectProperties,
}

impl BufferIO for Disconnect {
    // the length of the dsiconnect variable header
    fn length(&self) -> usize {
         // The Reason Code and Property Length can be omitted if the Reason Code is 0x00 (Normal disconnecton) and there are no Properties. In this case the DISCONNECT has a Remaining Length of 0.
        if self.reason_code == DisconnectReasonCode::NormalDisconnection && self.properties.length() == 0 {
            return 0;
        }
        return self.properties.length() + self.properties.variable_length() + 1 // 1 is for the reason code abovegit stat
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::Disconnect, 0, self.length()).write(buf)?;

        u8::from(self.reason_code).write(buf);

        if self.reason_code == DisconnectReasonCode::NormalDisconnection && self.properties.length() == 0 {
            return Ok(());
        }

        self.properties.write(buf)?;
        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        let mut packet = Self::default();
        packet.reason_code = DisconnectReasonCode::try_from(u8::read(buf)?).map_err(MQTTError::UnknownData)?;
        
        if buf.has_remaining() {
            packet.properties = DisconnectProperties::read(buf)?;
        }

        Ok(packet)
    }

}


#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::*;

    #[test]
    fn read_write_disconnect_without_properties_and_normal_reasoncode() {
        let packet = Disconnect::default();
        let mut buf = BytesMut::with_capacity(10);
        packet.write(&mut buf).unwrap();

        assert_eq!(buf, b"\xe0\0\0".to_vec());

        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 0);
        assert_eq!(fixed_header.packet_type, Packet::Disconnect);

        let read_packet = Disconnect::read(&mut read_buf).unwrap();
        assert_eq!(read_packet.reason_code, DisconnectReasonCode::NormalDisconnection);
    }

    #[test]
    fn read_write_disconect_without_properties_other_reasoncode() {
        let mut packet = Disconnect::default();
        packet.reason_code = DisconnectReasonCode::MaximumConnectTime;

        let mut buf = BytesMut::with_capacity(10);
        packet.write(&mut buf).unwrap();

        assert_eq!(buf, b"\xe0\x02\xa0\0".to_vec());

        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 2);
        assert_eq!(fixed_header.packet_type, Packet::Disconnect);
        
        let read_packet = Disconnect::read(&mut read_buf).unwrap();
        assert_eq!(read_packet.reason_code, DisconnectReasonCode::MaximumConnectTime);
        assert_eq!(packet, read_packet);
    }

    #[test]
    fn read_write_disconnect_packet_with_properties() {
        let mut packet = Disconnect::default();
        packet.reason_code = DisconnectReasonCode::MaximumConnectTime;
        packet.properties = DisconnectProperties {session_expiry_interval: Some(0x3A), 
            reason_string: Some("aVery good string3898 &**".into()), user_property: vec![], server_reference: Some("mqtt5.0.dev".into())};

        let mut buf = BytesMut::with_capacity(10);
        packet.write(&mut buf).unwrap();

        let expected = b"\xe01\xa0/\x11\0\0\0:\x1f\0\x19aVery good string3898 &**\x1c\0\x0bmqtt5.0.dev".to_vec();

        assert_eq!(buf, expected);

        let mut read_buf = Bytes::from_iter(expected.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.flags, 0);
        assert_eq!(fixed_header.remaining_length, 49);
        assert_eq!(fixed_header.packet_type, Packet::Disconnect);
        
        let read_packet = Disconnect::read(&mut read_buf).unwrap();
        assert_eq!(read_packet.reason_code, DisconnectReasonCode::MaximumConnectTime);
        assert_eq!(packet, read_packet);
    }
}