mod properties;

use properties::{PubCompProperties, PubCompReasonCode};

use crate::v5::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, Default, PartialEq, Eq)]
pub  struct PubComp {
    packet_identifier: u16,
    reason_code: PubCompReasonCode,
    properties: PubCompProperties,
}

impl BufferIO for PubComp {
    fn length(&self) -> usize {
        let mut len = std::mem::size_of::<u16>(); // packet identifier
        if self.reason_code == PubCompReasonCode::Success && self.properties.length() == 0 { return len; }
        len += 1 + self.properties.length() + self.properties.variable_length(); // reason code
        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::PubComp, 0, self.length()).write(buf)?;

        self.packet_identifier.write(buf);
        if self.properties.length() == 0 && self.reason_code == PubCompReasonCode::Success { return Ok(()) }
        
        u8::from(self.reason_code).write(buf);
        self.properties.write(buf)?;
        Ok(())
    }
    

    fn read_with_fixedheader(buf: &mut bytes::Bytes, header: FixedHeader) -> Result<Self, MQTTError> {
        let mut packet = Self::default();
        packet.packet_identifier = u16::read(buf)?;

        if header.remaining_length == 2 {
            packet.reason_code = PubCompReasonCode::Success;
            return Ok(packet);
        }
        
        packet.reason_code = PubCompReasonCode::try_from(u8::read(buf)?).map_err(|e| MQTTError::UnknownData(e))?;
        packet.properties = PubCompProperties::read(buf)?;
        
        Ok(packet)
    }
}


#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::*;

    #[test]
    fn read_write_with_no_properties() {
        let mut packet = PubComp::default();
        packet.reason_code = PubCompReasonCode::Success;

        let mut buf = BytesMut::with_capacity(20);
        packet.write(&mut buf).unwrap();

        assert_eq!(buf.to_vec(), b"p\x02\0\0".to_vec());

        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(header.remaining_length, 2);
        assert_eq!(header.packet_type, Packet::PubComp);

        let received_packet = PubComp::read_with_fixedheader(&mut read_buf, header).unwrap();
        assert_eq!(packet, received_packet);
    }

    #[test]
    fn read_write_with_neither_properties_nor_reasoncode() {
        let packet = PubComp::default();

        let mut buf = BytesMut::with_capacity(10);
        packet.write(&mut buf).unwrap();

        assert_eq!(buf.to_vec(),  b"p\x02\0\0".to_vec());
        
        let mut read_buf = Bytes::from_iter(buf.to_vec());
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.remaining_length, 2);
        assert_eq!(fixed_header.packet_type, Packet::PubComp);

        let received_packet = PubComp::read_with_fixedheader(&mut read_buf, fixed_header).unwrap();
        assert_eq!(packet, received_packet);
        assert_eq!(packet.reason_code, PubCompReasonCode::Success);
    }

    #[test]
    fn read_write_with_properties_and_reasoncode() {
        let mut packet = PubComp::default();
        packet.properties.reason_string = Some(String::from("thisIsAReasonStriing--andMoreAndMore"));
        packet.properties.user_property = vec![(String::from("notAuthorized"), String::from("value"))];
        packet.reason_code = PubCompReasonCode::PacketIdentifierNotFound;

        let mut buf = BytesMut::with_capacity(50);
        packet.write(&mut buf).unwrap();

        let expected = b"pB\0\0\x92>\x1f\0$thisIsAReasonStriing--andMoreAndMore&\0\rnotAuthorized\0\x05value".to_vec();
        assert_eq!(buf.to_vec(), expected);

        let mut read_buf = Bytes::from_iter(expected);
        let fixed_header = FixedHeader::read(&mut read_buf).unwrap();

        assert_eq!(fixed_header.packet_type, Packet::PubComp);

        let received_packet = PubComp::read_with_fixedheader(&mut read_buf, fixed_header).unwrap();
        assert_eq!(packet, received_packet);
    }
}