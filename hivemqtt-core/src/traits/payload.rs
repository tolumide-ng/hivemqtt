use crate::commons::packet_type::PacketType::{self, *};


pub(crate) enum FieldState {
    Required,
    Optional,
    None,
}

/// Payload (Present on some MQTT Control Packets)
/// Last Update: 2.1
pub(crate) trait Payload {
    // the payload in the PUBLISH packet is called `Application Message`
    
    /// Whether or not the packet type requires a `payload`
    fn has_payload(packet: PacketType) -> FieldState {
        match packet {
            Connect | Subscribe | SubAck | UnSubscribe | UnSubAck => FieldState::Required,
            Publish(_) => FieldState::Optional,
            _ => FieldState::None
        }
    }
}