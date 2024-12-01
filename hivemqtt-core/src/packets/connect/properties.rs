use bytes::Bytes;
use hivemqtt_macros::DataSize;


// pub(crate) enum Auth {
//     /// (Authentication Method, Authentication Data)
//     MethodAndData(String, Bytes), 
//     NoAuth,
//     /// (Authentication method)
//     AuthMethodOnly(String),
// }

#[derive(Debug, Clone, DataSize)]
pub(crate) struct ConnectProperties {
    /// 3.1.2.11.2
    #[bytes(4)]
    session_expiry_interval: Option<u32>,
    /// 3.1.2.11.3: Default is u16::MAX
    #[bytes(2)]
    receive_maximum: Option<u16>,
    /// 3.1.2.11.4
    #[bytes(4)]
    maximum_packet_size: Option<u32>,
    /// 3.1.2.11.5
    #[bytes(2)]
    topic_alias_maximum: Option<u16>,
    /// 3.1.2.11.6: Default is false(0). If the value is true(1) the server MAY return response information in CONNACK, else it MUST NOT return response information
    #[bytes(1)]
    request_response_information: bool,
    /// 3.1.2.11.7: Default value is true(1).
    /// The Client uses this value to indicate whether the Reason String or User Properties are sent in case of failures
    /// If the value is 0 - The server MUST NOT return a Reason String or User Properties on packets other than PUBLISH, CONNACK or DISCONNECT
    #[bytes(1)]
    request_problem_information: bool,
    /// 3.1.2.11.8
    #[bytes(kv_2)]
    user_property: Vec<(String, String)>,
    /// 3.1.2.11.9
    #[bytes(wl_2)]
    authentication_method: Option<String>,
    /// 3.1.2.11.10
    // do not allow empty bytes here, if the user provides, binary data with a length of 0, just use None, directly
    #[bytes(wl_2)] 
    authentication_data: Option<Bytes>,
}


impl ConnectProperties {
    pub(crate) fn write() {}
}