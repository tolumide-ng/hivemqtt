#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum MQTTError {
    #[error("Malformed mqtt packet")]
    MalformedPacket = 0x81,
    #[error("Payload too long")]
    PayloadTooLong,
    #[error("Incomplete Packet")]
    IncompletePacket,
}