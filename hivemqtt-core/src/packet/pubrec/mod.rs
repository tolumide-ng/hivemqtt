mod properties;
use properties::{PubRecProperties, PubRecReasonCode};

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct PubRec {
    packet_identifier: u16,
    reason_code: PubRecReasonCode,
    properties: PubRecProperties,
}

impl BufferIO for PubRec {
    // length of the variable header, encoded as a variable byte integer
    fn length(&self) -> usize {
        let mut len = std::mem::size_of::<u16>(); // packet identifier
        
        if self.reason_code == PubRecReasonCode::Success && self.properties.length() == 0 { return len; }
        
        len += 1 + self.properties.length() + self.properties.variable_length(); // reason code
        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::PubRec, 0, self.length()).write(buf)?;

        self.packet_identifier.write(buf);
        if self.properties.length() == 0 && self.reason_code == PubRecReasonCode::Success { return Ok(()) }

        u8::from(self.reason_code).write(buf);
        self.properties.write(buf)?;
        Ok(())
    }

    fn read_with_fixedheader(buf: &mut bytes::Bytes, header: FixedHeader) -> Result<Self, MQTTError> {
        let mut packet = Self::default();
        packet.packet_identifier = u16::read(buf)?;

        if header.remaining_length == 2 {
            packet.reason_code = PubRecReasonCode::Success;
            return Ok(packet);
        }
        
        packet.reason_code = PubRecReasonCode::try_from(u8::read(buf)?).map_err(|e| MQTTError::UnknownData(e))?;
        packet.properties = PubRecProperties::read(buf)?;
        
        Ok(packet)
    }
}