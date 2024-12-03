use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::DataSize;

use crate::{commons::{property::Property, qos::QoS, variable_byte_integer::{variable_integer, variable_length}}, traits::write::Write};


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
    #[bytes(kv_2)]
    user_property: Vec<(String, String)>,
}


impl Write for WillProperties {
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


#[derive(Debug, DataSize)]
pub(crate) struct Will {
    properties: WillProperties,
    /// 3.1.3.3
    #[bytes(wl_2)]
    topic: String,
    /// 3.1.3.4
    #[bytes(wl_2)]
    payload: Bytes,
    /// 3.1.2.6 (flag)
    pub(super) qos: QoS,
    /// 3.1.2.7 (flag)
    pub(super) retain: bool,
}


impl Write for Will {
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