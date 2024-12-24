use std::{borrow::Cow, ops::Deref};

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::commons::error::MQTTError;

use super::{BufferIO, Property};


#[derive(Debug, Length, Default, PartialEq, Eq)]
pub struct SubscribeProperties {
    pub subscription_id: Option<usize>,
    pub user_property: Vec<(String, String)>,
}

impl BufferIO for SubscribeProperties {
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?;

        if let Some(id) = &self.subscription_id { Property::SubscriptionIdentifier(Cow::Borrowed(id)).w(buf); }
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let Some(len) = Self::parse_len(buf)? else { return Ok(Self::default()) };
        let mut props = Self::default();
        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;

            match property {
                Property::SubscriptionIdentifier(ref v) => Self::try_update(&mut props.subscription_id, Some(*v.deref()))(property)?,
                Property::UserProperty(v) => props.user_property.push(v.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }
        Ok(props)
    }

}
