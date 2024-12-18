use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, Default, Length, PartialEq, Eq)]
pub struct AuthProperties {
    pub auth_method: Option<String>,
    pub auth_data: Option<Bytes>,
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

use hivemqtt_macros::FromU8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromU8)]
pub enum AuthReasonCode {
    #[default]
    Success = 0,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
}

impl BufferIO for AuthProperties {
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;
        
        Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed)).w(buf);
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let len = Self::decode(buf)?;
        let mut props = Self::default();
        if len == 0 { return Ok(props) }
        else if len > buf.len() { return Err(MQTTError::IncompleteData("AuthProperties", len, buf.len()))};

        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;
            match property {
                Property::AuthenticationMethod(ref v) => Self::try_update(&mut props.auth_method, v.as_deref().map(String::from))(property)?,
                Property::AuthenticationData(ref value) => Self::try_update(&mut props.auth_data, value.as_deref().map(|x| Bytes::from_iter(x.to_vec())))(property)?,
                Property::ReasonString(ref v) => Self::try_update(&mut props.reason_string, v.as_deref().map(String::from))(property)?,
                Property::UserProperty(v) => props.user_property.push(v.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }
            if data.is_empty() { break; }
        }

        Ok(props)
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len()).unwrap();

        Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed)).w(buf);
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
    }
}


