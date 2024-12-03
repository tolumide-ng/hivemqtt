use bytes::BufMut;

use crate::{commons::{packets::Packet, variable_byte_integer::variable_integer}, traits::write::ControlPacket};

pub(crate) struct Connack {}


impl ControlPacket for Connack {
    /// In this case:
    /// This is the length of the Variable Header
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::ConnAck));
        let _ = variable_integer(buf, self.length()); // Variable Header encoded as Variable Byte Integer
    }
}