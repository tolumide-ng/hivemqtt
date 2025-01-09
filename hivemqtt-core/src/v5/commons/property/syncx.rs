use std::borrow::Cow;
use std::fmt::Display;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::v5::traits::{syncx::read::Read, syncx::write::Write};
use crate::v5::traits::syncx::bufferio::BufferIO;
use crate::v5::commons::error::MQTTError;

/// Must be encoded using the VBI
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Property<'a> {
    PayloadFormatIndicator(Option<u8>) = 1,
    MessageExpiryInterval(Option<u32>) = 2,
    ContentType(Option<Cow<'a, str>>) = 3,
    ResponseTopic(Option<Cow<'a, str>>) = 8,
    CorrelationData(Option<Cow<'a, [u8]>>) = 9,
    SubscriptionIdentifier(Cow<'a, usize>) = 11, 
    SessionExpiryInterval(Option<u32>) = 17,
    AssignedClientIdentifier(Option<Cow<'a, str>>) = 18,
    ServerKeepAlive(Option<u16>) = 19,
    AuthenticationMethod(Option<Cow<'a, str>>) = 21,
    AuthenticationData(Option<Cow<'a, [u8]>>) = 22,
    RequestProblemInformation(Option<u8>) = 23,
    WillDelayInterval(Option<u32>) = 24,
    RequestResponseInformation(Option<u8>) = 25,
    ResponseInformation(Option<Cow<'a, str>>) = 26,
    ServerReference(Option<Cow<'a, str>>) = 28,
    ReasonString(Option<Cow<'a, str>>) = 31,
    ReceiveMaximum(Option<u16>) = 33,
    TopicAliasMaximum(Option<u16>) = 34,
    TopicAlias(Option<u16>) = 35,
    MaximumQoS(Option<u8>) = 36,
    RetainAvailable(Option<u8>) = 37,
    UserProperty(Cow<'a, (String, String)>) = 38,
    MaximumPacketSize(Option<u32>) = 39,
    WildCardSubscription(Option<u8>) = 40,
    SubscriptionIdentifierAvailable(Option<u8>) = 41,
    SharedSubscriptionAvailable(Option<u8>) = 42,
}

impl<'a> From<&Property<'a>> for u8 {
    fn from(value: &Property) -> Self {
        match value {
            Property::PayloadFormatIndicator(_) => 1,
            Property::MessageExpiryInterval(_) => 2,
            Property::ContentType(_) => 3,
            Property::ResponseTopic(_) => 8,
            Property::CorrelationData(_) => 9,
            Property::SubscriptionIdentifier(_) => 11, 
            Property::SessionExpiryInterval(_) => 17,
            Property::AssignedClientIdentifier(_) => 18,
            Property::ServerKeepAlive(_) => 19,
            Property::AuthenticationMethod(_) => 21,
            Property::AuthenticationData(_) => 22,
            Property::RequestProblemInformation(_) => 23,
            Property::WillDelayInterval(_) => 24,
            Property::RequestResponseInformation(_) => 25,
            Property::ResponseInformation(_) => 26,
            Property::ServerReference(_) => 28,
            Property::ReasonString(_) => 31,
            Property::ReceiveMaximum(_) => 33,
            Property::TopicAliasMaximum(_) => 34,
            Property::TopicAlias(_) => 35,
            Property::MaximumQoS(_) => 36,
            Property::RetainAvailable(_) => 37,
            Property::UserProperty(_) => 38,
            Property::MaximumPacketSize(_) => 39,
            Property::WildCardSubscription(_) => 40,
            Property::SubscriptionIdentifierAvailable(_) => 41,
            Property::SharedSubscriptionAvailable(_) => 42,
        }
    }
}


impl<'a> TryFrom<u8> for Property<'a> {
    type Error = MQTTError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        
        match value {
            1 => Ok(Property::PayloadFormatIndicator(None)),
            2 => Ok(Property::MessageExpiryInterval(None)),
            3 => Ok(Property::ContentType(None)),
            8 => Ok(Property::ResponseTopic(None)),
            9 => Ok(Property::CorrelationData(None)),
            11 => Ok(Property::SubscriptionIdentifier(Cow::Owned(0))),
            17 => Ok(Property::SessionExpiryInterval(None)),
            18 => Ok(Property::AssignedClientIdentifier(None)),
            19 => Ok(Property::ServerKeepAlive(None)),
            21 => Ok(Property::AuthenticationMethod(None)),
            22 => Ok(Property::AuthenticationData(None)),
            23 => Ok(Property::RequestProblemInformation(None)),
            24 => Ok(Property::WillDelayInterval(None)),
            25 => Ok(Property::RequestResponseInformation(None)),
            26 => Ok(Property::ResponseInformation(None)),
            28 => Ok(Property::ServerReference(None)),
            31 => Ok(Property::ReasonString(None)),
            33 => Ok(Property::ReceiveMaximum(None)),
            34 => Ok(Property::TopicAliasMaximum(None)),
            35 => Ok(Property::TopicAlias(None)),
            36 => Ok(Property::MaximumQoS(None)),
            37 => Ok(Property::RetainAvailable(None)),
            38 => Ok(Property::UserProperty(Cow::Owned((String::from(""), String::from(""))))),
            39 => Ok(Property::MaximumPacketSize(None)),
            40 => Ok(Property::WildCardSubscription(None)),
            41 => Ok(Property::SubscriptionIdentifierAvailable(None)),
            42 => Ok(Property::SharedSubscriptionAvailable(None)),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }
}



impl<'a> Property<'a> {
    fn with_id<F>(&self, buf: &mut BytesMut, func: F)
        where F: Fn(&mut BytesMut) {
            u8::from(self).write(buf);
            // buf.put_u8(u8::from(self));
            func(buf);
    }
}


// #[cfg(feature = "sync")]
impl<'a> BufferIO for Property<'a> {
    fn length(&self) -> usize {
        match self {
            Self::SubscriptionIdentifier(length) => **length,
            _ => 0
        }
    }
    fn w(&self, buf: &mut BytesMut) {
        match self {
            Self::SessionExpiryInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::ReceiveMaximum(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::MaximumPacketSize(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::TopicAliasMaximum(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::TopicAlias(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::RequestResponseInformation(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::RequestProblemInformation(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::UserProperty(Cow::Borrowed((ref k, ref v))) => self.with_id(buf, |b| { k.write(b); v.write(b); }),
            Self::AuthenticationMethod(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::AuthenticationData(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.to_vec()).write(b)),
            Self::WillDelayInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::PayloadFormatIndicator(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::MessageExpiryInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::ContentType(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::ResponseTopic(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.as_bytes().to_vec()).write(b)),
            Self::CorrelationData(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.to_vec()).write(b)),
            // NOTE: this needs to be tested for if this method of writing is correct or not!
            Self::SubscriptionIdentifier(_) => self.with_id(buf, |b| { self.encode(b).unwrap() }),
            Self::AssignedClientIdentifier(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::ServerKeepAlive(Some(p)) => self.with_id(buf, |b| p.write(b)),
            Self::ResponseInformation(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::ServerReference(Some(data )) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::ReasonString(Some(data )) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
            Self::MaximumQoS(Some(i)) => self.with_id(buf, |b| i.write(b)),
            Self::RetainAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
            Self::WildCardSubscription(Some(i)) => self.with_id(buf, |b| i.write(b)),
            Self::SubscriptionIdentifierAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
            Self::SharedSubscriptionAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
            // _ => {unreachable!("Unrecognized enum variant or argument!")}
            _ => {}
        }
    }


    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        if buf.is_empty() { return Err(MQTTError::IncompleteData("MQTT Property", 1, 0))}

        match buf.get_u8() {
            1  =>  Ok(Property::PayloadFormatIndicator(Some(u8::read(buf)?))),
            2  =>  Ok(Property::MessageExpiryInterval(Some(u32::read(buf)?))),
            3  =>  Ok(Property::ContentType(Some(Cow::Owned(String::read(buf)?)))),
            8  =>  Ok(Property::ResponseTopic(Some(Cow::Owned(String::read(buf)?)))),
            9  =>  Ok(Property::CorrelationData(Some(Cow::Owned(Bytes::read(buf)?.to_vec())))),
            11 =>  Ok(Property::SubscriptionIdentifier(Cow::Owned(Self::decode(buf)?.0))),
            17 =>  Ok(Property::SessionExpiryInterval(Some(u32::read(buf)?))),
            18 =>  Ok(Property::AssignedClientIdentifier(Some(Cow::Owned(String::read(buf)?)))),
            19 =>  Ok(Property::ServerKeepAlive(Some(u16::read(buf)?))),
            21 =>  Ok(Property::AuthenticationMethod(Some(Cow::Owned(String::read(buf)?)))),
            22 =>  Ok(Property::AuthenticationData(Some(Cow::Owned((Bytes::read(buf)?).to_vec())))),
            23 =>  Ok(Property::RequestProblemInformation(Some(u8::read(buf)?))),
            24 =>  Ok(Property::WillDelayInterval(Some(u32::read(buf)?))),
            25 =>  Ok(Property::RequestResponseInformation(Some(u8::read(buf)?))),
            26 =>  Ok(Property::ResponseInformation(Some(Cow::Owned(String::read(buf)?)))),
            28 =>  Ok(Property::ServerReference(Some(Cow::Owned(String::read(buf)?)))),
            31 =>  Ok(Property::ReasonString(Some(Cow::Owned(String::read(buf)?)))),
            33 =>  Ok(Property::ReceiveMaximum(Some(u16::read(buf)?))),
            34 =>  Ok(Property::TopicAliasMaximum(Some(u16::read(buf)?))),
            35 =>  Ok(Property::TopicAlias(Some(u16::read(buf)?))),
            36 =>  Ok(Property::MaximumQoS(Some(u8::read(buf)?))),
            37 =>  Ok(Property::RetainAvailable(Some(u8::read(buf)?))),
            38 =>  Ok(Property::UserProperty(Cow::Owned((String::read(buf)?, String::read(buf)?)))),
            39 =>  Ok(Property::MaximumPacketSize(Some(u32::read(buf)?))),
            40 =>  Ok(Property::WildCardSubscription(Some(u8::read(buf)?))),
            41 =>  Ok(Property::SubscriptionIdentifierAvailable(Some(u8::read(buf)?))),
            42 =>  Ok(Property::SharedSubscriptionAvailable(Some(u8::read(buf)?))),
            v => Err(MQTTError::UnknownProperty(v))
        }
    }
}



/// this would eventually be changed to use derive_more lib
impl<'a> Display for Property<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this would be changed eventually to use derive_more::Error")
    }
}