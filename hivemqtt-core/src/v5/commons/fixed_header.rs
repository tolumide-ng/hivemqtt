use bytes::{Bytes, BytesMut};

use super::{error::MQTTError, packet_type::PacketType};
use crate::v5::traits::{syncx::bufferio::BufferIO, syncx::read::Read, syncx::write::Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FixedHeader {
    pub(crate) packet_type: PacketType,
    pub(crate) flags: Option<u8>,
    /// Variable Byte Integer representing the number of bytes in the Variable Header and the Payload.
    pub(crate) remaining_length: usize,
    pub(crate) header_len: usize,
}

impl FixedHeader {
    pub(crate) fn new(packet_type: PacketType, flags: u8, remaining_length: usize) -> Self {
        Self {
            packet_type,
            flags: Some(flags).filter(|f| *f != 0),
            remaining_length,
            header_len: 0,
        }
    }

    pub(crate) fn with_len(&mut self, header_len: usize) -> Self {
        Self {
            header_len,
            ..*self
        }
    }
}

pub use synx::*;

mod synx {
    use super::{FixedHeader, PacketType};
    use crate::v5::{
        commons::error::MQTTError,
        traits::syncx::{bufferio::BufferIO, read::Read, write::Write},
    };
    use bytes::{Bytes, BytesMut};

    impl BufferIO for FixedHeader {
        fn length(&self) -> usize {
            self.remaining_length
        }

        fn write(&self, buf: &mut BytesMut) -> Result<(), MQTTError> {
            // let f = self.flags.unwrap_or(0);
            ((self.packet_type as u8) | &self.flags.unwrap_or(0)).write(buf);
            self.encode(buf)?;

            Ok(())
        }

        fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
            if buf.len() < 2 {
                return Err(MQTTError::InsufficientBytes);
            }

            let byte0 = u8::read(buf)?;
            let packet = byte0 & 0b11110000;
            let packet_type = PacketType::try_from(packet).map_err(|_| {
                MQTTError::UnknownData(format!("Unexpected packet type: {}", packet))
            })?;

            let (remaining_length, header_len) = Self::decode(buf)?;

            Ok(Self {
                packet_type,
                flags: Some(byte0 & 0b00001111).filter(|n| *n != 0),
                remaining_length,
                header_len,
            })
        }
    }
}

mod asyncx {
    use futures::{AsyncReadExt, AsyncWriteExt};

    use super::{FixedHeader, PacketType};
    use crate::v5::{
        commons::error::MQTTError,
        traits::asyncx::{bufferio::BufferIO, read::Read, write::Write},
    };

    impl<R, W> BufferIO<R, W> for FixedHeader
    where
        R: AsyncReadExt + Unpin,
        W: AsyncWriteExt + Unpin,
    {
        fn length(&self) -> usize {
            self.remaining_length
        }

        async fn read(stream: &mut R) -> Result<Self, crate::v5::commons::error::MQTTError> {
            let byte0 = u8::read(stream).await?;
            let packet = byte0 & 0b11110000;
            let packet_type = PacketType::try_from(packet).map_err(|_| {
                MQTTError::UnknownData(format!("Unexpected packet type: {}", packet))
            })?;

            let (remaining_length, header_len) = <Self as BufferIO<R, W>>::decode(stream).await?;

            Ok(Self {
                packet_type,
                flags: Some(byte0 & 0b00001111).filter(|n| *n != 0),
                remaining_length,
                header_len,
            })
        }

        async fn write(&self, stream: &mut W) -> Result<(), crate::v5::commons::error::MQTTError> {
            let byte0 = (self.packet_type as u8) | &self.flags.unwrap_or(0);
            (byte0 as u8).write(stream).await?;

            let encoded_length = <Self as BufferIO<R, W>>::encode(self).await?;
            encoded_length.write(stream).await?;

            Ok(())
        }
    }
}
