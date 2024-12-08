use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{write::ControlPacket, read::Read}};

pub struct UnSubscribe {
    packet_identifier: u16,
    properties: UnSubscribeProperties,
    payload: Vec<String>,
}

impl ControlPacket for UnSubscribe {
    /// Length of the Variable Header (2 bytes) plus the length of the Payload
    fn length(&self) -> usize {
        // packet identidier + string len
        2 + self.payload.iter().fold(0, |acc, x| acc + x.len() + 2) + self.properties.len() + Self::get_variable_length(self.properties.len())
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8((Packet::UnSubscribe as u8) | 0010);
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);

        self.properties.w(buf);
        self.payload.iter().for_each(|topic| self.ws(buf, topic.as_bytes()));

    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = UnSubscribeProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            payload.push(String::read(buf)?);
            if buf.is_empty() { break; }
        }
        
        Ok(Self { packet_identifier, properties, payload })
    }
}


#[derive(Debug, Length, Default)]
pub struct UnSubscribeProperties {
    user_property: Vec<(String, String)>,
}

impl ControlPacket for UnSubscribeProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());

        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(up)).w(buf));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();

        if len == 0 { return Ok(properties) }
        if len > buf.len() {
            return Err(MQTTError::IncompleteData("UnSubscribeProperties", len, buf.len()));
        }

        let mut data = buf.split_to(len);

        loop {
            match Property::read(&mut data)? {
                Property::UserProperty(value) => properties.user_property.push(value.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }

        Ok(properties)
    }
}