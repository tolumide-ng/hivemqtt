mod reason_code;
mod properties;

pub use properties::DisconnectProperties;
pub use reason_code::DisconnectReasonCode;


use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read, write::Write}};

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
        packet.properties = DisconnectProperties::read(buf)?;

        Ok(packet)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_disconnect_without_properties() {}

    #[test]
    fn read_write_disconnect_packet_with_properties() {}
}