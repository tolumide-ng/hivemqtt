mod properties;
mod reason_code;

pub use properties::{AuthProperties, AuthReasonCode};

use bytes::{BufMut, Bytes};

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
        packet.reason_code = AuthReasonCode::try_from(u8::read(buf)?).map_err(MQTTError::UnknownData)?;
        packet.properties = AuthProperties::read(buf)?;

        Ok(packet)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_when_reason_code_code_and_property_length_is_omitted() {
        // reason code and property length can be omitted if the reason code is 0x00(Success) and there are no properties
    }
}