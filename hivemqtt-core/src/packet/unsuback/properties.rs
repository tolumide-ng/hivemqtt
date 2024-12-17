use std::borrow::Cow;

use hivemqtt_macros::Length;

use crate::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, Default, Length)]
pub struct UnSubAckProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

impl BufferIO for UnSubAckProperties {
    fn length(&self) -> usize { self.len() }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();
        if len == 0 { return Ok(properties) };
        if len > buf.len() { return Err(MQTTError::IncompleteData("UnSubAckProperties", len, buf.len())) }

        let mut data = buf.split_to(len);

        loop {
            match Property::read(&mut data)? {
                Property::ReasonString(rs) => properties.reason_string = rs.map(|v| String::from(v)),
                Property::UserProperty(value) => properties.user_property.push(value.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }
        
        Ok(properties)
    }


    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.length());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(up)).w(buf));
    }
}

