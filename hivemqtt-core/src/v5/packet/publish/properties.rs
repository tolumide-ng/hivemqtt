use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, Length, Default, PartialEq, Eq)]
pub struct PublishProperties {
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_internal: Option<u32>,
    pub topic_alias: Option<u16>,
    /// the presence of a Response Topic identifies the Message as a Request
    pub response_topic: Option<String>,
    pub correlation_data: Option<Bytes>,
    pub user_property: Vec<(String, String)>,
    pub subscription_identifier: Vec<usize>,
    pub content_type: Option<String>,
}

impl BufferIO for PublishProperties {
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;
        
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buf);
        Property::MessageExpiryInterval(self.message_expiry_internal).w(buf);
        Property::TopicAlias(self.topic_alias).w(buf);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buf);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        self.subscription_identifier.iter().for_each(|si| Property::SubscriptionIdentifier(Cow::Borrowed(&si)).w(buf));
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buf);
        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let Some(len) = Self::parse_len(buf)? else { return Ok(Self::default()) };
        let mut props = Self::default();
        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;

            match property {
                Property::PayloadFormatIndicator(value) => Self::try_update(&mut props.payload_format_indicator, value)(property)?,
                Property::MessageExpiryInterval(value) => Self::try_update(&mut props.message_expiry_internal, value)(property)?,
                Property::TopicAlias(value) => Self::try_update(&mut props.topic_alias, value)(property)?,
                Property::ResponseTopic(ref v) => Self::try_update(&mut props.response_topic, v.as_deref().map(String::from))(property)?,
                // Property::CorrelationData(ref v) => Self::try_update(&mut props.correlation_data, v.to_owned().map(|x| Bytes::from_iter(x.into_owned())))(property)?,
                Property::CorrelationData(ref value) => Self::try_update(&mut props.correlation_data, value.as_deref().map(|x| Bytes::from_iter(x.to_vec())))(property)?,
                Property::UserProperty(value) => props.user_property.push(value.into_owned()),
                Property::SubscriptionIdentifier(value) => props.subscription_identifier.push(value.into_owned()),
                Property::ContentType(ref v) => Self::try_update(&mut props.content_type, v.as_deref().map(String::from))(property)?,
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }

        Ok(props)
    }
}