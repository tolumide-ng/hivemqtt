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
    // will_topic:
    // will_payload: 
    // username: 
    // password: 
}


struct WillProperties {
    /// The length of the Will Properties (this struct) encoded as Variable Byte Integer
    property_length: usize,
    // (0x18) 24 Byte, default value is zero(0)
    will_delay_interval: u32,
    // (0x01) 1 Byte
    // 0 Byte: indicates that the Will Message is unspecified bytes, which is equivalent to not sending a Payload Format Indicator
    // 1 Byte: indicates that the Will message is UTF-8 Encoded Character Data.
    payload_format_indicator: bool,
}