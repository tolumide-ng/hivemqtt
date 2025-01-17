use crate::v5::traits::{syncx::read::Read, syncx::write::Write};
use bytes::{Bytes, BytesMut};

use crate::v5::commons::error::MQTTError;

use super::{FixedHeader, StreamIOBase};

pub(crate) trait StreamIO: Sized + StreamIOBase {
    /// Decodes a Variable byte Inetger
    fn decode(buf: &mut Bytes) -> Result<(usize, usize), MQTTError> {
        let mut result = 0;

        for i in 0..4 {
            if buf.is_empty() {
                return Err(MQTTError::MalformedPacket);
            }
            let byte = u8::read(buf)?;

            result += ((byte as usize) & 0x7F) << (7 * i);

            if (byte & 0x80) == 0 {
                return Ok((result, i + 1));
            }
        }

        return Err(MQTTError::MalformedPacket);
    }

    fn parse_len(buf: &mut Bytes) -> Result<Option<usize>, MQTTError>
    where
        Self: Default,
    {
        let (len, _) = Self::decode(buf)?;

        if len == 0 {
            return Ok(None);
        }
        if len > buf.len() {
            return Err(MQTTError::IncompleteData("", len, buf.len()));
        };

        Ok(Some(len))
    }

    fn w(&self, buf: &mut BytesMut) {}

    fn write(&self, buf: &mut BytesMut) -> Result<(), MQTTError> {
        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }

    fn read_with_fixedheader(_buf: &mut Bytes, _header: FixedHeader) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }
}
