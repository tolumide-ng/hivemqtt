#[cfg(not(feature = "syncx"))]
pub(crate) mod asyncx;
pub(crate) mod syncx;

use crate::v5::commons::fixed_header::FixedHeader;
use crate::v5::commons::property::Property;
use bytes::Bytes;

use crate::v5::commons::error::MQTTError;

pub(crate) trait StreamIOBase: Sized {
    fn variable_length(&self) -> usize {
        let len = self.length();
        if len >= 2_097_152 {
            4
        } else if len >= 16_384 {
            3
        } else if len >= 128 {
            2
        } else {
            1
        }
    }

    /// Encodes a non-negative Integer into the Variable Byte Integer encoding
    fn encode(&self) -> Result<Vec<u8>, MQTTError> {
        let mut len = self.length();
        let mut result = vec![];

        // 268_435_455
        if len > 0xFFFFFFF {
            return Err(MQTTError::PayloadTooLong);
        }

        for _ in 0..4 {
            let mut byte = len % 128;
            len /= 128;

            if len > 0 {
                byte |= 128;
            }

            // (byte as u8).write(buf); // writes the encoded byte into the buffer
            result.push(byte as u8);
            if len == 0 {
                break;
            }
        }

        Ok(result)
    }

    /// Applies to fields that results in Protocol Error if their value appears more than once
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

    /// Allows a struct specify what it's length is to it's external users
    /// Normally this is obtainable using the .len() method (internally on structs implementing Length(formerly DataSize)),
    /// However, this method allows the struct customize what its actual length is.
    /// NOTE: The eventual plan is to make this the only property accessible externally and
    ///     make `.len()` internal while probably enforcing that all struct's implementing this method/trait
    ///     must also implement `DataSize` proc. So that there is a default accurate length property
    fn length(&self) -> usize {
        0
    }

    fn read_data(_buf: &mut Bytes) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }

    fn read_with_fixedheader(_buf: &mut Bytes, _header: FixedHeader) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }
}

impl<T> StreamIOBase for T where T: {}
