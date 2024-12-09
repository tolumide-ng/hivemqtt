use std::borrow::Cow;

use bytes::{BufMut, Bytes};
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read}};

pub struct Auth {
    reason_code: AuthReasonCode,
    properties: AuthProperties,
}

impl BufferIO for Auth {
    fn length(&self) -> usize {
        1 + 1 + self.properties.length()
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Auth as u8);
        let _ = Self::write_variable_integer(buf, self.length());

        if self.reason_code == AuthReasonCode::Success && self.properties.len() == 0 {
            return;
        }

        buf.put_u8(self.reason_code as u8);
        self.properties.w(buf);
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let reason_code = AuthReasonCode::try_from(u8::read(buf)?)?;
        let properties = AuthProperties::read(buf)?;

        Ok(Self { reason_code, properties })
    }
}


#[derive(Debug, Default, Length)]
pub struct AuthProperties {
    auth_method: Option<String>,
    auth_data: Option<Bytes>,
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}

impl BufferIO for AuthProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len()).unwrap();

        Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed)).w(buf);
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
    }

    fn read(buf: &mut Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();
        if len == 0 { return Ok(properties) }
        if len > buf.len() { return Err(MQTTError::IncompleteData("SubAckProperties", len, buf.len()))}

        let mut data = buf.split_to(len);

        loop {
            match Property::read(&mut data)? {
                Property::AuthenticationMethod(value) => {
                    if properties.auth_method.is_some() { return Err(MQTTError::DuplicateProperty("".to_string())) }
                    properties.auth_method = value.map(|x| String::from(x))
                },
                Property::AuthenticationData(value) => {
                    if properties.auth_data.is_some() { return Err(MQTTError::DuplicateProperty("".to_string())) }
                    properties.auth_data = value.map(|x| Bytes::from_iter(x.into_owned()))
                },
                Property::ReasonString(value) => {
                    if properties.reason_string.is_some() { return Err(MQTTError::DuplicateProperty("".to_string())) }
                    properties.reason_string = value.map(|x| String::from(x))
                },
                Property::UserProperty(value) => { properties.user_property.push(value.into_owned()) },
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }

        Ok(properties)
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AuthReasonCode {
    #[default]
    Success = 0,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
}

impl TryFrom<u8> for AuthReasonCode {
    type Error = MQTTError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            24 => Ok(Self::ContinueAuthentication),
            25 => Ok(Self::ReAuthenticate),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }
}