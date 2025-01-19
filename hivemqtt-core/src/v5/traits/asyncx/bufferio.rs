use crate::v5::commons::property::Property;
use crate::v5::traits::asyncx::read::Read;
use futures::{AsyncReadExt, AsyncWriteExt};

use crate::v5::commons::error::MQTTError;

pub(crate) trait BufferIO<R, W>: Sized
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    async fn variable_length(&self) -> usize {
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
    async fn encode(&self) -> Result<Vec<u8>, MQTTError> {
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

            // (byte as u8).write(stream).await?; // writes the encoded byte into the buffer
            result.push(byte as u8);
            if len == 0 {
                break;
            }
        }

        return Ok(result);
    }

    /// Decodes a Variable byte Integer
    async fn decode(stream: &mut R) -> Result<(usize, usize), MQTTError> {
        let mut result = 0;

        for i in 0..4 {
            let byte = u8::read(stream).await?;

            result += ((byte as usize) & 0x7F) << (7 * i);

            if (byte & 0x80) == 0 {
                return Ok((result, i + 1));
            }
        }

        return Err(MQTTError::MalformedPacket);
    }

    /// Applies to fields that results in Protocol Error if their value appears more than once
    async fn try_update<T>(
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

    async fn read(stream: &mut R) -> Result<Self, MQTTError>;

    // async fn read_with_fixedheader(_buf: &mut R, _header: FixedHeader) -> Result<Self, MQTTError>;

    async fn parse_len(stream: &mut R) -> Result<Option<usize>, MQTTError>
    where
        Self: Default,
    {
        let (len, _) = Self::decode(stream).await?;
        if len == 0 {
            return Ok(None);
        }
        Ok(Some(len))
    }

    async fn write(&self, stream: &mut W) -> Result<(), MQTTError>;
}
