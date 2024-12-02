use std::borrow::Cow;

use bytes::{BufMut, Bytes, BytesMut};

use crate::traits::write::Write;

/// Must be encoded using the VBI
#[derive(Debug, Clone)]
#[repr(u8)]
pub(crate) enum Property<'a> {
    PayloadFormatIndicator(Option<u8>) = 1,
    MessageExpiryInterval(Option<u32>) = 2,
    ContentType(Option<String>) = 3,
    ResponseTopic(Option<String>) = 8,
    CorrelationData(Option<Bytes>) = 9,
    // this can be a Option<usize> or Vec<usize>, we can create an extra enum for this if there is a need for it.
    SubscriptionIdentifier(Option<usize>) = 11, 
    SessionExpiryInterval(Option<u32>) = 17,
    AssignedClientIdentifier(Option<String>) = 18,
    ServerKeepAlive(Option<u16>) = 19,
    AuthenticationMethod(Option<Cow<'a, str>>) = 21,
    AuthenticationData(Option<Cow<'a, [u8]>>) = 22,
    RequestProblemInformation(Option<u8>) = 23,
    WillDelayInterval(Option<u32>) = 24,
    RequestResponseInformation(Option<u8>) = 25,
    ResponseInformation(Option<String>) = 26,
    ServerReference(Option<String>) = 28,
    ReasonString(Option<String>) = 31,
    ReceiveMaximum(Option<u16>) = 33,
    TopicAliasMaximum(Option<u16>) = 34,
    MaximumQoS(Option<u8>) = 36,
    RetainAvailable(Option<u8>) = 37,
    UserProperty(Cow<'a, Vec<(String, String)>>) = 38,
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



impl<'a> Write for Property<'a> {
    fn w(&self, buf: &mut BytesMut) {
        buf.put_u8(u8::from(self));
        match self {
            Self::SessionExpiryInterval(Some(p)) => buf.put_u32(*p),
            Self::ReceiveMaximum(Some(p)) => buf.put_u16(*p),
            Self::MaximumPacketSize(Some(p)) => buf.put_u32(*p),
            Self::TopicAliasMaximum(Some(p)) => buf.put_u16(*p),
            Self::RequestResponseInformation(Some(p)) => buf.put_u8(*p),
            Self::RequestProblemInformation(Some(p)) => buf.put_u8(*p),
            Self::UserProperty(p) => {
                p.iter().enumerate().for_each(|(index, (k, v))| {
                    if index > 0  { buf.put_u8(u8::from(self));}
                    self.ws(buf, k.as_bytes());
                    self.ws(buf, v.as_bytes());

                });
            }
            Self::AuthenticationMethod(Some(data)) => self.ws(buf, data.as_bytes()),
            Self::AuthenticationData(Some(data)) => self.ws(buf, &data),
            Self::WillDelayInterval(Some(p)) => buf.put_u32(*p),
            Self::PayloadFormatIndicator(Some(p)) => buf.put_u8(*p),
            Self::MessageExpiryInterval(Some(p)) => buf.put_u32(*p),
            Self::ContentType(Some(p)) => self.ws(buf, p.as_bytes()),
            Self::ResponseTopic(Some(p)) => self.ws(buf, p.as_bytes()),
            Self::CorrelationData(Some(p)) => self.ws(buf, p),
            // Self::CorrelationData(Some(p)) => self.ws(buf, p),
            // Self::CorrelationData(Some(p)) => self.ws(buf, p),

            // Self::RequestProblemInformation(Some(_p)) => 
            // }
            _ => {}
        }
    }
}