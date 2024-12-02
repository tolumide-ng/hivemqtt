use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::DataSize;

use crate::{commons::{property::Property, variable_byte_integer::encode_varint}, traits::write::Write};


#[derive(Debug, Clone, DataSize)]
pub(crate) struct ConnectProperties {
    /// 3.1.2.11.2
    #[bytes(4)]
    session_expiry_interval: Option<u32>,
    /// 3.1.2.11.3
    #[bytes(2)]
    receive_maximum: Option<u16>,
    /// 3.1.2.11.4
    #[bytes(4)]
    maximum_packet_size: Option<u32>,
    /// 3.1.2.11.5
    #[bytes(2)]
    topic_alias_maximum: Option<u16>,
    /// 3.1.2.11.6
    #[bytes(1)]
    request_response_information: Option<u8>,
    /// 3.1.2.11.7
    #[bytes(1)]
    request_problem_information: Option<u8>,
    /// 3.1.2.11.8
    #[bytes(kv_2)]
    user_property: Vec<(String, String)>,
    /// 3.1.2.11.9
    #[bytes(wl_2)]
    authentication_method: Option<String>,
    /// 3.1.2.11.10
    #[bytes(wl_2)] 
    authentication_data: Option<Bytes>,
}


impl Write for ConnectProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = encode_varint(buf, self.len()).unwrap(); // from the DataSize macro
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::RequestResponseInformation(self.request_response_information).w(buf);
        Property::RequestProblemInformation(self.request_problem_information).w(buf);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buf);
    }
}