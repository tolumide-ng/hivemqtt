use bytes::Bytes;
use hivemqtt_macros::DataSize;

/// The CONNECT payload contains 1 or more length-prefixed fields
/// whose presence is determined by the flags in the Variable Header.
pub(crate) struct Payload {
    // todo: Add utility method to enable generation of random id's when [rnadom] feature flag is enabled
    client_identifider: String,
    will_properties: WillProperties,
    /// If the will_flag is set to 1 (connect flags of the CONNECT variable header),
    /// then this property must be provided
    will_topic: Option<String>,
    /// If the will_flag is set to 1 (connect flags of the CONNECT variable header),
    /// then this property must be provided
    will_payload: Option<String>,
    /// If the username flag is set to 1 (connect flags of the CONNECT variable header),
    /// then this property must be provided
    /// Can be used by the Server for authentication and authorization
    username: Option<String>,
    /// If the password flag is set to 1 (connect flags of the CONNECT variable header),
    /// then this property must be provided
    /// MUST BE BINARY DATA
    password: Option<String>,
}


#[derive(DataSize, Debug, Clone)]
pub(crate) struct WillProperties {
    /// 3.1.3.2.2
    #[bytes(4)]
    delay_interval: Option<u32>,
    /// 3.1.3.2.3
    #[bytes(1)]
    payload_format_indicator: Option<bool>,
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
