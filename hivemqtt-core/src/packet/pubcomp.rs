use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property}, traits::bufferio::BufferIO};

pub  struct PubComp {
    packet_identifier: u16,
    reason_code: PubCompReasonCode,
    properties: Option<PubCompProperties>,
}

impl BufferIO for PubComp {
    fn length(&self) -> usize {
        let mut len = 2; // packet identifier

        // only add reason code if there's no properties
        if self.reason_code == PubCompReasonCode::Success && self.properties.is_none() {
            return len;
        }
        len += 1; // reason code

        if let Some(ppt) = &self.properties {
            len += ppt.len() + Self::get_variable_length(ppt.len())
        }
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::PubComp));
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);

        if self.reason_code == PubCompReasonCode::Success && self.properties.is_none() {
            return;
        } else {
            let _ = Self::write_variable_integer(buf, 0);
        }
    }
}


#[derive(Debug, Length)]
pub struct PubCompProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>,
}


impl BufferIO for PubCompProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum PubCompReasonCode {
    Success = 0,
    PacketIdentifierNotFound = 146,
}