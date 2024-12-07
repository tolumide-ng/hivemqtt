use std::borrow::Cow;

use bytes::{Buf, BufMut, Bytes};
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property, qos::QoS}, traits::{write::ControlPacket, read::Read}};


pub struct  Subscribe {
    packet_identifier: u16,
    properties: SubcribeProperties,
    payload: Vec<(String, SubscriptionOptions)>,
}


impl ControlPacket for Subscribe {
    /// (Length of Variable Header + Length of the Payload)
    fn length(&self) -> usize {
        let mut len = 0;

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
            buf.put_u8(u8::from(*options));
        }
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = SubcribeProperties::read(buf)?;
        Err(MQTTError::MalformedPacket)
    }
}



#[derive(Debug, Length, Default)]
pub struct SubcribeProperties {
    subscription_id: Option<usize>,
    user_property: Vec<(String, String)>,
}

impl ControlPacket for SubcribeProperties {
    fn length(&self) -> usize { self.len() }

    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.length());
        
        if let Some(id) = self.subscription_id {
            Property::SubscriptionIdentifier(Cow::Borrowed(&vec![id])).w(buf);
            Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
        }
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();

        if len == 0 { return Ok(properties) }
        else if buf.len() < len {
            return Err(MQTTError::IncompleteData("SubscribeProperties", len, buf.len()));
        }

        let mut data = buf.split_to(len);

        loop {
            // match 
            if data.is_empty() {
                break;
            }
        }
        
        Err(MQTTError::MalformedPacket)
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


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RetainHandling {
    /// Send the retained messages at the time of the subscribe
    Zero = 0,
    /// Send retained messages at subscribe only if subscription does not currently exist
    One = 1,
    /// Do not send reatined messages at the time of the subscription
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

impl ControlPacket for SubscriptionOptions {
    fn w(&self, buf: &mut bytes::BytesMut) {
        
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
