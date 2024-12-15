use std::borrow::Cow;

use hivemqtt_macros::{FromU8, Length};

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, PartialEq, Eq, Default, FromU8, Clone, Copy)]
pub enum PubCompReasonCode {
    #[default]
    Success = 0,
    PacketIdentifierNotFound = 146,
}

#[derive(Debug, Length, Default, PartialEq, Eq, Clone)]
pub struct PubCompProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}


impl BufferIO for PubCompProperties {
    fn length(&self) -> usize {self.len()}

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        self.encode(buf)?;

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let len = Self::decode(buf)?;
        let mut props = Self::default();

        if len == 0 { return Ok(props) }
        else if len > buf.len() { return Err(MQTTError::IncompleteData("PubRecProperties", len, buf.len()))};

        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;

            match property {
                Property::ReasonString(ref v) => Self::try_update(&mut props.reason_string, v.as_deref().map(String::from))(property)?,
                Property::UserProperty(value) => props.user_property.push(value.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            };

            if data.is_empty() { break; }
        }

        Ok(props)
    }
}