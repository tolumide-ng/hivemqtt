use std::borrow::Cow;

use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};


#[derive(Debug, Length, Default, PartialEq, Eq)]
pub struct DisconnectProperties {
    session_expiry_interval: Option<u32>,
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
    server_reference: Option<String>,
}

impl BufferIO for DisconnectProperties {
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
        Property::ServerReference(self.server_reference.as_deref().map(Cow::Borrowed)).w(buf);
        Ok(())
    }


    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        let len = Self::decode(buf)?;
        let mut props = Self::default();
        if len == 0 { return Ok(props) }
        else if len > buf.len() { return Err(MQTTError::IncompleteData("DisconnectProperties", len, buf.len()))};

        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;
            match property {
                Property::ReasonString(ref v) => Self::try_update(&mut props.reason_string, v.as_deref().map(String::from))(property)?,
                Property::UserProperty(v) => props.user_property.push(v.into_owned()),
                Property::SessionExpiryInterval(v) => Self::try_update(&mut props.session_expiry_interval, v)(property)?,
                Property::ServerReference(ref v) => Self::try_update(&mut props.server_reference, v.as_deref().map(String::from))(property)?,
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }

        Ok(props)
    }
}