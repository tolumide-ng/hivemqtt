use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};

/// CONNECT Properties (3.1.2.11)
#[derive(Debug, Clone, Length, Default)]
pub struct ConnectProperties {
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<u16>,
    pub maximum_packet_size: Option<u32>,
    pub topic_alias_maximum: Option<u16>,
    pub request_response_information: Option<u8>,
    pub request_problem_information: Option<u8>,
    pub user_property: Vec<(String, String)>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<Bytes>,
}


impl BufferIO for ConnectProperties {
    /// The length of the Properties in the CONNECT packet Variable Header encoded as a Variable Byte Integer 3.1.2.11.1
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?; // 3.1.2.11.1 (Property Length)
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::RequestResponseInformation(self.request_response_information).w(buf);
        Property::RequestProblemInformation(self.request_problem_information).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
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
                Property::MaximumPacketSize(value) => { Self::try_update(&mut properties.maximum_packet_size, value)(property)?; },
                Property::TopicAliasMaximum(value) => { Self::try_update(&mut properties.topic_alias_maximum, value)(property)?; },
                Property::RequestResponseInformation(value) => { Self::try_update(&mut properties.request_response_information, value)(property)? }
                Property::RequestProblemInformation(value) => { Self::try_update(&mut properties.request_problem_information, value)(property)? }
                Property::UserProperty(value) => { properties.user_property.push(value.into_owned()); }
                Property::AuthenticationMethod(ref value) => { Self::try_update(&mut properties.authentication_method, value.as_deref().map(|x| String::from(x)))(property)? }
                Property::AuthenticationData(ref value) => {
                    Self::try_update(&mut properties.authentication_data, value.to_owned().map(|x| Bytes::from_iter(x.into_owned())))(property)?
                }
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }
            if data.is_empty() { break; }
        }
        
        Ok(properties)
    }
}


