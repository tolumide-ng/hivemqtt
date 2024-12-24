use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::{commons::error::MQTTError, traits::bufferio::BufferIO};
use crate::v5::commons::property::Property;
use std::borrow::{Borrow, Cow};

#[derive(Debug, Length, Default, PartialEq, Eq)]
pub struct ConnAckProperties {
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<u16>,
    pub maximum_qos: Option<bool>,
    pub retain_available: Option<bool>,
    pub maximum_packet_size: Option<u32>,
    pub assigned_client_id: Option<String>,
    pub topic_alias_maximum: Option<u16>,
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
    pub wildcard_subscription_available: Option<bool>,
    pub subscription_identifiers_available: Option<bool>,
    pub shared_subscription_available: Option<bool>,
    pub server_keep_alive: Option<u16>,
    pub response_information: Option<String>,
    pub server_reference: Option<String>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>
}

impl BufferIO for ConnAckProperties {
    /// Length of the properties in the CONNACK packet Variable Header
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;

        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumQoS(self.maximum_qos.map(|q| q as u8)).w(buf);
        Property::RetainAvailable(self.retain_available.map(|x| x as u8)).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::AssignedClientIdentifier(self.assigned_client_id.as_deref().map(Cow::Borrowed)).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::ReasonString(self.reason_string.borrow().as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv: &(String, String)| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Property::WildCardSubscription(self.wildcard_subscription_available.map(|x| x as u8)).w(buf);
        Property::SubscriptionIdentifierAvailable(self.subscription_identifiers_available.map(|x| x as u8)).w(buf);
        Property::SharedSubscriptionAvailable(self.shared_subscription_available.map(|x| x as u8)).w(buf);
        Property::ServerKeepAlive(self.server_keep_alive).w(buf);
        Property::ResponseInformation(self.response_information.as_deref().map(Cow::Borrowed)).w(buf);
        Property::ServerReference(self.server_reference.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buf);
        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let Some(len) = Self::parse_len(buf)? else { return Ok(Self::default()) };
        let mut properties = Self::default();
        let mut data = buf.split_to(len);
        
        loop {
            let property = Property::read(&mut data)?;

            match property {
                Property::SessionExpiryInterval(value) => Self::try_update(&mut properties.session_expiry_interval, value)(property)?,
                Property::ReceiveMaximum(value) => Self::try_update(&mut properties.receive_maximum, value)(property)?,
                Property::MaximumQoS(value) => Self::try_update(&mut properties.maximum_qos, value.map(|x| x != 0))(property)?,
                Property::RetainAvailable(value) => Self::try_update(&mut properties.retain_available, value.map(|x| x!= 0))(property)?,
                Property::MaximumPacketSize(value) => Self::try_update(&mut properties.maximum_packet_size, value)(property)?,
                Property::AssignedClientIdentifier(ref v) => Self::try_update(&mut properties.assigned_client_id, v.as_deref().map(|x| String::from(x)))(property)?,
                Property::TopicAliasMaximum(value) => Self::try_update(&mut properties.topic_alias_maximum, value)(property)?,
                Property::ReasonString(ref v) => Self::try_update(&mut properties.reason_string, v.as_deref().map(|x| String::from(x)))(property)?,
                Property::UserProperty(value) => properties.user_property.push(value.into_owned()),
                Property::WildCardSubscription(value) => Self::try_update(&mut properties.wildcard_subscription_available, value.map(|x| x != 0))(property)?,
                Property::SubscriptionIdentifierAvailable(value) => Self::try_update(&mut properties.subscription_identifiers_available, value.map(|x| x != 0))(property)?,
                Property::SharedSubscriptionAvailable(value) => Self::try_update(&mut properties.shared_subscription_available, value.map(|x| x != 0))(property)?,
                Property::ServerKeepAlive(value) => Self::try_update(&mut properties.server_keep_alive, value)(property)?,
                Property::ResponseInformation(ref v) => Self::try_update(&mut properties.response_information, v.as_deref().map(|x| String::from(x)))(property)?,
                Property::ServerReference(ref v) => Self::try_update(&mut properties.server_reference, v.as_deref().map(|x| String::from(x)))(property)?,
                Property::AuthenticationMethod(ref v) => Self::try_update(&mut properties.authentication_method, v.as_deref().map(|x| String::from(x)))(property)?,
                Property::AuthenticationData(ref v) => Self::try_update(&mut properties.authentication_data, v.to_owned().map(|x| Bytes::from_iter(x.into_owned())))(property)?,
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }


        Ok(properties)
    }
}
