mod properties;

use bytes::BufMut;
use properties::Properties;

use crate::{commons::{packets::Packet, variable_byte_integer::{variable_integer, variable_length}}, traits::write::ControlPacket};


#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum ConnAckReasonCode {
    #[default]
    Success = 0,
    UnspecifiedError = 128,
    MalformedPacket = 129,
    ProtocolError = 130,
    ImplementationSpecificError =131,
    UnSupportedProtocolVersion = 132,
    ClientIdentifierNotValid = 133,
    BadUserNameOrPassword = 134,
    NotAuthorized = 135,
    ServerUnAvailable = 136,
    ServerBusy = 137,
    Banned = 138,
    BadAuthenticationMethod = 140,
    TopicNameInvalid = 144,
    PacketTooLarge = 149,
    QuotaExceeded = 151,
    PayloadFormatInvalid = 153,
    RetainNotSupported = 154,
    QoSNotSupported = 155,
    UseAnotherServer = 156,
    ServeMoved = 157,
    ConnectionRateExceeded = 159,
}

pub(crate) struct ConnAck {
    /// 3.2.2.1.1 Connect Acknowledge flag
    session_present: bool, // bit 0 of the COnnect Acknowledge flag
    reason: ConnAckReasonCode,
    properties: Properties,


}


impl ControlPacket for ConnAck {
    /// In this case:
    /// This is the length of the Variable Header
    fn length(&self) -> usize {
        let mut len = 1 + 1; // session present + reason code

        len += self.properties.length();
        len += variable_length(self.properties.length());
        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(Packet::ConnAck));
        let _ = variable_integer(buf, self.length()); // Variable Header encoded as Variable Byte Integer
        buf.put_u8(self.session_present as u8);
        buf.put_u8(self.reason as u8);
        self.properties.w(buf);
    }
}