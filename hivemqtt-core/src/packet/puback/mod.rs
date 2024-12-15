mod properties;
pub use properties::{PubAckProperties, PubAckReasonCode};

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

#[derive(Debug, Default)]
pub(crate) struct PubAck {
    packet_identifier: u16,
    reason_code: PubAckReasonCode,
    properties: PubAckProperties,
}


impl BufferIO for PubAck {
    /// Length of the Variable Header, encoded as Variable Byte Integer
    fn length(&self) -> usize {
        let mut len = std::mem::size_of::<u16>(); // packet identifier

        // only add reason code if there's no properties
        if self.reason_code == PubAckReasonCode::Success && self.properties.length() == 0 { return len; }
        
        len += 1; // reason code
        len += self.properties.length() + self.properties.variable_length();
        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::PubAck, 0, self.length());

        self.packet_identifier.write(buf);
        if self.reason_code == PubAckReasonCode::Success && self.properties.length() == 0 { 
            return Ok(())
        }

        u8::from(self.reason_code).write(buf);
        self.properties.write(buf)?;
        Ok(())
    }

    fn read_with_fixedheader(buf: &mut bytes::Bytes, header: FixedHeader) -> Result<Self, crate::commons::error::MQTTError> {
        let mut packet = Self::default();
        packet.packet_identifier = u16::read(buf)?;

        if header.remaining_length == 2 { 
            packet.reason_code = PubAckReasonCode::Success;
            return Ok(packet)
        }

        packet.reason_code = PubAckReasonCode::try_from(u8::read(buf)?).map_err(|e| MQTTError::UnknownData(format!("Uknown reason code: {e}")))?;
        packet.properties = PubAckProperties::read(buf)?;

        Ok(packet)
    }
}