use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::{commons::{property::Property, qos::QoS, variable_byte_integer::{variable_integer, variable_length}}, traits::write::ControlPacket};


#[derive(Length, Debug, Clone)]
pub(crate) struct WillProperties {
    delay_interval: Option<u32>,
    payload_format_indicator: Option<u8>,
    message_expiry_interval: Option<u32>,
    content_type: Option<String>,
    response_topic: Option<String>,
    correlation_data: Option<Bytes>,
    user_property: Vec<(String, String)>,
}


impl ControlPacket for WillProperties {
    fn w(&self, buff: &mut bytes::BytesMut) {
        let _ = variable_integer(buff, self.len()).unwrap();
        Property::WillDelayInterval(self.delay_interval).w(buff);
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buff);
        Property::MessageExpiryInterval(self.message_expiry_interval).w(buff);
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buff);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buff);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buff);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buff);
    }

    fn length(&self) -> usize {
        self.len()
    }
}


#[derive(Debug, Length)]
pub(crate) struct Will {
    properties: WillProperties,
    topic: String,
    payload: Bytes,
    #[bytes(ignore)]
    pub(super) qos: QoS,
    #[bytes(ignore)]
    pub(super) retain: bool,
}


impl ControlPacket for Will {
    fn w(&self, buff: &mut bytes::BytesMut) {
        self.properties.w(buff);
        self.ws(buff, self.topic.as_bytes());
        self.ws(buff, &self.payload);
    }

    fn length(&self) -> usize {
        let ppts = self.properties.length();
        self.len() + variable_length(ppts) + ppts
    }
}