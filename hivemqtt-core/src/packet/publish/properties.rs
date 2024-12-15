use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use super::{BufferIO, Property};

#[derive(Debug, Length)]
pub struct PublishProperties {
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

impl BufferIO for PublishProperties {
    fn length(&self) -> usize { self.len() }
    
    fn w(&self, buf: &mut bytes::BytesMut) {
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buf);
        Property::MessageExpiryInterval(self.message_expiry_internal).w(buf);
        Property::TopicAlias(self.topic_alias).w(buf);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buf);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        self.subscription_identifier.iter().for_each(|si| Property::SubscriptionIdentifier(Cow::Borrowed(&si)).w(buf));
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buf);
    }
}