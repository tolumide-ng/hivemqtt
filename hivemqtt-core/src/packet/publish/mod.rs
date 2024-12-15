mod properties;
pub use properties::PublishProperties;

use bytes::{BufMut, Bytes};

use crate::{commons::{packets::Packet, property::Property, qos::QoS}, traits::bufferio::BufferIO};

#[derive(Debug)]
pub(crate) struct Publish {
    dup: bool,
    retain: bool,
    qos: QoS,
    topic: String,
    packet_identifier: u16,
    properties: PublishProperties,
    payload: Bytes,
}

impl BufferIO for Publish {
    fn length(&self) -> usize {
        // (variable header + length of the payload), encoded as Variable Byte Integer
        let mut len = self.topic.len() + 2;
        len += self.properties.length() + Self::get_variable_length(self.properties.length());
        len += self.payload.len();
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::Publish) | (self.dup as u8) << 3 | (self.qos as u8) << 1 | (self.retain as u8));
        // this part below needs to be reconfirmed
        let _ = Self::write_variable_integer(buf, self.length()); // not sure yet.
        self.ws(buf, self.topic.as_bytes());

        if self.qos !=QoS::Zero {buf.put_u16(self.packet_identifier)}
        let _ = Self::write_variable_integer(buf, self.properties.length());
        self.properties.w(buf);
        buf.extend_from_slice(&self.payload);
    }
}