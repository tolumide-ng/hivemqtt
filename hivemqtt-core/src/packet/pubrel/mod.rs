mod properties;
pub use properties::{PubRelProperties, PubRelReasonCode};

use bytes::BufMut;

use crate::{commons::{packets::Packet, property::Property}, traits::bufferio::BufferIO};

pub struct PubRel {
    packet_identifier: u16,
    reason_code: PubRelReasonCode,
    properties: Option<PubRelProperties>,
}

impl BufferIO for PubRel {
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