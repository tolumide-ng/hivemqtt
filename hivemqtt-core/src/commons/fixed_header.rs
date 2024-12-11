use bytes::{Bytes, BytesMut};

use super::{encode_vbi, error::MQTTError, packets::Packet};
use crate::traits::write::Write;

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

    pub(crate) fn write(&self, buf: &mut BytesMut) -> Result<(), MQTTError> {
        ((self.packet_type as u8) | self.flags).write(buf);
        encode_vbi(buf, self.remaining_length)?;

        Ok(())
    }


    // pub(crate) fn read(buf: &mut Bytes) {}
}