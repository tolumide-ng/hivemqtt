use std::borrow::Cow;

use bytes::{BufMut, Bytes};
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property, qos::QoS}, traits::write::ControlPacket};

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

impl ControlPacket for Publish {
    fn length(&self) -> usize {
        // (variable header + length of the payload), encoded as Variable Byte Integer
        let mut len = self.topic.len() + 2;
        len += self.properties.len() + Self::get_variable_length(self.properties.len());
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


#[derive(Debug, Length)]
pub(crate) struct PublishProperties {
    payload_format_indicator: Option<u8>,
    message_expiry_internal: Option<u32>,
    topic_alias: Option<u16>,
    /// the presence of a Response Topic identifies the Message as a Request
    response_topic: Option<String>,
    correlation_data: Option<Bytes>,
    user_property: Vec<(String, String)>,
    subscription_identifier: Vec<usize>,
    content_type: Option<String>,
}

impl ControlPacket for PublishProperties {
    fn length(&self) -> usize {
        self.len()
    }
    
    fn w(&self, buf: &mut bytes::BytesMut) {
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buf);
        Property::MessageExpiryInterval(self.message_expiry_internal).w(buf);
        Property::TopicAlias(self.topic_alias).w(buf);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buf);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buf);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
        Property::SubscriptionIdentifier(Cow::Borrowed(&self.subscription_identifier)).w(buf);
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buf);
    }
}