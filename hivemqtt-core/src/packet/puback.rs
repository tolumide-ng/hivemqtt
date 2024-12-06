use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property}, traits::write::ControlPacket};

#[derive(Debug, Length)]
pub(crate) struct PubAck {
    #[bytes(ignore)]
    reason_code: PubAckReasonCode,
    reason_string: Option<String>,
}


impl ControlPacket for PubAck {
    /// Length of the Variable Header, encoded as Variable Byte Integer
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::PubAck));
        let _ = Self::encode_variable_length(buf, self.length());

        Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);

    }
}



#[repr(u8)]
#[derive(Debug)]
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
