use hivemqtt_macros::FromU8;

#[derive(Debug, Clone, Copy, FromU8)]
#[repr(u8)]
pub enum UnSubAckReasonCode {
    Success = 0,
    NoSubscriptionExpired = 17,
    UnspecifiedError = 128,
    ImplementationSpecificError =131,
    NotAuthorized = 135,
    TopicFilterInvalid = 143,
    PacketIdentifierInUse = 145,
}