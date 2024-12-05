use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, qos::QoS}, traits::write::ControlPacket};

#[derive(Debug, Length)]
pub(crate) struct Publish {
    dup: bool,
    retain: bool,
    qos: QoS,
}

impl ControlPacket for Publish {
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::Publish) | (self.dup as u8) << 3 | (self.qos as u8) << 1 | (self.retain as u8));
    }
}