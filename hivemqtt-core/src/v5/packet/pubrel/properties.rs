use std::borrow::Cow;

use hivemqtt_macros::{FromU8, Length};

use crate::v5::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromU8, Default)]
pub enum PubRelReasonCode {
    #[default]
    Success = 0,
    PacketIdentifierNotFound = 146,
}

#[derive(Debug, Length, PartialEq, Eq, Default)]
pub struct PubRelProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>
}

impl BufferIO for PubRelProperties {
    fn length(&self) -> usize {self.len()}

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Ok(())
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        let Some(len) = Self::parse_len(buf)? else { return Ok(Self::default()) };
        let mut props = Self::default();
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