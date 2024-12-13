use std::string::FromUtf8Error;

#[derive(Clone, Debug, thiserror::Error)]
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
    #[error("Multiple instances of {0} Property found")]
    DuplicateProperty(String), // property converted to string
    #[error("Error generating utf-8 string from {0}")]
    Utf8Error(FromUtf8Error),
    #[error("{0} is not allowed on: {1}")]
    UnexpectedProperty(String, String),
    #[error("Version {0} not supported")]
    VersionNotSupported(u8)
}