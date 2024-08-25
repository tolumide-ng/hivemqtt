#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum MQTTError {
    #[error("Payload too long")]
    PayloadTooLong,
    #[error("Malformed mqtt packet")]
    MalformedPacket,
    #[error("Incomplete Packet")]
    IncompletePacket,
}