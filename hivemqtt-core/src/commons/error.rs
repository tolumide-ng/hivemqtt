#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum MQTTError {
    #[error("Malformed mqtt packet")]
    MalformedPacket,
    #[error("Payload too long")]
    PayloadTooLong,
    #[error("Incomplete Packet")]
    IncompletePacket,
    #[error("Received QoS: {0} which is unsupported")]
    UnsupportedQoS(u8),
    #[error("Incomplete Data: {0} Expected {1} bytes but found {2}")]
    IncompleteData(&'static str, usize, usize),
    #[error("Received an Unknown Property: {0}")]
    UnknownProperty(u8),
}