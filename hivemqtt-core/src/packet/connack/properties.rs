use bytes::Bytes;
use hivemqtt_macros::DataSize;

use super::ControlPacket;

#[derive(Debug, DataSize)]
pub(crate) struct Properties {
    #[bytes(4)]
    session_expiry_interval: Option<u32>,
    #[bytes(2)]
    receive_maximum: Option<u16>,
    #[bytes(1)]
    maximum_qos: Option<bool>,
    #[bytes(1)]
    retain_available: Option<bool>,
    #[bytes(4)]
    maximum_packet_size: Option<u32>,
    #[bytes(wl_2)]
    assigned_client_id: Option<String>,
    #[bytes(2)]
    topic_alias_maximum: Option<u16>,
    #[bytes(wl_2)]
    reason_string: Option<String>,
    #[bytes(kv_2)]
    user_property: Vec<(String, String)>,
    #[bytes(1)]
    wildcard_subscription_available: Option<bool>,
    #[bytes(1)]
    subscription_identifiers_available: Option<bool>,
    #[bytes(1)]
    shared_subscription_available: Option<bool>,
    #[bytes(2)]
    server_keep_alive: Option<u16>,
    #[bytes(wl_2)]
    response_information: Option<String>,
    #[bytes[wl_2]]
    server_reference: Option<String>,
    #[bytes[wl_2]]
    authentication_method: Option<String>,
    
    authentication_data: Option<Bytes>
}


// The server uses this value to give additional information to the Client. 
//      

impl ControlPacket for Properties {
    /// Length of the properties in the CONNACK packet Variable Header encoded as Variable Byte Integer
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buff: &mut bytes::BytesMut) {
        
    }
}