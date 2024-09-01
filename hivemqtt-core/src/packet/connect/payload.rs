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
    // /// The length of the Will Properties (this struct) encoded as Variable Byte Integer
    // property_length: usize,
    // (0x18) 24 Byte, default value is zero(0)
    #[bytes(4)]
    delay_interval: Option<u32>,
    // 0 Byte: indicates that the Will Message is unspecified bytes, which is equivalent to not sending a Payload Format Indicator
    // 1 Byte: indicates that the Will message is UTF-8 Encoded Character Data.
    #[bytes(1)]
    payload_format_indicator: Option<bool>,
    // 4 Byte integer
    #[bytes(4)]
    message_expiry_interval: Option<u32>,
    /// The value of the Content Type is defined by the sending and receiving application
    content_type: Option<String>,
    /// The presence of a Response Topic identifies the will mesasge as a request
    response_topic: Option<String>,
    // Binary Data
    correlation_data: Option<Bytes>,
    /// This property is intended to provide a means of transferring application layer name-value
    /// tags whose meaning and interpretation are known only by the application prgrams responsible
    /// for sending and receiving them.
    #[bytes(wl(2))]
    user_property: Vec<(String, String)>,
}
