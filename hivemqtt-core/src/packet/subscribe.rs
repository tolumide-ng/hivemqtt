use std::borrow::Cow;

use bytes::{Buf, BufMut, Bytes};
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property, qos::QoS}, traits::{write::BufferIO, read::Read}};


pub struct  Subscribe {
    packet_identifier: u16,
    properties: SubcribeProperties,
    payload: Vec<(String, SubscriptionOptions)>,
}


impl BufferIO for Subscribe {
    /// (Length of Variable Header + Length of the Payload)
    fn length(&self) -> usize {
        let mut len = 2 + self.properties.length(); // packet identifier + properties
        len += self.payload.iter().fold(0, |acc, x| acc + (1 + (2 + x.0.len()))); // u8(len) + (string(2) + topic.len())

        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Subscribe as u8 | 1 << 1);
        //  Encoded as Variable Byte Integer
        let _ = Self::write_variable_integer(buf, self.length());
        
        buf.put_u16(self.packet_identifier);
        self.properties.w(buf);

        for (topic, options) in &self.payload {
            self.ws(buf, topic.as_bytes());
            options.w(buf);
        }
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = SubcribeProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            let topic = String::read(buf)?;
            let options = SubscriptionOptions::read(buf)?;

            payload.push((topic, options));

            if buf.is_empty() { break }
        }

        Ok(Self { packet_identifier, properties, payload })
    }
}



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


#[derive(Debug, Clone, Copy)]
pub struct SubscriptionOptions {
    qos: QoS,
    no_local: bool,
    retain_as_published: bool,
    retain_handling: RetainHandling
}

impl From<SubscriptionOptions> for u8 {
    fn from(v: SubscriptionOptions) -> Self {
        u8::from(v.qos) | u8::from(v.no_local) << 2 | u8::from(v.retain_as_published) << 3 | (v.retain_handling as u8) << 4
    }
}

impl BufferIO for SubscriptionOptions {
    fn length(&self) -> usize { 1 }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(*self));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let byte = buf.get_u8();

        let qos = QoS::try_from(byte & 0b0000_0011)?;
        let no_local = (byte & 0b0000_0100) != 0;
        let retain_as_published = (byte & 0b0000_1000) != 0;
        let retain_handling = RetainHandling::try_from(byte & 0b0011_0000)?;

        Ok(Self { qos, no_local, retain_as_published, retain_handling })
    }
}



#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RetainHandling {
    /// Send the retained messages at the time of the subscribe
    Zero = 0,
    /// Send retained messages at subscribe only if subscription does not currently exist
    One = 1,
    /// Do not send retained messages at the time of the subscription
    Two = 2,
}

impl TryFrom<u8> for RetainHandling {
    type Error = MQTTError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(MQTTError::MalformedPacket)
        }
    }
    
}
