use std::borrow::Cow;

use hivemqtt_macros::{FromU8, Length};

use super::{BufferIO, Property};

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromU8)]
pub enum PubRelReasonCode {
    Success = 0,
    PacketIdentifierNotFound = 146,
}

#[derive(Debug, Length)]
pub struct PubRelProperties {
    reason_string: Option<String>,
    user_property: Vec<(String, String)>
}

impl BufferIO for PubRelProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::write_variable_integer(buf, self.len());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
    }
}