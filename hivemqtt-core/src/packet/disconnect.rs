use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property}, traits::{bufferio::BufferIO, read::Read}};


pub struct Disconnect {
    reason_code: DisconnectReasonCode,
    properties: DisconnectProperties,
}

impl BufferIO for Disconnect {
    // the length of the dsiconnect variable header
    fn length(&self) -> usize {
         // The Reason Code and Property Length can be omitted if the Reason Code is 0x00 (Normal disconnecton) and there are no Properties. In this case the DISCONNECT has a Remaining Length of 0.
        if self.reason_code == DisconnectReasonCode::NormalDisconnection && self.properties.len() == 0 {
            return 0;
        }
        return self.properties.len() + Self::get_variable_length(self.properties.len()) + 1 // 1 is for the reason code abovegit stat
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Disconnect as u8);
        let _ = Self::write_variable_integer(buf, self.length());
        
        if self.reason_code == DisconnectReasonCode::NormalDisconnection && self.properties.len() == 0 {
            return;
        }
        
        buf.put_u8(self.reason_code as u8);
        self.properties.w(buf);
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        // the assumption here is that the provided buffer has already been advanced by the Fixed Header length
        let reason_code = DisconnectReasonCode::try_from(u8::read(buf)?)?;
        let properties = DisconnectProperties::read(buf)?;

        Ok(Self { reason_code, properties })
    }
}


#[derive(Debug, Length, Default)]
pub struct DisconnectProperties {
    session_expiry_interval: Option<u32>,
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
    server_reference: Option<String>,
}

impl BufferIO for DisconnectProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|up| Property::UserProperty(Cow::Borrowed(up)).w(buf));
        Property::ServerReference(self.server_reference.as_deref().map(Cow::Borrowed)).w(buf);
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let (len, _) = Self::read_variable_integer(buf)?;

        let mut properties = Self::default();
        if len == 0 { return Ok(properties) }
        if len > buf.len() { return Err(MQTTError::IncompleteData("DisconnectProperties", len, buf.len()))}

        let mut data = buf.split_to(len);
        loop {
            match Property::read(&mut data)? {
                Property::SessionExpiryInterval(value) => {
                    if properties.session_expiry_interval.is_some() { return Err(MQTTError::DuplicateProperty("".to_string()))};
                    properties.session_expiry_interval = value},
                Property::ReasonString(value) => {
                    if properties.reason_string.is_some() { return Err(MQTTError::DuplicateProperty("".to_string()))};
                    properties.reason_string = value.map(|x| String::from(x))
                },
                Property::ServerReference(value) => {
                    if properties.server_reference.is_some() { return Err(MQTTError::DuplicateProperty("".to_string()))};
                    properties.server_reference = value.map(|x| x.into_owned())
                },
                Property::UserProperty(value) => properties.user_property.push(value.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }


        Ok(properties)
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DisconnectReasonCode {
    #[default]
    NormalDisconnection = 0,
    DisconnectWithWillMessage = 4,
    UnspecifiedError = 128,
    MalformedPacket = 129,
    ProtocolError = 130,
    ImplementationSpecificError = 131,
    NotAuthorized = 135,
    ServerBusy = 137,
    ServerShuttingDown = 139,
    KeepAliveTimeout = 141,
    SessionTakenOver = 142,
    TopicFilterInvalid = 143,
    TopicNameInvalid = 144,
    ReceiveMaximumExceeded = 147,
    TopicAliasInvalid = 148,
    PacketTooLarge = 149,
    MessageRateTooHigh = 150,
    QuotaExceeded = 151,
    AdministrativeAction = 152,
    PayloadFormatIndicator = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServerMoved = 157,
    SharedSubscriptionsNotSupported = 158,
    ConnectionRateExceeded = 159,
    MaximumConnectTime = 160,
    SubscriptionIdentifiers = 161,
    WildcardSubscriptionsNotSupported = 162,
}


impl TryFrom<u8> for DisconnectReasonCode {
    type Error = MQTTError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NormalDisconnection),
            4 => Ok(Self::DisconnectWithWillMessage),
            128 => Ok(Self::UnspecifiedError),
            129 => Ok(Self::MalformedPacket),
            130 => Ok(Self::ProtocolError),
            131 => Ok(Self::ImplementationSpecificError),
            135 => Ok(Self::NotAuthorized),
            137 => Ok(Self::ServerBusy),
            139 => Ok(Self::ServerShuttingDown),
            141 => Ok(Self::KeepAliveTimeout),
            142 => Ok(Self::SessionTakenOver),
            143 => Ok(Self::TopicFilterInvalid),
            144 => Ok(Self::TopicNameInvalid),
            147 => Ok(Self::ReceiveMaximumExceeded),
            148 => Ok(Self::TopicAliasInvalid),
            149 => Ok(Self::PacketTooLarge),
            150 => Ok(Self::MessageRateTooHigh),
            151 => Ok(Self::QuotaExceeded),
            152 => Ok(Self::AdministrativeAction),
            153 => Ok(Self::PayloadFormatIndicator),
            154 => Ok(Self::RetainNotSupported),
            155 => Ok(Self::QoSNotSupported),
            156 => Ok(Self::UseAnotherServer),
            157 => Ok(Self::ServerMoved),
            158 => Ok(Self::SharedSubscriptionsNotSupported),
            159 => Ok(Self::ConnectionRateExceeded),
            160 => Ok(Self::MaximumConnectTime),
            161 => Ok(Self::SubscriptionIdentifiers),
            162 => Ok(Self::WildcardSubscriptionsNotSupported),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }
}