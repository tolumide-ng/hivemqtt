use bytes::{Bytes, BytesMut};

use super::{error::MQTTError, packet_type::PacketType};
use crate::v5::traits::{bufferio::BufferIO, read::Read, write::Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FixedHeader {
    pub(crate) packet_type: PacketType,
    pub(crate) flags: u8,
    /// Variable Byte Integer representing the number of bytes in the Variable Header and the Payload.
    pub(crate) remaining_length: usize,
    pub(crate) header_len: usize,
}


impl FixedHeader {
    pub(crate) fn new(packet_type: PacketType, flags: u8, remaining_length: usize) -> Self {
        Self { packet_type, flags, remaining_length, header_len: 0 }
    }

    pub(crate) fn with_len(&mut self, header_len: usize) -> Self {
        Self { header_len, ..*self }
    }
}



impl BufferIO for FixedHeader {
    fn length(&self) -> usize { self.remaining_length }

    fn write(&self, buf: &mut BytesMut) -> Result<(), MQTTError> {
        ((self.packet_type as u8) | self.flags).write(buf);
        self.encode(buf)?;

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        if buf.len() < 2 { return Err(MQTTError::InsufficientBytes) }

        let byte0 = u8::read(buf)?;
        let packet = byte0 & 0b11110000;
        let packet_type = PacketType::try_from(packet).map_err(|_| MQTTError::UnknownData(format!("Unexpected packet type: {}", packet)))?;

        let (remaining_length, header_len) = Self::decode(buf)?;

        Ok(Self {
            packet_type,
            flags: byte0 & 0b00001111,
            remaining_length,
            header_len,
        })
    }
}