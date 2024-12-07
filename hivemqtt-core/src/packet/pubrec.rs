use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property}, traits::write::ControlPacket};

pub struct PubRec {
    packet_identifier: u16,
    reason_code: PubRecReasonCode,
    properties: Option<PubRecProperties>,
}

impl ControlPacket for PubRec {
    fn length(&self) -> usize {
        let mut len = 0;
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::PubRec));
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);

        if self.reason_code == PubRecReasonCode::Success && self.properties.is_none() {
            return;
        }

        buf.put_u8(self.reason_code as u8);
        if let Some(ppts) = &self.properties {
            ppts.w(buf);
        } else {
            let _ = Self::write_variable_integer(buf, 0);
        };

    }
}


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PubRecReasonCode {
    Success = 0,
    NoMatchingSubscribers = 16,
    UnspecifiedError = 128,
    ImplementationSpecificError = 131,
    NotAuthorized = 135,
    TopicNameInvalid = 144,
    PacketIdentifierInUse = 145,
    QuotaExceeded = 151,
    PayloadFormatIndicator = 153,
}


#[derive(Debug, Length)]
pub struct PubRecProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}

impl ControlPacket for PubRecProperties {
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.length());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
    }
}