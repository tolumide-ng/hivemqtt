#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum MQTTError {
    #[error("Payload too long")]
    PayloadTooLong
}