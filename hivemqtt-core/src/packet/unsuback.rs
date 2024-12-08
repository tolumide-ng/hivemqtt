use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{write::ControlPacket, read::Read}};

/// Sent by the Server to the Client to confirm receipt of an UNSUBSCRIBE packet
pub struct UnSubAck {
    packet_identifier: u16,
    properties: UnSubAckProperties,
    payload: Vec<UnSubAckReasonCode>,
}


impl ControlPacket for UnSubAck {
    // Length of the Variable Header plus the length of the Payload 
    fn length(&self) -> usize {
        2 + self.properties.length() + Self::get_variable_length(self.properties.length()) + self.payload.len()
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::UnSubAck as u8);
        let _ = Self::write_variable_integer(buf, self.length());

        self.properties.w(buf);
        self.payload.iter().for_each(|rc| buf.put_u8(*rc as u8));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = UnSubAckProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            payload.push(UnSubAckReasonCode::try_from(u8::read(buf)?)?);
            if buf.is_empty() { break; }
        }


        Ok(Self { packet_identifier, properties, payload })
    }
}



#[derive(Debug, Default, Length)]
pub struct UnSubAckProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}

impl ControlPacket for UnSubAckProperties {
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



#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UnSubAckReasonCode {
    Success = 0,
    NoSubscriptionExpired = 17,
    UnspecifiedError = 128,
    ImplementationSpecificError =131,
    NotAuthorized = 135,
    TopicFilterInvalid = 143,
    PacketIdentifierInUse = 145,
}


impl TryFrom<u8> for UnSubAckReasonCode {
    type Error = MQTTError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            17 => Ok(Self::NoSubscriptionExpired),
            128 => Ok(Self::UnspecifiedError),
            131 => Ok(Self::ImplementationSpecificError),
            135 => Ok(Self::NotAuthorized),
            143 => Ok(Self::TopicFilterInvalid),
            145 => Ok(Self::PacketIdentifierInUse),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }

}