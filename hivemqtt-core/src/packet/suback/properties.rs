use std::borrow::Cow;

use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};


#[derive(Debug, Length, Default)]
pub struct SubAckProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

impl BufferIO for SubAckProperties {
    fn length(&self) -> usize { self.len() }
    
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;


        let mut properties = Self::default();
        if len == 0 { return Ok(properties) }
        if len > buf.len() { return Err(MQTTError::IncompleteData("SubAckProperties", len, buf.len()))}

        let mut data = buf.split_to(len);

        loop {
            match Property::read(&mut data)? {
                Property::ReasonString(s) => {
                    if properties.reason_string.is_some() { return Err(MQTTError::DuplicateProperty("".to_string())) }
                    properties.reason_string = s.map(|v| String::from(v))
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
