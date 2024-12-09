use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property}, traits::write::BufferIO};

#[derive(Debug)]
pub(crate) struct PubAck {
    packet_identifier: u16,
    reason_code: PubAckReasonCode,
    properties: Option<PubAckProperties>,
}


impl BufferIO for PubAck {
    /// Length of the Variable Header, encoded as Variable Byte Integer
    fn length(&self) -> usize {
        let mut len = 2; // packet identifier

        // only add reason code if there's no properties
        if self.reason_code == PubAckReasonCode::Success && self.properties.is_none() {
            return len;
        }
        len += 1; // reason code

        if let Some(ppt) = &self.properties {
            len += ppt.len() + Self::get_variable_length(ppt.len())
        }
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::PubAck));
        let _ = Self::write_variable_integer(buf, self.length());
        
        buf.put_u16(self.packet_identifier);

        if self.reason_code == PubAckReasonCode::Success && self.properties.is_none() {
            return;
        }

        buf.put_u8(self.reason_code as u8);

        if let Some(ppts) = &self.properties {
            ppts.w(buf);
        } else {
            let _ = Self::write_variable_integer(buf, 0);
        }
    }
}


#[derive(Debug, Length)]
pub(crate) struct PubAckProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}

impl BufferIO for PubAckProperties {
    
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());
        
        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
    }
}




#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum PubAckReasonCode {
    Success = 0,
    NoMatchingSubscribers = 16,
    UnspecifiedError = 128,
    ImplementationSpecificError = 131,
    NotAuthorized = 135,
    TopicNameInvalid = 144,
    PacketIdentifierInUse = 145,
    QuotaExceeded = 151,
    PayloadFormatInvalid = 153,
}
