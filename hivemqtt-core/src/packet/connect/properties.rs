use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::{commons::{property::Property, variable_byte_integer::variable_integer}, traits::write::BufferIO};



/// CONNECT Properties (3.1.2.11)
#[derive(Debug, Clone, Length)]
pub(crate) struct ConnectProperties {
    session_expiry_interval: Option<u32>,
    receive_maximum: Option<u16>,
    maximum_packet_size: Option<u32>,
    topic_alias_maximum: Option<u16>,
    request_response_information: Option<u8>,
    request_problem_information: Option<u8>,
    user_property: Vec<(String, String)>,
    authentication_method: Option<String>,
    authentication_data: Option<Bytes>,
}


impl BufferIO for ConnectProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = variable_integer(buf, self.len()).unwrap(); // from the DataSize macro (3.1.2.11.1)
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::RequestResponseInformation(self.request_response_information).w(buf);
        Property::RequestProblemInformation(self.request_problem_information).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buf);
    }

    fn length(&self) -> usize {
        self.len()
    }
}