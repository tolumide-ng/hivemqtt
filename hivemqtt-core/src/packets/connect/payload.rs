use bytes::Bytes;
use hivemqtt_macros::DataSize;
use secrecy::SecretString;

/// The CONNECT payload contains 1 or more length-prefixed fields
/// whose presence is determined by the flags in the Variable Header.
pub(crate) struct Payload {
    /// 3.1.3.1
    client_id: String,
    /// 3.1.3.2
    will_properties: WillProperties,
    /// 3.1.3.3
    will_topic: Option<String>,
    /// 3.1.3.4
    will_payload: Option<String>,
    /// 3.1.3.5
    username: Option<String>,
    /// 3.1.3.6
    password: Option<SecretString>,
}


// if the flag topic is set to 1, the will topic is the next field in the Payload.
// The will topic MUST be a UTF-8 encoded string



#[derive(DataSize, Debug, Clone)]
pub(crate) struct WillProperties {
    /// 3.1.3.2.2: Default value is 0
    #[bytes(4)]
    delay_interval: u32,
    /// 3.1.3.2.3
    #[bytes(1)]
    payload_format_indicator: bool,
    /// 3.1.3.2.4
    #[bytes(4)]
    message_expiry_interval: Option<u32>,
    /// 3.1.3.2.5
    #[bytes(wl_2)]
    content_type: Option<String>,
    /// 3.1.3.2.6
    #[bytes(wl_2)]
    response_topic: Option<String>,
    /// 3.1.3.2.7
    #[bytes(wl_2)]
    correlation_data: Option<Bytes>,
    /// 3.1.3.2.8
    #[bytes(wl_2)]
    user_property: Vec<(String, String)>,
}
