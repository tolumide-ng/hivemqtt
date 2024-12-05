use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property, qos::QoS}, traits::write::ControlPacket};

#[derive(Debug, Length)]
pub(crate) struct Publish {
    dup: bool,
    retain: bool,
    qos: QoS,
    topic: String,
    packet_identifier: u16,
    #[bytes(ignore)]
    properties: PublishProperties,
}

impl ControlPacket for Publish {
    fn length(&self) -> usize {
        // (variable header + length of the payload), encoded as Variable Byte Integer
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::Publish) | (self.dup as u8) << 3 | (self.qos as u8) << 1 | (self.retain as u8));
        // this part below needs to be reconfirmed
        let _ = self.encode_variable_length(buf, self.length()); // not sure yet.
        self.ws(buf, self.topic.as_bytes());

        if self.qos !=QoS::Zero {buf.put_u16(self.packet_identifier)}
        let _ = self.encode_variable_length(buf, self.properties.length());
    }
}


#[derive(Debug, Length)]
pub(crate) struct PublishProperties {
    payload_format_indicator: Option<u8>,
}

impl ControlPacket for PublishProperties {
    fn length(&self) -> usize {
        self.len()
    }
    
    fn w(&self, buf: &mut bytes::BytesMut) {
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buf);
    }
}