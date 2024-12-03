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
}

impl ControlPacket for Properties {
    /// Length of the properties in the CONNACK packet Variable Header encoded as Variable Byte Integer
    fn length(&self) -> usize {
        0
    }

    fn w(&self, buff: &mut bytes::BytesMut) {
        
    }
}