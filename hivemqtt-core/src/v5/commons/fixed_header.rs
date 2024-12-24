use bytes::{Bytes, BytesMut};

use super::{error::MQTTError, packets::Packet};
use crate::v5::traits::{bufferio::BufferIO, read::Read, write::Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FixedHeader {
    pub(crate) packet_type: Packet,
    pub(crate) flags: u8,
    /// Variable Byte Integer representing the number of bytes in the Variable Header and the Payload.
    pub(crate) remaining_length: usize,
}


impl FixedHeader {
    pub(crate) fn new(packet_type: Packet, flags: u8, remaining_length: usize) -> Self {
        Self { packet_type, flags, remaining_length }
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
        let byte0 = u8::read(buf)?;
        let packet = byte0 & 0b11110000;
        let packet_type = Packet::try_from(packet).map_err(|_| MQTTError::UnknownData(format!("Unexpected packet type: {}", packet)))?;

        Ok(Self {
            packet_type,
            flags: byte0 & 0b00001111,
            remaining_length: Self::decode(buf)?,
        })
    }
}