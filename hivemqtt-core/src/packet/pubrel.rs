use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property}, traits::write::ControlPacket};

pub struct PubRel {
    packet_identifier: u16,
    reason_code: PubRelReasonCode,
    properties: Option<PubRelProperties>,
}

impl ControlPacket for PubRel {
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::PubRel) | 1 << 1);
        let _ = Self::write_variable_integer(buf, self.length());

        buf.put_u16(self.packet_identifier);

        if self.reason_code == PubRelReasonCode::Success && self.properties.is_none() {
            return;
        }

        buf.put_u8(self.reason_code as u8);
        if let Some(ppt) = &self.properties {
            ppt.w(buf);
        } else {
            let _ = Self::write_variable_integer(buf, 0);
        }
    }
}


#[derive(Debug, Length)]
pub struct PubRelProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>
}

impl ControlPacket for PubRelProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PubRelReasonCode {
    Success = 0,
    PacketIdentifierNotFound = 146,
}