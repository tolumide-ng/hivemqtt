use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};


#[derive(Debug, Length, Default)]
pub struct SubcribeProperties {
    subscription_id: Option<usize>,
    user_property: Vec<(String, String)>,
}

impl BufferIO for SubcribeProperties {
    fn length(&self) -> usize { self.len() }

    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.length());
        
        if let Some(id) = self.subscription_id {
            Property::SubscriptionIdentifier(Cow::Borrowed(&id)).w(buf);
            self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        }
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();

        if len == 0 { return Ok(properties) }
        else if len > buf.len() {
            return Err(MQTTError::IncompleteData("SubscribeProperties", len, buf.len()));
        }

        let mut data = buf.split_to(len);

        loop {
            match Property::read(&mut data)? {
                Property::SubscriptionIdentifier(value) => {
                    if properties.subscription_id.is_some() { return Err(MQTTError::DuplicateProperty("".to_string()))}
                    properties.subscription_id = Some(*value);
                }
                Property::UserProperty(value) => {
                    properties.user_property.push(value.into_owned());
                }
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }
            if data.is_empty() { break; }
        }
        
        Ok(properties)
    }
}
