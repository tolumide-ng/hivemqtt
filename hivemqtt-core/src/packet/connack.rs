use bytes::BufMut;

use std::borrow::{Borrow, Cow};
use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, variable_byte_integer::{variable_integer, variable_length}}, traits::write::ControlPacket};
use crate::commons::property::Property;

#[derive(Debug, Length)]
pub(crate) struct Properties {
    session_expiry_interval: Option<u32>,
    receive_maximum: Option<u16>,
    maximum_qos: Option<bool>,
    retain_available: Option<bool>,
    maximum_packet_size: Option<u32>,
    assigned_client_id: Option<String>,
    topic_alias_maximum: Option<u16>,
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
    wildcard_subscription_available: Option<bool>,
    subscription_identifiers_available: Option<bool>,
    shared_subscription_available: Option<bool>,
    server_keep_alive: Option<u16>,
    response_information: Option<String>,
    server_reference: Option<String>,
    authentication_method: Option<String>,
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
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buff));
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


#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum ConnAckReasonCode {
    #[default]
    Success = 0,
    UnspecifiedError = 128,
    MalformedPacket = 129,
    ProtocolError = 130,
    ImplementationSpecificError =131,
    UnSupportedProtocolVersion = 132,
    ClientIdentifierNotValid = 133,
    BadUserNameOrPassword = 134,
    NotAuthorized = 135,
    ServerUnAvailable = 136,
    ServerBusy = 137,
    Banned = 138,
    BadAuthenticationMethod = 140,
    TopicNameInvalid = 144,
    PacketTooLarge = 149,
    QuotaExceeded = 151,
    PayloadFormatInvalid = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServeMoved = 157,
    ConnectionRateExceeded = 159,
}

pub(crate) struct ConnAck {
    /// 3.2.2.1.1 Connect Acknowledge flag
    session_present: bool, // bit 0 of the COnnect Acknowledge flag
    reason: ConnAckReasonCode,
    properties: Properties,


}


impl ControlPacket for ConnAck {
    

    /// In this case:
    /// This is the length of the Variable Header
    fn length(&self) -> usize {
        let mut len = 1 + 1; // session present + reason code

        len += self.properties.length();
        len += variable_length(self.properties.length());
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::ConnAck));
        let _ = variable_integer(buf, self.length()); // Variable Header encoded as Variable Byte Integer
        buf.put_u8(self.session_present as u8);
        buf.put_u8(self.reason as u8);
        self.properties.w(buf);
    }
}