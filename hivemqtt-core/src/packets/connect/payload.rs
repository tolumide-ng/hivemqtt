use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::DataSize;

use crate::{commons::{property::Property, qos::QoS, variable_byte_integer::encode_varint}, traits::write::Write};

// if the flag topic is set to 1, the will topic is the next field in the Payload.
// The will topic MUST be a UTF-8 encoded string


pub(crate) struct Will {
    properties: WillProperties,
    /// 3.1.3.3
    topic: String,
    /// 3.1.3.4
    payload: Bytes,
    /// 3.1.2.6
    qos: QoS,
    /// 3.1.2.7
    retain: bool,
}


#[derive(DataSize, Debug, Clone)]
pub(crate) struct WillProperties {
    /// 3.1.3.2.2: Default value is 0
    #[bytes(4)]
    delay_interval: Option<u32>,
    /// 3.1.3.2.3
    #[bytes(1)]
    payload_format_indicator: Option<u8>,
    /// 3.1.3.2.4
    #[bytes(4)]
    message_expiry_interval: Option<u32>,
    /// 3.1.3.2.5
    #[bytes(wl_2)]
    content_type: Option<String>,
    /// 3.1.3.2.6
    #[bytes(wl_2)]
    response_topic: Option<String>,
    /// 3.1.3.2.7
    #[bytes(wl_2)]
    correlation_data: Option<Bytes>,
    /// 3.1.3.2.8
    #[bytes(wl_2)]
    user_property: Vec<(String, String)>,
}


impl Write for WillProperties {
    fn w(&self, buff: &mut bytes::BytesMut) {
        let _ = encode_varint(buff, self.len()).unwrap();
        Property::WillDelayInterval(self.delay_interval).w(buff);
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buff);
        Property::MessageExpiryInterval(self.message_expiry_interval).w(buff);
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buff);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buff);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buff);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buff);
    }
}