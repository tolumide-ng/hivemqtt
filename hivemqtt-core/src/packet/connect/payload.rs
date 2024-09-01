use bytes::Bytes;
use hivemqtt_macros::DataSize;

/// The CONNECT payload contains 1 or more length-prefixed fields
/// whose presence is determined by the flags in the Variable Header.
pub(crate) struct Payload {
    /// Identifies the Client to the Server. Each Client connecting 
    /// to the Server has a unique ClientID (required).
    client_identifider: String,
    /// If the will flag is set to 1 (in the variable header),
    /// the Will Properties is the next field in the Payload
    /// Will Properties: contains the Application Message properties
    /// to be sent with the Will Message when it is published, and
    /// properties about when to publish the Will Message. 
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


#[derive(DataSize)]
struct WillProperties {
    #[bytes(4)]
    delay_interval: Option<u32>,
    #[bytes(1)]
    payload_format_indicator: Option<bool>,
    #[bytes(4)]
    message_expiry_interval: Option<u32>,
    #[bytes(wl_2)]
    content_type: Option<String>,
    #[bytes(wl_2)]
    response_topic: Option<String>,
    #[bytes(wl_2)]
    correlation_data: Option<Bytes>,
    #[bytes(wl_2)]
    user_property: Vec<(String, String)>,
}
