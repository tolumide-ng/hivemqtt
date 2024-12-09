use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{write::BufferIO, read::Read}};

/// 3.9: Sent by the Server to the Client to confirm receipt and processing of a SUBSCRIBE packet.
pub struct SubAck {
    packet_identifier: u16,
    payload: Vec<SubAckReasonCode>,
    properties: SubAckProperties,
}

impl BufferIO for SubAck {
    // Length of the Variable Header plus the length of the Payload
    fn length(&self) -> usize {
        // packet identifier + ...
        2 + self.payload.len() + self.properties.len() + Self::get_variable_length(self.properties.len())
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::SubAck));
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);
        self.properties.w(buf);
        buf.extend_from_slice(&(self.payload.iter().map(|x| *x as u8).collect::<Vec<u8>>()));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let packet_identifier = u16::read(buf)?;
        let properties = SubAckProperties::read(buf)?;
        let mut payload = Vec::new();

        loop {
            payload.push(SubAckReasonCode::try_from(u8::read(buf)?)?);

            if buf.is_empty() { break; }
        }
        
        
        Ok(Self { packet_identifier, payload, properties })
    }
}


#[derive(Debug, Length, Default)]
pub struct SubAckProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}

impl BufferIO for SubAckProperties {
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



#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SubAckReasonCode {
    /// The subscription is accepted and the maximum Qos sent will be QoS1 (This might be lower than requested)
    GrantedQos0 = 0,
    /// The Subscription is accepted and the maximum QoS sent will be QoS1 (This might be lower than requested)
    GrantedQoS1 = 1,
    /// The subscription is accepted and any received QoS will be sent to this subscription
    GrantedQoS2 = 2,
    UnspecifiedError = 128,
    /// Subscribe packet is valid, but the server does not accept it
    ImplementationSpecificError = 131,
    NotAuhtorized = 135,
    TopicFilterInvalid = 143,
    PacketIdentifierInUse = 145,
    QuotaExceeded = 151,
    SharedSubscriptionNotSupported = 158,
    SubscriptionIdentifiersNotSupported = 161,
    WildCardSubscriptionNotSupported = 162,
}


impl TryFrom<u8> for SubAckReasonCode {
    type Error = MQTTError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubAckReasonCode::GrantedQos0),
            1 => Ok(SubAckReasonCode::GrantedQoS1),
            2 => Ok(SubAckReasonCode::GrantedQoS2),
            128 => Ok(SubAckReasonCode::UnspecifiedError),
            131 => Ok(SubAckReasonCode::ImplementationSpecificError),
            135 => Ok(SubAckReasonCode::NotAuhtorized),
            143 => Ok(SubAckReasonCode::TopicFilterInvalid),
            145 => Ok(SubAckReasonCode::PacketIdentifierInUse),
            151 => Ok(SubAckReasonCode::QuotaExceeded),
            158 => Ok(SubAckReasonCode::SharedSubscriptionNotSupported),
            161 => Ok(SubAckReasonCode::SubscriptionIdentifiersNotSupported),
            162 => Ok(SubAckReasonCode::WildCardSubscriptionNotSupported),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }
}