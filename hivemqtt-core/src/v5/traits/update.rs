use bytes::Bytes;

use crate::v5::commons::{error::MQTTError, property::Property};
use crate::v5::traits::syncx::read::Read;

#[cfg(not(feature = "asyncx"))]
use super::bufferio::BufferIO;
#[cfg(feature = "asyncx")]
use super::streamio::StreamIO;

// use super::{bufferio::BufferIO, streamio::StreamIO};

pub(crate) fn try_update<T>(
    field: &mut Option<T>,
    value: Option<T>,
) -> impl Fn(Property) -> Result<(), MQTTError> {
    let is_duplicate = field.is_some();
    *field = value;

    move |ppt| {
        if is_duplicate {
            return Err(MQTTError::DuplicateProperty(ppt.to_string()));
        }
        return Ok(());
    }
}

/// Decodes a Variable byte Inetger
/// To be moved into a utils traits implemented automatically for bytes(?)
pub(crate) fn decode(buf: &mut Bytes) -> Result<(usize, usize), MQTTError> {
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

pub(crate) trait TryUpdate {
    fn try_update<T>(
        field: &mut Option<T>,
        value: Option<T>,
    ) -> impl Fn(Property) -> Result<(), MQTTError> {
        let is_duplicate = field.is_some();
        *field = value;

        move |ppt| {
            if is_duplicate {
                return Err(MQTTError::DuplicateProperty(ppt.to_string()));
            }
            return Ok(());
        }
    }
}

#[cfg(feature = "asyncx")]
impl<T: StreamIO> TryUpdate for T {}

#[cfg(not(feature = "asyncx"))]
impl<T: BufferIO> TryUpdate for T {}
