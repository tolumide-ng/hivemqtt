use std::borrow::{Borrow, Cow};

use bytes::Bytes;
use hivemqtt_macros::DataSize;

use crate::commons::property::Property;

use super::{variable_integer, ControlPacket};

#[derive(Debug, DataSize)]
pub(crate) struct Properties {
    #[bytes(4)]
    session_expiry_interval: Option<u32>,
    #[bytes(2)]
    receive_maximum: Option<u16>,
    #[bytes(1)]
    maximum_qos: Option<bool>,
    #[bytes(1)]
    retain_available: Option<bool>,
    #[bytes(4)]
    maximum_packet_size: Option<u32>,
    #[bytes(wl_2)]
    assigned_client_id: Option<String>,
    #[bytes(2)]
    topic_alias_maximum: Option<u16>,
    #[bytes(wl_2)]
    reason_string: Option<String>,
    #[bytes(kv_2)]
    user_property: Vec<(String, String)>,
    #[bytes(1)]
    wildcard_subscription_available: Option<bool>,
    #[bytes(1)]
    subscription_identifiers_available: Option<bool>,
    #[bytes(1)]
    shared_subscription_available: Option<bool>,
    #[bytes(2)]
    server_keep_alive: Option<u16>,
    #[bytes(wl_2)]
    response_information: Option<String>,
    #[bytes[wl_2]]
    server_reference: Option<String>,
    #[bytes[wl_2]]
    authentication_method: Option<String>,
    #[bytes[wl_2]]
    authentication_data: Option<Bytes>
}

impl ControlPacket for Properties {
    /// Length of the properties in the CONNACK packet Variable Header encoded as Variable Byte Integer
    fn length(&self) -> usize {
        self.len()
    }

    fn w(&self, buff: &mut bytes::BytesMut) {
        let _ = variable_integer(buff, self.length());

        Property::SessionExpiryInterval(self.session_expiry_interval).w(buff);
        Property::ReceiveMaximum(self.receive_maximum).w(buff);
        Property::MaximumQoS(self.maximum_qos.map(|q| q as u8)).w(buff);
        Property::RetainAvailable(self.retain_available.map(|x| x as u8)).w(buff);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buff);
        Property::AssignedClientIdentifier(self.assigned_client_id.as_deref().map(Cow::Borrowed)).w(buff);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buff);
        Property::ReasonString(self.reason_string.borrow().as_deref().map(Cow::Borrowed)).w(buff);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buff);
        Property::WildCardSubscription(self.wildcard_subscription_available.map(|x| x as u8)).w(buff);
        Property::SubscriptionIdentifierAvailable(self.session_expiry_interval.map(|x| x as u8)).w(buff);
        Property::SharedSubscriptionAvailable(self.shared_subscription_available.map(|x| x as u8)).w(buff);
        Property::ServerKeepAlive(self.server_keep_alive).w(buff);
        Property::ResponseInformation(self.response_information.as_deref().map(Cow::Borrowed)).w(buff);
        Property::ServerReference(self.server_reference.as_deref().map(Cow::Borrowed)).w(buff);
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buff);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buff);
    }
}