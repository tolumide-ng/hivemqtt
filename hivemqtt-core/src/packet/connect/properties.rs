use bytes::Bytes;
use hivemqtt_macros::DataSize;

#[derive(Debug, Clone, DataSize)]
pub(crate) struct ConnectProperties {
    /// 3.1.2.11.2
    #[bytes(4)]
    session_expiry_interval: Option<u32>,
    /// 3.1.2.11.3
    /// Must be greater than zero(0) for QoS1 and QoS2, else there would be a protocol error.
    /// Default is u16::MAX
    #[bytes(2)]
    receive_maximum: Option<u16>,
    /// 3.1.2.11.4
    #[bytes(4)]
    maximum_packet_size: Option<u16>,
    /// 3.1.2.11.5
    #[bytes(2)]
    topic_alias_maximum: Option<u16>,
    /// 3.1.2.11.6
    #[bytes(1)]
    request_response_information: Option<bool>,
    /// 3.1.2.11.7
    #[bytes(1)]
    request_problem_information: Option<bool>,
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